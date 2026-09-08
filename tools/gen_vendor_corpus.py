"""Generate original vendor-smoke MIDlets for the compat lab."""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "apk" / "corpus"
STUBS = Path(__file__).resolve().parent / "_midp_stubs"

VENDORS = [
    ("generic", "Generic", "RustJava", None, ""),
    ("nokia_s40", "Nokia", "NokiaN73", "nokia", "com.nokia.mid.ui.DeviceControl.startVibra(80, 20L);"),
    ("k800_se", "SonyEricsson", "SonyEricssonK800i", "sonyericsson", 'com.sonyericsson.device.Device.vibrate(20);'),
    ("samsung", "Samsung", "Samsung-SGH-D900", "samsung", "com.samsung.util.Vibration.start(20, 50);"),
    ("siemens", "Siemens", "SIE-C75", "siemens", "com.siemens.mp.game.Vibrator.triggerVibrator(20);"),
    ("motorola", "Motorola", "Motorola-E1000", "motorola", "com.motorola.funlight.FunLight.getControl();"),
    ("lg", "Lg", "LG-KU990", "lg", "com.lg.util.Vibration.startVibra(20);"),
    ("vodafone", "Vodafone", "Vodafone", "vodafone", "com.vodafone.v10.Sound.isSupported();"),
    ("sprint", "Sprint", "Sprint", "sprint", "com.sprintpcs.media.Player.isSupported();"),
]

GAMES_PER_VENDOR = 9  # 9 vendors * 9 = 81 original titles
EXTRA_GAMES = 100
FAMILIES = ["canvas", "gamecanvas", "sprite", "rms", "form", "list", "fontgfx", "tone"]


