package javax.microedition.lcdui;

public class Image {
    public static Image createImage(int width, int height) { return new Image(); }
    public static Image createRGBImage(int[] rgb, int width, int height, boolean processAlpha) { return new Image(); }
    public Graphics getGraphics() { return new Graphics(); }
    public int getWidth() { return 48; }
    public int getHeight() { return 48; }
}
