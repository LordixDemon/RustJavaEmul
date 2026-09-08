package javax.microedition.lcdui.game;

import javax.microedition.lcdui.Graphics;

public abstract class Layer {
    public int getWidth() { return 0; }
    public int getHeight() { return 0; }
    public void setPosition(int x, int y) {}
    public abstract void paint(Graphics g);
}