def write_stubs(stub_dir: Path) -> None:
    files = {
        "javax/microedition/midlet/MIDlet.java": """
package javax.microedition.midlet;
public abstract class MIDlet {
    protected MIDlet() {}
    protected abstract void startApp();
    protected abstract void pauseApp();
    protected abstract void destroyApp(boolean unconditional);
    public final void notifyDestroyed() {}
    public final String getAppProperty(String key) { return null; }
}
""",
        "javax/microedition/lcdui/Displayable.java": """
package javax.microedition.lcdui;
public abstract class Displayable {
    public void setTitle(String title) {}
    public int getWidth() { return 240; }
    public int getHeight() { return 320; }
}
""",
        "javax/microedition/lcdui/Canvas.java": """
package javax.microedition.lcdui;
public abstract class Canvas extends Displayable {
    public static final int UP = 1, DOWN = 6, LEFT = 2, RIGHT = 5, FIRE = 8;
    protected Canvas() {}
    public void setFullScreenMode(boolean mode) {}
    public abstract void paint(Graphics g);
    public void keyPressed(int keyCode) {}
    public void repaint() {}
    public int getGameAction(int keyCode) { return 0; }
}
""",
        "javax/microedition/lcdui/Graphics.java": """
package javax.microedition.lcdui;
public class Graphics {
    public static final int TOP = 16, LEFT = 4;
    public void setColor(int rgb) {}
    public void fillRect(int x, int y, int w, int h) {}
    public void drawRect(int x, int y, int w, int h) {}
    public void drawLine(int x1, int y1, int x2, int y2) {}
    public void fillTriangle(int x1, int y1, int x2, int y2, int x3, int y3) {}
    public void drawString(String str, int x, int y, int anchor) {}
    public void drawImage(Image img, int x, int y, int anchor) {}
    public void setFont(Font font) {}
    public int getClipWidth() { return 240; }
    public int getClipHeight() { return 320; }
}
""",
        "javax/microedition/lcdui/Image.java": """
package javax.microedition.lcdui;
public class Image {
    public static Image createImage(int w, int h) { return new Image(); }
    public int getWidth() { return 16; }
    public int getHeight() { return 16; }
}
""",
        "javax/microedition/lcdui/Font.java": """
package javax.microedition.lcdui;
public class Font {
    public static final int FACE_SYSTEM = 0, STYLE_PLAIN = 0, SIZE_MEDIUM = 0;
    public static Font getDefaultFont() { return new Font(); }
    public static Font getFont(int face, int style, int size) { return new Font(); }
    public int stringWidth(String s) { return s == null ? 0 : s.length() * 8; }
    public int getHeight() { return 16; }
}
""",
        "javax/microedition/lcdui/Form.java": """
package javax.microedition.lcdui;
public class Form extends Displayable {
    public Form(String title) { setTitle(title); }
    public int append(String text) { return 0; }
    public int size() { return 1; }
}
""",
        "javax/microedition/lcdui/List.java": """
package javax.microedition.lcdui;
public class List extends Displayable {
    public static final int IMPLICIT = 3;
    public List(String title, int listType) { setTitle(title); }
    public int append(String stringPart, Image imagePart) { return 0; }
}
""",
        "javax/microedition/lcdui/Alert.java": """
package javax.microedition.lcdui;
public class Alert extends Displayable {
    public Alert(String title) { setTitle(title); }
    public void setString(String text) {}
    public void setTimeout(int time) {}
}
""",
        "javax/microedition/lcdui/game/GameCanvas.java": """
package javax.microedition.lcdui.game;
import javax.microedition.lcdui.Canvas;
import javax.microedition.lcdui.Graphics;
public class GameCanvas extends Canvas {
    public static final int UP_PRESSED = 2, FIRE_PRESSED = 256;
    protected GameCanvas(boolean suppressKeyEvents) {}
    public Graphics getGraphics() { return new Graphics(); }
    public void flushGraphics() {}
    public int getKeyStates() { return 0; }
    public void paint(Graphics g) {}
}
""",
        "javax/microedition/lcdui/game/Layer.java": """
package javax.microedition.lcdui.game;
import javax.microedition.lcdui.Graphics;
public abstract class Layer {
    public abstract void paint(Graphics g);
    public void setPosition(int x, int y) {}
    public int getWidth() { return 16; }
    public int getHeight() { return 16; }
}
""",
        "javax/microedition/lcdui/game/Sprite.java": """
package javax.microedition.lcdui.game;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;
public class Sprite extends Layer {
    public Sprite(Image image) {}
    public void nextFrame() {}
    public void paint(Graphics g) {}
}
""",
        "javax/microedition/lcdui/game/LayerManager.java": """
package javax.microedition.lcdui.game;
import javax.microedition.lcdui.Graphics;
public class LayerManager {
    public LayerManager() {}
    public void append(Layer l) {}
    public void paint(Graphics g, int x, int y) {}
    public int getSize() { return 1; }
}
""",
        "javax/microedition/rms/RecordStore.java": """
package javax.microedition.rms;
public class RecordStore {
    public static RecordStore openRecordStore(String name, boolean create) { return new RecordStore(); }
    public int addRecord(byte[] data, int offset, int numBytes) { return 1; }
    public int getNumRecords() { return 1; }
    public void closeRecordStore() {}
}
""",
        "javax/microedition/media/Manager.java": """
package javax.microedition.media;
public class Manager {
    public static void playTone(int note, int duration, int volume) {}
    public static String[] getSupportedContentTypes(String protocol) { return new String[0]; }
}
""",
        "javax/microedition/lcdui/Display.java": """
package javax.microedition.lcdui;
import javax.microedition.midlet.MIDlet;
public class Display {
    public static Display getDisplay(MIDlet m) { return new Display(); }
    public void setCurrent(Displayable next) {}
}
""",
        "com/nokia/mid/ui/DeviceControl.java": """
package com.nokia.mid.ui;
public class DeviceControl {
    public static void startVibra(int freq, long duration) {}
    public static void stopVibra() {}
    public static void setLights(int num, int level) {}
}
""",
        "com/sonyericsson/device/Device.java": """
package com.sonyericsson.device;
public class Device {
    public static void vibrate(int duration) {}
    public static String getProperty(String key) { return ""; }
}
""",
        "com/samsung/util/Vibration.java": """
package com.samsung.util;
public class Vibration {
    public static void start(int duration, int strength) {}
    public static boolean isSupported() { return true; }
}
""",
        "com/siemens/mp/game/Vibrator.java": """
package com.siemens.mp.game;
public class Vibrator {
    public static void triggerVibrator(int duration) {}
}
""",
        "com/motorola/funlight/FunLight.java": """
package com.motorola.funlight;
public class FunLight {
    public static int getControl() { return 1; }
}
""",
        "com/lg/util/Vibration.java": """
package com.lg.util;
public class Vibration {
    public static void startVibra(int duration) {}
}
""",
        "com/vodafone/v10/Sound.java": """
package com.vodafone.v10;
public class Sound {
    public static boolean isSupported() { return true; }
}
""",
        "com/sprintpcs/media/Player.java": """
package com.sprintpcs.media;
public class Player {
    public static boolean isSupported() { return true; }
}
""",
    }
    for rel, src in files.items():
        path = stub_dir / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(src.strip() + "\n", encoding="utf-8")


def vendor_line(vendor_call: str) -> str:
    return f"        {vendor_call}\n" if vendor_call else ""


