#!/usr/bin/env python3
"""Collect and download Java ME games and programs from Spaces.im.

The parser follows the same workflow as the Symbian ``spaces.rs`` tool next to
this project: build a manifest first, then download into a resumable corpus
with atomic ``.part`` files, hashes, metadata, and a progress report.

Only the standard library is used so the tool can be run on a clean checkout:

    python tools/spaces_java_parser.py corpus --target 500
    python tools/spaces_java_parser.py catalog --section programs --all-pages

The default output is ``apk/spaces-java``.  The source site currently puts a
small JavaScript gate in front of HTML pages.  ``js_passed=2`` is the cookie
the page itself sets after the gate completes; it is sent only for Spaces.im
HTML requests.  Binary files are downloaded directly from the file host link
published by each file page.  If ``IPV4.txt`` exists in the working directory,
its HTTP proxies are rotated between requests and retries; the accepted format
is ``host:port:user:password`` (HTTP/HTTPS proxy URLs are also accepted).
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import html as html_lib
import json
import os
import re
import sys
import tempfile
import threading
import time
import unicodedata
import zipfile
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import asdict, dataclass, field
from html.parser import HTMLParser
from pathlib import Path
from typing import Iterable, Iterator
from urllib.error import HTTPError, URLError
from urllib.parse import quote, unquote, urljoin, urlsplit, urlunsplit
from urllib.request import ProxyHandler, Request, build_opener, urlopen


ROOT_URL = "https://spaces.im/sz/igry/java-igry/"
SOURCE_URL = "https://spaces.im/sz/igry/java-igry/?Link_id=1693820"
PROGRAMS_ROOT_URL = "https://spaces.im/sz/programmy/java/"
PROGRAMS_SOURCE_URL = "https://spaces.im/sz/programmy/java/?Link_id=1390628"
DEFAULT_OUTPUT = Path("apk") / "spaces-java"
DEFAULT_PROXY_FILE = Path("IPV4.txt")
DEFAULT_COOKIE = "js_passed=2"
DEFAULT_USER_AGENT = "rust-java-spaces-java-parser/0.1"

DEFAULT_DELAY_MS = 250
REQUEST_ATTEMPTS = 4
REQUEST_TIMEOUT_SECONDS = 45
MAX_HTML_BYTES = 8 * 1024 * 1024
MAX_DOWNLOAD_BYTES = 1024 * 1024 * 1024
CHUNK_BYTES = 64 * 1024
REPORT_CHECKPOINT_EVERY = 25


@dataclass(frozen=True)
class Category:
    slug: str
    title: str
    url: str


def category_url(slug: str) -> str:
    return f"{ROOT_URL}{slug}/?Link_id=1693820"


CATEGORIES = (
    Category("alcatel", "Alcatel", category_url("alcatel")),
    Category("benq-siemens", "BenQ-Siemens", category_url("benq-siemens")),
    Category("fly", "Fly", category_url("fly")),
    Category("lg", "LG", category_url("lg")),
    Category("motorola", "Motorola", category_url("motorola")),
    Category("nokia", "Nokia", category_url("nokia")),
    Category("samsung", "Samsung", category_url("samsung")),
    Category("sony-ericsson", "Sony Ericsson", category_url("sony-ericsson")),
)

ALL_GAMES_CATEGORY = Category("all-games", "All Java games", SOURCE_URL)
JAVA_PROGRAMS_CATEGORY = Category("java-programs", "Java programs", PROGRAMS_SOURCE_URL)

CATALOG_SECTIONS = {
    "devices": CATEGORIES,
    "games": (ALL_GAMES_CATEGORY,),
    "programs": (JAVA_PROGRAMS_CATEGORY,),
    "all": (JAVA_PROGRAMS_CATEGORY, ALL_GAMES_CATEGORY),
}

SECTION_ALIASES = {
    "device": "devices",
    "device-games": "devices",
    "game": "games",
    "program": "programs",
}

CATEGORY_ALIASES = {
    "benq": "benq-siemens",
    "siemens": "benq-siemens",
    "sony": "sony-ericsson",
    "se": "sony-ericsson",
}


@dataclass
class CatalogEntry:
    category: str
    category_title: str
    id: int
    title: str
    extension: str
    view_url: str
    source_page: str
    categories: list[str] = field(default_factory=list)

    def as_dict(self) -> dict[str, object]:
        value = asdict(self)
        if not value["categories"]:
            value["categories"] = [self.category]
        return value


@dataclass
class DownloadCard:
    title: str
    extension: str
    download_url: str
    content_size: str | None
    description: str


@dataclass
class JarInfo:
    class_count: int
    entry_count: int
    midlet_name: str | None = None
    midlet_class: str | None = None
    midlet_vendor: str | None = None
    configuration: str | None = None
    profile: str | None = None


class SpacesError(RuntimeError):
    """An error that includes enough context to continue with the next file."""


class ChallengePageError(SpacesError):
    pass


class InvalidJarError(SpacesError):
    pass


def parse_proxy_line(raw: str) -> str | None:
    """Normalize a proxy-file line to a urllib-compatible proxy URL.

    The proxy list commonly uses ``host:port:user:password``.  We also accept
    ``host:port``, ``user:password@host:port`` and explicit HTTP/HTTPS URLs.
    Blank lines and comments are ignored.
    """

    value = raw.split("#", 1)[0].strip()
    if not value:
        return None

    if "://" not in value:
        parts = value.split(":", 3)
        if len(parts) == 4:
            host, port, username, password = parts
            value = (
                "http://"
                f"{quote(username, safe='')}:{quote(password, safe='')}@{host}:{port}"
            )
        else:
            value = f"http://{value}"

    parsed = urlsplit(value)
    scheme = parsed.scheme.lower()
    if scheme not in {"http", "https"}:
        raise ValueError("only HTTP and HTTPS proxies are supported")
    if not parsed.hostname:
        raise ValueError("proxy host is missing")
    try:
        port = parsed.port
    except ValueError as error:
        raise ValueError("proxy port is invalid") from error
    if port is None or not 1 <= port <= 65535:
        raise ValueError("proxy port must be between 1 and 65535")
    return urlunsplit((scheme, parsed.netloc, "", "", ""))


def load_proxy_file(path: Path) -> list[str]:
    """Read, validate and de-duplicate proxies without exposing credentials."""

    try:
        lines = path.read_text(encoding="utf-8").splitlines()
    except OSError as error:
        raise SpacesError(f"cannot read proxy file {path}: {error}") from error

    proxies: list[str] = []
    seen: set[str] = set()
    for line_number, raw in enumerate(lines, start=1):
        try:
            proxy = parse_proxy_line(raw)
        except ValueError as error:
            print(
                f"[proxy] ignoring invalid line {line_number}: {error}",
                file=sys.stderr,
            )
            continue
        if proxy and proxy not in seen:
            seen.add(proxy)
            proxies.append(proxy)
    if not proxies:
        raise SpacesError(f"proxy file has no valid proxies: {path}")
    return proxies


class ProxyPool:
    """Thread-safe round-robin proxy selection and opener cache."""

    def __init__(self, proxies: Iterable[str]) -> None:
        self.proxies = tuple(dict.fromkeys(proxies))
        self._next_index = 0
        self._selection_lock = threading.Lock()
        self._openers: dict[str, object] = {}
        self._openers_lock = threading.Lock()

    def next(self) -> str | None:
        if not self.proxies:
            return None
        with self._selection_lock:
            proxy = self.proxies[self._next_index]
            self._next_index = (self._next_index + 1) % len(self.proxies)
        return proxy

    def opener_for(self, proxy: str) -> object:
        opener = self._openers.get(proxy)
        if opener is not None:
            return opener
        with self._openers_lock:
            opener = self._openers.get(proxy)
            if opener is None:
                opener = build_opener(ProxyHandler({"http": proxy, "https": proxy}))
                self._openers[proxy] = opener
        return opener


class SpacesClient:
    def __init__(
        self,
        *,
        cookie: str = DEFAULT_COOKIE,
        user_agent: str = DEFAULT_USER_AGENT,
        timeout: int = REQUEST_TIMEOUT_SECONDS,
        proxies: Iterable[str] = (),
    ) -> None:
        self.cookie = cookie
        self.user_agent = user_agent
        self.timeout = timeout
        self.proxy_pool = ProxyPool(proxies)

    @property
    def proxy_count(self) -> int:
        return len(self.proxy_pool.proxies)

    def _headers(self, url: str, *, referer: str | None = None) -> dict[str, str]:
        headers = {
            "Accept": "text/html,application/xhtml+xml,application/java-archive;q=0.9,*/*;q=0.1",
            "Accept-Encoding": "identity",
            "Accept-Language": "ru,en;q=0.8",
            "User-Agent": self.user_agent,
        }
        if self.cookie and urlsplit(url).hostname in {"spaces.im", "www.spaces.im"}:
            headers["Cookie"] = self.cookie
        if referer:
            headers["Referer"] = referer
        return headers

    def _open(self, request: Request) -> object:
        proxy = self.proxy_pool.next()
        if proxy is None:
            return urlopen(request, timeout=self.timeout)
        opener = self.proxy_pool.opener_for(proxy)
        return opener.open(request, timeout=self.timeout)  # type: ignore[union-attr]

    def fetch_bytes(
        self,
        url: str,
        *,
        max_bytes: int,
        expect_html: bool = False,
        referer: str | None = None,
    ) -> bytes:
        last_error: Exception | None = None
        for attempt in range(1, REQUEST_ATTEMPTS + 1):
            try:
                request = Request(url, headers=self._headers(url, referer=referer), method="GET")
                with self._open(request) as response:  # type: ignore[union-attr]
                    content_length = response.headers.get("Content-Length")
                    if content_length:
                        try:
                            if int(content_length) > max_bytes:
                                raise SpacesError(
                                    f"response is larger than the {max_bytes} byte limit"
                                )
                        except ValueError:
                            pass
                    data = read_limited(response, max_bytes)
                if expect_html and is_challenge_page(data):
                    raise ChallengePageError(
                        "Spaces.im returned its JavaScript gate; try a fresh cookie with --cookie"
                    )
                return data
            except ChallengePageError:
                raise
            except (HTTPError, URLError, TimeoutError, OSError, SpacesError) as error:
                last_error = error
                if attempt != REQUEST_ATTEMPTS:
                    time.sleep(0.5 * attempt)
        detail = str(last_error) if last_error else "unknown HTTP error"
        raise SpacesError(f"request failed after {REQUEST_ATTEMPTS} attempts: {url}: {detail}")

    def fetch_text(self, url: str, *, referer: str | None = None) -> str:
        data = self.fetch_bytes(
            url,
            max_bytes=MAX_HTML_BYTES,
            expect_html=True,
            referer=referer,
        )
        return decode_html(data)

    def download_file(
        self,
        url: str,
        destination: Path,
        *,
        referer: str | None = None,
    ) -> tuple[int, str, JarInfo]:
        destination.parent.mkdir(parents=True, exist_ok=True)
        temporary = destination.with_name(destination.name + ".part")
        last_error: Exception | None = None

        for attempt in range(1, REQUEST_ATTEMPTS + 1):
            try:
                request = Request(url, headers=self._headers(url, referer=referer), method="GET")
                with self._open(request) as response:  # type: ignore[union-attr]
                    content_length = response.headers.get("Content-Length")
                    if content_length:
                        try:
                            if int(content_length) > MAX_DOWNLOAD_BYTES:
                                raise SpacesError("download exceeds the per-file size limit")
                        except ValueError:
                            pass

                    digest = hashlib.sha256()
                    total = 0
                    with temporary.open("wb") as output:
                        while True:
                            chunk = response.read(CHUNK_BYTES)
                            if not chunk:
                                break
                            total += len(chunk)
                            if total > MAX_DOWNLOAD_BYTES:
                                raise SpacesError("download exceeds the per-file size limit")
                            output.write(chunk)
                            digest.update(chunk)
                        output.flush()

                if total == 0:
                    raise SpacesError("download returned an empty file")
                jar_info = validate_jar(temporary)
                os.replace(temporary, destination)
                return total, digest.hexdigest(), jar_info
            except InvalidJarError:
                try:
                    temporary.unlink(missing_ok=True)
                except OSError:
                    pass
                raise
            except (HTTPError, URLError, TimeoutError, OSError, SpacesError, zipfile.BadZipFile) as error:
                last_error = error
                try:
                    temporary.unlink(missing_ok=True)
                except OSError:
                    pass
                if attempt != REQUEST_ATTEMPTS:
                    time.sleep(0.5 * attempt)

        detail = str(last_error) if last_error else "unknown download error"
        raise SpacesError(
            f"download failed after {REQUEST_ATTEMPTS} attempts: {url}: {detail}"
        )


class ListingParser(HTMLParser):
    """Extract the file cards used by Spaces.im listing pages."""

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.current: dict[str, object] | None = None
        self.entries: list[dict[str, object]] = []
        self.active_field: tuple[str, str] | None = None

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attributes = dict(attrs)
        if tag.lower() == "a" and self.current is None:
            href = attributes.get("href") or ""
            file_id_value = file_id(href)
            if file_id_value is not None:
                self.current = {
                    "href": href,
                    "id": file_id_value,
                    "text": [],
                    "title": [],
                    "extension": [],
                }
                self.active_field = None
                return

        if self.current is None:
            return
        classes = set((attributes.get("class") or "").split())
        if tag.lower() == "b" and "darkblue" in classes:
            self.active_field = ("title", tag.lower())
        elif tag.lower() == "b" and "lightgrey" in classes:
            self.active_field = ("extension", tag.lower())

    def handle_data(self, data: str) -> None:
        if self.current is None:
            return
        self.current["text"].append(data)  # type: ignore[union-attr]
        if self.active_field:
            self.current[self.active_field[0]].append(data)  # type: ignore[union-attr]

    def handle_endtag(self, tag: str) -> None:
        if self.current is None:
            return
        if self.active_field and self.active_field[1] == tag.lower():
            self.active_field = None
        if tag.lower() != "a":
            return
        value = self.current
        title = normalized_text("".join(value["title"])) or normalized_text("".join(value["text"]))  # type: ignore[arg-type]
        extension = sanitize_extension(normalized_text("".join(value["extension"])))  # type: ignore[arg-type]
        self.entries.append(
            {
                "href": str(value["href"]),
                "id": int(value["id"]),
                "title": title,
                "extension": extension or infer_extension_from_name(title),
            }
        )
        self.current = None
        self.active_field = None


class DetailParser(HTMLParser):
    """Extract the download card and metadata from one file page."""

    _void_tags = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.depth = 0
        self.download_url: str | None = None
        self.download_urls: list[str] = []
        self.content_size: str | None = None
        self.collectors: list[dict[str, object]] = []
        self.title = ""
        self.description = ""
        self.raw_html = ""

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        tag = tag.lower()
        attributes = dict(attrs)
        href = attributes.get("href") or ""
        if tag == "a" and "/files/download/" in href:
            self.download_urls.append(href)

        if tag == "meta" and attributes.get("itemprop") == "contentSize":
            self.content_size = attributes.get("content")

        if tag not in self._void_tags:
            self.depth += 1
            if tag == "h1" and attributes.get("itemprop") == "name":
                self.collectors.append({"kind": "title", "depth": self.depth, "parts": []})
            elif tag == "div" and attributes.get("itemprop") == "description":
                self.collectors.append({"kind": "description", "depth": self.depth, "parts": []})

    def handle_data(self, data: str) -> None:
        for collector in self.collectors:
            collector["parts"].append(data)  # type: ignore[union-attr]

    def handle_endtag(self, tag: str) -> None:
        tag = tag.lower()
        if tag in self._void_tags:
            return
        finished = [collector for collector in self.collectors if collector["depth"] == self.depth]
        for collector in finished:
            value = normalized_text("".join(collector["parts"]))  # type: ignore[arg-type]
            if collector["kind"] == "title" and not self.title:
                self.title = value
            elif collector["kind"] == "description" and not self.description:
                self.description = value
            self.collectors.remove(collector)
        self.depth = max(0, self.depth - 1)

    def card(self, page_url: str) -> DownloadCard:
        if not self.download_urls:
            raise SpacesError("file page has no download link")
        absolute_urls = [urljoin(page_url, value) for value in self.download_urls]
        # Some pages publish both the JAR and its optional JAD descriptor.  A
        # JAD is useful metadata but is not an emulator input, so prefer JAR.
        download_url = next(
            (value for value in absolute_urls if extension_from_url(value) == "jar"),
            absolute_urls[0],
        )
        title = self.title
        description = self.description
        for collector in self.collectors:
            kind = collector["kind"]
            value = normalized_text("".join(collector["parts"]))  # type: ignore[arg-type]
            if kind == "title" and not title:
                title = value
            elif kind == "description" and not description:
                description = value
        # Well-formed HTML closes these elements, but a truncated response can
        # leave a collector open.  The second pass recovers those values.
        if not title:
            title = normalized_tag_text(self.raw_html, "h1", "itemprop", "name")
        if not description:
            description = normalized_tag_text(self.raw_html, "div", "itemprop", "description")
        extension = extension_from_url(download_url) or infer_extension_from_name(title)
        return DownloadCard(title, extension, download_url, self.content_size, description)


def read_limited(response: object, max_bytes: int) -> bytes:
    chunks: list[bytes] = []
    total = 0
    reader = response  # urllib response exposes read(); keeping this typed loosely supports test doubles.
    while True:
        chunk = reader.read(CHUNK_BYTES)  # type: ignore[attr-defined]
        if not chunk:
            break
        total += len(chunk)
        if total > max_bytes:
            raise SpacesError(f"response exceeds the {max_bytes} byte limit")
        chunks.append(chunk)
    return b"".join(chunks)


def decode_html(data: bytes) -> str:
    # Spaces.im declares UTF-8 in its XHTML/WAP pages.  The fallback keeps the
    # parser useful for older pages that contain a malformed charset marker.
    match = re.search(rb"charset\s*=\s*['\"]?\s*([A-Za-z0-9._-]+)", data[:4096], re.I)
    encoding = match.group(1).decode("ascii", "ignore") if match else "utf-8"
    try:
        decoded = data.decode(encoding, "replace")
    except LookupError:
        decoded = data.decode("utf-8", "replace")
    # A few older Spaces pages advertise UTF-8 but contain Windows-1251 text.
    # Prefer the fallback only when it clearly removes replacement characters.
    if decoded.count("\ufffd") > 4:
        legacy = data.decode("cp1251", "replace")
        if legacy.count("\ufffd") < decoded.count("\ufffd"):
            return legacy
    return decoded


def is_challenge_page(data: bytes) -> bool:
    sample = data[:16_384].decode("utf-8", "ignore").lower()
    return "<title>загрузка" in sample or ("js_passed=2" in sample and "challenge-platform" in sample)


def normalized_text(value: str) -> str:
    return " ".join(value.replace("\xa0", " ").split())


def normalized_tag_text(source: str, tag: str, attribute: str, value: str) -> str:
    pattern = re.compile(
        rf"<{tag}\b[^>]*\b{re.escape(attribute)}\s*=\s*['\"]{re.escape(value)}['\"][^>]*>(.*?)</{tag}\s*>",
        re.I | re.S,
    )
    match = pattern.search(source)
    if not match:
        return ""
    return normalized_text(re.sub(r"<[^>]+>", " ", html_lib.unescape(match.group(1))))


def file_id(href: str) -> int | None:
    match = re.search(r"/files/view/(\d+)(?:[/?:]|$)", href)
    return int(match.group(1)) if match else None


def sanitize_extension(extension: str) -> str | None:
    extension = extension.strip().lstrip(".")
    if not extension or len(extension) > 12 or not extension.isascii() or not extension.isalnum():
        return None
    return extension.lower()


def infer_extension_from_name(name: str) -> str:
    suffix = name.rsplit(".", 1)[-1] if "." in name else ""
    return sanitize_extension(suffix) or "jar"


def extension_from_url(url: str) -> str:
    path = unquote(urlsplit(url).path)
    return sanitize_extension(path.rsplit("/", 1)[-1].rsplit(".", 1)[-1]) or ""


def direct_file_url(download_url: str) -> str:
    """Convert a Spaces redirect URL to the published fp.spac.me file URL.

    The normal browser follows this redirect.  The direct HTTP response from
    spaces.im can be cached as the JavaScript gate page, so using the final
    file-host form makes command-line downloads deterministic and still uses
    the exact path advertised by the file page.
    """

    parsed = urlsplit(download_url)
    if parsed.hostname not in {"spaces.im", "www.spaces.im"}:
        return download_url
    match = re.match(r"^/files/download/([^/]+)/file/(.+)$", parsed.path)
    if not match:
        return download_url
    token, tail = match.groups()
    tail = unquote(tail)
    return urlunsplit((parsed.scheme or "https", "fp.spac.me", f"/{token}/{tail}", "", ""))


def safe_filename(title: str, *, max_length: int = 96) -> str:
    normalized = unicodedata.normalize("NFKC", title)
    result: list[str] = []
    separator = False
    for character in normalized:
        if character.isalnum() or character in "-_":
            result.append(character)
            separator = False
        elif result and not separator:
            result.append("_")
            separator = True
        if len(result) >= max_length:
            break
    value = "".join(result).strip("_. ") or "game"
    if value.upper() in {"CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "LPT1", "LPT2", "LPT3"}:
        value = f"game_{value}"
    return value


def parse_listing_entries(category: Category, html: str, page_url: str) -> list[CatalogEntry]:
    parser = ListingParser()
    parser.feed(html)
    parser.close()
    result: list[CatalogEntry] = []
    seen: set[int] = set()
    for item in parser.entries:
        entry_id = int(item["id"])
        if entry_id in seen:
            continue
        seen.add(entry_id)
        result.append(
            CatalogEntry(
                category=category.slug,
                category_title=category.title,
                id=entry_id,
                title=str(item["title"]),
                extension=str(item["extension"] or "jar"),
                view_url=urljoin(page_url, str(item["href"])),
                source_page=page_url,
                categories=[category.slug],
            )
        )
    return result


def parse_page_total(html: str) -> int | None:
    matches = re.findall(
        r"<span\b[^>]*\bclass\s*=\s*['\"][^'\"]*\bblk\b[^'\"]*['\"][^>]*>(.*?)</span\s*>",
        html,
        re.I | re.S,
    )
    for value in matches:
        text = normalized_text(re.sub(r"<[^>]+>", " ", html_lib.unescape(value)))
        match = re.search(r"(?:из|of)\s*(\d{1,6})", text, re.I)
        if match:
            return int(match.group(1))
    return None


def parse_next_url(html: str, page_url: str) -> str | None:
    for match in re.finditer(
        r"<a\b([^>]*\bhref\s*=\s*['\"][^'\"]+['\"][^>]*)>(.*?)</a\s*>",
        html,
        re.I | re.S,
    ):
        attributes = match.group(1)
        href_match = re.search(r"\bhref\s*=\s*['\"]([^'\"]+)", attributes, re.I)
        if not href_match:
            continue
        href = html_lib.unescape(href_match.group(1))
        # File titles can themselves contain words such as "Next" or
        # "следующий".  Only pagination links have the site's /pN/ shape;
        # requiring it prevents a program card from being mistaken for Next.
        if not re.search(r"/p\d+(?:/|$)", urlsplit(urljoin(page_url, href)).path, re.I):
            continue
        text = normalized_text(re.sub(r"<[^>]+>", " ", html_lib.unescape(match.group(2))))
        if not re.search(r"впер[её]д|next|след", text, re.I):
            continue
        return urljoin(page_url, href)
    return None


def parse_detail_card(html: str, page_url: str) -> DownloadCard:
    parser = DetailParser()
    parser.raw_html = html
    parser.feed(html)
    parser.close()
    return parser.card(page_url)


def select_categories(names: Iterable[str]) -> list[Category]:
    requested = list(names)
    if not requested or any(name.lower() == "all" for name in requested):
        return list(CATEGORIES)
    by_slug = {category.slug: category for category in CATEGORIES}
    result: list[Category] = []
    for raw_name in requested:
        name = CATEGORY_ALIASES.get(raw_name.lower(), raw_name.lower())
        if name not in by_slug:
            available = ", ".join(category.slug for category in CATEGORIES)
            raise SpacesError(f"unknown category '{raw_name}', choose one of: {available}")
        if name not in {category.slug for category in result}:
            result.append(by_slug[name])
    return result


def select_catalog_categories(sections: Iterable[str], names: Iterable[str]) -> list[Category]:
    requested_sections = list(sections)
    requested_categories = list(names)
    if requested_sections and requested_categories:
        raise SpacesError("use either --section or --category for catalog selection, not both")
    if not requested_sections:
        return select_categories(requested_categories)

    result: list[Category] = []
    for raw_name in requested_sections:
        name = SECTION_ALIASES.get(raw_name.lower(), raw_name.lower())
        if name not in CATALOG_SECTIONS:
            available = ", ".join(CATALOG_SECTIONS)
            raise SpacesError(f"unknown section '{raw_name}', choose one of: {available}")
        for category in CATALOG_SECTIONS[name]:
            if category.slug not in {item.slug for item in result}:
                result.append(category)
    return result


def collect_catalog(
    client: SpacesClient,
    *,
    categories: list[Category],
    limit: int,
    per_category: int,
    delay_ms: int,
    max_pages: int | None = None,
    workers: int = 1,
) -> list[CatalogEntry]:
    by_id: dict[int, CatalogEntry] = {}
    workers = max(1, workers)

    def page_url_from_template(template_url: str, page_number: int) -> str:
        parts = urlsplit(template_url)
        match = re.search(r"/p\d+(?:/|$)", parts.path, re.I)
        if not match:
            raise SpacesError(f"cannot derive numbered page URL from {template_url}")
        path = f"{parts.path[:match.start()]}/p{page_number}{parts.path[match.end():]}"
        return urlunsplit((parts.scheme, parts.netloc, path, parts.query, parts.fragment))

    for category in categories:
        pages_seen: set[str] = set()
        category_count = 0

        def record_page(page_number: int, page_url: str, html: str) -> None:
            nonlocal category_count
            page_entries = parse_listing_entries(category, html, page_url)
            added = 0
            for entry in page_entries:
                existing = by_id.get(entry.id)
                if existing is not None:
                    if category.slug not in existing.categories:
                        existing.categories.append(category.slug)
                    continue
                if entry.extension != "jar":
                    continue
                by_id[entry.id] = entry
                category_count += 1
                added += 1
                if category_count >= per_category or len(by_id) >= limit:
                    break
            print(
                f"[catalog] {category.slug}: page {page_number} "
                f"+{added}, category={category_count}, unique={len(by_id)}",
                file=sys.stderr,
                flush=True,
            )

        first_page_url = category.url
        first_page_html = client.fetch_text(first_page_url)
        pages_seen.add(first_page_url)
        record_page(1, first_page_url, first_page_html)

        if len(by_id) >= limit or category_count >= per_category:
            break

        first_next_url = parse_next_url(first_page_html, first_page_url)
        page_total = parse_page_total(first_page_html)
        page_limit = page_total
        if max_pages is not None:
            page_limit = min(page_limit, max_pages) if page_limit is not None else max_pages

        # Exhaustive catalogs can contain thousands of pages.  The first page
        # exposes both the total and the site's numbered pagination URL, so
        # fetch the remaining pages in bounded batches and merge them here.
        if workers > 1 and first_next_url and page_limit is not None and page_limit > 1:
            template_url = first_next_url
            batch_size = workers * 4
            for batch_start in range(2, page_limit + 1, batch_size):
                if len(by_id) >= limit or category_count >= per_category:
                    break
                page_numbers = list(range(batch_start, min(page_limit + 1, batch_start + batch_size)))
                page_urls = {
                    page_number: page_url_from_template(template_url, page_number)
                    for page_number in page_numbers
                }
                failures: list[tuple[int, str, Exception]] = []
                with ThreadPoolExecutor(max_workers=workers, thread_name_prefix="spaces-java-catalog") as executor:
                    futures = {
                        executor.submit(client.fetch_text, page_url): (page_number, page_url)
                        for page_number, page_url in page_urls.items()
                    }
                    for future in as_completed(futures):
                        page_number, page_url = futures[future]
                        pages_seen.add(page_url)
                        try:
                            page_html = future.result()
                        except Exception as error:
                            failures.append((page_number, page_url, error))
                            continue
                        record_page(page_number, page_url, page_html)

                # A failed page is retried after its batch so a transient
                # response cannot silently create an incomplete manifest.
                for page_number, page_url, original_error in failures:
                    try:
                        page_html = client.fetch_text(page_url)
                    except Exception as error:
                        raise SpacesError(
                            f"failed to collect page {page_number} of {category.slug}: {error}"
                        ) from original_error
                    record_page(page_number, page_url, page_html)
                sleep_ms(delay_ms)
        else:
            page_url = first_next_url
            page_number = 1
            while page_url and page_url not in pages_seen:
                if max_pages is not None and page_number >= max_pages:
                    break
                pages_seen.add(page_url)
                page_number += 1
                html = client.fetch_text(page_url)
                record_page(page_number, page_url, html)
                if len(by_id) >= limit or category_count >= per_category:
                    break
                page_url = parse_next_url(html, page_url)
                sleep_ms(delay_ms)
        if len(by_id) >= limit:
            break

    result = list(by_id.values())
    result.sort(key=lambda entry: (categories.index(next(category for category in categories if category.slug == entry.category)), entry.title.casefold(), entry.id))
    return result[:limit]


def balanced_entries(entries: list[CatalogEntry], limit: int | None) -> list[CatalogEntry]:
    """Round-robin manufacturers so a partial download remains device-diverse."""

    groups: dict[str, list[CatalogEntry]] = {}
    order: list[str] = []
    for entry in entries:
        if entry.category not in groups:
            groups[entry.category] = []
            order.append(entry.category)
        groups[entry.category].append(entry)

    result: list[CatalogEntry] = []
    while any(groups.get(category) for category in order):
        for category in order:
            values = groups.get(category) or []
            if values:
                result.append(values.pop(0))
                if limit is not None and len(result) >= limit:
                    return result
    return result


def manifest_payload(entries: list[CatalogEntry], *, source: str = SOURCE_URL) -> dict[str, object]:
    return {
        "source": source,
        "generated_unix_seconds": int(time.time()),
        "entries": [entry.as_dict() for entry in entries],
    }


def write_catalog_exports(
    manifest_path: Path,
    entries: list[CatalogEntry],
    *,
    source: str = SOURCE_URL,
    categories: Iterable[Category] = CATEGORIES,
    heading: str = "Java ME games from Spaces.im",
    item_label: str = "Game",
) -> None:
    payload = manifest_payload(entries, source=source)
    atomic_write_json(manifest_path, payload)

    csv_path = manifest_path.with_suffix(".csv")
    csv_path.parent.mkdir(parents=True, exist_ok=True)
    with csv_path.open("w", encoding="utf-8", newline="") as output:
        writer = csv.DictWriter(
            output,
            fieldnames=[
                "category",
                "category_title",
                "id",
                "title",
                "extension",
                "view_url",
                "source_page",
                "categories",
            ],
        )
        writer.writeheader()
        for entry in entries:
            row = entry.as_dict()
            row["categories"] = ";".join(entry.categories)
            writer.writerow(row)

    markdown_path = manifest_path.with_suffix(".md")
    lines = [
        f"# {heading}",
        "",
        f"Source: {source}",
        "",
        f"Total catalog entries: {len(entries)}.",
    ]
    category_list = list(categories)
    known_slugs = {category.slug for category in category_list}
    for entry in entries:
        if entry.category not in known_slugs:
            category_list.append(Category(entry.category, entry.category_title, entry.source_page))
            known_slugs.add(entry.category)
    for category in category_list:
        category_entries = [entry for entry in entries if entry.category == category.slug]
        if not category_entries:
            continue
        lines.extend(
            [
                "",
                f"## {category.title} ({len(category_entries)})",
                "",
                f"| # | {item_label} | Package | Spaces ID |",
                "|---:|---|---|---:|",
            ]
        )
        for index, entry in enumerate(category_entries, start=1):
            title = markdown_cell(entry.title)
            lines.append(f"| {index} | [{title}]({entry.view_url}) | `{entry.extension}` | {entry.id} |")
    atomic_write_text(markdown_path, "\n".join(lines) + "\n")


def load_manifest(path: Path) -> list[CatalogEntry]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    raw_entries = payload.get("entries") if isinstance(payload, dict) else None
    if not isinstance(raw_entries, list):
        raise SpacesError(f"manifest has no entries array: {path}")
    result: list[CatalogEntry] = []
    for raw in raw_entries:
        if not isinstance(raw, dict):
            continue
        try:
            category = str(raw["category"])
            categories = [str(value) for value in raw.get("categories", [category])]
            result.append(
                CatalogEntry(
                    category=category,
                    category_title=str(raw.get("category_title", category)),
                    id=int(raw["id"]),
                    title=str(raw["title"]),
                    extension=str(raw.get("extension", "jar")),
                    view_url=str(raw["view_url"]),
                    source_page=str(raw.get("source_page", raw["view_url"])),
                    categories=categories,
                )
            )
        except (KeyError, TypeError, ValueError) as error:
            raise SpacesError(f"invalid manifest entry in {path}: {error}") from error
    return result


def download_catalog(
    client: SpacesClient,
    *,
    manifest_path: Path,
    output_root: Path,
    categories: list[str],
    ids: set[int],
    limit: int | None,
    min_success: int | None,
    delay_ms: int,
    workers: int,
) -> tuple[list[dict[str, object]], int]:
    entries = load_manifest(manifest_path)
    selected = [entry for entry in entries if entry.extension.lower() == "jar"]
    if categories:
        category_set = {CATEGORY_ALIASES.get(name.lower(), name.lower()) for name in categories}
        selected = [entry for entry in selected if entry.category in category_set]
    if ids:
        selected = [entry for entry in selected if entry.id in ids]
    selected = balanced_entries(selected, limit)

    output_root.mkdir(parents=True, exist_ok=True)
    report_path = output_root / "download-report.json"
    results_by_id: dict[int, dict[str, object]] = {}
    if report_path.is_file():
        try:
            old_report = json.loads(report_path.read_text(encoding="utf-8"))
            for raw in old_report.get("entries", []):
                if isinstance(raw, dict) and isinstance(raw.get("id"), int):
                    results_by_id[int(raw["id"])] = raw
        except (OSError, json.JSONDecodeError):
            print(f"[download] ignoring unreadable report {report_path}", file=sys.stderr)

    # Validate each previously downloaded path once.  The retry path below
    # needs to know which entries can be skipped; re-running the ZIP check a
    # second time made large resumptions unnecessarily slow.
    valid_previous_ids = {
        int(result_id)
        for result_id, result in results_by_id.items()
        if result.get("status") in {"downloaded", "present"}
        and result_path_is_valid(result)
    }
    success_count = len(valid_previous_ids)
    pending: list[tuple[int, CatalogEntry]] = []
    for index, entry in enumerate(selected, start=1):
        previous = results_by_id.get(entry.id)
        if entry.id in valid_previous_ids:
            print(f"[download] {index}/{len(selected)} present #{entry.id} {entry.title}", file=sys.stderr, flush=True)
            continue
        pending.append((index, entry))

    # Use small batches: this keeps the number of in-flight downloads bounded,
    # writes a usable report after every result, and stops close to the target
    # instead of downloading the whole candidate reserve.
    workers = max(1, workers)
    results_since_checkpoint = 0
    for batch_start in range(0, len(pending), workers):
        if min_success is not None and success_count >= min_success:
            break
        batch = pending[batch_start : batch_start + workers]
        with ThreadPoolExecutor(max_workers=workers, thread_name_prefix="spaces-java") as executor:
            futures = {}
            for index, entry in batch:
                print(
                    f"[download] {index}/{len(selected)} start {entry.category} #{entry.id} {entry.title}",
                    file=sys.stderr,
                    flush=True,
                )
                futures[executor.submit(download_one, client, entry, output_root)] = entry
            for future in as_completed(futures):
                entry = futures[future]
                try:
                    result = future.result()
                except Exception as error:  # one bad/missing source must not stop a 500-file corpus
                    result = error_result(entry, error)
                results_by_id[entry.id] = result
                if result.get("status") in {"downloaded", "present"}:
                    # download_one validates a new file before returning, and
                    # result_path_is_valid() checked old report entries above.
                    success_count += 1
                results_since_checkpoint += 1
                if results_since_checkpoint >= REPORT_CHECKPOINT_EVERY:
                    atomic_write_json(
                        report_path,
                        {
                            "manifest": str(manifest_path.resolve()),
                            "updated_unix_seconds": int(time.time()),
                            "entries": list(results_by_id.values()),
                        },
                    )
                    results_since_checkpoint = 0
                state = "ok" if result.get("status") in {"downloaded", "present"} else "error"
                print(
                    f"[download] {state} #{entry.id}; valid={success_count} "
                    f"target={min_success or '-'}",
                    file=sys.stderr,
                    flush=True,
                )
        sleep_ms(delay_ms)

    results = list(results_by_id.values())
    results.sort(key=lambda value: (str(value.get("category", "")), int(value.get("id", 0))))
    atomic_write_json(
        report_path,
        {
            "manifest": str(manifest_path.resolve()),
            "updated_unix_seconds": int(time.time()),
            "entries": results,
        },
    )
    return results, success_count


def error_result(entry: CatalogEntry, error: Exception) -> dict[str, object]:
    return {
        "category": entry.category,
        "category_title": entry.category_title,
        "id": entry.id,
        "title": entry.title,
        "status": "error",
        "path": None,
        "bytes": None,
        "sha256": None,
        "content_size": None,
        "description": None,
        "device_model": None,
        "jar": None,
        "error": str(error),
        "view_url": entry.view_url,
    }


def download_one(client: SpacesClient, entry: CatalogEntry, output_root: Path) -> dict[str, object]:
    detail_html = client.fetch_text(entry.view_url)
    card = parse_detail_card(detail_html, entry.view_url)
    file_url = direct_file_url(card.download_url)
    title = card.title or entry.title
    destination = output_root / entry.category / f"{entry.id}-{safe_filename(title)}.jar"

    if destination.is_file():
        try:
            jar_info = validate_jar(destination)
            bytes_count, digest = hash_file(destination)
            return result_for_download(
                entry,
                card,
                destination,
                status="present",
                bytes_count=bytes_count,
                digest=digest,
                jar_info=jar_info,
            )
        except (OSError, zipfile.BadZipFile, SpacesError):
            pass

    bytes_count, digest, jar_info = client.download_file(
        file_url,
        destination,
        referer=entry.view_url,
    )
    return result_for_download(
        entry,
        card,
        destination,
        status="downloaded",
        bytes_count=bytes_count,
        digest=digest,
        jar_info=jar_info,
    )


def result_for_download(
    entry: CatalogEntry,
    card: DownloadCard,
    destination: Path,
    *,
    status: str,
    bytes_count: int,
    digest: str,
    jar_info: JarInfo,
) -> dict[str, object]:
    device_model = infer_phone_model(" ".join((entry.title, card.title, card.description)))
    return {
        "category": entry.category,
        "category_title": entry.category_title,
        "id": entry.id,
        "title": entry.title,
        "resolved_title": card.title or entry.title,
        "status": status,
        "path": str(destination.resolve()),
        "bytes": bytes_count,
        "sha256": digest,
        "content_size": card.content_size,
        "description": card.description or None,
        "device_model": device_model or None,
        "jar": asdict(jar_info),
        "error": None,
        "view_url": entry.view_url,
    }


def validate_jar(path: Path) -> JarInfo:
    try:
        with zipfile.ZipFile(path) as archive:
            infos = archive.infolist()
            if not infos:
                raise InvalidJarError("JAR is an empty ZIP archive")
            class_names = [info.filename for info in infos if info.filename.lower().endswith(".class")]
            if not class_names:
                raise InvalidJarError("ZIP has no Java class files")
            bad_entry = archive.testzip()
            if bad_entry:
                raise InvalidJarError(f"JAR CRC check failed for {bad_entry}")
            manifest = read_jar_manifest(archive)
            return JarInfo(
                class_count=len(class_names),
                entry_count=len(infos),
                midlet_name=manifest.get("MIDlet-Name"),
                midlet_class=midlet_class_from_manifest(manifest),
                midlet_vendor=manifest.get("MIDlet-Vendor"),
                configuration=manifest.get("MicroEdition-Configuration"),
                profile=manifest.get("MicroEdition-Profile"),
            )
    except zipfile.BadZipFile as error:
        raise InvalidJarError("downloaded file is not a valid JAR/ZIP") from error


def read_jar_manifest(archive: zipfile.ZipFile) -> dict[str, str]:
    name = next(
        (info.filename for info in archive.infolist() if info.filename.lower() == "meta-inf/manifest.mf"),
        None,
    )
    if not name:
        return {}
    try:
        raw = archive.read(name).decode("utf-8", "replace")
    except (KeyError, UnicodeDecodeError):
        return {}
    lines: list[str] = []
    for line in raw.replace("\r\n", "\n").replace("\r", "\n").split("\n"):
        if line.startswith(" ") and lines:
            lines[-1] += line[1:]
        else:
            lines.append(line)
    values: dict[str, str] = {}
    for line in lines:
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        values[key.strip()] = value.strip()
    return values


def midlet_class_from_manifest(manifest: dict[str, str]) -> str | None:
    value = manifest.get("MIDlet-1", "")
    parts = [part.strip() for part in value.split(",")]
    return parts[2] if len(parts) >= 3 and parts[2] else None


def infer_phone_model(text: str) -> str:
    normalized = normalized_text(text)
    brands = ["Sony Ericsson", "BenQ-Siemens", "Alcatel", "Motorola", "Samsung", "Nokia", "Siemens", "LG", "Fly"]
    for brand in brands:
        match = re.search(
            rf"\b{re.escape(brand)}(?:\s+[A-Za-z0-9][A-Za-z0-9+._-]*){{1,3}}",
            normalized,
            re.I,
        )
        if match:
            return match.group(0).strip(" ,.;:()[]")
    return ""


def hash_file(path: Path) -> tuple[int, str]:
    digest = hashlib.sha256()
    total = 0
    with path.open("rb") as source:
        while True:
            chunk = source.read(CHUNK_BYTES)
            if not chunk:
                break
            total += len(chunk)
            digest.update(chunk)
    return total, digest.hexdigest()


def result_path_is_valid(result: dict[str, object]) -> bool:
    raw_path = result.get("path")
    if not isinstance(raw_path, str):
        return False
    path = Path(raw_path)
    if not path.is_file() or path.stat().st_size == 0:
        return False
    try:
        validate_jar(path)
        return True
    except (OSError, SpacesError, zipfile.BadZipFile):
        return False


def count_valid_downloads(results: Iterable[dict[str, object]]) -> int:
    return sum(
        1
        for result in results
        if result.get("status") in {"downloaded", "present"} and result_path_is_valid(result)
    )


def atomic_write_json(path: Path, value: object) -> None:
    atomic_write_text(path, json.dumps(value, ensure_ascii=False, indent=2) + "\n")


def atomic_write_text(path: Path, value: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix=f".{path.name}.",
        suffix=".tmp",
        dir=path.parent,
        text=True,
    )
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as output:
            output.write(value)
            output.flush()
        for attempt in range(6):
            try:
                os.replace(temporary, path)
                return
            except PermissionError:
                if attempt == 5:
                    raise
                time.sleep(0.25 * (attempt + 1))
    finally:
        temporary.unlink(missing_ok=True)


def csv_field(value: str) -> str:
    return value.replace("|", "\\|").replace("\r", " ").replace("\n", " ").replace("]", "\\]")


def markdown_cell(value: str) -> str:
    return csv_field(value)


def sleep_ms(milliseconds: int) -> None:
    if milliseconds > 0:
        time.sleep(milliseconds / 1000)


def run_catalog(args: argparse.Namespace, client: SpacesClient) -> int:
    categories = select_catalog_categories(args.section, args.category)
    limit = args.limit if args.limit is not None else (1_000_000 if args.all_pages else 800)
    per_category = args.per_category if args.per_category is not None else (1_000_000 if args.all_pages else 80)
    entries = collect_catalog(
        client,
        categories=categories,
        limit=limit,
        per_category=per_category,
        delay_ms=args.delay_ms,
        max_pages=args.max_pages,
        workers=args.workers if args.all_pages else 1,
    )
    if len(categories) == 1 and categories[0].slug == JAVA_PROGRAMS_CATEGORY.slug:
        heading = "Java programs from Spaces.im"
        item_label = "Program"
    elif len(categories) == 1 and categories[0].slug == ALL_GAMES_CATEGORY.slug:
        heading = "All Java games from Spaces.im"
        item_label = "Game"
    else:
        heading = "Java ME games from Spaces.im"
        item_label = "Game"
    source = categories[0].url if len(categories) == 1 else SOURCE_URL
    write_catalog_exports(
        Path(args.output),
        entries,
        source=source,
        categories=categories,
        heading=heading,
        item_label=item_label,
    )
    by_category: dict[str, int] = {}
    for entry in entries:
        by_category[entry.category] = by_category.get(entry.category, 0) + 1
    breakdown = ", ".join(f"{category}={count}" for category, count in by_category.items())
    print(f"Catalog: {len(entries)} unique JAR entries -> {Path(args.output).resolve()}")
    print(f"Phones: {breakdown}")
    return 0


def run_download(args: argparse.Namespace, client: SpacesClient) -> int:
    results, success_count = download_catalog(
        client,
        manifest_path=Path(args.manifest),
        output_root=Path(args.output),
        categories=args.category,
        ids=set(args.id),
        limit=args.limit,
        min_success=args.min_success,
        delay_ms=args.delay_ms,
        workers=args.workers,
    )
    errors = sum(1 for result in results if result.get("status") == "error")
    downloaded = sum(1 for result in results if result.get("status") in {"downloaded", "present"})
    print(f"Downloads: {success_count} valid JARs ({downloaded} recorded, {errors} errors)")
    print(f"Report: {(Path(args.output) / 'download-report.json').resolve()}")
    if args.min_success is not None and success_count < args.min_success:
        print(
            f"Only {success_count}/{args.min_success} valid JARs were obtained; "
            "rerun the same command to resume.",
            file=sys.stderr,
        )
        return 1
    return 0


def run_corpus(args: argparse.Namespace, client: SpacesClient) -> int:
    if args.candidates < args.target:
        raise SpacesError("--candidates must be at least --target")
    output = Path(args.output)
    categories = select_categories(args.category)
    entries = collect_catalog(
        client,
        categories=categories,
        limit=args.candidates,
        per_category=args.per_category,
        delay_ms=args.delay_ms,
        max_pages=args.max_pages,
    )
    manifest_path = output / "catalog.json"
    write_catalog_exports(manifest_path, entries)
    print(f"[corpus] catalog ready: {len(entries)} candidates", file=sys.stderr, flush=True)
    results, success_count = download_catalog(
        client,
        manifest_path=manifest_path,
        output_root=output,
        categories=[],
        ids=set(),
        limit=args.candidates,
        min_success=args.target,
        delay_ms=args.delay_ms,
        workers=args.workers,
    )
    errors = sum(1 for result in results if result.get("status") == "error")
    print(
        f"Corpus: {success_count}/{args.target} valid JARs, "
        f"{errors} errors; output {output.resolve()}"
    )
    return 0 if success_count >= args.target else 1


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cookie", default=DEFAULT_COOKIE, help="Cookie for Spaces.im HTML pages")
    parser.add_argument("--user-agent", default=DEFAULT_USER_AGENT)
    parser.add_argument("--timeout", type=int, default=REQUEST_TIMEOUT_SECONDS)
    parser.add_argument(
        "--proxy-file",
        help="proxy list (default: IPV4.txt when present; use an empty value for direct access)",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    catalog = subparsers.add_parser("catalog", help="collect a JSON/CSV/Markdown JAR manifest")
    catalog.add_argument("--output", default=str(DEFAULT_OUTPUT / "catalog.json"))
    catalog.add_argument("--limit", type=int)
    catalog.add_argument("--per-category", type=int)
    catalog.add_argument("--max-pages", type=int)
    catalog.add_argument("--delay-ms", type=int, default=DEFAULT_DELAY_MS)
    catalog.add_argument(
        "--workers",
        type=int,
        default=8,
        help="parallel listing-page requests for --all-pages",
    )
    catalog.add_argument("--category", action="append", default=[])
    catalog.add_argument(
        "--section",
        action="append",
        choices=tuple(CATALOG_SECTIONS),
        default=[],
        help="catalog source: devices (default), games (all games), or programs",
    )
    catalog.add_argument(
        "--all-pages",
        action="store_true",
        help="follow pagination until the source has no next page",
    )
    catalog.set_defaults(handler=run_catalog)

    download = subparsers.add_parser("download", help="download and verify games from a manifest")
    download.add_argument("manifest")
    download.add_argument("--output", default=str(DEFAULT_OUTPUT))
    download.add_argument("--limit", type=int)
    download.add_argument("--min-success", type=int)
    download.add_argument("--delay-ms", type=int, default=DEFAULT_DELAY_MS)
    download.add_argument("--workers", type=int, default=4)
    download.add_argument("--category", action="append", default=[])
    download.add_argument("--id", type=int, action="append", default=[])
    download.set_defaults(handler=run_download)

    corpus = subparsers.add_parser("corpus", help="collect a diverse catalog and download a target corpus")
    corpus.add_argument("--output", default=str(DEFAULT_OUTPUT))
    corpus.add_argument("--target", type=int, default=500)
    corpus.add_argument("--candidates", type=int, default=800)
    corpus.add_argument("--per-category", type=int, default=100)
    corpus.add_argument("--max-pages", type=int)
    corpus.add_argument("--delay-ms", type=int, default=DEFAULT_DELAY_MS)
    corpus.add_argument("--workers", type=int, default=4)
    corpus.add_argument("--category", action="append", default=[])
    corpus.set_defaults(handler=run_corpus)

    return parser


def run_self_tests() -> None:
    listing = """
      <a href='https://spaces.im/files/view/123/'>
        <b class='darkblue'>Nokia N73: Test</b><b class='lightgrey'>.jar</b>
      </a>
      <div class='pag'><span class='blk'>1 из 7028</span>
        <a href='/sz/igry/java-igry/nokia/popular/p2/'>Вперёд</a></div>
    """
    category = CATEGORIES[5]
    entries = parse_listing_entries(category, listing, category.url)
    assert len(entries) == 1 and entries[0].id == 123
    assert entries[0].title == "Nokia N73: Test" and entries[0].extension == "jar"
    assert parse_page_total(listing) == 7028
    assert parse_next_url(listing, category.url).endswith("/p2/")  # type: ignore[union-attr]

    detail = """
      <meta itemprop='contentSize' content='629.9 Kб'>
      <h1 itemprop='name'>Soul Of Darkness</h1>
      <div itemprop='description'>Sony Ericsson W910i 240x320</div>
      <a href='https://spaces.im/files/download/htz-cs15/file/f/token/178/123/0/hash/game%2528x%2529-spaces.im.jar'>Скачать</a>
    """
    card = parse_detail_card(detail, "https://spaces.im/files/view/123/")
    assert card.title == "Soul Of Darkness" and card.extension == "jar"
    assert card.content_size == "629.9 Kб"
    assert direct_file_url(card.download_url).startswith("https://fp.spac.me/htz-cs15/f/")
    assert safe_filename("Asphalt: N-Gage / 2") == "Asphalt_N-Gage_2"
    assert parse_proxy_line("127.0.0.1:8080:user:pass word") == (
        "http://user:pass%20word@127.0.0.1:8080"
    )
    assert parse_proxy_line("https://user:pass@127.0.0.1:8443") == (
        "https://user:pass@127.0.0.1:8443"
    )
    assert parse_proxy_line("# ignored") is None
    print("self-tests: OK")


def main(argv: list[str] | None = None) -> int:
    if argv is not None and argv == ["self-test"]:
        run_self_tests()
        return 0
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        if args.proxy_file is not None:
            proxy_path = Path(args.proxy_file) if args.proxy_file else None
        elif DEFAULT_PROXY_FILE.is_file():
            proxy_path = DEFAULT_PROXY_FILE
        else:
            proxy_path = None
        proxies = load_proxy_file(proxy_path) if proxy_path is not None else []
        if proxy_path is not None:
            print(
                f"[proxy] loaded {len(proxies)} proxies from {proxy_path.resolve()}",
                file=sys.stderr,
                flush=True,
            )
        client = SpacesClient(
            cookie=args.cookie,
            user_agent=args.user_agent,
            timeout=args.timeout,
            proxies=proxies,
        )
        return int(args.handler(args, client))
    except KeyboardInterrupt:
        print("Interrupted; the report can be used to resume.", file=sys.stderr)
        return 130
    except (SpacesError, OSError, json.JSONDecodeError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    if len(sys.argv) == 2 and sys.argv[1] == "self-test":
        raise SystemExit(main(["self-test"]))
    raise SystemExit(main())
