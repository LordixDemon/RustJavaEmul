package org.rustjava.bench;

import javax.microedition.lcdui.Canvas;
import javax.microedition.lcdui.Graphics;

public class BenchCanvas extends Canvas {
    public String status;
    public String lastScore;

    public BenchCanvas() {
        status = "running";
        lastScore = "";
        setFullScreenMode(true);
    }

    public void paint(Graphics g) {
        int w = getWidth();
        int h = getHeight();
        g.setColor(0x101820);
        g.fillRect(0, 0, w, h);
        g.setColor(0xffffff);
        g.drawString("RustJavaBench", 6, 6, Graphics.LEFT | Graphics.TOP);
        g.setColor(0xa0c0ff);
        g.drawString(status, 6, 22, Graphics.LEFT | Graphics.TOP);
        if (lastScore != null && lastScore.length() > 0) {
            g.setColor(0xd0d0d0);
            String line = lastScore;
            if (line.startsWith("RUSTJAVA_BENCH\t")) {
                line = line.substring(15);
            }
            int y = 40;
            int start = 0;
            int shown = 0;
            while (start < line.length() && shown < 12) {
                int tab = line.indexOf('\t', start);
                if (tab < 0) {
                    tab = line.length();
                }
                g.drawString(line.substring(start, tab), 6, y, Graphics.LEFT | Graphics.TOP);
                y += 14;
                start = tab + 1;
                shown++;
            }
        }
        g.setColor(0x80ff80);
        g.drawString("same JAR on KEmulator / FreeJ2ME / J2ME-Loader", 6, h - 18, Graphics.LEFT | Graphics.TOP);
    }
}
