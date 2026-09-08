package org.rustjava.bench;

import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;
import javax.microedition.lcdui.game.Sprite;

public class GameTests {
    public static void run(Runner runner) {
        int width = runner.screenW;
        int height = runner.screenH;
        Image sheet = Image.createImage(64, 16);
        Graphics sg = sheet.getGraphics();
        sg.setColor(0xff0000);
        sg.fillRect(0, 0, 16, 16);
        sg.setColor(0x00ff00);
        sg.fillRect(16, 0, 16, 16);
        sg.setColor(0x0000ff);
        sg.fillRect(32, 0, 16, 16);
        sg.setColor(0xffff00);
        sg.fillRect(48, 0, 16, 16);

        final Image target = Image.createImage(width, height);
        final Graphics g = target.getGraphics();
        final Sprite sprite = new Sprite(sheet, 16, 16);
        final Sprite other = new Sprite(sheet, 16, 16);
        other.setPosition(40, 40);

        runner.runTimed(32, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    sprite.setFrame(i & 3);
                    sprite.nextFrame();
                    sprite.setTransform(i & 7);
                    sprite.setPosition((a + i) % (width - 16), (a * 3 + i) % (height - 16));
                    sprite.paint(g);
                    if (sprite.collidesWith(other, false)) {
                        a++;
                    }
                    a ^= sprite.getFrame();
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("game", "sprite", ops, "calls", sink, ms);
            }
        });
    }
}
