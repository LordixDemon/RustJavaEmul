package org.rustjava.bench;

import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public class GfxTests {
    public static void run(Runner runner) {
        int width = runner.screenW;
        int height = runner.screenH;
        if (width < 32) {
            width = 240;
        }
        if (height < 32) {
            height = 320;
        }
        Image target = Image.createImage(width, height);
        Graphics g = target.getGraphics();
        Image sprite = makeSprite(48);

        fillRect(runner, g, width, height);
        drawLine(runner, g, width, height);
        fillTriangle(runner, g, width, height);
        drawRgb(runner, g, width, height);
        drawString(runner, g, width, height);
        drawImage(runner, g, sprite, width, height);
        drawRegion(runner, g, sprite, width, height);
    }

    private static Image makeSprite(int size) {
        int[] rgb = new int[size * size];
        for (int y = 0; y < size; y++) {
            for (int x = 0; x < size; x++) {
                int a = ((x + y) & 3) == 0 ? 0x80000000 : 0xff000000;
                rgb[y * size + x] = a | ((x * 5) << 16) | ((y * 7) << 8) | ((x ^ y) & 255);
            }
        }
        return Image.createRGBImage(rgb, size, size, true);
    }

    private static void fillRect(final Runner runner, final Graphics g, final int width, final int height) {
        runner.runTimed(64, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    g.setColor(0x203040 + i);
                    int x = (a + i) % (width - 20);
                    int y = (a * 3 + i) % (height - 20);
                    g.fillRect(x, y, 20, 16);
                    a += x + y;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "fill_rect", ops, "calls", sink, ms);
            }
        });
    }

    private static void drawLine(final Runner runner, final Graphics g, final int width, final int height) {
        runner.runTimed(32, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                g.setColor(0xffffff);
                for (int i = 0; i < batch; i++) {
                    int x2 = (a + i * 13) % width;
                    int y2 = (a * 7 + i) % height;
                    g.drawLine(width / 2, height / 2, x2, y2);
                    a ^= x2 + y2;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "draw_line", ops, "calls", sink, ms);
            }
        });
    }

    private static void fillTriangle(final Runner runner, final Graphics g, final int width, final int height) {
        runner.runTimed(8, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                g.setColor(0xff8040);
                for (int i = 0; i < batch; i++) {
                    int x1 = (a + i) % width;
                    int y1 = 8;
                    int x2 = 8;
                    int y2 = height - 8;
                    int x3 = width - 8;
                    int y3 = (a * 5 + i) % height;
                    g.fillTriangle(x1, y1, x2, y2, x3, y3);
                    a += x1 + y3;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "fill_triangle", ops, "calls", sink, ms);
            }
        });
    }

    private static void drawRgb(final Runner runner, final Graphics g, final int width, final int height) {
        final int[] rgb = new int[48 * 48];
        for (int i = 0; i < rgb.length; i++) {
            rgb[i] = 0x80000000 | (i * 17);
        }
        runner.runTimed(16, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    int x = (a + i * 9) % (width - 48);
                    int y = (a * 3 + i) % (height - 48);
                    g.drawRGB(rgb, 0, 48, x, y, 48, 48, true);
                    a += x + y;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "draw_rgb", ops, "calls", sink, ms);
            }
        });
    }

    private static void drawString(final Runner runner, final Graphics g, final int width, final int height) {
        runner.runTimed(32, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                g.setColor(0xffffff);
                for (int i = 0; i < batch; i++) {
                    int x = (a + i * 11) % (width - 40);
                    int y = 16 + ((a + i) % (height - 32));
                    StringBuffer text = new StringBuffer("SCORE ");
                    text.append(i);
                    g.drawString(text.toString(), x, y, Graphics.LEFT | Graphics.TOP);
                    a ^= x + y + i;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "draw_string", ops, "calls", sink, ms);
            }
        });
    }

    private static void drawImage(final Runner runner, final Graphics g, final Image sprite, final int width, final int height) {
        final int sw = sprite.getWidth();
        final int sh = sprite.getHeight();
        runner.runTimed(32, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    int x = (a + i * 7) % (width - sw);
                    int y = (a * 5 + i) % (height - sh);
                    g.drawImage(sprite, x, y, Graphics.LEFT | Graphics.TOP);
                    a += x + y;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "draw_image", ops, "calls", sink, ms);
            }
        });
    }

    private static void drawRegion(final Runner runner, final Graphics g, final Image sprite, final int width, final int height) {
        final int sw = sprite.getWidth();
        final int sh = sprite.getHeight();
        runner.runTimed(8, new TimedWork() {
            public int run(int sink, int batch) {
                int a = sink;
                for (int i = 0; i < batch; i++) {
                    int transform = i & 7;
                    int x = (a + i * 5) % (width - sw);
                    int y = (a * 3 + i) % (height - sh);
                    g.drawRegion(sprite, 0, 0, sw, sh, transform, x, y, Graphics.LEFT | Graphics.TOP);
                    a += transform + x;
                }
                return a;
            }

            public void finish(Runner r, long ops, int sink, long ms) {
                r.record("gfx", "draw_region", ops, "calls", sink, ms);
            }
        });
    }
}
