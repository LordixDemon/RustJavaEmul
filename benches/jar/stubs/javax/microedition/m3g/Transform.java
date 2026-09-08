package javax.microedition.m3g;

public class Transform {
    public Transform() {}
    public void setIdentity() {}
    public void postRotate(float angle, float ax, float ay, float az) {}
    public void postTranslate(float tx, float ty, float tz) {}
    public void postScale(float sx, float sy, float sz) {}
    public void postMultiply(Transform transform) {}
    public void invert() {}
    public void get(float[] matrix) {}
    public void transform(float[] vectors) {}
}