def jar_name_for(vendor_key: str, token: str | None, stem: str) -> str:
    token_part = f"_{token}" if token else "_generic"
    if vendor_key == "lg":
        return f"corpus{token_part}_{stem}.jar"
    return f"corpus{token_part}_{vendor_key}_{stem}.jar"


def family_source(pkg: str, class_name: str, title: str, vendor_call: str, family: str) -> str:
    extra = vendor_line(vendor_call)
    if family == "gamecanvas":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.lcdui.game.GameCanvas;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        PlayCanvas canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    class PlayCanvas extends GameCanvas {{
        PlayCanvas() {{ super(false); }}
        public void paint(Graphics g) {{
            Graphics bg = getGraphics();
            bg.setColor(0x102030);
            bg.fillRect(0, 0, getWidth(), getHeight());
            bg.setColor(0xF0E060);
            bg.fillRect(20, 40, 24, 24);
            flushGraphics();
            getKeyStates();
        }}
    }}
}}
"""
    if family == "sprite":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.lcdui.game.*;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        PlayCanvas canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    class PlayCanvas extends Canvas {{
        public void paint(Graphics g) {{
            Image image = Image.createImage(16, 16);
            Sprite sprite = new Sprite(image);
            sprite.setPosition(12, 24);
            sprite.nextFrame();
            LayerManager layers = new LayerManager();
            layers.append(sprite);
            g.setColor(0x081828);
            g.fillRect(0, 0, getWidth(), getHeight());
            layers.paint(g, 0, 0);
            g.drawImage(image, 8, 8, Graphics.TOP | Graphics.LEFT);
        }}
    }}
}}
"""
    if family == "rms":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.midlet.MIDlet;
