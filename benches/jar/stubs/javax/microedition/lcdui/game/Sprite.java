package javax.microedition.lcdui.game;

import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public class Sprite extends Layer {
    public static final int TRANS_NONE = 0;
    public static final int TRANS_ROT90 = 5;
    public Sprite(Image image) {}
    public Sprite(Image image, int frameWidth, int frameHeight) {}
    public void setFrame(int sequenceIndex) {}
    public int getFrame() { return 0; }
    public void nextFrame() {}
    public void setTransform(int transform) {}
    public boolean collidesWith(Sprite s, boolean pixelLevel) { return false; }
    public void paint(Graphics g) {}
}
