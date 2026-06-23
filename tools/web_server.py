from argparse import ArgumentParser
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import ssl


class NoCacheHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
        super().end_headers()


def main():
    parser = ArgumentParser()
    parser.add_argument("--bind", default="0.0.0.0")
    parser.add_argument("--port", default=8080, type=int)
    parser.add_argument(
        "--directory",
        default=str(Path(__file__).resolve().parents[1] / "web"),
    )
    parser.add_argument("--cert")
    parser.add_argument("--key")
    args = parser.parse_args()

    handler = partial(NoCacheHandler, directory=args.directory)
    with ThreadingHTTPServer((args.bind, args.port), handler) as server:
        scheme = "http"
        if args.cert or args.key:
            if not args.cert or not args.key:
                raise SystemExit("--cert and --key must be passed together")
            context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
            context.load_cert_chain(args.cert, args.key)
            server.socket = context.wrap_socket(server.socket, server_side=True)
            scheme = "https"

        print(f"Serving {args.directory} on {scheme}://{args.bind}:{args.port}/", flush=True)
        server.serve_forever()


if __name__ == "__main__":
    main()