import javax.microedition.rms.RecordStore;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        RecordStore store = RecordStore.openRecordStore("s{class_name}", true);
        byte[] payload = new byte[] {{1, 2, 3, 4}};
        store.addRecord(payload, 0, payload.length);
        store.getNumRecords();
        store.closeRecordStore();
        PlayCanvas canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    class PlayCanvas extends Canvas {{
        public void paint(Graphics g) {{
            g.setColor(0x202010);
            g.fillRect(0, 0, getWidth(), getHeight());
            g.setColor(0xFFFFFF);
            g.drawString("{title}", 8, 8, Graphics.TOP | Graphics.LEFT);
        }}
    }}
}}
"""
    if family == "form":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        Form form = new Form("{title}");
        form.append("RustJava API corpus");
        form.size();
        Display.getDisplay(this).setCurrent(form);
{extra}    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
}}
"""
    if family == "list":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        List list = new List("{title}", List.IMPLICIT);
        list.append("Start", null);
        list.append("Scores", null);
        Alert alert = new Alert("{title}");
        alert.setString("ready");
        alert.setTimeout(100);
        Display.getDisplay(this).setCurrent(list);
{extra}    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
}}
"""
    if family == "fontgfx":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        PlayCanvas canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    class PlayCanvas extends Canvas {{
        public void paint(Graphics g) {{
            Font font = Font.getFont(Font.FACE_SYSTEM, Font.STYLE_PLAIN, Font.SIZE_MEDIUM);
            g.setFont(font);
            g.setColor(0x001020);
            g.fillRect(0, 0, getWidth(), getHeight());
            g.setColor(0x40A0FF);
            g.drawLine(0, 40, getWidth(), 40);
            g.fillTriangle(20, 80, 40, 40, 60, 80);
            g.setColor(0xFFFFFF);
            g.drawString("{title}", 8, 8, Graphics.TOP | Graphics.LEFT);
            font.stringWidth("{title}");
            font.getHeight();
        }}
    }}
}}
"""
    if family == "tone":
        return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.media.Manager;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet {{
    public void startApp() {{
        Manager.playTone(69, 40, 80);
        Manager.getSupportedContentTypes(null);
        PlayCanvas canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    class PlayCanvas extends Canvas {{
        public void paint(Graphics g) {{
            g.setColor(0x180018);
            g.fillRect(0, 0, getWidth(), getHeight());
            g.setColor(0xFFCC00);
            g.fillRect(30, 60, 18, 18);
            g.setColor(0xFFFFFF);
            g.drawString("{title}", 8, 8, Graphics.TOP | Graphics.LEFT);
        }}
    }}
}}
"""
    return game_source(pkg, class_name, title, vendor_call)


def game_source(pkg: str, class_name: str, title: str, vendor_call: str) -> str:
    extra = f"        {vendor_call}\n" if vendor_call else ""
    return f"""package {pkg};
import javax.microedition.lcdui.*;
import javax.microedition.midlet.MIDlet;
public class {class_name} extends MIDlet implements Runnable {{
    private PlayCanvas canvas;
    public void startApp() {{
        canvas = new PlayCanvas();
        canvas.setTitle("{title}");
        Display.getDisplay(this).setCurrent(canvas);
{extra}        canvas.repaint();
    }}
    public void pauseApp() {{}}
    public void destroyApp(boolean u) {{}}
    public void run() {{}}
    class PlayCanvas extends Canvas {{
        public void paint(Graphics g) {{
            g.setColor(0x102030);
            g.fillRect(0, 0, getWidth(), getHeight());
            g.setColor(0xF0E060);
            g.fillRect(16 + (getWidth() % 40), 40, 24, 24);
            g.setColor(0xFFFFFF);
            g.drawString("{title}", 8, 8, Graphics.TOP | Graphics.LEFT);
        }}
        public void keyPressed(int keyCode) {{
            getGameAction(keyCode);
            repaint();
        }}
    }}
}}
"""


def main() -> int:
    javac = shutil.which("javac")
    jar = shutil.which("jar")
    if not javac or not jar:
        print("javac/jar not found", file=sys.stderr)
        return 1

    OUT.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="rj-corpus-") as tmp:
        tmp_path = Path(tmp)
        stub_dir = tmp_path / "stubs"
        src_dir = tmp_path / "src"
        classes_dir = tmp_path / "classes"
        stub_dir.mkdir()
        src_dir.mkdir()
        classes_dir.mkdir()
        write_stubs(stub_dir)

        sources: list[Path] = []
        jobs: list[tuple[str, str, str, str]] = []
        for vendor_key, _profile, platform, token, vendor_call in VENDORS:
            for index in range(1, GAMES_PER_VENDOR + 1):
                class_name = f"G{index:02d}"
                pkg = f"corpus.{vendor_key.replace('_', '')}.g{index}"
                title = f"{platform} Mini {index}"
                token_part = f"_{token}" if token else "_generic"
                if vendor_key == "lg":
                    jar_name = f"corpus{token_part}_game_{index:02d}.jar"
                else:
                    jar_name = f"corpus{token_part}_{vendor_key}_game_{index:02d}.jar"
                rel = pkg.replace(".", "/") + f"/{class_name}.java"
                path = src_dir / rel
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(game_source(pkg, class_name, title, vendor_call), encoding="utf-8")
                sources.append(path)
                jobs.append((jar_name, pkg, class_name, title))

        for extra_index in range(1, EXTRA_GAMES + 1):
            vendor_key, _profile, platform, token, vendor_call = VENDORS[(extra_index - 1) % len(VENDORS)]
            family = FAMILIES[(extra_index - 1) % len(FAMILIES)]
            class_name = f"A{extra_index:03d}"
            pkg = f"corpus.api.{vendor_key.replace('_', '')}.{family}.a{extra_index}"
            title = f"{platform} {family.title()} {extra_index}"
            jar_name = jar_name_for(vendor_key, token, f"api_{family}_{extra_index:03d}")
            rel = pkg.replace(".", "/") + f"/{class_name}.java"
            path = src_dir / rel
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(family_source(pkg, class_name, title, vendor_call, family), encoding="utf-8")
            sources.append(path)
            jobs.append((jar_name, pkg, class_name, title))

        compile_cmd = [
            javac,
            "-encoding",
            "UTF-8",
            "--release",
            "8",
            "-cp",
            str(stub_dir),
            "-d",
            str(classes_dir),
        ] + [str(p) for p in sources]
        subprocess.check_call(compile_cmd)

        for jar_name, pkg, class_name, title in jobs:
            manifest = tmp_path / "MANIFEST.MF"
            midlet = f"{pkg}.{class_name}"
            manifest.write_text(
                "\n".join(
                    [
                        "Manifest-Version: 1.0",
                        f"MIDlet-Name: {title}",
                        "MIDlet-Vendor: RustJava Corpus",
                        "MIDlet-Version: 1.0.0",
                        f"MIDlet-1: {title},,{midlet}",
                        "MicroEdition-Configuration: CLDC-1.1",
                        "MicroEdition-Profile: MIDP-2.0",
                        "",
                    ]
                ),
                encoding="utf-8",
            )
            pkg_dir = pkg.replace(".", "/")
            out_jar = OUT / jar_name
            subprocess.check_call(
                [jar, "cfm", str(out_jar), str(manifest), "-C", str(classes_dir), pkg_dir],
                stdout=subprocess.DEVNULL,
            )

    print(f"wrote {len(jobs)} jars to {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
