# Runtime public API dump

Generated from registered JavaClassProto members.

## com/mascotcapsule/micro3d/v3/ActionTable
- field data [B
- field frameCounts [I
- field valid Z
- field disposed Z
- method <init> ([B)V
- method <init> (Ljava/lang/String;)V
- method dispose ()V
- method getNumAction ()I
- method getNumActions ()I
- method getNumFrame (I)I
- method getNumFrames (I)I

## com/mascotcapsule/micro3d/v3/AffineTrans
- field rotationX I
- field rotationY I
- field rotationZ I
- field m00 I
- field m01 I
- field m02 I
- field m03 I
- field m10 I
- field m11 I
- field m12 I
- field m13 I
- field m20 I
- field m21 I
- field m22 I
- field m23 I
- method <init> ()V
- method <init> (IIIIIIIIIIII)V
- method <init> (Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method <init> ([I)V
- method <init> ([[I)V
- method <init> ([II)V
- method set ([I)V
- method set ([[I)V
- method set ([II)V
- method set (IIIIIIIIIIII)V
- method set (Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method setIdentity ()V
- method rotationX (I)V
- method rotationY (I)V
- method rotationZ (I)V
- method setRotationX (I)V
- method setRotationY (I)V
- method setRotationZ (I)V
- method mul (Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method multiply (Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method mul (Lcom/mascotcapsule/micro3d/v3/AffineTrans;Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method multiply (Lcom/mascotcapsule/micro3d/v3/AffineTrans;Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method get ([I)V
- method get ([II)V
- method transform (Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;
- method transPoint (Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;
- method rotationV (Lcom/mascotcapsule/micro3d/v3/Vector3D;I)V
- method setRotation (Lcom/mascotcapsule/micro3d/v3/Vector3D;I)V
- method lookAt (Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method setViewTrans (Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method rotate (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method scale (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V

## com/mascotcapsule/micro3d/v3/Effect3D
- field NORMAL_SHADING I
- field TOON_SHADING I
- field light Lcom/mascotcapsule/micro3d/v3/Light;
- field shadingType I
- field transparency Z
- field sphereTexture Lcom/mascotcapsule/micro3d/v3/Texture;
- field toonThreshold I
- field toonHigh I
- field toonLow I
- method <clinit> ()V
- method <init> ()V
- method <init> (Lcom/mascotcapsule/micro3d/v3/Light;IZLcom/mascotcapsule/micro3d/v3/Texture;)V
- method getLight ()Lcom/mascotcapsule/micro3d/v3/Light;
- method setLight (Lcom/mascotcapsule/micro3d/v3/Light;)V
- method getShading ()I
- method getShadingType ()I
- method setShading (I)V
- method setShadingType (I)V
- method getThreshold ()I
- method getToonThreshold ()I
- method getThresholdHigh ()I
- method getToonHigh ()I
- method getThresholdLow ()I
- method getToonLow ()I
- method setThreshold (III)V
- method setToonParams (III)V
- method isSemiTransparentEnabled ()Z
- method isTransparency ()Z
- method setSemiTransparentEnabled (Z)V
- method setTransparency (Z)V
- method getSphereMap ()Lcom/mascotcapsule/micro3d/v3/Texture;
- method getSphereTexture ()Lcom/mascotcapsule/micro3d/v3/Texture;
- method setSphereMap (Lcom/mascotcapsule/micro3d/v3/Texture;)V
- method setSphereTexture (Lcom/mascotcapsule/micro3d/v3/Texture;)V

## com/mascotcapsule/micro3d/v3/Figure
- field data [B
- field vertexCount I
- field vertices [S
- field polyC3 [S
- field polyC4 [S
- field polyT3 [S
- field polyT4 [S
- field colors [I
- field patterns [I
- field patternCount I
- field patternSlots I
- field bones [I
- field postureBones [I
- field numPolyC3 I
- field numPolyC4 I
- field numPolyT3 I
- field numPolyT4 I
- field allMatsOr I
- field allMatsAnd I
- field selectedPattern I
- field textureIndex I
- field texture Lcom/mascotcapsule/micro3d/v3/Texture;
- field textures [Lcom/mascotcapsule/micro3d/v3/Texture;
- field postureAction Lcom/mascotcapsule/micro3d/v3/ActionTable;
- field postureActionIndex I
- field postureFrame I
- field disposed Z
- method <init> ([B)V
- method <init> (Ljava/lang/String;)V
- method dispose ()V
- method setPosture (Lcom/mascotcapsule/micro3d/v3/ActionTable;II)V
- method setTexture (Lcom/mascotcapsule/micro3d/v3/Texture;)V
- method getTexture ()Lcom/mascotcapsule/micro3d/v3/Texture;
- method setTexture ([Lcom/mascotcapsule/micro3d/v3/Texture;)V
- method getNumTextures ()I
- method selectTexture (I)V
- method getNumPattern ()I
- method setPattern (I)V

## com/mascotcapsule/micro3d/v3/FigureLayout
- field affineTrans Lcom/mascotcapsule/micro3d/v3/AffineTrans;
- field affineArray [Lcom/mascotcapsule/micro3d/v3/AffineTrans;
- field affineMatrix [I
- field centerX I
- field centerY I
- field near I
- field far I
- field perspective I
- field projection I
- field projectionMode I
- field parallelWidth I
- field parallelHeight I
- field scaleX I
- field scaleY I
- method <init> ()V
- method <init> (Lcom/mascotcapsule/micro3d/v3/AffineTrans;IIII)V
- method getAffineTrans ()Lcom/mascotcapsule/micro3d/v3/AffineTrans;
- method setAffineTrans (Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method setAffineTrans ([Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method setAffineTransArray ([Lcom/mascotcapsule/micro3d/v3/AffineTrans;)V
- method selectAffineTrans (I)V
- method getCenterX ()I
- method getCenterY ()I
- method setCenter (II)V
- method setPerspective (III)V
- method setPerspective (IIII)V
- method getParallelWidth ()I
- method getParallelHeight ()I
- method setParallelSize (II)V
- method getScaleX ()I
- method getScaleY ()I
- method setScale (II)V

## com/mascotcapsule/micro3d/v3/Graphics3D
- field COMMAND_LIST_VERSION_1_0 I
- field COMMAND_END I
- field COMMAND_NOP I
- field COMMAND_FLUSH I
- field COMMAND_ATTRIBUTE I
- field COMMAND_CLIP I
- field COMMAND_CENTER I
- field COMMAND_TEXTURE_INDEX I
- field COMMAND_AFFINE_INDEX I
- field COMMAND_PARALLEL_SCALE I
- field COMMAND_PARALLEL_SIZE I
- field COMMAND_PERSPECTIVE_FOV I
- field COMMAND_PERSPECTIVE_WH I
- field COMMAND_AMBIENT_LIGHT I
- field COMMAND_DIRECTION_LIGHT I
- field COMMAND_THRESHOLD I
- field ENV_ATTR_LIGHTING I
- field ENV_ATTR_SPHERE_MAP I
- field ENV_ATTR_TOON_SHADING I
- field ENV_ATTR_SEMI_TRANSPARENT I
- field PATTR_LIGHTING I
- field PATTR_SPHERE_MAP I
- field PATTR_COLORKEY I
- field PATTR_BLEND_NORMAL I
- field PATTR_BLEND_HALF I
- field PATTR_BLEND_ADD I
- field PATTR_BLEND_SUB I
- field PDATA_NORMAL_NONE I
- field PDATA_NORMAL_PER_FACE I
- field PDATA_NORMAL_PER_VERTEX I
- field PDATA_COLOR_NONE I
- field PDATA_COLOR_PER_COMMAND I
- field PDATA_COLOR_PER_FACE I
- field PDATA_TEXURE_COORD_NONE I
- field PDATA_TEXURE_COORD I
- field PDATA_POINT_SPRITE_PARAMS_PER_CMD I
- field PDATA_POINT_SPRITE_PARAMS_PER_FACE I
- field PDATA_POINT_SPRITE_PARAMS_PER_VERTEX I
- field POINT_SPRITE_LOCAL_SIZE I
- field POINT_SPRITE_PIXEL_SIZE I
- field POINT_SPRITE_PERSPECTIVE I
- field POINT_SPRITE_NO_PERS I
- field PRIMITVE_POINTS I
- field PRIMITVE_LINES I
- field PRIMITVE_TRIANGLES I
- field PRIMITVE_QUADS I
- field PRIMITVE_POINT_SPRITES I
- field boundGraphics Ljavax/microedition/lcdui/Graphics;
- field renderCount I
- method <clinit> ()V
- method <init> ()V
- method getInstance ()Lcom/mascotcapsule/micro3d/v3/Graphics3D;
- method bind (Ljavax/microedition/lcdui/Graphics;)V
- method release (Ljavax/microedition/lcdui/Graphics;)V
- method flush ()V
- method dispose ()V
- method renderFigure (Lcom/mascotcapsule/micro3d/v3/Figure;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;)V
- method drawFigure (Lcom/mascotcapsule/micro3d/v3/Figure;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;)V
- method renderPrimitives (Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;II[I[I[I[I)V
- method drawCommandList (Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;[I)V
- method drawCommandList ([Lcom/mascotcapsule/micro3d/v3/Texture;IILcom/mascotcapsule/micro3d/v3/FigureLayout;Lcom/mascotcapsule/micro3d/v3/Effect3D;[I)V

## com/mascotcapsule/micro3d/v3/Light
- field direction Lcom/mascotcapsule/micro3d/v3/Vector3D;
- field ambientIntensity I
- field diffuseIntensity I
- method <init> ()V
- method <init> (Lcom/mascotcapsule/micro3d/v3/Vector3D;II)V
- method getAmbIntensity ()I
- method getAmbientIntensity ()I
- method setAmbIntensity (I)V
- method setAmbientIntensity (I)V
- method getDirection ()Lcom/mascotcapsule/micro3d/v3/Vector3D;
- method getParallelLightDirection ()Lcom/mascotcapsule/micro3d/v3/Vector3D;
- method setDirection (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method setParallelLightDirection (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method getDirIntensity ()I
- method getParallelLightIntensity ()I
- method setDirIntensity (I)V
- method setParallelLightIntensity (I)V

## com/mascotcapsule/micro3d/v3/Texture
- field resourceName Ljava/lang/String;
- field mipMap Z
- field isForModel Z
- field isSphere Z
- field width I
- field height I
- field pixels [I
- field indices [B
- field palette [I
- field colorKey I
- field disposed Z
- method <init> (Ljava/lang/String;Z)V
- method <init> ([BZ)V
- method dispose ()V

## com/mascotcapsule/micro3d/v3/Util3D
- method sqrt (I)I
- method sin (I)I
- method cos (I)I

## com/mascotcapsule/micro3d/v3/Vector3D
- field x I
- field y I
- field z I
- method <init> ()V
- method <init> (III)V
- method <init> (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method getX ()I
- method getY ()I
- method getZ ()I
- method set (III)V
- method set (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method setX (I)V
- method setY (I)V
- method setZ (I)V
- method unit ()V
- method innerProduct (Lcom/mascotcapsule/micro3d/v3/Vector3D;)I
- method innerProduct (Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)I
- method outerProduct (Lcom/mascotcapsule/micro3d/v3/Vector3D;)V
- method outerProduct (Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)Lcom/mascotcapsule/micro3d/v3/Vector3D;
- method outerProduct (Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;Lcom/mascotcapsule/micro3d/v3/Vector3D;)V

## com/nokia/mid/sound/Sound
- field state I
- field gain I
- field listener Lcom/nokia/mid/sound/SoundListener;
- field SOUND_PLAYING I
- field SOUND_STOPPED I
- field SOUND_UNINITIALIZED I
- field FORMAT_TONE I
- field FORMAT_WAV I
- method <clinit> ()V
- method <init> (II)V
- method <init> (IJ)V
- method <init> ([BI)V
- method init (II)V
- method init ([BI)V
- method play (I)V
- method stop ()V
- method resume ()V
- method release ()V
- method getState ()I
- method getGain ()I
- method setGain (I)I
- method setGain (I)V
- method setSoundListener (Lcom/nokia/mid/sound/SoundListener;)V
- method getConcurrentSoundCount (I)I
- method getSupportedFormats ()[I
- method init (IJ)V
- method init (IJ)V

## com/nokia/mid/sound/SoundListener

## com/nokia/mid/ui/DeviceControl
- method flashLights (J)V
- method setLights (II)V
- method startVibra (IJ)V
- method stopVibra ()V
- method resetUserInactivityTime ()V
- method resetUserInactivityTime ()V

## com/nokia/mid/ui/DirectGraphics
- field argbColor I
- field graphics Ljavax/microedition/lcdui/Graphics;
- field TYPE_USHORT_4444_ARGB I
- field TYPE_USHORT_444_RGB I
- field TYPE_USHORT_555_RGB I
- field TYPE_USHORT_1555_ARGB I
- field TYPE_USHORT_565_RGB I
- field TYPE_INT_888_RGB I
- field TYPE_INT_8888_ARGB I
- field TYPE_BYTE_1_GRAY I
- field TYPE_BYTE_1_GRAY_VERTICAL I
- field TYPE_BYTE_2_GRAY I
- field TYPE_BYTE_4_GRAY I
- field TYPE_BYTE_8_GRAY I
- field TYPE_BYTE_332_RGB I
- field FLIP_HORIZONTAL I
- field FLIP_VERTICAL I
- field ROTATE_90 I
- field ROTATE_180 I
- field ROTATE_270 I
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljavax/microedition/lcdui/Graphics;)V
- method setARGBColor (I)V
- method getAlphaComponent ()I
- method getNativePixelFormat ()I
- method drawPixels ([B[BIIIIIIII)V
- method drawPixels ([S[BIIIIIIII)V
- method drawPixels ([I[BIIIIIIII)V
- method getPixels ([B[BIIIIIII)V
- method getPixels ([S[BIIIIIII)V
- method getPixels ([I[BIIIIIII)V
- method drawPolygon ([I[III)V
- method fillPolygon ([I[III)V
- method drawTriangle (IIIIII)V
- method fillTriangle (IIIIII)V
- method drawImage (Ljavax/microedition/lcdui/Image;IIII)V
- method drawPixels ([IZIIIIIIII)V
- method drawPixels ([SZIIIIIIII)V
- method fillPolygon ([II[IIII)V
- method fillTriangle (IIIIIII)V
- method drawHighlight (IIIII)V
- method drawHighlight (IIIII)V
- method drawPolygon ([II[IIII)V
- method drawPolygon ([II[IIII)V
- method drawTriangle (IIIIIII)V
- method drawTriangle (IIIIIII)V
- method getPixels ([IIIIIIII)V
- method getPixels ([IIIIIIII)V
- method getPixels ([SIIIIIII)V
- method getPixels ([SIIIIIII)V

## com/nokia/mid/ui/DirectUtils
- method getDirectGraphics (Ljavax/microedition/lcdui/Graphics;)Lcom/nokia/mid/ui/DirectGraphics;
- method createImage (II)Ljavax/microedition/lcdui/Image;
- method createImage (III)Ljavax/microedition/lcdui/Image;
- method createImage (Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;
- method createImage ([BII)Ljavax/microedition/lcdui/Image;
- method createImage ([BII)Ljavax/microedition/lcdui/Image;
- method getFont (III)Ljavax/microedition/lcdui/Font;
- method getFont (III)Ljavax/microedition/lcdui/Font;
- method setHeader (Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;Ljavax/microedition/lcdui/Image;III)Z
- method setHeader (Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;Ljavax/microedition/lcdui/Image;III)Z

## com/nokia/mid/ui/FullCanvas
- field KEY_UP_ARROW I
- field KEY_DOWN_ARROW I
- field KEY_LEFT_ARROW I
- field KEY_RIGHT_ARROW I
- field KEY_SOFTKEY1 I
- field KEY_SOFTKEY2 I
- field KEY_SOFTKEY3 I
- method <clinit> ()V
- method <init> ()V

## com/siemens/mp/game/Light
- method setLightOn ()V
- method setLightOff ()V
- method <init> ()V

## com/siemens/mp/game/Vibrator
- method triggerVibrator (I)V
- method stopVibrator ()V
- method startVibrator ()V
- method startVibrator ()V

## com/siemens/mp/game/Sound
- method playTone (II)V
- method stopTone ()V

## com/siemens/mp/game/Melody
- method <init> ([B)V
- method play ()V
- method stop ()V

## com/siemens/mp/game/GraphicObject
- field visible Z
- method <init> ()V
- method setVisible (Z)V
- method getVisible ()Z

## com/siemens/mp/game/Sprite
- field x I
- field y I
- field frame I
- method <init> ([BII)V
- method setPosition (II)V
- method getXPosition ()I
- method getYPosition ()I
- method setFrame (I)V
- method getFrame ()I

## com/siemens/mp/ui/Image
- method createImageFromBitmap ([BII)Ljavax/microedition/lcdui/Image;
- method createImageWithoutScaling (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method createImageWithoutScaling (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method createRGBImage ([BIII)Ljavax/microedition/lcdui/Image;
- method createRGBImage ([BIII)Ljavax/microedition/lcdui/Image;
- method mirrorImageHorizontally (Ljavax/microedition/lcdui/Image;)V
- method mirrorImageHorizontally (Ljavax/microedition/lcdui/Image;)V
- method mirrorImageVertically (Ljavax/microedition/lcdui/Image;)V
- method mirrorImageVertically (Ljavax/microedition/lcdui/Image;)V

## com/siemens/mp/io/file/FileConnection
- method <init> ()V
- method openDataInputStream ()Ljava/io/DataInputStream;
- method availableSize ()J
- method availableSize ()J
- method canRead ()Z
- method canRead ()Z
- method canWrite ()Z
- method canWrite ()Z
- method close ()V
- method close ()V
- method create ()V
- method create ()V
- method delete ()V
- method delete ()V
- method directorySize (Z)J
- method directorySize (Z)J
- method exists ()Z
- method exists ()Z
- method fileSize ()J
- method fileSize ()J
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method isDirectory ()Z
- method isDirectory ()Z
- method isHidden ()Z
- method isHidden ()Z
- method isOpen ()Z
- method isOpen ()Z
- method lastModified ()J
- method lastModified ()J
- method list ()Ljava/util/Enumeration;
- method list ()Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method mkdir ()V
- method mkdir ()V
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method rename (Ljava/lang/String;)V
- method rename (Ljava/lang/String;)V
- method setFileConnection (Ljava/lang/String;)V
- method setFileConnection (Ljava/lang/String;)V
- method setHidden (Z)V
- method setHidden (Z)V
- method setReadable (Z)V
- method setReadable (Z)V
- method setWritable (Z)V
- method setWritable (Z)V
- method setWriteable (Z)V
- method setWriteable (Z)V
- method totalSize ()J
- method totalSize ()J
- method truncate (J)V
- method truncate (J)V
- method usedSize ()J
- method usedSize ()J

## com/siemens/mp/lcdui/Command
- method <init> (Ljava/lang/String;IIC)V
- method <init> (Ljava/lang/String;II)V
- method <init> (Ljava/lang/String;Ljava/lang/String;II)V
- method <init> (Ljava/lang/String;Ljava/lang/String;IIC)V

## com/siemens/mp/lcdui/Image
- method writeImageToFile (Ljavax/microedition/lcdui/Image;Ljava/lang/String;I)V
- method clipAndScaleImage (Ljavax/microedition/lcdui/Image;IIIIII)Ljavax/microedition/lcdui/Image;
- method clipAndScaleImage (Ljavax/microedition/lcdui/Image;IIIIII)Ljavax/microedition/lcdui/Image;
- method createImageFromFile (Ljava/lang/String;II)Ljavax/microedition/lcdui/Image;
- method createImageFromFile (Ljava/lang/String;II)Ljavax/microedition/lcdui/Image;
- method createImageFromFile (Ljava/lang/String;Z)Ljavax/microedition/lcdui/Image;
- method createImageFromFile (Ljava/lang/String;Z)Ljavax/microedition/lcdui/Image;
- method getPixelColor (Ljavax/microedition/lcdui/Image;II)I
- method getPixelColor (Ljavax/microedition/lcdui/Image;II)I
- method setPixelColor (Ljavax/microedition/lcdui/Image;III)V
- method setPixelColor (Ljavax/microedition/lcdui/Image;III)V
- method writeBmpToFile (Ljavax/microedition/lcdui/Image;Ljava/lang/String;)V
- method writeBmpToFile (Ljavax/microedition/lcdui/Image;Ljava/lang/String;)V

## com/siemens/mp/media/Manager
- method createPlayer (Ljava/io/InputStream;Ljava/lang/String;)Lcom/siemens/mp/media/Player;
- method createPlayer (Ljava/lang/String;)Lcom/siemens/mp/media/Player;
- method createPlayer (Ljava/lang/String;)Lcom/siemens/mp/media/Player;
- method getSupportedContentTypes (Ljava/lang/String;)[Ljava/lang/String;
- method getSupportedContentTypes (Ljava/lang/String;)[Ljava/lang/String;
- method getSupportedProtocols (Ljava/lang/String;)[Ljava/lang/String;
- method getSupportedProtocols (Ljava/lang/String;)[Ljava/lang/String;
- method playTone (III)V
- method playTone (III)V

## com/siemens/mp/media/MediaException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## com/siemens/mp/media/Player
- field state I
- method <init> ()V
- method getState ()I
- method prefetch ()V
- method start ()V
- method stop ()V
- method addPlayerListener (Lcom/siemens/mp/media/PlayerListener;)V
- method addPlayerListener (Lcom/siemens/mp/media/PlayerListener;)V
- method close ()V
- method close ()V
- method deallocate ()V
- method deallocate ()V
- method getContentType ()Ljava/lang/String;
- method getContentType ()Ljava/lang/String;
- method getControl (Ljava/lang/String;)Lcom/siemens/mp/media/Control;
- method getControl (Ljava/lang/String;)Lcom/siemens/mp/media/Control;
- method getDuration ()J
- method getDuration ()J
- method getMediaTime ()J
- method getMediaTime ()J
- method realize ()V
- method realize ()V
- method removePlayerListener (Lcom/siemens/mp/media/PlayerListener;)V
- method removePlayerListener (Lcom/siemens/mp/media/PlayerListener;)V
- method setLoopCount (I)V
- method setLoopCount (I)V
- method setMediaTime (J)J
- method setMediaTime (J)J

## com/siemens/mp/resource/Resource
- method getCenterKeyIcon (I)C
- method getColor (I)I
- method getColor (I)I
- method getSpecialCharacter (I)C
- method getSpecialCharacter (I)C

## com/siemens/mp/wireless/messaging/MessageConnection
- method <init> ()V
- method close ()V
- method close ()V
- method newMessage (Ljava/lang/String;)Ljavax/wireless/messaging/Message;
- method newMessage (Ljava/lang/String;)Ljavax/wireless/messaging/Message;
- method send (Ljavax/wireless/messaging/Message;)V
- method send (Ljavax/wireless/messaging/Message;)V

## com/siemens/mp/wireless/messaging/MessagePart
- method <init> (Ljava/io/InputStream;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method <init> ([BLjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method getLength ()I
- method getLength ()I

## com/siemens/mp/wireless/messaging/MultipartMessage
- method <init> ()V
- method addAddress (Ljava/lang/String;Ljava/lang/String;)Z
- method addMessagePart (Lcom/siemens/mp/wireless/messaging/MessagePart;)V
- method setSubject (Ljava/lang/String;)V
- method removeAddresses ()V
- method removeAddresses ()V
- method setAddress (Ljava/lang/String;)V
- method setAddress (Ljava/lang/String;)V

## com/samsung/util/Vibration
- method start (II)V
- method stop ()V
- method isSupported ()Z
- method <init> ()V
- method startVibration (II)V
- method startVibration (II)V

## com/samsung/util/AudioClip
- method <init> (ILjava/lang/String;)V
- method <init> (I[BII)V
- method play (II)V
- method stop ()V
- method pause ()V
- method resume ()V
- method isSupported ()Z
- method isSupported ()Z

## com/motorola/funlight/FunLight
- field BLANK I
- field OFF I
- field ON I
- field BLACK I
- field BLUE I
- field CYAN I
- field GREEN I
- field MAGENTA I
- field RED I
- field WHITE I
- field YELLOW I
- method <clinit> ()V
- method getControl ()I
- method releaseControl ()V
- method getRegion (I)Lcom/motorola/funlight/FunLightRegion;
- method getRegions ()[Lcom/motorola/funlight/FunLightRegion;
- method getControl ()V
- method getControl ()V
- method getRegion (I)Lcom/motorola/funlight/Region;
- method getRegion (I)Lcom/motorola/funlight/Region;
- method getRegions ()[Lcom/motorola/funlight/Region;
- method getRegions ()[Lcom/motorola/funlight/Region;
- method getRegionsIDs ()[I
- method getRegionsIDs ()[I

## com/motorola/funlight/FunLightRegion
- field id I
- field color I
- method <init> (I)V
- method getID ()I
- method getColor ()I
- method setColor (I)I
- method getControl ()I
- method releaseControl ()V

## com/lg/util/Vibration
- method startVibra (I)V
- method stopVibra ()V

## com/sonyericsson/device/Device
- method vibrate (I)V
- method getProperty (Ljava/lang/String;)Ljava/lang/String;

## com/vodafone/v10/Sound
- method <init> ()V
- method play ()V
- method stop ()V
- method isSupported ()Z

## com/vodafone/util/ImageEncoder
- method <init> ()V
- method createEncoder (I)Lcom/vodafone/util/ImageEncoder;
- method encodeOffscreen (Ljavax/microedition/lcdui/Image;IIII)[B

## com/sprintpcs/media/Player
- method <init> ()V
- method play ()V
- method play (Lcom/sprintpcs/media/Clip;I)V
- method stop ()V
- method pause ()V
- method isSupported ()Z
- method playBackground (Lcom/sprintpcs/media/Clip;I)V
- method playBackground (Lcom/sprintpcs/media/Clip;I)V

## com/sprintpcs/util/Muglet
- method <init> ()V
- method getMuglet ()Lcom/sprintpcs/util/Muglet;
- method getURI ()Ljava/lang/String;

## com/sprintpcs/media/Clip
- method <init> ([BLjava/lang/String;II)V
- method <init> (Ljava/lang/String;Ljava/lang/String;II)V

## com/sprintpcs/media/Vibrator
- method vibrate (I)V

## com/sprintpcs/util/System
- method setExitURI (Ljava/lang/String;)V
- method setSystemSetting (Ljava/lang/String;Ljava/lang/String;)V
- method setSystemSetting (Ljava/lang/String;Ljava/lang/String;)V

## java/io/BufferedInputStream
- method <init> (Ljava/io/InputStream;)V
- method <init> (Ljava/io/InputStream;I)V

## java/io/BufferedReader
- field in Ljava/io/Reader;
- field buf [C
- field bufSize I
- method <init> (Ljava/io/Reader;)V
- method readLine ()Ljava/lang/String;
- method close ()V
- method <init> (Ljava/io/Reader;I)V

## java/io/BufferedWriter
- field out Ljava/io/Writer;
- method <init> (Ljava/io/Writer;)V
- method write ([CII)I
- method newLine ()V
- method flush ()V
- method close ()V

## java/io/ByteArrayInputStream
- field buf [B
- field pos I
- field count I
- field mark I
- method <init> ([B)V
- method <init> ([BII)V
- method available ()I
- method read ([BII)I
- method read ()I
- method close ()V
- method skip (J)J
- method mark (I)V
- method markSupported ()Z
- method reset ()V

## java/io/ByteArrayOutputStream
- field buf [B
- field pos I
- field count I
- method <init> ()V
- method <init> (I)V
- method write (I)V
- method write ([BII)V
- method toByteArray ()[B
- method size ()I
- method reset ()V
- method close ()V
- method yopish ()V
- method yopish ()V
- method закрыть ()V
- method закрыть ()V

## java/io/DataInput
- method readBoolean ()Z
- method readByte ()B
- method readChar ()C
- method readDouble ()D
- method readFloat ()F
- method readFully ([B)V
- method readFully ([BII)V
- method readInt ()I
- method readLong ()J
- method readShort ()S
- method readUnsignedByte ()I
- method readUnsignedShort ()I
- method readUTF ()Ljava/lang/String;
- method skipBytes (I)I

## java/io/DataInputStream
- method <init> (Ljava/io/InputStream;)V
- method readBoolean ()Z
- method readByte ()B
- method readChar ()C
- method readDouble ()D
- method readFloat ()F
- method readFully ([B)V
- method readFully ([BII)V
- method readInt ()I
- method readLong ()J
- method readShort ()S
- method readUnsignedByte ()I
- method readUnsignedShort ()I
- method readUTF ()Ljava/lang/String;
- method readUTF (Ljava/io/DataInput;)Ljava/lang/String;
- method skipBytes (I)I

## java/io/DataOutput
- method write (I)V
- method writeByte (I)V
- method writeBoolean (Z)V
- method writeInt (I)V
- method writeShort (I)V
- method writeLong (J)V
- method writeFloat (F)V
- method writeDouble (D)V
- method writeChars (Ljava/lang/String;)V
- method writeUTF (Ljava/lang/String;)V
- method close ()V
- method flush ()V
- method write ([B)V
- method write ([B)V
- method write ([BII)V
- method write ([BII)V
- method writeChar (I)V
- method writeChar (I)V

## java/io/DataOutputStream
- method <init> (Ljava/io/OutputStream;)V
- method write (I)V
- method writeByte (I)V
- method writeBoolean (Z)V
- method writeInt (I)V
- method writeShort (I)V
- method writeChar (I)V
- method writeLong (J)V
- method writeFloat (F)V
- method writeDouble (D)V
- method writeChars (Ljava/lang/String;)V
- method writeUTF (Ljava/lang/String;)V
- method close ()V
- method flush ()V
- method writeBytes (Ljava/lang/String;)V
- method writeBytes (Ljava/lang/String;)V
- method yopish ()V
- method yopish ()V

## java/io/EOFException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/File
- field path Ljava/lang/String;
- field separator Ljava/lang/String;
- field separatorChar C
- field pathSeparator Ljava/lang/String;
- method <clinit> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/io/File;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method getPath ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getParent ()Ljava/lang/String;
- method getParentFile ()Ljava/io/File;
- method getAbsolutePath ()Ljava/lang/String;
- method getAbsoluteFile ()Ljava/io/File;
- method isAbsolute ()Z
- method exists ()Z
- method isDirectory ()Z
- method isFile ()Z
- method delete ()Z
- method length ()J
- method list ()[Ljava/lang/String;
- method listRoots ()[Ljava/io/File;
- method <init> (Ljava/net/URI;)V
- method canRead ()Z
- method canRead ()Z
- method canWrite ()Z
- method canWrite ()Z
- method compareTo (Ljava/io/File;)I
- method compareTo (Ljava/io/File;)I
- method createNewFile ()Z
- method createNewFile ()Z
- method createTempFile (Ljava/lang/String;Ljava/lang/String;)Ljava/io/File;
- method createTempFile (Ljava/lang/String;Ljava/lang/String;)Ljava/io/File;
- method createTempFile (Ljava/lang/String;Ljava/lang/String;Ljava/io/File;)Ljava/io/File;
- method createTempFile (Ljava/lang/String;Ljava/lang/String;Ljava/io/File;)Ljava/io/File;
- method deleteOnExit ()V
- method deleteOnExit ()V
- method getCanonicalFile ()Ljava/io/File;
- method getCanonicalFile ()Ljava/io/File;
- method getCanonicalPath ()Ljava/lang/String;
- method getCanonicalPath ()Ljava/lang/String;
- method isHidden ()Z
- method isHidden ()Z
- method lastModified ()J
- method lastModified ()J
- method list (Ljava/io/FilenameFilter;)[Ljava/lang/String;
- method list (Ljava/io/FilenameFilter;)[Ljava/lang/String;
- method listFiles ()[Ljava/io/File;
- method listFiles ()[Ljava/io/File;
- method listFiles (Ljava/io/FileFilter;)[Ljava/io/File;
- method listFiles (Ljava/io/FileFilter;)[Ljava/io/File;
- method mkdir ()Z
- method mkdir ()Z
- method mkdirs ()Z
- method mkdirs ()Z
- method renameTo (Ljava/io/File;)Z
- method renameTo (Ljava/io/File;)Z
- method setLastModified (J)Z
- method setLastModified (J)Z
- method toPath ()Ljava/nio/file/Path;
- method toPath ()Ljava/nio/file/Path;
- method toURI ()Ljava/net/URI;
- method toURI ()Ljava/net/URI;

## java/io/FileDescriptor
- field fd I
- field err Ljava/io/FileDescriptor;
- field in Ljava/io/FileDescriptor;
- field out Ljava/io/FileDescriptor;
- method <init> ()V
- method <clinit> ()V
- method sync ()V
- method sync ()V

## java/io/FileInputStream
- field fd Ljava/io/FileDescriptor;
- field in Ljava/io/InputStream;
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/io/FileDescriptor;)V
- method read ()I
- method read ([BII)I
- method available ()I
- method close ()V
- method getChannel ()Ljava/nio/channels/FileChannel;
- method getChannel ()Ljava/nio/channels/FileChannel;

## java/io/FileNotFoundException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/FileOutputStream
- field fd Ljava/io/FileDescriptor;
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Z)V
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/io/File;Z)V
- method <init> (Ljava/io/FileDescriptor;)V
- method write ([BII)V
- method write (I)V
- method close ()V
- method getChannel ()Ljava/nio/channels/FileChannel;
- method getChannel ()Ljava/nio/channels/FileChannel;
- method getFD ()Ljava/io/FileDescriptor;
- method getFD ()Ljava/io/FileDescriptor;

## java/io/FilterInputStream
- field in Ljava/io/InputStream;
- method <init> (Ljava/io/InputStream;)V
- method available ()I
- method close ()V
- method read ()I
- method read ([B)I
- method read ([BII)I
- method reset ()V
- method skip (J)J
- method mark (I)V
- method markSupported ()Z

## java/io/FilterOutputStream
- field out Ljava/io/OutputStream;
- method <init> (Ljava/io/OutputStream;)V
- method write ([BII)V
- method write (I)V

## java/io/InputStream
- method <init> ()V
- method available ()I
- method read ([BII)I
- method read ([B)I
- method read ()I
- method close ()V
- method skip (J)J
- method mark (I)V
- method markSupported ()Z
- method reset ()V
- method 0 ()V
- method 0 ()V
- method yopish ()V
- method yopish ()V
- method Закрыть ()V
- method Закрыть ()V
- method закрыть ()V
- method закрыть ()V
- method не показывать (J)J
- method не показывать (J)J
- method читать ([BII)I
- method читать ([BII)I
- method чтение ([BII)I
- method чтение ([BII)I

## java/io/InputStreamReader
- field in Ljava/io/InputStream;
- field readBuf [B
- field readBufSize I
- field writeBuf [C
- field writeBufSize I
- field charset Ljava/lang/String;
- method <init> (Ljava/io/InputStream;)V
- method <init> (Ljava/io/InputStream;Ljava/lang/String;)V
- method read ()I
- method read ([CII)I
- method close ()V
- method skip (J)J
- method <init> (Ljava/io/InputStream;Ljava/nio/charset/Charset;)V
- method mark (I)V
- method mark (I)V
- method markSupported ()Z
- method markSupported ()Z
- method ready ()Z
- method ready ()Z
- method reset ()V
- method reset ()V

## java/io/InterruptedIOException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/IOException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/Throwable;)V

## java/io/UnsupportedEncodingException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/UTFDataFormatException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/OutputStream
- method <init> ()V
- method write ([B)V
- method write ([BII)V
- method write (I)V
- method flush ()V
- method close ()V
- method 0 ()V
- method 0 ()V
- method yopish ()V
- method yopish ()V
- method Закрыть ()V
- method Закрыть ()V
- method закрыть ()V
- method закрыть ()V
- method написать ([BII)V
- method написать ([BII)V

## java/io/OutputStreamWriter
- field out Ljava/io/OutputStream;
- method <init> (Ljava/io/OutputStream;)V
- method <init> (Ljava/io/OutputStream;Ljava/lang/String;)V
- method write ([CII)I
- method flush ()V
- method close ()V
- method <init> (Ljava/io/OutputStream;Ljava/nio/charset/Charset;)V
- method <init> (Ljava/io/OutputStream;Ljava/nio/charset/CharsetEncoder;)V
- method write (Ljava/lang/String;II)V
- method write (Ljava/lang/String;II)V
- method write ([C)V
- method write ([C)V
- method write ([CII)V
- method write ([CII)V

## java/io/PrintStream
- method <init> (Ljava/io/OutputStream;)V
- method <init> (Ljava/io/OutputStream;Z)V
- method print (Ljava/lang/String;)V
- method print (C)V
- method print (Ljava/lang/Object;)V
- method print (D)V
- method print (I)V
- method print (J)V
- method print (Z)V
- method checkError ()Z
- method println ()V
- method println (Ljava/lang/Object;)V
- method println (Ljava/lang/String;)V
- method println (I)V
- method println (J)V
- method println (C)V
- method println (B)V
- method println (S)V
- method println (Z)V
- method println (D)V
- method println (F)V
- method println ([C)V
- method printf (Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintStream;
- method delete (Ljava/lang/StringBuffer;II)Ljava/lang/StringBuffer;
- method delete (Ljava/lang/StringBuffer;II)Ljava/lang/StringBuffer;
- method getResourceAsStream (Ljava/lang/Class;Ljava/lang/String;)Ljava/io/InputStream;
- method getResourceAsStream (Ljava/lang/Class;Ljava/lang/String;)Ljava/io/InputStream;

## java/io/PrintWriter
- field out Ljava/io/Writer;
- method <init> (Ljava/io/Writer;)V
- method <init> (Ljava/io/Writer;Z)V
- method <init> (Ljava/io/OutputStream;)V
- method <init> (Ljava/io/OutputStream;Z)V
- method write ([CII)I
- method print (Ljava/lang/String;)V
- method println ()V
- method println (Ljava/lang/String;)V
- method println (Ljava/lang/Object;)V
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/io/PrintWriter;

## java/io/RandomAccessFile
- field fd Ljava/io/FileDescriptor;
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/io/File;Ljava/lang/String;)V
- method read ([B)I
- method read ([BII)I
- method write ([B)V
- method write ([BII)V
- method length ()J
- method getFilePointer ()J
- method getFD ()Ljava/io/FileDescriptor;
- method seek (J)V
- method setLength (J)V
- method close ()V
- method getChannel ()Ljava/nio/channels/FileChannel;
- method getChannel ()Ljava/nio/channels/FileChannel;
- method read ()I
- method read ()I

## java/io/Reader
- field lock Ljava/lang/Object;
- field lock Ljava/lang/Object;
- method <init> ()V
- method read ()I
- method read ([C)I
- method read ([CII)I
- method close ()V
- method <init> (Ljava/lang/Object;)V
- method mark (I)V
- method mark (I)V
- method markSupported ()Z
- method markSupported ()Z
- method read (Ljava/nio/CharBuffer;)I
- method read (Ljava/nio/CharBuffer;)I
- method ready ()Z
- method ready ()Z
- method reset ()V
- method reset ()V
- method skip (J)J
- method skip (J)J

## java/io/Serializable

## java/io/StringWriter
- field buf Ljava/lang/StringBuffer;
- method <init> ()V
- method write ([CII)I
- method toString ()Ljava/lang/String;
- method <init> (I)V
- method getBuffer ()Ljava/lang/StringBuffer;
- method getBuffer ()Ljava/lang/StringBuffer;
- method write ([CII)V
- method write ([CII)V

## java/io/Writer
- field lock Ljava/lang/Object;
- field lock Ljava/lang/Object;
- method <init> ()V
- method write (I)V
- method write ([C)V
- method write ([CII)V
- method write ([CII)I
- method write (Ljava/lang/String;)V
- method flush ()V
- method close ()V
- method <init> (Ljava/lang/Object;)V
- method append (C)Ljava/io/Writer;
- method append (C)Ljava/io/Writer;
- method append (Ljava/lang/CharSequence;)Ljava/io/Writer;
- method append (Ljava/lang/CharSequence;)Ljava/io/Writer;
- method append (Ljava/lang/CharSequence;II)Ljava/io/Writer;
- method append (Ljava/lang/CharSequence;II)Ljava/io/Writer;
- method write (Ljava/lang/String;II)V
- method write (Ljava/lang/String;II)V

## java/security/MessageDigest
- field algorithm Ljava/lang/String;
- field buffer [B
- field count I
- method <init> (Ljava/lang/String;)V
- method getInstance (Ljava/lang/String;)Ljava/security/MessageDigest;
- method update ([BII)V
- method digest ([BII)I
- method digest ()[B
- method digest ()[B
- method digest ([B)[B
- method digest ([B)[B
- method reset ()V
- method reset ()V

## java/security/NoSuchAlgorithmException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/AbstractMethodError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/ArithmeticException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/ArrayIndexOutOfBoundsException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (I)V

## java/lang/ArrayStoreException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Boolean
- field value Z
- field TRUE Ljava/lang/Boolean;
- field FALSE Ljava/lang/Boolean;
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (Z)V
- method <init> (Ljava/lang/String;)V
- method booleanValue ()Z
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (Z)Ljava/lang/String;
- method valueOf (Z)Ljava/lang/Boolean;
- method valueOf (Ljava/lang/String;)Ljava/lang/Boolean;
- method getBoolean (Ljava/lang/String;)Z
- method getBoolean (Ljava/lang/String;)Z
- method parseBoolean (Ljava/lang/String;)Z
- method parseBoolean (Ljava/lang/String;)Z

## java/lang/Byte
- field value B
- field MIN_VALUE B
- field MAX_VALUE B
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (B)V
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (B)Ljava/lang/String;
- method parseByte (Ljava/lang/String;)B
- method valueOf (B)Ljava/lang/Byte;
- method parseByte (Ljava/lang/String;I)B
- method parseByte (Ljava/lang/String;I)B

## java/lang/Character
- field value C
- field MIN_VALUE C
- field MAX_VALUE C
- field MIN_RADIX I
- field MAX_RADIX I
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (C)V
- method charValue ()C
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method digit (CI)I
- method isDigit (C)Z
- method isLowerCase (C)Z
- method isUpperCase (C)Z
- method toLowerCase (C)C
- method toUpperCase (C)C
- method valueOf (C)Ljava/lang/Character;
- method charCount (I)I
- method charCount (I)I
- method codePointAt (Ljava/lang/CharSequence;I)I
- method codePointAt (Ljava/lang/CharSequence;I)I
- method getType (C)I
- method getType (C)I
- method isHighSurrogate (C)Z
- method isHighSurrogate (C)Z
- method isLetter (C)Z
- method isLetter (C)Z
- method isLetterOrDigit (C)Z
- method isLetterOrDigit (C)Z
- method isSpaceChar (C)Z
- method isSpaceChar (C)Z
- method isTitleCase (C)Z
- method isTitleCase (C)Z
- method isWhitespace (C)Z
- method isWhitespace (C)Z
- method toChars (I)[C
- method toChars (I)[C
- method toString (C)Ljava/lang/String;
- method toString (C)Ljava/lang/String;
- method toTitleCase (C)C
- method toTitleCase (C)C

## java/lang/Class
- field nameBytes [B
- field classLoader Ljava/lang/ClassLoader;
- method <init> ()V
- method getName ()Ljava/lang/String;
- method isAssignableFrom (Ljava/lang/Class;)Z
- method isInterface ()Z
- method isArray ()Z
- method isInstance (Ljava/lang/Object;)Z
- method getSuperclass ()Ljava/lang/Class;
- method getModifiers ()I
- method newInstance ()Ljava/lang/Object;
- method getResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method forName (Ljava/lang/String;)Ljava/lang/Class;
- method cast (Ljava/lang/Object;)Ljava/lang/Object;
- method cast (Ljava/lang/Object;)Ljava/lang/Object;
- method delete (Ljava/lang/StringBuffer;II)Ljava/lang/StringBuffer;
- method delete (Ljava/lang/StringBuffer;II)Ljava/lang/StringBuffer;
- method desiredAssertionStatus ()Z
- method desiredAssertionStatus ()Z
- method forName (Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
- method forName (Ljava/lang/String;ZLjava/lang/ClassLoader;)Ljava/lang/Class;
- method getAnnotation (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
- method getAnnotation (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
- method getClassLoader ()Ljava/lang/ClassLoader;
- method getClassLoader ()Ljava/lang/ClassLoader;
- method getComponentType ()Ljava/lang/Class;
- method getComponentType ()Ljava/lang/Class;
- method getConstructor ([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;
- method getConstructor ([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;
- method getConstructors ()[Ljava/lang/reflect/Constructor;
- method getConstructors ()[Ljava/lang/reflect/Constructor;
- method getDeclaredConstructor ([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;
- method getDeclaredConstructor ([Ljava/lang/Class;)Ljava/lang/reflect/Constructor;
- method getDeclaredField (Ljava/lang/String;)Ljava/lang/reflect/Field;
- method getDeclaredField (Ljava/lang/String;)Ljava/lang/reflect/Field;
- method getDeclaredFields ()[Ljava/lang/reflect/Field;
- method getDeclaredFields ()[Ljava/lang/reflect/Field;
- method getDeclaredMethod (Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;
- method getDeclaredMethod (Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;
- method getDeclaredMethods ()[Ljava/lang/reflect/Method;
- method getDeclaredMethods ()[Ljava/lang/reflect/Method;
- method getEnclosingClass ()Ljava/lang/Class;
- method getEnclosingClass ()Ljava/lang/Class;
- method getEnumConstants ()[Ljava/lang/Object;
- method getEnumConstants ()[Ljava/lang/Object;
- method getField (Ljava/lang/String;)Ljava/lang/reflect/Field;
- method getField (Ljava/lang/String;)Ljava/lang/reflect/Field;
- method getFields ()[Ljava/lang/reflect/Field;
- method getFields ()[Ljava/lang/reflect/Field;
- method getGenericInterfaces ()[Ljava/lang/reflect/Type;
- method getGenericInterfaces ()[Ljava/lang/reflect/Type;
- method getGenericSuperclass ()Ljava/lang/reflect/Type;
- method getGenericSuperclass ()Ljava/lang/reflect/Type;
- method getInterfaces ()[Ljava/lang/Class;
- method getInterfaces ()[Ljava/lang/Class;
- method getMethod (Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;
- method getMethod (Ljava/lang/String;[Ljava/lang/Class;)Ljava/lang/reflect/Method;
- method getMethods ()[Ljava/lang/reflect/Method;
- method getMethods ()[Ljava/lang/reflect/Method;
- method getPackage ()Ljava/lang/Package;
- method getPackage ()Ljava/lang/Package;
- method getProtectionDomain ()Ljava/security/ProtectionDomain;
- method getProtectionDomain ()Ljava/security/ProtectionDomain;
- method getResource (Ljava/lang/String;)Ljava/net/URL;
- method getResource (Ljava/lang/String;)Ljava/net/URL;
- method getResourceAsStream (Ljava/lang/Class;Ljava/lang/String;)Ljava/io/InputStream;
- method getResourceAsStream (Ljava/lang/Class;Ljava/lang/String;)Ljava/io/InputStream;
- method getSimpleName ()Ljava/lang/String;
- method getSimpleName ()Ljava/lang/String;
- method getTypeParameters ()[Ljava/lang/reflect/TypeVariable;
- method getTypeParameters ()[Ljava/lang/reflect/TypeVariable;
- method isAnnotation ()Z
- method isAnnotation ()Z
- method isAnonymousClass ()Z
- method isAnonymousClass ()Z
- method isEnum ()Z
- method isEnum ()Z
- method isLocalClass ()Z
- method isLocalClass ()Z
- method isMemberClass ()Z
- method isMemberClass ()Z
- method isPrimitive ()Z
- method isPrimitive ()Z

## java/lang/ClassCastException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/ClassFormatError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/ClassLoader
- field systemClassLoader Ljava/lang/ClassLoader;
- field parent Ljava/lang/ClassLoader;
- method <init> (Ljava/lang/ClassLoader;)V
- method loadClass (Ljava/lang/String;)Ljava/lang/Class;
- method findClass (Ljava/lang/String;)Ljava/lang/Class;
- method findLoadedClass (Ljava/lang/String;)Ljava/lang/Class;
- method getSystemClassLoader ()Ljava/lang/ClassLoader;
- method getResource (Ljava/lang/String;)Ljava/net/URL;
- method getResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method findResource (Ljava/lang/String;)Ljava/net/URL;
- method defineClass (Ljava/lang/String;[BII)Ljava/lang/Class;
- method getSystemResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method getSystemResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method loadClass (Ljava/lang/String;Z)Ljava/lang/Class;
- method loadClass (Ljava/lang/String;Z)Ljava/lang/Class;
- method resolveClass (Ljava/lang/Class;)V
- method resolveClass (Ljava/lang/Class;)V

## java/lang/ClassNotFoundException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Cloneable

## java/lang/CloneNotSupportedException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Comparable
- method compareTo (Ljava/lang/Object;)I
- method compareTo (Ljava/lang/Object;)I

## java/lang/Double
- field value D
- field NaN D
- field POSITIVE_INFINITY D
- field NEGATIVE_INFINITY D
- field MIN_VALUE D
- field MAX_VALUE D
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (D)V
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (D)Ljava/lang/String;
- method parseDouble (Ljava/lang/String;)D
- method valueOf (D)Ljava/lang/Double;
- method valueOf (Ljava/lang/String;)Ljava/lang/Double;
- method isNaN ()Z
- method isNaN (D)Z
- method isInfinite ()Z
- method isInfinite (D)Z
- method doubleToLongBits (D)J
- method longBitsToDouble (J)D
- method compare (DD)I
- method compare (DD)I
- method doubleToRawLongBits (D)J
- method doubleToRawLongBits (D)J

## java/lang/Error
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/Throwable;)V

## java/lang/Exception
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V

## java/lang/ExceptionInInitializerError
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/Throwable;)V
- method getException ()Ljava/lang/Throwable;

## java/lang/Float
- field value F
- field NaN F
- field POSITIVE_INFINITY F
- field NEGATIVE_INFINITY F
- field MAX_VALUE F
- field MIN_VALUE F
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (F)V
- method <init> (Ljava/lang/String;)V
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method floatToIntBits (F)I
- method intBitsToFloat (I)F
- method isNaN (F)Z
- method isNaN ()Z
- method isInfinite (F)Z
- method isInfinite ()Z
- method parseFloat (Ljava/lang/String;)F
- method toString ()Ljava/lang/String;
- method toString (F)Ljava/lang/String;
- method valueOf (F)Ljava/lang/Float;
- method valueOf (Ljava/lang/String;)Ljava/lang/Float;
- method <init> (D)V
- method compare (FF)I
- method compare (FF)I
- method floatToRawIntBits (F)I
- method floatToRawIntBits (F)I

## java/lang/IllegalAccessException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/IllegalArgumentException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V

## java/lang/IllegalMonitorStateException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/IllegalStateException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/Throwable;)V

## java/lang/InstantiationError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/InstantiationException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/IncompatibleClassChangeError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/IndexOutOfBoundsException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Integer
- field value I
- field MIN_VALUE I
- field MAX_VALUE I
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (I)V
- method parseInt (Ljava/lang/String;)I
- method parseInt (Ljava/lang/String;I)I
- method valueOf (I)Ljava/lang/Integer;
- method valueOf (Ljava/lang/String;)Ljava/lang/Integer;
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method toString ()Ljava/lang/String;
- method <init> (Ljava/lang/String;)V
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString (I)Ljava/lang/String;
- method toString (II)Ljava/lang/String;
- method toBinaryString (I)Ljava/lang/String;
- method toHexString (I)Ljava/lang/String;
- method toOctalString (I)Ljava/lang/String;
- method valueOf (Ljava/lang/String;I)Ljava/lang/Integer;
- method compareTo (Ljava/lang/Integer;)I
- method decode (Ljava/lang/String;)Ljava/lang/Integer;
- method highestOneBit (I)I
- method cinitclone ()V
- method cinitclone ()V

## java/lang/InternalError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/InterruptedException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/LinkageError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Long
- field value J
- field MIN_VALUE J
- field MAX_VALUE J
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (J)V
- method <init> (Ljava/lang/String;)V
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (J)Ljava/lang/String;
- method toString (JI)Ljava/lang/String;
- method parseLong (Ljava/lang/String;)J
- method parseLong (Ljava/lang/String;I)J
- method valueOf (J)Ljava/lang/Long;
- method valueOf (Ljava/lang/String;)Ljava/lang/Long;
- method decode (Ljava/lang/String;)Ljava/lang/Long;
- method decode (Ljava/lang/String;)Ljava/lang/Long;
- method toHexString (J)Ljava/lang/String;
- method toHexString (J)Ljava/lang/String;

## java/lang/Math
- field E D
- field PI D
- method <clinit> ()V
- method abs (I)I
- method abs (J)J
- method abs (F)F
- method abs (D)D
- method max (II)I
- method max (JJ)J
- method max (FF)F
- method max (DD)D
- method min (II)I
- method min (JJ)J
- method min (FF)F
- method min (DD)D
- method sqrt (D)D
- method sin (D)D
- method cos (D)D
- method tan (D)D
- method asin (D)D
- method acos (D)D
- method atan (D)D
- method atan2 (DD)D
- method floor (D)D
- method ceil (D)D
- method round (F)I
- method round (D)J
- method pow (DD)D
- method exp (D)D
- method log (D)D
- method toRadians (D)D
- method toDegrees (D)D
- method copySign (DD)D
- method copySign (FF)F
- method log10 (D)D
- method random ()D

## java/lang/NegativeArraySizeException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/NoClassDefFoundError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/NoSuchFieldError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/NoSuchMethodError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/NullPointerException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Number
- method <init> ()V
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method byteValue ()B
- method shortValue ()S

## java/lang/NumberFormatException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Object
- method <init> ()V
- method getClass ()Ljava/lang/Class;
- method hashCode ()I
- method equals (Ljava/lang/Object;)Z
- method clone ()Ljava/lang/Object;
- method toString ()Ljava/lang/String;
- method notify ()V
- method notifyAll ()V
- method wait (J)V
- method wait (JI)V
- method wait ()V
- method finalize ()V
- method <init> (Ljava/util/Vector;)V
- method toPNG (J)V
- method toPNG (J)V

## java/lang/OutOfMemoryError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/reflect/Array
- method newInstance (Ljava/lang/Class;[I)Ljava/lang/Object;
- method get (Ljava/lang/Object;I)Ljava/lang/Object;
- method get (Ljava/lang/Object;I)Ljava/lang/Object;
- method getBoolean (Ljava/lang/Object;I)Z
- method getBoolean (Ljava/lang/Object;I)Z
- method getByte (Ljava/lang/Object;I)B
- method getByte (Ljava/lang/Object;I)B
- method getChar (Ljava/lang/Object;I)C
- method getChar (Ljava/lang/Object;I)C
- method getDouble (Ljava/lang/Object;I)D
- method getDouble (Ljava/lang/Object;I)D
- method getFloat (Ljava/lang/Object;I)F
- method getFloat (Ljava/lang/Object;I)F
- method getInt (Ljava/lang/Object;I)I
- method getInt (Ljava/lang/Object;I)I
- method getLength (Ljava/lang/Object;)I
- method getLength (Ljava/lang/Object;)I
- method getLong (Ljava/lang/Object;I)J
- method getLong (Ljava/lang/Object;I)J
- method getShort (Ljava/lang/Object;I)S
- method getShort (Ljava/lang/Object;I)S
- method newInstance (Ljava/lang/Class;I)Ljava/lang/Object;
- method newInstance (Ljava/lang/Class;I)Ljava/lang/Object;
- method set (Ljava/lang/Object;ILjava/lang/Object;)V
- method set (Ljava/lang/Object;ILjava/lang/Object;)V
- method setBoolean (Ljava/lang/Object;IZ)V
- method setBoolean (Ljava/lang/Object;IZ)V
- method setByte (Ljava/lang/Object;IB)V
- method setByte (Ljava/lang/Object;IB)V
- method setChar (Ljava/lang/Object;IC)V
- method setChar (Ljava/lang/Object;IC)V
- method setDouble (Ljava/lang/Object;ID)V
- method setDouble (Ljava/lang/Object;ID)V
- method setFloat (Ljava/lang/Object;IF)V
- method setFloat (Ljava/lang/Object;IF)V
- method setInt (Ljava/lang/Object;II)V
- method setInt (Ljava/lang/Object;II)V
- method setLong (Ljava/lang/Object;IJ)V
- method setLong (Ljava/lang/Object;IJ)V
- method setShort (Ljava/lang/Object;IS)V
- method setShort (Ljava/lang/Object;IS)V

## java/lang/Runnable
- method run ()V

## java/lang/Runtime
- method <init> ()V
- method getRuntime ()Ljava/lang/Runtime;
- method totalMemory ()J
- method freeMemory ()J
- method gc ()V
- method exit (I)V
- method halt (I)V
- method availableProcessors ()I
- method addShutdownHook (Ljava/lang/Thread;)V
- method addShutdownHook (Ljava/lang/Thread;)V
- method exec (Ljava/lang/String;)Ljava/lang/Process;
- method exec (Ljava/lang/String;)Ljava/lang/Process;
- method exec ([Ljava/lang/String;)Ljava/lang/Process;
- method exec ([Ljava/lang/String;)Ljava/lang/Process;
- method exec ([Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;
- method exec ([Ljava/lang/String;[Ljava/lang/String;Ljava/io/File;)Ljava/lang/Process;
- method removeShutdownHook (Ljava/lang/Thread;)Z
- method removeShutdownHook (Ljava/lang/Thread;)Z

## java/lang/RuntimeException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V

## java/lang/SecurityException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Short
- field value S
- field MIN_VALUE S
- field MAX_VALUE S
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <clinit> ()V
- method <init> (S)V
- method byteValue ()B
- method shortValue ()S
- method intValue ()I
- method longValue ()J
- method floatValue ()F
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (S)Ljava/lang/String;
- method parseShort (Ljava/lang/String;)S
- method parseShort (Ljava/lang/String;I)S
- method valueOf (S)Ljava/lang/Short;

## java/lang/String
- field value [C
- method <init> ()V
- method <init> ([B)V
- method <init> ([C)V
- method <init> ([CII)V
- method <init> ([BII)V
- method <init> ([BLjava/lang/String;)V
- method <init> ([BIILjava/lang/String;)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/StringBuffer;)V
- method equals (Ljava/lang/Object;)Z
- method equalsIgnoreCase (Ljava/lang/String;)Z
- method compareTo (Ljava/lang/String;)I
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method charAt (I)C
- method getBytes ()[B
- method getBytes (Ljava/lang/String;)[B
- method getChars (II[CI)V
- method toCharArray ()[C
- method toUpperCase ()Ljava/lang/String;
- method toLowerCase ()Ljava/lang/String;
- method length ()I
- method concat (Ljava/lang/String;)Ljava/lang/String;
- method substring (I)Ljava/lang/String;
- method substring (II)Ljava/lang/String;
- method replace (CC)Ljava/lang/String;
- method regionMatches (ZILjava/lang/String;II)Z
- method valueOf (Z)Ljava/lang/String;
- method valueOf (C)Ljava/lang/String;
- method valueOf (I)Ljava/lang/String;
- method valueOf (J)Ljava/lang/String;
- method valueOf (F)Ljava/lang/String;
- method valueOf (D)Ljava/lang/String;
- method valueOf ([C)Ljava/lang/String;
- method valueOf ([CII)Ljava/lang/String;
- method valueOf (Ljava/lang/Object;)Ljava/lang/String;
- method indexOf (I)I
- method indexOf (II)I
- method indexOf (Ljava/lang/String;)I
- method indexOf (Ljava/lang/String;I)I
- method lastIndexOf (I)I
- method lastIndexOf (II)I
- method trim ()Ljava/lang/String;
- method startsWith (Ljava/lang/String;)Z
- method startsWith (Ljava/lang/String;I)Z
- method endsWith (Ljava/lang/String;)Z
- method intern ()Ljava/lang/String;
- method isEmpty ()Z
- method lastIndexOf (Ljava/lang/String;)I
- method lastIndexOf (Ljava/lang/String;I)I
- method contains (Ljava/lang/CharSequence;)Z
- method compareToIgnoreCase (Ljava/lang/String;)I
- method toLowerCase (Ljava/util/Locale;)Ljava/lang/String;
- method toUpperCase (Ljava/util/Locale;)Ljava/lang/String;
- method replace (Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Ljava/lang/String;
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
- method format (Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
- method <init> ([BLjava/nio/charset/Charset;)V
- method codePointAt (I)I
- method codePointAt (I)I
- method getBytes (Ljava/nio/charset/Charset;)[B
- method getBytes (Ljava/nio/charset/Charset;)[B
- method matches (Ljava/lang/String;)Z
- method matches (Ljava/lang/String;)Z
- method replaceAll (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method replaceAll (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method replaceFirst (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method replaceFirst (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method split (Ljava/lang/String;)[Ljava/lang/String;
- method split (Ljava/lang/String;)[Ljava/lang/String;
- method split (Ljava/lang/String;I)[Ljava/lang/String;
- method split (Ljava/lang/String;I)[Ljava/lang/String;
- method Длина ()I
- method Длина ()I
- method длина ()I
- method длина ()I
- method подстрока (I)Ljava/lang/String;
- method подстрока (I)Ljava/lang/String;
- method подстрока (II)Ljava/lang/String;
- method подстрока (II)Ljava/lang/String;

## java/lang/StringBuffer
- field value [C
- field count I
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/lang/String;)V
- method append (Ljava/lang/String;)Ljava/lang/StringBuffer;
- method append (Ljava/lang/Object;)Ljava/lang/StringBuffer;
- method append (Z)Ljava/lang/StringBuffer;
- method append (I)Ljava/lang/StringBuffer;
- method append (J)Ljava/lang/StringBuffer;
- method append (C)Ljava/lang/StringBuffer;
- method append (F)Ljava/lang/StringBuffer;
- method append (D)Ljava/lang/StringBuffer;
- method append ([CII)Ljava/lang/StringBuffer;
- method append ([C)Ljava/lang/StringBuffer;
- method capacity ()I
- method ensureCapacity (I)V
- method reverse ()Ljava/lang/StringBuffer;
- method getChars (II[CI)V
- method delete (II)Ljava/lang/StringBuffer;
- method deleteCharAt (I)Ljava/lang/StringBuffer;
- method insert (ILjava/lang/String;)Ljava/lang/StringBuffer;
- method insert (IC)Ljava/lang/StringBuffer;
- method insert (II)Ljava/lang/StringBuffer;
- method insert (IJ)Ljava/lang/StringBuffer;
- method insert (ILjava/lang/Object;)Ljava/lang/StringBuffer;
- method toString ()Ljava/lang/String;
- method setLength (I)V
- method length ()I
- method charAt (I)C
- method setCharAt (IC)V
- method indexOf (Ljava/lang/String;)I
- method indexOf (Ljava/lang/String;)I
- method indexOf (Ljava/lang/String;I)I
- method indexOf (Ljava/lang/String;I)I
- method insert (IZ)Ljava/lang/StringBuffer;
- method insert (IZ)Ljava/lang/StringBuffer;
- method insert (I[C)Ljava/lang/StringBuffer;
- method insert (I[C)Ljava/lang/StringBuffer;
- method replace (IILjava/lang/String;)Ljava/lang/StringBuffer;
- method replace (IILjava/lang/String;)Ljava/lang/StringBuffer;
- method substring (II)Ljava/lang/String;
- method substring (II)Ljava/lang/String;

## java/lang/StringIndexOutOfBoundsException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (I)V

## java/lang/System
- field out Ljava/io/PrintStream;
- field err Ljava/io/PrintStream;
- field props Ljava/util/Properties;
- field in Ljava/io/InputStream;
- field in Ljava/io/InputStream;
- field Выход Ljava/io/PrintStream;
- field Выход Ljava/io/PrintStream;
- field из  Ljava/io/PrintStream;
- field из  Ljava/io/PrintStream;
- method <clinit> ()V
- method currentTimeMillis ()J
- method gc ()V
- method arraycopy (Ljava/lang/Object;ILjava/lang/Object;II)V
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method getProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method getProperties ()Ljava/util/Properties;
- method setProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;
- method exit (I)V
- method identityHashCode (Ljava/lang/Object;)I
- method nanoTime ()J
- method setOut (Ljava/io/PrintStream;)V
- method setErr (Ljava/io/PrintStream;)V
- method getInstance ()J
- method getInstance ()J
- method getenv (Ljava/lang/String;)Ljava/lang/String;
- method getenv (Ljava/lang/String;)Ljava/lang/String;
- method load (Ljava/lang/String;)V
- method load (Ljava/lang/String;)V
- method loadLibrary (Ljava/lang/String;)V
- method loadLibrary (Ljava/lang/String;)V
- method mapLibraryName (Ljava/lang/String;)Ljava/lang/String;
- method mapLibraryName (Ljava/lang/String;)Ljava/lang/String;
- method setProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method setProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method Вихід (I)V
- method Вихід (I)V

## java/lang/Thread
- field id J
- field target Ljava/lang/Runnable;
- field alive Z
- field priority I
- field name Ljava/lang/String;
- field interrupted Z
- field MIN_PRIORITY I
- field NORM_PRIORITY I
- field MAX_PRIORITY I
- method <init> ()V
- method <clinit> ()V
- method <init> (Ljava/lang/Runnable;)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/Runnable;Ljava/lang/String;)V
- method start ()V
- method join ()V
- method run ()V
- method isAlive ()Z
- method sleep (J)V
- method yield ()V
- method setPriority (I)V
- method getPriority ()I
- method setName (Ljava/lang/String;)V
- method getName ()Ljava/lang/String;
- method interrupt ()V
- method isInterrupted ()Z
- method interrupted ()Z
- method activeCount ()I
- method join (J)V
- method currentThread ()Ljava/lang/Thread;
- method <init> (Z)V
- method <init> (Ljava/lang/ThreadGroup;Ljava/lang/Runnable;Ljava/lang/String;J)V
- method getContextClassLoader ()Ljava/lang/ClassLoader;
- method getContextClassLoader ()Ljava/lang/ClassLoader;
- method getThreadGroup ()Ljava/lang/ThreadGroup;
- method getThreadGroup ()Ljava/lang/ThreadGroup;
- method setDaemon (Z)V
- method setDaemon (Z)V
- method setDefaultUncaughtExceptionHandler (Ljava/lang/Thread$UncaughtExceptionHandler;)V
- method setDefaultUncaughtExceptionHandler (Ljava/lang/Thread$UncaughtExceptionHandler;)V
- method setUncaughtExceptionHandler (Ljava/lang/Thread$UncaughtExceptionHandler;)V
- method setUncaughtExceptionHandler (Ljava/lang/Thread$UncaughtExceptionHandler;)V
- method Старт ()V
- method Старт ()V

## java/lang/Throwable
- field detailMessage Ljava/lang/String;
- field cause Ljava/lang/Throwable;
- field stackTrace [Ljava/lang/String;
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/Throwable;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V
- method getMessage ()Ljava/lang/String;
- method getCause ()Ljava/lang/Throwable;
- method initCause (Ljava/lang/Throwable;)Ljava/lang/Throwable;
- method toString ()Ljava/lang/String;
- method fillInStackTrace ()Ljava/lang/Throwable;
- method printStackTrace ()V
- method printStackTrace (Ljava/io/PrintStream;)V
- method printStackTrace (Ljava/io/PrintWriter;)V
- method addSuppressed (Ljava/lang/Throwable;)V
- method addSuppressed (Ljava/lang/Throwable;)V
- method getStackTrace ()[Ljava/lang/StackTraceElement;
- method getStackTrace ()[Ljava/lang/StackTraceElement;

## java/lang/UnknownError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/UnsatisfiedLinkError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/UnsupportedOperationException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/VerifyError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/VirtualMachineError
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/net/HttpURLConnection
- field method Ljava/lang/String;
- method <init> (Ljava/net/URL;)V
- method setRequestMethod (Ljava/lang/String;)V
- method connect ()V
- method connect ()V
- method disconnect ()V
- method disconnect ()V
- method getContentLength ()I
- method getContentLength ()I
- method getErrorStream ()Ljava/io/InputStream;
- method getErrorStream ()Ljava/io/InputStream;
- method getHeaderField (I)Ljava/lang/String;
- method getHeaderField (I)Ljava/lang/String;
- method getHeaderFieldDate (Ljava/lang/String;J)J
- method getHeaderFieldDate (Ljava/lang/String;J)J
- method getHeaderFieldKey (I)Ljava/lang/String;
- method getHeaderFieldKey (I)Ljava/lang/String;
- method getRequestMethod ()Ljava/lang/String;
- method getRequestMethod ()Ljava/lang/String;
- method getResponseCode ()I
- method getResponseCode ()I
- method getResponseMessage ()Ljava/lang/String;
- method getResponseMessage ()Ljava/lang/String;
- method setConnectTimeout (I)V
- method setConnectTimeout (I)V
- method setDefaultUseCaches (Z)V
- method setDefaultUseCaches (Z)V
- method setDoInput (Z)V
- method setDoInput (Z)V
- method setReadTimeout (I)V
- method setReadTimeout (I)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setUseCaches (Z)V
- method setUseCaches (Z)V

## java/net/JarURLConnection
- field fileUrl Ljava/net/URL;
- field entry Ljava/lang/String;
- method <init> (Ljava/net/URL;)V
- method getJarFile ()Ljava/util/jar/JarFile;
- method getEntryName ()Ljava/lang/String;
- method getJarFileURL ()Ljava/net/URL;
- method getJarEntry ()Ljava/util/jar/JarEntry;
- method getMainAttributes ()Ljava/util/jar/Attributes;

## java/net/MalformedURLException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/io/UnknownServiceException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/net/URL
- field protocol Ljava/lang/String;
- field host Ljava/lang/String;
- field port I
- field file Ljava/lang/String;
- field handler Ljava/net/URLStreamHandler;
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/net/URL;Ljava/lang/String;)V
- method <init> (Ljava/net/URL;Ljava/lang/String;Ljava/net/URLStreamHandler;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V
- method openConnection ()Ljava/net/URLConnection;
- method openStream ()Ljava/io/InputStream;
- method set (Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V
- method getPort ()I
- method getProtocol ()Ljava/lang/String;
- method getHost ()Ljava/lang/String;
- method getFile ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getQuery ()Ljava/lang/String;
- method getQuery ()Ljava/lang/String;
- method getRef ()Ljava/lang/String;
- method getRef ()Ljava/lang/String;
- method openConnection (Ljava/net/Proxy;)Ljava/net/URLConnection;
- method openConnection (Ljava/net/Proxy;)Ljava/net/URLConnection;
- method toExternalForm ()Ljava/lang/String;
- method toExternalForm ()Ljava/lang/String;
- method toURI ()Ljava/net/URI;
- method toURI ()Ljava/net/URI;

## java/net/URLClassLoader
- field urls [Ljava/net/URL;
- method <init> ([Ljava/net/URL;Ljava/lang/ClassLoader;)V
- method findClass (Ljava/lang/String;)Ljava/lang/Class;
- method findResource (Ljava/lang/String;)Ljava/net/URL;

## java/net/URLConnection
- field url Ljava/net/URL;
- field doOutput Z
- field outputStream Ljava/io/OutputStream;
- method <init> (Ljava/net/URL;)V
- method getInputStream ()Ljava/io/InputStream;
- method getOutputStream ()Ljava/io/OutputStream;
- method setDoOutput (Z)V
- method getContentEncoding ()Ljava/lang/String;
- method getContentEncoding ()Ljava/lang/String;
- method getContentLength ()I
- method getContentLength ()I
- method getContentType ()Ljava/lang/String;
- method getContentType ()Ljava/lang/String;
- method getDate ()J
- method getDate ()J
- method getExpiration ()J
- method getExpiration ()J
- method getHeaderField (Ljava/lang/String;)Ljava/lang/String;
- method getHeaderField (Ljava/lang/String;)Ljava/lang/String;
- method getHeaderFieldInt (Ljava/lang/String;I)I
- method getHeaderFieldInt (Ljava/lang/String;I)I
- method getLastModified ()J
- method getLastModified ()J
- method getRequestProperty (Ljava/lang/String;)Ljava/lang/String;
- method getRequestProperty (Ljava/lang/String;)Ljava/lang/String;
- method getURL ()Ljava/net/URL;
- method getURL ()Ljava/net/URL;
- method setConnectTimeout (I)V
- method setConnectTimeout (I)V
- method setDoInput (Z)V
- method setDoInput (Z)V
- method setReadTimeout (I)V
- method setReadTimeout (I)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V

## java/net/URLEncoder
- method encode (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;

## java/net/URLStreamHandler
- method <init> ()V
- method openConnection (Ljava/net/URL;)Ljava/net/URLConnection;
- method parseURL (Ljava/net/URL;Ljava/lang/String;II)V
- method setURL (Ljava/net/URL;Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V

## java/nio/Buffer
- field position I
- method <init> ()V
- method position (I)Ljava/nio/Buffer;
- method rewind ()Ljava/nio/Buffer;
- method capacity ()I
- method capacity ()I
- method clear ()Ljava/nio/Buffer;
- method clear ()Ljava/nio/Buffer;
- method flip ()Ljava/nio/Buffer;
- method flip ()Ljava/nio/Buffer;
- method limit ()I
- method limit ()I
- method limit (I)Ljava/nio/Buffer;
- method limit (I)Ljava/nio/Buffer;
- method position ()I
- method position ()I
- method remaining ()I
- method remaining ()I

## java/nio/ByteBuffer
- field data [B
- method <init> (I)V
- method allocateDirect (I)Ljava/nio/ByteBuffer;
- method position (I)Ljava/nio/Buffer;
- method putInt (I)Ljava/nio/ByteBuffer;
- method putShort (S)Ljava/nio/ByteBuffer;
- method rewind ()Ljava/nio/Buffer;
- method allocate (I)Ljava/nio/ByteBuffer;
- method allocate (I)Ljava/nio/ByteBuffer;
- method array ()[B
- method array ()[B
- method asCharBuffer ()Ljava/nio/CharBuffer;
- method asCharBuffer ()Ljava/nio/CharBuffer;
- method asDoubleBuffer ()Ljava/nio/DoubleBuffer;
- method asDoubleBuffer ()Ljava/nio/DoubleBuffer;
- method asFloatBuffer ()Ljava/nio/FloatBuffer;
- method asFloatBuffer ()Ljava/nio/FloatBuffer;
- method asIntBuffer ()Ljava/nio/IntBuffer;
- method asIntBuffer ()Ljava/nio/IntBuffer;
- method asLongBuffer ()Ljava/nio/LongBuffer;
- method asLongBuffer ()Ljava/nio/LongBuffer;
- method asShortBuffer ()Ljava/nio/ShortBuffer;
- method asShortBuffer ()Ljava/nio/ShortBuffer;
- method clear ()Ljava/nio/Buffer;
- method clear ()Ljava/nio/Buffer;
- method compact ()Ljava/nio/ByteBuffer;
- method compact ()Ljava/nio/ByteBuffer;
- method flip ()Ljava/nio/Buffer;
- method flip ()Ljava/nio/Buffer;
- method get ()B
- method get ()B
- method get (I)B
- method get (I)B
- method get ([B)Ljava/nio/ByteBuffer;
- method get ([B)Ljava/nio/ByteBuffer;
- method get ([BII)Ljava/nio/ByteBuffer;
- method get ([BII)Ljava/nio/ByteBuffer;
- method getInt ()I
- method getInt ()I
- method getInt (I)I
- method getInt (I)I
- method hasRemaining ()Z
- method hasRemaining ()Z
- method isDirect ()Z
- method isDirect ()Z
- method order (Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;
- method order (Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;
- method position ()I
- method position ()I
- method put (B)Ljava/nio/ByteBuffer;
- method put (B)Ljava/nio/ByteBuffer;
- method put (Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;
- method put (Ljava/nio/ByteBuffer;)Ljava/nio/ByteBuffer;
- method put ([B)Ljava/nio/ByteBuffer;
- method put ([B)Ljava/nio/ByteBuffer;
- method put ([BII)Ljava/nio/ByteBuffer;
- method put ([BII)Ljava/nio/ByteBuffer;
- method putInt (II)Ljava/nio/ByteBuffer;
- method putInt (II)Ljava/nio/ByteBuffer;
- method putLong (J)Ljava/nio/ByteBuffer;
- method putLong (J)Ljava/nio/ByteBuffer;
- method remaining ()I
- method remaining ()I
- method wrap ([B)Ljava/nio/ByteBuffer;
- method wrap ([B)Ljava/nio/ByteBuffer;
- method wrap ([BII)Ljava/nio/ByteBuffer;
- method wrap ([BII)Ljava/nio/ByteBuffer;

## java/util/AbstractCollection
- method <init> ()V
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/AbstractList
- method <init> ()V
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;

## java/util/Calendar
- field time J
- field fields [I
- field zone Ljava/util/TimeZone;
- field DATE I
- field DATE I
- field DAY_OF_MONTH I
- field DAY_OF_MONTH I
- field DAY_OF_WEEK I
- field DAY_OF_WEEK I
- field HOUR_OF_DAY I
- field HOUR_OF_DAY I
- field MINUTE I
- field MINUTE I
- field MONTH I
- field MONTH I
- field SECOND I
- field SECOND I
- field YEAR I
- field YEAR I
- method <init> ()V
- method getInstance ()Ljava/util/Calendar;
- method getInstance (Ljava/util/TimeZone;)Ljava/util/Calendar;
- method setTime (Ljava/util/Date;)V
- method getTime ()Ljava/util/Date;
- method set (II)V
- method get (I)I
- method setTimeZone (Ljava/util/TimeZone;)V
- method getTimeZone ()Ljava/util/TimeZone;
- method computeTime ()V
- method computeFields ()V
- method add (II)V
- method add (II)V
- method after (Ljava/lang/Object;)Z
- method after (Ljava/lang/Object;)Z
- method before (Ljava/lang/Object;)Z
- method before (Ljava/lang/Object;)Z
- method compareTo (Ljava/util/Calendar;)I
- method compareTo (Ljava/util/Calendar;)I
- method getActualMaximum (I)I
- method getActualMaximum (I)I
- method getActualMinimum (I)I
- method getActualMinimum (I)I
- method getLeastMaximum (I)I
- method getLeastMaximum (I)I
- method getMaximum (I)I
- method getMaximum (I)I
- method set (III)V
- method set (III)V
- method setLenient (Z)V
- method setLenient (Z)V
- method setTimeInMillis (J)V
- method setTimeInMillis (J)V

## java/util/Date
- field value J
- method <init> ()V
- method <init> (J)V
- method getTime ()J
- method setTime (J)V
- method toString ()Ljava/lang/String;
- method after (Ljava/util/Date;)Z
- method after (Ljava/util/Date;)Z
- method before (Ljava/util/Date;)Z
- method before (Ljava/util/Date;)Z
- method compareTo (Ljava/util/Date;)I
- method compareTo (Ljava/util/Date;)I

## java/util/Dictionary
- method <init> ()V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method isEmpty ()Z
- method keys ()Ljava/util/Enumeration;
- method elements ()Ljava/util/Enumeration;

## java/util/EmptyStackException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/util/Enumeration
- method hasMoreElements ()Z
- method nextElement ()Ljava/lang/Object;

## java/util/GregorianCalendar
- method <init> ()V
- method <init> (Ljava/util/TimeZone;)V
- method computeTime ()V
- method computeFields ()V
- method <init> (IIIIII)V
- method <init> (Ljava/util/TimeZone;Ljava/util/Locale;)V
- method set (IIIII)V
- method set (IIIII)V

## java/util/Hashtable
- field table [Ljava/util/Hashtable$Entry;
- field count I
- field threshold I
- method <init> ()V
- method <init> (I)V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method isEmpty ()Z
- method keys ()Ljava/util/Enumeration;
- method elements ()Ljava/util/Enumeration;
- method values ()Ljava/util/Collection;
- method rehash ()V

## java/util/Hashtable$Entry
- field hash I
- field key Ljava/lang/Object;
- field value Ljava/lang/Object;
- field next Ljava/util/Hashtable$Entry;
- method <init> (ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V

## java/util/Hashtable$Enumerator
- field table [Ljava/util/Hashtable$Entry;
- field index I
- field entry Ljava/util/Hashtable$Entry;
- field keys Z
- method <init> (Ljava/util/Hashtable;Z)V
- method hasMoreElements ()Z
- method nextElement ()Ljava/lang/Object;

## java/util/NoSuchElementException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/util/Properties
- field defaults Ljava/util/Properties;
- method <init> ()V
- method <init> (Ljava/util/Properties;)V
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method getProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method setProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;
- method load (Ljava/io/InputStream;)V
- method store (Ljava/io/OutputStream;Ljava/lang/String;)V
- method save (Ljava/io/OutputStream;Ljava/lang/String;)V
- method propertyNames ()Ljava/util/Enumeration;
- method entrySet ()Ljava/util/Set;
- method entrySet ()Ljava/util/Set;
- method keySet ()Ljava/util/Set;
- method keySet ()Ljava/util/Set;
- method load (Ljava/io/Reader;)V
- method load (Ljava/io/Reader;)V

## java/util/Random
- field seed J
- field haveNextNextGaussian Z
- field nextNextGaussian D
- method <init> ()V
- method <init> (J)V
- method nextInt ()I
- method nextInt (I)I
- method nextLong ()J
- method nextBoolean ()Z
- method nextFloat ()F
- method nextDouble ()D
- method nextGaussian ()D
- method next (I)I
- method setSeed (J)V

## java/util/SimpleTimeZone
- method <init> (Ljava/lang/String;)V

## java/util/Stack
- method <init> ()V
- method empty ()Z
- method peek ()Ljava/lang/Object;
- method pop ()Ljava/lang/Object;
- method push (Ljava/lang/Object;)Ljava/lang/Object;
- method search (Ljava/lang/Object;)I
- method <init> (Ljava/util/Vector;)V

## java/util/Timer
- field tasks Ljava/util/Vector;
- field thread Ljava/lang/Thread;
- field cancelled Z
- method <init> ()V
- method schedule (Ljava/util/TimerTask;JJ)V
- method schedule (Ljava/util/TimerTask;J)V
- method schedule (Ljava/util/TimerTask;Ljava/util/Date;J)V
- method cancel ()V
- method scheduleAtFixedRate (Ljava/util/TimerTask;JJ)V
- method <init> (Ljava/lang/String;Z)V
- method schedule (Ljava/util/TimerTask;Ljava/util/Date;)V
- method schedule (Ljava/util/TimerTask;Ljava/util/Date;)V
- method scheduleAtFixedRate (Ljava/util/TimerTask;Ljava/util/Date;J)V
- method scheduleAtFixedRate (Ljava/util/TimerTask;Ljava/util/Date;J)V
- method Отмена ()V
- method Отмена ()V

## java/util/TimerTask
- field nextExecutionTime J
- field period J
- field cancelled Z
- method <init> ()V
- method run ()V
- method cancel ()Z
- method scheduledExecutionTime ()J
- method scheduledExecutionTime ()J

## java/util/Timer$TimerThread
- field tasks Ljava/util/Vector;
- method <init> (Ljava/util/Vector;)V
- method run ()V

## java/util/TimeZone
- method <init> ()V
- method getTimeZone (Ljava/lang/String;)Ljava/util/TimeZone;
- method getDefault ()Ljava/util/TimeZone;
- method getRawOffset ()I
- method useDaylightTime ()Z
- method getID ()Ljava/lang/String;
- method getAvailableIDs ()[Ljava/lang/String;
- method getAvailableIDs ()[Ljava/lang/String;
- method getDisplayName (ZILjava/util/Locale;)Ljava/lang/String;
- method getDisplayName (ZILjava/util/Locale;)Ljava/lang/String;
- method getOffset (IIIIII)I
- method getOffset (IIIIII)I

## java/util/Vector
- field elementData [Ljava/lang/Object;
- field elementCount I
- field capacityIncrement I
- method <init> ()V
- method <init> (I)V
- method <init> (II)V
- method <init> (Ljava/util/Collection;)V
- method add (Ljava/lang/Object;)Z
- method add (ILjava/lang/Object;)V
- method addAll (Ljava/util/Collection;)Z
- method addElement (Ljava/lang/Object;)V
- method capacity ()I
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method copyInto ([Ljava/lang/Object;)V
- method ensureCapacity (I)V
- method insertElementAt (Ljava/lang/Object;I)V
- method elementAt (I)Ljava/lang/Object;
- method get (I)Ljava/lang/Object;
- method set (ILjava/lang/Object;)Ljava/lang/Object;
- method setElementAt (Ljava/lang/Object;I)V
- method setSize (I)V
- method size ()I
- method isEmpty ()Z
- method remove (I)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Z
- method removeAllElements ()V
- method removeElementAt (I)V
- method indexOf (Ljava/lang/Object;)I
- method indexOf (Ljava/lang/Object;I)I
- method lastIndexOf (Ljava/lang/Object;)I
- method lastIndexOf (Ljava/lang/Object;I)I
- method firstElement ()Ljava/lang/Object;
- method lastElement ()Ljava/lang/Object;
- method removeElement (Ljava/lang/Object;)Z
- method trimToSize ()V
- method elements ()Ljava/util/Enumeration;
- method clone ()Ljava/lang/Object;
- method toString ()Ljava/lang/String;

## java/util/Vector$Enumerator
- field vector Ljava/util/Vector;
- field count I
- method <init> (Ljava/util/Vector;)V
- method hasMoreElements ()Z
- method nextElement ()Ljava/lang/Object;

## java/util/jar/Attributes
- field map Ljava/util/Map;
- method <init> ()V
- method putValue (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method getValue (Ljava/lang/String;)Ljava/lang/String;
- method entrySet ()Ljava/util/Set;
- method entrySet ()Ljava/util/Set;

## java/util/jar/JarEntry
- method <init> (Ljava/util/zip/ZipEntry;)V
- method <init> (Ljava/lang/String;)V

## java/util/jar/JarFile
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/lang/String;)V
- method getJarEntry (Ljava/lang/String;)Ljava/util/jar/JarEntry;
- method entries ()Ljava/util/Enumeration;
- method getManifest ()Ljava/util/jar/Manifest;

## java/util/jar/JarFile$Entries
- field entries Ljava/util/zip/ZipFile$Entries;
- method <init> (Ljava/util/zip/ZipFile$Entries;)V
- method hasMoreElements ()Z
- method nextElement ()Ljava/lang/Object;

## java/util/jar/Manifest
- field attrs Ljava/util/jar/Attributes;
- method <init> (Ljava/io/InputStream;)V
- method read (Ljava/io/InputStream;)V
- method getMainAttributes ()Ljava/util/jar/Attributes;

## java/util/zip/ZipEntry
- field name Ljava/lang/String;
- field size J
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/util/zip/ZipEntry;)V
- method getName ()Ljava/lang/String;
- method setSize (J)V
- method getSize ()J
- method getCompressedSize ()J
- method getCompressedSize ()J
- method getExtra ()[B
- method getExtra ()[B
- method getMethod ()I
- method getMethod ()I
- method getTime ()J
- method getTime ()J
- method isDirectory ()Z
- method isDirectory ()Z
- method setComment (Ljava/lang/String;)V
- method setComment (Ljava/lang/String;)V
- method setCompressedSize (J)V
- method setCompressedSize (J)V
- method setCrc (J)V
- method setCrc (J)V
- method setExtra ([B)V
- method setExtra ([B)V
- method setMethod (I)V
- method setMethod (I)V
- method setTime (J)V
- method setTime (J)V

## java/util/zip/ZipFile
- field zipData [B
- field zipHandle J
- method <init> (Ljava/io/File;)V
- method getEntry (Ljava/lang/String;)Ljava/util/zip/ZipEntry;
- method getInputStream (Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;
- method entries ()Ljava/util/Enumeration;
- method <init> (Ljava/lang/String;)V
- method close ()V
- method close ()V

## java/util/zip/ZipFile$Entries
- field zipFile Ljava/util/zip/ZipFile;
- field names [Ljava/lang/String;
- field i I
- method <init> (Ljava/util/zip/ZipFile;[Ljava/lang/String;)V
- method hasMoreElements ()Z
- method nextElement ()Ljava/lang/Object;

## java/util/zip/ZipException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/io/CommConnection
- field url Ljava/lang/String;
- field baud I
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method getBaudRate ()I
- method setBaudRate (I)I
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;

## javax/microedition/io/Connection
- method close ()V
- method 0 ()V
- method 0 ()V
- method getResponseCode ()I
- method getResponseCode ()I
- method setRequestMethod (Ljava/lang/String;)V
- method setRequestMethod (Ljava/lang/String;)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method Закрыть ()V
- method Закрыть ()V

## javax/microedition/io/ConnectionNotFoundException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/io/Connector
- field READ I
- field WRITE I
- field READ_WRITE I
- method <clinit> ()V
- method open (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method open (Ljava/lang/String;I)Ljavax/microedition/io/Connection;
- method open (Ljava/lang/String;IZ)Ljavax/microedition/io/Connection;
- method openInputStream (Ljava/lang/String;)Ljava/io/InputStream;
- method openOutputStream (Ljava/lang/String;)Ljava/io/OutputStream;
- method openDataInputStream (Ljava/lang/String;)Ljava/io/DataInputStream;
- method openDataOutputStream (Ljava/lang/String;)Ljava/io/DataOutputStream;
- method открыть (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method открыть (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method открыть (Ljava/lang/String;I)Ljavax/microedition/io/Connection;
- method открыть (Ljava/lang/String;I)Ljavax/microedition/io/Connection;

## javax/microedition/io/ContentConnection
- method getType ()Ljava/lang/String;
- method getEncoding ()Ljava/lang/String;
- method getLength ()J

## javax/microedition/io/Datagram
- field data [B
- field length I
- field offset I
- field address Ljava/lang/String;
- method <init> ([BI)V
- method getAddress ()Ljava/lang/String;
- method getData ()[B
- method getLength ()I
- method getOffset ()I
- method reset ()V
- method setAddress (Ljava/lang/String;)V
- method setAddress (Ljavax/microedition/io/Datagram;)V
- method setData ([BII)V
- method setLength (I)V
- method readByte ()B
- method readByte ()B
- method readInt ()I
- method readInt ()I
- method readShort ()S
- method readShort ()S
- method write (I)V
- method write (I)V
- method write ([B)V
- method write ([B)V
- method writeByte (I)V
- method writeByte (I)V
- method writeInt (I)V
- method writeInt (I)V
- method writeUTF (Ljava/lang/String;)V
- method writeUTF (Ljava/lang/String;)V

## javax/microedition/io/DatagramConnection
- method getMaximumLength ()I
- method getNominalLength ()I
- method send (Ljavax/microedition/io/Datagram;)V
- method receive (Ljavax/microedition/io/Datagram;)V
- method newDatagram (I)Ljavax/microedition/io/Datagram;
- method newDatagram (ILjava/lang/String;)Ljavax/microedition/io/Datagram;
- method newDatagram ([BI)Ljavax/microedition/io/Datagram;
- method newDatagram ([BILjava/lang/String;)Ljavax/microedition/io/Datagram;

## javax/microedition/io/file/FileConnection
- field url Ljava/lang/String;
- field open Z
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method exists ()Z
- method isDirectory ()Z
- method isOpen ()Z
- method fileSize ()J
- method directorySize (Z)J
- method canRead ()Z
- method canWrite ()Z
- method isHidden ()Z
- method setReadable (Z)V
- method setWritable (Z)V
- method setHidden (Z)V
- method list ()Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method mkdir ()V
- method create ()V
- method delete ()V
- method rename (Ljava/lang/String;)V
- method truncate (J)V
- method setFileConnection (Ljava/lang/String;)V
- method getName ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method lastModified ()J
- method availableSize ()J
- method totalSize ()J
- method usedSize ()J
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method <init> ()V
- method close (I)I
- method close (I)I
- method isDirectory (Ljava/lang/String;)Z
- method isDirectory (Ljava/lang/String;)Z
- method list (Ljava/lang/String;)[Ljava/lang/String;
- method list (Ljava/lang/String;)[Ljava/lang/String;
- method open (Ljava/lang/String;)I
- method open (Ljava/lang/String;)I
- method setWriteable (Z)V
- method setWriteable (Z)V
- method spaceAvailable ()I
- method spaceAvailable ()I
- method write (I[BII)I
- method write (I[BII)I
- method Переименовать (Ljava/lang/String;)V
- method Переименовать (Ljava/lang/String;)V
- method Удалить ()V
- method Удалить ()V
- method список ()Ljava/util/Enumeration;
- method список ()Ljava/util/Enumeration;
- method существует ()Z
- method существует ()Z

## javax/microedition/io/file/FileSystemListener
- field ROOT_ADDED I
- field ROOT_REMOVED I
- method rootChanged (ILjava/lang/String;)V

## javax/microedition/io/file/FileSystemRegistry
- method listRoots ()Ljava/util/Enumeration;
- method addFileSystemListener (Ljavax/microedition/io/file/FileSystemListener;)Z
- method removeFileSystemListener (Ljavax/microedition/io/file/FileSystemListener;)Z

## javax/microedition/io/HttpConnection
- field url Ljava/lang/String;
- field mode I
- field method Ljava/lang/String;
- field responseCode I
- field responseBody [B
- field GET Ljava/lang/String;
- field POST Ljava/lang/String;
- field HEAD Ljava/lang/String;
- field HTTP_OK I
- field HTTP_CREATED I
- field HTTP_ACCEPTED I
- field HTTP_NOT_AUTHORITATIVE I
- field HTTP_NO_CONTENT I
- field HTTP_RESET I
- field HTTP_PARTIAL I
- field HTTP_MULT_CHOICE I
- field HTTP_MOVED_PERM I
- field HTTP_MOVED_TEMP I
- field HTTP_SEE_OTHER I
- field HTTP_NOT_MODIFIED I
- field HTTP_USE_PROXY I
- field HTTP_TEMP_REDIRECT I
- field HTTP_BAD_REQUEST I
- field HTTP_UNAUTHORIZED I
- field HTTP_PAYMENT_REQUIRED I
- field HTTP_FORBIDDEN I
- field HTTP_NOT_FOUND I
- field HTTP_BAD_METHOD I
- field HTTP_NOT_ACCEPTABLE I
- field HTTP_PROXY_AUTH I
- field HTTP_CLIENT_TIMEOUT I
- field HTTP_CONFLICT I
- field HTTP_GONE I
- field HTTP_LENGTH_REQUIRED I
- field HTTP_PRECON_FAILED I
- field HTTP_ENTITY_TOO_LARGE I
- field HTTP_REQ_TOO_LONG I
- field HTTP_UNSUPPORTED_TYPE I
- field HTTP_INTERNAL_ERROR I
- field HTTP_NOT_IMPLEMENTED I
- field HTTP_BAD_GATEWAY I
- field HTTP_UNAVAILABLE I
- field HTTP_GATEWAY_TIMEOUT I
- field HTTP_VERSION I
- method <clinit> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method openInputStream ()Ljava/io/InputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method getURL ()Ljava/lang/String;
- method getProtocol ()Ljava/lang/String;
- method getHost ()Ljava/lang/String;
- method getFile ()Ljava/lang/String;
- method getRef ()Ljava/lang/String;
- method getQuery ()Ljava/lang/String;
- method getPort ()I
- method getRequestMethod ()Ljava/lang/String;
- method setRequestMethod (Ljava/lang/String;)V
- method getRequestProperty (Ljava/lang/String;)Ljava/lang/String;
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method getResponseCode ()I
- method getResponseMessage ()Ljava/lang/String;
- method getExpiration ()J
- method getDate ()J
- method getLastModified ()J
- method getHeaderField (Ljava/lang/String;)Ljava/lang/String;
- method getHeaderField (I)Ljava/lang/String;
- method getHeaderFieldKey (I)Ljava/lang/String;
- method getHeaderFieldInt (Ljava/lang/String;I)I
- method getHeaderFieldDate (Ljava/lang/String;J)J
- method getType ()Ljava/lang/String;
- method getEncoding ()Ljava/lang/String;
- method getLength ()J
- method yopish ()V
- method yopish ()V
- method закрыть ()V
- method закрыть ()V

## javax/microedition/io/HttpsConnection
- field url Ljava/lang/String;
- field mode I
- field method Ljava/lang/String;
- field responseCode I
- field responseBody [B
- field GET Ljava/lang/String;
- field POST Ljava/lang/String;
- field HEAD Ljava/lang/String;
- field HTTP_OK I
- field HTTP_CREATED I
- field HTTP_ACCEPTED I
- field HTTP_NOT_AUTHORITATIVE I
- field HTTP_NO_CONTENT I
- field HTTP_RESET I
- field HTTP_PARTIAL I
- field HTTP_MULT_CHOICE I
- field HTTP_MOVED_PERM I
- field HTTP_MOVED_TEMP I
- field HTTP_SEE_OTHER I
- field HTTP_NOT_MODIFIED I
- field HTTP_USE_PROXY I
- field HTTP_TEMP_REDIRECT I
- field HTTP_BAD_REQUEST I
- field HTTP_UNAUTHORIZED I
- field HTTP_PAYMENT_REQUIRED I
- field HTTP_FORBIDDEN I
- field HTTP_NOT_FOUND I
- field HTTP_BAD_METHOD I
- field HTTP_NOT_ACCEPTABLE I
- field HTTP_PROXY_AUTH I
- field HTTP_CLIENT_TIMEOUT I
- field HTTP_CONFLICT I
- field HTTP_GONE I
- field HTTP_LENGTH_REQUIRED I
- field HTTP_PRECON_FAILED I
- field HTTP_ENTITY_TOO_LARGE I
- field HTTP_REQ_TOO_LONG I
- field HTTP_UNSUPPORTED_TYPE I
- field HTTP_INTERNAL_ERROR I
- field HTTP_NOT_IMPLEMENTED I
- field HTTP_BAD_GATEWAY I
- field HTTP_UNAVAILABLE I
- field HTTP_GATEWAY_TIMEOUT I
- field HTTP_VERSION I
- method <clinit> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method openInputStream ()Ljava/io/InputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method getURL ()Ljava/lang/String;
- method getProtocol ()Ljava/lang/String;
- method getHost ()Ljava/lang/String;
- method getFile ()Ljava/lang/String;
- method getRef ()Ljava/lang/String;
- method getQuery ()Ljava/lang/String;
- method getPort ()I
- method getRequestMethod ()Ljava/lang/String;
- method setRequestMethod (Ljava/lang/String;)V
- method getRequestProperty (Ljava/lang/String;)Ljava/lang/String;
- method setRequestProperty (Ljava/lang/String;Ljava/lang/String;)V
- method getResponseCode ()I
- method getResponseMessage ()Ljava/lang/String;
- method getExpiration ()J
- method getDate ()J
- method getLastModified ()J
- method getHeaderField (Ljava/lang/String;)Ljava/lang/String;
- method getHeaderField (I)Ljava/lang/String;
- method getHeaderFieldKey (I)Ljava/lang/String;
- method getHeaderFieldInt (Ljava/lang/String;I)I
- method getHeaderFieldDate (Ljava/lang/String;J)J
- method getType ()Ljava/lang/String;
- method getEncoding ()Ljava/lang/String;
- method getLength ()J
- method getPort ()I
- method getSecurityInfo ()Ljavax/microedition/io/SecurityInfo;

## javax/microedition/io/InputConnection
- method openInputStream ()Ljava/io/InputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;

## javax/microedition/io/OutputConnection
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;

## javax/microedition/io/PushRegistry
- method registerConnection (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method unregisterConnection (Ljava/lang/String;)Z
- method listConnections (Z)[Ljava/lang/String;
- method getMIDlet (Ljava/lang/String;)Ljava/lang/String;
- method getFilter (Ljava/lang/String;)Ljava/lang/String;
- method registerAlarm (Ljava/lang/String;J)J

## javax/microedition/io/SecureConnection
- field url Ljava/lang/String;
- field DELAY B
- field LINGER B
- field KEEPALIVE B
- field RCVBUF B
- field SNDBUF B
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method getLocalAddress ()Ljava/lang/String;
- method getLocalPort ()I
- method getAddress ()Ljava/lang/String;
- method getPort ()I
- method setSocketOption (BI)V
- method getSocketOption (B)I
- method getSecurityInfo ()Ljavax/microedition/io/SecurityInfo;
- method getSecurityInfo ()Ljavax/microedition/io/SecurityInfo;

## javax/microedition/io/SecurityInfo
- method getCipherSuite ()Ljava/lang/String;
- method getProtocolName ()Ljava/lang/String;
- method getProtocolVersion ()Ljava/lang/String;
- method getServerCertificate ()Ljavax/microedition/pki/Certificate;
- method getServerCertificate ()Ljavax/microedition/pki/Certificate;

## javax/microedition/io/ServerSocketConnection
- field url Ljava/lang/String;
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method acceptAndOpen ()Ljavax/microedition/io/StreamConnection;
- method getLocalAddress ()Ljava/lang/String;
- method getLocalPort ()I

## javax/microedition/io/SocketConnection
- field url Ljava/lang/String;
- field DELAY B
- field LINGER B
- field KEEPALIVE B
- field RCVBUF B
- field SNDBUF B
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method getLocalAddress ()Ljava/lang/String;
- method getLocalPort ()I
- method getAddress ()Ljava/lang/String;
- method getPort ()I
- method setSocketOption (BI)V
- method getSocketOption (B)I

## javax/microedition/io/StreamConnection
- method open (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method open (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method open (Ljava/lang/String;IZ)Ljavax/microedition/io/Connection;
- method open (Ljava/lang/String;IZ)Ljavax/microedition/io/Connection;
- method openDataInputStream (Ljava/lang/String;)Ljava/io/DataInputStream;
- method openDataInputStream (Ljava/lang/String;)Ljava/io/DataInputStream;
- method openDataOutputStream (Ljava/lang/String;)Ljava/io/DataOutputStream;
- method openDataOutputStream (Ljava/lang/String;)Ljava/io/DataOutputStream;
- method openInputStream (Ljava/lang/String;)Ljava/io/InputStream;
- method openInputStream (Ljava/lang/String;)Ljava/io/InputStream;
- method openOutputStream (Ljava/lang/String;)Ljava/io/OutputStream;
- method openOutputStream (Ljava/lang/String;)Ljava/io/OutputStream;

## javax/microedition/io/StreamConnectionNotifier
- method acceptAndOpen ()Ljavax/microedition/io/StreamConnection;

## javax/microedition/io/UDPDatagramConnection
- field url Ljava/lang/String;
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method getMaximumLength ()I
- method getNominalLength ()I
- method send (Ljavax/microedition/io/Datagram;)V
- method receive (Ljavax/microedition/io/Datagram;)V
- method newDatagram (I)Ljavax/microedition/io/Datagram;
- method newDatagram (ILjava/lang/String;)Ljavax/microedition/io/Datagram;
- method newDatagram ([BI)Ljavax/microedition/io/Datagram;
- method newDatagram ([BILjava/lang/String;)Ljavax/microedition/io/Datagram;
- method getLocalAddress ()Ljava/lang/String;
- method getLocalPort ()I

## javax/microedition/khronos/egl/EGL

## javax/microedition/khronos/egl/EGL10
- field EGL_DEFAULT_DISPLAY Ljava/lang/Object;
- field EGL_NO_CONTEXT Ljavax/microedition/khronos/egl/EGLContext;
- field EGL_NO_SURFACE Ljavax/microedition/khronos/egl/EGLSurface;
- field EGL_NO_DISPLAY Ljavax/microedition/khronos/egl/EGLDisplay;
- field EGL_NO_DISPLAY Ljavax/microedition/khronos/egl/EGLDisplay;
- method <clinit> ()V
- method <init> ()V
- method eglChooseConfig (Ljavax/microedition/khronos/egl/EGLDisplay;[I[Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z
- method eglCreateContext (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;Ljavax/microedition/khronos/egl/EGLContext;[I)Ljavax/microedition/khronos/egl/EGLContext;
- method eglCreateWindowSurface (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;Ljava/lang/Object;[I)Ljavax/microedition/khronos/egl/EGLSurface;
- method eglDestroyContext (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLContext;)Z
- method eglDestroySurface (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;)Z
- method eglGetCurrentContext ()Ljavax/microedition/khronos/egl/EGLContext;
- method eglGetDisplay (Ljava/lang/Object;)Ljavax/microedition/khronos/egl/EGLDisplay;
- method eglGetError ()I
- method eglInitialize (Ljavax/microedition/khronos/egl/EGLDisplay;[I)Z
- method eglMakeCurrent (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;Ljavax/microedition/khronos/egl/EGLSurface;Ljavax/microedition/khronos/egl/EGLContext;)Z
- method eglSwapBuffers (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLSurface;)Z
- method eglTerminate (Ljavax/microedition/khronos/egl/EGLDisplay;)Z
- method eglWaitGL ()Z
- method eglWaitNative (ILjava/lang/Object;)Z
- method eglGetConfigAttrib (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z
- method eglGetConfigAttrib (Ljavax/microedition/khronos/egl/EGLDisplay;Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z
- method eglGetConfigs (Ljavax/microedition/khronos/egl/EGLDisplay;[Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z
- method eglGetConfigs (Ljavax/microedition/khronos/egl/EGLDisplay;[Ljavax/microedition/khronos/egl/EGLConfig;I[I)Z
- method eglQueryString (Ljavax/microedition/khronos/egl/EGLDisplay;I)Ljava/lang/String;
- method eglQueryString (Ljavax/microedition/khronos/egl/EGLDisplay;I)Ljava/lang/String;

## javax/microedition/khronos/egl/EGLConfig
- method <init> ()V

## javax/microedition/khronos/egl/EGLContext
- method <init> ()V
- method getEGL ()Ljavax/microedition/khronos/egl/EGL;
- method getGL ()Ljavax/microedition/khronos/opengles/GL;

## javax/microedition/khronos/egl/EGLDisplay
- method <init> ()V

## javax/microedition/khronos/egl/EGLSurface
- method <init> ()V

## javax/microedition/khronos/opengles/GL

## javax/microedition/khronos/opengles/GL10
- method <init> ()V
- method glAlphaFuncx (II)V
- method glBindTexture (II)V
- method glBlendFunc (II)V
- method glClear (I)V
- method glClearColorx (IIII)V
- method glColor4x (IIII)V
- method glCullFace (I)V
- method glDepthFunc (I)V
- method glDepthMask (Z)V
- method glDepthRangex (II)V
- method glDisable (I)V
- method glDisableClientState (I)V
- method glDrawArrays (III)V
- method glDrawElements (IIILjava/nio/Buffer;)V
- method glEnable (I)V
- method glEnableClientState (I)V
- method glFlush ()V
- method glFogf (IF)V
- method glFogx (II)V
- method glFogxv (I[II)V
- method glFrontFace (I)V
- method glGenTextures (I[II)V
- method glGetIntegerv (I[II)V
- method glGetString (I)Ljava/lang/String;
- method glHint (II)V
- method glLightModelx (II)V
- method glLightModelxv (I[II)V
- method glLightxv (II[II)V
- method glLoadIdentity ()V
- method glLoadMatrixx ([II)V
- method glMaterialx (III)V
- method glMaterialxv (II[II)V
- method glMatrixMode (I)V
- method glNormalPointer (IILjava/nio/Buffer;)V
- method glOrthox (IIIIII)V
- method glPixelStorei (II)V
- method glPopMatrix ()V
- method glPushMatrix ()V
- method glScalex (III)V
- method glScissor (IIII)V
- method glShadeModel (I)V
- method glTexCoordPointer (IIILjava/nio/Buffer;)V
- method glTexEnvx (III)V
- method glTexImage2D (IIIIIIIILjava/nio/Buffer;)V
- method glTexParameterf (IIF)V
- method glTexParameterx (III)V
- method glTranslatex (III)V
- method glVertexPointer (IIILjava/nio/Buffer;)V
- method glViewport (IIII)V
- method glGetError ()I
- method glGetError ()I

## javax/microedition/lcdui/Alert
- field title Ljava/lang/String;
- field text Ljava/lang/String;
- field timeout I
- field alertType Ljavax/microedition/lcdui/AlertType;
- field FOREVER I
- field DISMISS_COMMAND Ljavax/microedition/lcdui/Command;
- method <clinit> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/AlertType;)V
- method getString ()Ljava/lang/String;
- method getTimeout ()I
- method setString (Ljava/lang/String;)V
- method setTimeout (I)V
- method setType (Ljavax/microedition/lcdui/AlertType;)V
- method showNotify ()V
- method getDefaultTimeout ()I
- method getDefaultTimeout ()I
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getIndicator ()Ljavax/microedition/lcdui/Gauge;
- method getIndicator ()Ljavax/microedition/lcdui/Gauge;
- method getType ()Ljavax/microedition/lcdui/AlertType;
- method getType ()Ljavax/microedition/lcdui/AlertType;
- method setImage (Ljavax/microedition/lcdui/Image;)V
- method setImage (Ljavax/microedition/lcdui/Image;)V
- method setIndicator (Ljavax/microedition/lcdui/Gauge;)V
- method setIndicator (Ljavax/microedition/lcdui/Gauge;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V

## javax/microedition/lcdui/AlertType
- field INFO Ljavax/microedition/lcdui/AlertType;
- field WARNING Ljavax/microedition/lcdui/AlertType;
- field ERROR Ljavax/microedition/lcdui/AlertType;
- field ALARM Ljavax/microedition/lcdui/AlertType;
- field CONFIRMATION Ljavax/microedition/lcdui/AlertType;
- field XATO Ljavax/microedition/lcdui/AlertType;
- field XATO Ljavax/microedition/lcdui/AlertType;
- field XATOOOO Ljavax/microedition/lcdui/AlertType;
- field XATOOOO Ljavax/microedition/lcdui/AlertType;
- field xatoooo Ljavax/microedition/lcdui/AlertType;
- field xatoooo Ljavax/microedition/lcdui/AlertType;
- field Инфо Ljavax/microedition/lcdui/AlertType;
- field Инфо Ljavax/microedition/lcdui/AlertType;
- field ОШИБКА Ljavax/microedition/lcdui/AlertType;
- field ОШИБКА Ljavax/microedition/lcdui/AlertType;
- field ПОПЕРЕДЖЕННЯ Ljavax/microedition/lcdui/AlertType;
- field ПОПЕРЕДЖЕННЯ Ljavax/microedition/lcdui/AlertType;
- method <clinit> ()V
- method <init> ()V
- method playSound (Ljavax/microedition/lcdui/Display;)Z
- method playSound (Lmultime/FakeDisplay;)Z
- method playSound (Lmultime/FakeDisplay;)Z

## javax/microedition/lcdui/Canvas
- field UP I
- field LEFT I
- field RIGHT I
- field DOWN I
- field FIRE I
- field GAME_A I
- field GAME_B I
- field GAME_C I
- field GAME_D I
- field KEY_NUM0 I
- field KEY_NUM1 I
- field KEY_NUM2 I
- field KEY_NUM3 I
- field KEY_NUM4 I
- field KEY_NUM5 I
- field KEY_NUM6 I
- field KEY_NUM7 I
- field KEY_NUM8 I
- field KEY_NUM9 I
- field KEY_STAR I
- field KEY_POUND I
- field repaintPending Z
- field isPainting Z
- method <clinit> ()V
- method <init> ()V
- method repaint ()V
- method repaint (IIII)V
- method serviceRepaints ()V
- method setFullScreenMode (Z)V
- method isDoubleBuffered ()Z
- method hasPointerEvents ()Z
- method hasPointerMotionEvents ()Z
- method hasRepeatEvents ()Z
- method getWidth ()I
- method getHeight ()I
- method getGameAction (I)I
- method getKeyCode (I)I
- method getKeyName (I)Ljava/lang/String;
- method keyPressed (I)V
- method keyReleased (I)V
- method keyRepeated (I)V
- method pointerPressed (II)V
- method pointerReleased (II)V
- method pointerDragged (II)V
- method sizeChanged (II)V
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method <init> (Z)V
- method __keyPressed (I)V
- method __keyPressed (I)V
- method __keyReleased (I)V
- method __keyReleased (I)V
- method __keyRepeated (I)V
- method __keyRepeated (I)V
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V

## javax/microedition/lcdui/Choice
- field EXCLUSIVE I
- field MULTIPLE I
- field IMPLICIT I
- field POPUP I
- field TEXT_WRAP_DEFAULT I
- field TEXT_WRAP_ON I
- field TEXT_WRAP_OFF I
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method delete (I)V
- method delete (I)V
- method deleteAll ()V
- method deleteAll ()V
- method getImage (I)Ljavax/microedition/lcdui/Image;
- method getImage (I)Ljavax/microedition/lcdui/Image;
- method getSelectedFlags ([Z)I
- method getSelectedFlags ([Z)I
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;
- method isSelected (I)Z
- method isSelected (I)Z
- method setFitPolicy (I)V
- method setFitPolicy (I)V
- method setSelectedFlags ([Z)V
- method setSelectedFlags ([Z)V
- method setSelectedIndex (IZ)V
- method setSelectedIndex (IZ)V
- method size ()I
- method size ()I

## javax/microedition/lcdui/ChoiceGroup
- field choiceType I
- field strings Ljava/util/Vector;
- field images Ljava/util/Vector;
- field selectedIndex I
- field fitPolicy I
- field EXCLUSIVE I
- field MULTIPLE I
- field IMPLICIT I
- field POPUP I
- method <clinit> ()V
- method <init> (Ljava/lang/String;I)V
- method <init> (Ljava/lang/String;I[Ljava/lang/String;[Ljavax/microedition/lcdui/Image;)V
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method delete (I)V
- method deleteAll ()V
- method getSelectedFlags ([Z)I
- method getSelectedIndex ()I
- method getString (I)Ljava/lang/String;
- method getImage (I)Ljavax/microedition/lcdui/Image;
- method insert (ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method isSelected (I)Z
- method set (ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setFitPolicy (I)V
- method setFont (ILjavax/microedition/lcdui/Font;)V
- method getFitPolicy ()I
- method setSelectedFlags ([Z)V
- method setSelectedIndex (IZ)V
- method size ()I
- method getFont (I)Ljavax/microedition/lcdui/Font;
- method getFont (I)Ljavax/microedition/lcdui/Font;
- method getPreferredHeight ()I
- method getPreferredHeight ()I
- method размер ()I
- method размер ()I

## javax/microedition/lcdui/Command
- field label Ljava/lang/String;
- field longLabel Ljava/lang/String;
- field commandType I
- field priority I
- field SCREEN I
- field BACK I
- field CANCEL I
- field OK I
- field HELP I
- field STOP I
- field EXIT I
- field ITEM I
- field Cancel I
- field Cancel I
- method <clinit> ()V
- method <init> (Ljava/lang/String;II)V
- method <init> (Ljava/lang/String;Ljava/lang/String;II)V
- method getLabel ()Ljava/lang/String;
- method getLongLabel ()Ljava/lang/String;
- method getCommandType ()I
- method getPriority ()I

## javax/microedition/lcdui/CommandListener
- method commandAction (Ljavax/microedition/lcdui/Command;Ljavax/microedition/lcdui/Displayable;)V

## javax/microedition/lcdui/CustomItem
- field KEY_PRESS I
- field KEY_RELEASE I
- field KEY_REPEAT I
- field POINTER_PRESS I
- field POINTER_RELEASE I
- field POINTER_DRAG I
- field NONE I
- field TRAVERSE_HORIZONTAL I
- field TRAVERSE_VERTICAL I
- method <clinit> ()V
- method <init> (Ljava/lang/String;)V
- method getMinContentWidth ()I
- method getMinContentHeight ()I
- method getPrefContentWidth (I)I
- method getPrefContentHeight (I)I
- method paint (Ljavax/microedition/lcdui/Graphics;II)V
- method getGameAction (I)I
- method invalidate ()V
- method repaint ()V
- method repaint (IIII)V
- method keyPressed (I)V
- method keyReleased (I)V
- method keyRepeated (I)V
- method pointerPressed (II)V
- method pointerReleased (II)V
- method pointerDragged (II)V
- method showNotify ()V
- method hideNotify ()V
- method sizeChanged (II)V
- method traverse (IIII[I)Z
- method traverseOut ()V
- method getInteractionModes ()I
- method getInteractionModes ()I
- method traverse (III[I)Z
- method traverse (III[I)Z

## javax/microedition/lcdui/DateField
- field date Ljava/util/Date;
- field inputMode I
- field DATE I
- field TIME I
- field DATE_TIME I
- method <clinit> ()V
- method <init> (Ljava/lang/String;I)V
- method <init> (Ljava/lang/String;ILjava/util/TimeZone;)V
- method getDate ()Ljava/util/Date;
- method getInputMode ()I
- method setDate (Ljava/util/Date;)V
- method setInputMode (I)V

## javax/microedition/lcdui/Display
- field instance Ljavax/microedition/lcdui/Display;
- field current Ljavax/microedition/lcdui/Displayable;
- field COLOR_BACKGROUND I
- field COLOR_BACKGROUND I
- field COLOR_BORDER I
- field COLOR_BORDER I
- field COLOR_FOREGROUND I
- field COLOR_FOREGROUND I
- field COLOR_HIGHLIGHTED_BACKGROUND I
- field COLOR_HIGHLIGHTED_BACKGROUND I
- field COLOR_HIGHLIGHTED_BORDER I
- field COLOR_HIGHLIGHTED_BORDER I
- field COLOR_HIGHLIGHTED_FOREGROUND I
- field COLOR_HIGHLIGHTED_FOREGROUND I
- field mc LBY/MidControl;
- field mc LBY/MidControl;
- field mc LBY/microedition/file/MidControl;
- field mc LBY/microedition/file/MidControl;
- field mc LBY/microedition/lcdui/MidControl;
- field mc LBY/microedition/lcdui/MidControl;
- field mc LBY/microedition/media/MidControl;
- field mc LBY/microedition/media/MidControl;
- field mc LBY/microedition/midlet/MidControl;
- field mc LBY/microedition/midlet/MidControl;
- field mc LMidControl;
- field mc LMidControl;
- field mc Ljavay/microedition/lcdui/MidControl;
- field mc Ljavay/microedition/lcdui/MidControl;
- field mc Lkavax/microedition/lcdui/MidControl;
- field mc Lkavax/microedition/lcdui/MidControl;
- field mc Lkavax/microedition/media/MidControl;
- field mc Lkavax/microedition/media/MidControl;
- field mc Llib/MidControl;
- field mc Llib/MidControl;
- field mc Lloi;
- field mc Lloi;
- field mc Lmamun/microedition/lcdui/MidControl;
- field mc Lmamun/microedition/lcdui/MidControl;
- method <init> ()V
- method getDisplay (Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;
- method setCurrent (Ljavax/microedition/lcdui/Displayable;)V
- method setCurrent (Ljavax/microedition/lcdui/Alert;Ljavax/microedition/lcdui/Displayable;)V
- method getCurrent ()Ljavax/microedition/lcdui/Displayable;
- method callSerially (Ljava/lang/Runnable;)V
- method flashBacklight (I)Z
- method vibrate (I)Z
- method isColor ()Z
- method numColors ()I
- method numAlphaLevels ()I
- method getBestImageWidth (I)I
- method getBestImageHeight (I)I
- method getColor (I)I
- method setCurrentItem (Ljavax/microedition/lcdui/Item;)V
- method Tebranish (I)Z
- method Tebranish (I)Z
- method getBorderStyle (Z)I
- method getBorderStyle (Z)I
- method getDisplay ()Ljavax/microedition/lcdui/Display;
- method getDisplay ()Ljavax/microedition/lcdui/Display;
- method getDisplay (LGame/GMidlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (LGame/GMidlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Law;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Law;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lbq;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lbq;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lc2hallofu/p;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lc2hallofu/p;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lca;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lca;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lcom/jarbull/jbf/JBMIDlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lcom/jarbull/jbf/JBMIDlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lengine/GameMidlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lengine/GameMidlet;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lf;)Ljavax/microedition/lcdui/Display;
- method getDisplay (Lf;)Ljavax/microedition/lcdui/Display;

## javax/microedition/lcdui/Displayable
- field commands Ljava/util/Vector;
- field commandListener Ljavax/microedition/lcdui/CommandListener;
- field title Ljava/lang/String;
- field ticker Ljavax/microedition/lcdui/Ticker;
- method <init> ()V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method setCommandListener (Ljavax/microedition/lcdui/CommandListener;)V
- method getWidth ()I
- method getHeight ()I
- method isShown ()Z
- method showNotify ()V
- method hideNotify ()V
- method setTitle (Ljava/lang/String;)V
- method getTitle ()Ljava/lang/String;
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method sizeChanged (II)V
- method sizeChanged (II)V

## javax/microedition/lcdui/Font
- field face I
- field style I
- field size I
- field FACE_SYSTEM I
- field FACE_MONOSPACE I
- field FACE_PROPORTIONAL I
- field STYLE_PLAIN I
- field STYLE_BOLD I
- field STYLE_ITALIC I
- field STYLE_UNDERLINED I
- field SIZE_SMALL I
- field SIZE_MEDIUM I
- field SIZE_LARGE I
- method <clinit> ()V
- method <init> (III)V
- method getFont (III)Ljavax/microedition/lcdui/Font;
- method getDefaultFont ()Ljavax/microedition/lcdui/Font;
- method getFace ()I
- method getStyle ()I
- method getSize ()I
- method isPlain ()Z
- method isBold ()Z
- method isItalic ()Z
- method isUnderlined ()Z
- method getHeight ()I
- method getBaselinePosition ()I
- method charWidth (C)I
- method charsWidth ([CII)I
- method stringWidth (Ljava/lang/String;)I
- method substringWidth (Ljava/lang/String;II)I
- method getFont (I)Ljavax/microedition/lcdui/Font;
- method getFont (I)Ljavax/microedition/lcdui/Font;

## javax/microedition/lcdui/Form
- field title Ljava/lang/String;
- field itemCount I
- field items Ljava/util/Vector;
- field itemStateListener Ljavax/microedition/lcdui/ItemStateListener;
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;[Ljavax/microedition/lcdui/Item;)V
- method append (Ljava/lang/String;)I
- method append (Ljavax/microedition/lcdui/Image;)I
- method append (Ljavax/microedition/lcdui/Item;)I
- method delete (I)V
- method deleteAll ()V
- method get (I)Ljavax/microedition/lcdui/Item;
- method getTitle ()Ljava/lang/String;
- method insert (ILjavax/microedition/lcdui/Item;)V
- method set (ILjavax/microedition/lcdui/Item;)V
- method setItemStateListener (Ljavax/microedition/lcdui/ItemStateListener;)V
- method setTitle (Ljava/lang/String;)V
- method size ()I
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method sizeChanged (II)V
- method sizeChanged (II)V

## javax/microedition/lcdui/game/GameCanvas
- field suppressKeyEvents Z
- field backBuffer Ljavax/microedition/lcdui/Image;
- field backGraphics Ljavax/microedition/lcdui/Graphics;
- field UP_PRESSED I
- field LEFT_PRESSED I
- field RIGHT_PRESSED I
- field DOWN_PRESSED I
- field FIRE_PRESSED I
- field GAME_A_PRESSED I
- field GAME_B_PRESSED I
- field GAME_C_PRESSED I
- field GAME_D_PRESSED I
- field a Ljavax/microedition/lcdui/Image;
- field a Ljavax/microedition/lcdui/Image;
- method <clinit> ()V
- method <init> (Z)V
- method getGraphics ()Ljavax/microedition/lcdui/Graphics;
- method flushGraphics ()V
- method flushGraphics (IIII)V
- method getKeyStates ()I
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method bgcolor. (I)Ljava/lang/String;
- method bgcolor. (I)Ljava/lang/String;

## javax/microedition/lcdui/Gauge
- field interactive Z
- field maxValue I
- field value I
- field INDEFINITE I
- field CONTINUOUS_IDLE I
- field INCREMENTAL_IDLE I
- field CONTINUOUS_RUNNING I
- field INCREMENTAL_UPDATING I
- method <clinit> ()V
- method <init> (Ljava/lang/String;ZII)V
- method getMaxValue ()I
- method getValue ()I
- method isInteractive ()Z
- method setMaxValue (I)V
- method setValue (I)V

## javax/microedition/lcdui/Graphics
- field color I
- field font Ljavax/microedition/lcdui/Font;
- field clipX I
- field clipY I
- field clipW I
- field clipH I
- field translateX I
- field translateY I
- field strokeStyle I
- field targetImage Ljavax/microedition/lcdui/Image;
- field HCENTER I
- field VCENTER I
- field LEFT I
- field RIGHT I
- field TOP I
- field BOTTOM I
- field BASELINE I
- field SOLID I
- field DOTTED I
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljavax/microedition/lcdui/Image;)V
- method setColor (I)V
- method setColor (III)V
- method getColor ()I
- method getRedComponent ()I
- method getGreenComponent ()I
- method getBlueComponent ()I
- method setGrayScale (I)V
- method getGrayScale ()I
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method getFont ()Ljavax/microedition/lcdui/Font;
- method translate (II)V
- method getTranslateX ()I
- method getTranslateY ()I
- method setClip (IIII)V
- method clipRect (IIII)V
- method getClipX ()I
- method getClipY ()I
- method getClipWidth ()I
- method getClipHeight ()I
- method setStrokeStyle (I)V
- method getStrokeStyle ()I
- method drawString (Ljava/lang/String;III)V
- method drawSubstring (Ljava/lang/String;IIIII)V
- method drawChar (CIII)V
- method drawChars ([CIIIII)V
- method drawImage (Ljavax/microedition/lcdui/Image;III)V
- method drawLine (IIII)V
- method drawRect (IIII)V
- method fillRect (IIII)V
- method drawRoundRect (IIIIII)V
- method fillRoundRect (IIIIII)V
- method drawArc (IIIIII)V
- method fillArc (IIIIII)V
- method fillTriangle (IIIIII)V
- method drawRegion (Ljavax/microedition/lcdui/Image;IIIIIIII)V
- method drawRGB ([IIIIIIIZ)V
- method copyArea (IIIIIII)V
- method getDisplayColor (I)I
- method drawImage (Ljavax/microedition/lcdui/Image;IIII)V
- method drawImage (Ljavax/microedition/lcdui/Image;IIII)V

## javax/microedition/lcdui/Image
- field width I
- field height I
- field mutable Z
- field argb [I
- method <init> (II)V
- method createImage (II)Ljavax/microedition/lcdui/Image;
- method createImage ([BII)Ljavax/microedition/lcdui/Image;
- method createImage (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljava/io/InputStream;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljavax/microedition/lcdui/Image;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;
- method createRGBImage ([IIIZ)Ljavax/microedition/lcdui/Image;
- method getGraphics ()Ljavax/microedition/lcdui/Graphics;
- method getWidth ()I
- method getHeight ()I
- method isMutable ()Z
- method getRGB ([IIIIIII)V
- method creatImage (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method creatImage (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljava/lanag/String;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljava/lanag/String;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljavax/lang/String;)Ljavax/microedition/lcdui/Image;
- method createImage (Ljavax/lang/String;)Ljavax/microedition/lcdui/Image;
- method cretateImage (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method cretateImage (Ljava/lang/String;)Ljavax/microedition/lcdui/Image;
- method j (Ljava/lang/Object;)Ljava/lang/String;
- method j (Ljava/lang/Object;)Ljava/lang/String;

## javax/microedition/lcdui/ImageItem
- field image Ljavax/microedition/lcdui/Image;
- field altText Ljava/lang/String;
- field appearanceMode I
- field PLAIN I
- field HYPERLINK I
- field BUTTON I
- method <clinit> ()V
- method <init> (Ljava/lang/String;Ljavax/microedition/lcdui/Image;ILjava/lang/String;)V
- method <init> (Ljava/lang/String;Ljavax/microedition/lcdui/Image;ILjava/lang/String;I)V
- method getAltText ()Ljava/lang/String;
- method getAppearanceMode ()I
- method getImage ()Ljavax/microedition/lcdui/Image;
- method setAltText (Ljava/lang/String;)V
- method setImage (Ljavax/microedition/lcdui/Image;)V
- method getMinimumHeight ()I
- method getMinimumHeight ()I
- method getPreferredHeight ()I
- method getPreferredHeight ()I
- method getPreferredWidth ()I
- method getPreferredWidth ()I

## javax/microedition/lcdui/Item
- field label Ljava/lang/String;
- field layout I
- field commands Ljava/util/Vector;
- field defaultCommand Ljavax/microedition/lcdui/Command;
- field itemCommandListener Ljavax/microedition/lcdui/ItemCommandListener;
- field LAYOUT_DEFAULT I
- field LAYOUT_LEFT I
- field LAYOUT_RIGHT I
- field LAYOUT_CENTER I
- field LAYOUT_TOP I
- field LAYOUT_BOTTOM I
- field LAYOUT_VCENTER I
- field LAYOUT_NEWLINE_BEFORE I
- field LAYOUT_NEWLINE_AFTER I
- field LAYOUT_SHRINK I
- field LAYOUT_EXPAND I
- field LAYOUT_VSHRINK I
- field LAYOUT_VEXPAND I
- field LAYOUT_2 I
- field PLAIN I
- field HYPERLINK I
- field BUTTON I
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method getLabel ()Ljava/lang/String;
- method getLayout ()I
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method setDefaultCommand (Ljavax/microedition/lcdui/Command;)V
- method setItemCommandListener (Ljavax/microedition/lcdui/ItemCommandListener;)V
- method setLabel (Ljava/lang/String;)V
- method setLayout (I)V
- method setPreferredSize (II)V
- method notifyStateChanged ()V
- method getMinimumHeight ()I
- method getMinimumHeight ()I
- method getMinimumWidth ()I
- method getMinimumWidth ()I
- method getPreferredHeight ()I
- method getPreferredHeight ()I
- method getPreferredWidth ()I
- method getPreferredWidth ()I

## javax/microedition/lcdui/ItemCommandListener
- method commandAction (Ljavax/microedition/lcdui/Command;Ljavax/microedition/lcdui/Item;)V

## javax/microedition/lcdui/ItemStateListener
- method itemStateChanged (Ljavax/microedition/lcdui/Item;)V

## javax/microedition/lcdui/game/Layer
- field x I
- field y I
- field width I
- field height I
- field visible Z
- method <init> ()V
- method getX ()I
- method getY ()I
- method getWidth ()I
- method getHeight ()I
- method isVisible ()Z
- method setVisible (Z)V
- method setPosition (II)V
- method move (II)V
- method paint (Ljavax/microedition/lcdui/Graphics;)V

## javax/microedition/lcdui/game/LayerManager
- field layers Ljava/util/Vector;
- field viewX I
- field viewY I
- field viewW I
- field viewH I
- method <init> ()V
- method append (Ljavax/microedition/lcdui/game/Layer;)V
- method insert (Ljavax/microedition/lcdui/game/Layer;I)V
- method getLayerAt (I)Ljavax/microedition/lcdui/game/Layer;
- method getSize ()I
- method remove (Ljavax/microedition/lcdui/game/Layer;)V
- method paint (Ljavax/microedition/lcdui/Graphics;II)V
- method setViewWindow (IIII)V

## javax/microedition/lcdui/List
- field title Ljava/lang/String;
- field listType I
- field strings Ljava/util/Vector;
- field images Ljava/util/Vector;
- field selectedIndex I
- field selectedFlags [Z
- field fitPolicy I
- field EXCLUSIVE I
- field MULTIPLE I
- field IMPLICIT I
- field TEXT_WRAP_DEFAULT I
- field TEXT_WRAP_ON I
- field TEXT_WRAP_OFF I
- field SELECT_COMMAND Ljavax/microedition/lcdui/Command;
- method <clinit> ()V
- method <init> (Ljava/lang/String;I)V
- method <init> (Ljava/lang/String;I[Ljava/lang/String;[Ljavax/microedition/lcdui/Image;)V
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method delete (I)V
- method deleteAll ()V
- method getSelectedFlags ([Z)I
- method getSelectedIndex ()I
- method getString (I)Ljava/lang/String;
- method insert (ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method isSelected (I)Z
- method set (ILjava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setFitPolicy (I)V
- method setFont (ILjavax/microedition/lcdui/Font;)V
- method setSelectedFlags ([Z)V
- method setSelectedIndex (IZ)V
- method setTitle (Ljava/lang/String;)V
- method setSelectCommand (Ljavax/microedition/lcdui/Command;)V
- method showNotify ()V
- method size ()I
- method getFont (I)Ljavax/microedition/lcdui/Font;
- method getFont (I)Ljavax/microedition/lcdui/Font;
- method getImage (I)Ljavax/microedition/lcdui/Image;
- method getImage (I)Ljavax/microedition/lcdui/Image;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method sizeChanged (II)V
- method sizeChanged (II)V
- method размер ()I
- method размер ()I
- method удалить (I)V
- method удалить (I)V

## javax/microedition/lcdui/Screen
- method <init> ()V
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V

## javax/microedition/lcdui/Spacer
- field minWidth I
- field minHeight I
- method <init> (II)V
- method setMinimumSize (II)V

## javax/microedition/lcdui/game/Sprite
- field image Ljavax/microedition/lcdui/Image;
- field frameWidth I
- field frameHeight I
- field frame I
- field x I
- field y I
- field refX I
- field refY I
- field transform I
- field sequence [I
- field collisionX I
- field collisionY I
- field collisionW I
- field collisionH I
- field TRANS_NONE I
- field TRANS_ROT90 I
- field TRANS_ROT180 I
- field TRANS_ROT270 I
- field TRANS_MIRROR I
- field TRANS_MIRROR_ROT90 I
- field TRANS_MIRROR_ROT180 I
- field TRANS_MIRROR_ROT270 I
- method <clinit> ()V
- method <init> (Ljavax/microedition/lcdui/Image;)V
- method <init> (Ljavax/microedition/lcdui/Image;II)V
- method defineReferencePixel (II)V
- method getFrame ()I
- method getHeight ()I
- method getRawFrameCount ()I
- method getWidth ()I
- method nextFrame ()V
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method setFrame (I)V
- method setPosition (II)V
- method setRefPixelPosition (II)V
- method setTransform (I)V
- method setFrameSequence ([I)V
- method getFrameSequenceLength ()I
- method prevFrame ()V
- method collidesWith (Ljavax/microedition/lcdui/game/Sprite;Z)Z
- method collidesWith (Ljavax/microedition/lcdui/game/TiledLayer;Z)Z
- method collidesWith (Ljavax/microedition/lcdui/Image;IIZ)Z
- method defineCollisionRectangle (IIII)V
- method <init> (Ljavax/microedition/lcdui/game/Sprite;)V
- method getRefPixelX ()I
- method getRefPixelY ()I
- method setImage (Ljavax/microedition/lcdui/Image;II)V
- method PAINT (Ljavax/microedition/lcdui/Graphics;)V
- method PAINT (Ljavax/microedition/lcdui/Graphics;)V
- method Paint (Ljavax/microedition/lcdui/Graphics;)V
- method Paint (Ljavax/microedition/lcdui/Graphics;)V
- method a (Ljavax/microedition/lcdui/Graphics;)V
- method a (Ljavax/microedition/lcdui/Graphics;)V
- method paiNT (Ljavax/microedition/lcdui/Graphics;)V
- method paiNT (Ljavax/microedition/lcdui/Graphics;)V
- method pai_m (Ljavax/microedition/lcdui/Graphics;)V
- method pai_m (Ljavax/microedition/lcdui/Graphics;)V
- method paink (Ljavax/microedition/lcdui/Graphics;)V
- method paink (Ljavax/microedition/lcdui/Graphics;)V

## javax/microedition/lcdui/StringItem
- field text Ljava/lang/String;
- field appearanceMode I
- field PLAIN I
- field HYPERLINK I
- field BUTTON I
- method <clinit> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;I)V
- method getAppearanceMode ()I
- method getText ()Ljava/lang/String;
- method setText (Ljava/lang/String;)V
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method setPreferredSize (II)V
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getPreferredWidth ()I
- method getPreferredWidth ()I

## javax/microedition/lcdui/TextBox
- field title Ljava/lang/String;
- field text Ljava/lang/String;
- field maxSize I
- field constraints I
- field ANY I
- field EMAILADDR I
- field NUMERIC I
- field PHONENUMBER I
- field URL I
- field DECIMAL I
- field PASSWORD I
- field UNEDITABLE I
- field SENSITIVE I
- field NON_PREDICTIVE I
- field INITIAL_CAPS_WORD I
- field INITIAL_CAPS_SENTENCE I
- method <clinit> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;II)V
- method delete (II)V
- method getChars ([C)I
- method getConstraints ()I
- method getMaxSize ()I
- method getString ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method insert (Ljava/lang/String;I)V
- method insert ([CIII)V
- method setConstraints (I)V
- method setMaxSize (I)I
- method setString (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method size ()I
- method getCaretPosition ()I
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method getTicker ()Ljavax/microedition/lcdui/Ticker;
- method setChars ([CII)V
- method setChars ([CII)V
- method setInitialInputMode (Ljava/lang/String;)V
- method setInitialInputMode (Ljava/lang/String;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V
- method setTicker (Ljavax/microedition/lcdui/Ticker;)V

## javax/microedition/lcdui/TextField
- field text Ljava/lang/String;
- field maxSize I
- field constraints I
- field initialInputMode Ljava/lang/String;
- field ANY I
- field EMAILADDR I
- field NUMERIC I
- field PHONENUMBER I
- field URL I
- field DECIMAL I
- field PASSWORD I
- field UNEDITABLE I
- field SENSITIVE I
- field NON_PREDICTIVE I
- field INITIAL_CAPS_WORD I
- field INITIAL_CAPS_SENTENCE I
- method <clinit> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;II)V
- method delete (II)V
- method getCaretPosition ()I
- method getChars ([C)I
- method getConstraints ()I
- method getMaxSize ()I
- method getString ()Ljava/lang/String;
- method insert (Ljava/lang/String;I)V
- method insert ([CIII)V
- method setConstraints (I)V
- method setInitialInputMode (Ljava/lang/String;)V
- method setMaxSize (I)I
- method setString (Ljava/lang/String;)V
- method size ()I
- method D (II)V
- method D (II)V
- method I (Ljava/lang/String;I)V
- method I (Ljava/lang/String;I)V
- method I ([CIII)V
- method I ([CIII)V
- method S ()I
- method S ()I
- method getMinimumHeight ()I
- method getMinimumHeight ()I
- method getPreferredHeight ()I
- method getPreferredHeight ()I
- method getPreferredWidth ()I
- method getPreferredWidth ()I
- method setChars ([CII)V
- method setChars ([CII)V

## javax/microedition/lcdui/Ticker
- field text Ljava/lang/String;
- method <init> (Ljava/lang/String;)V
- method getString ()Ljava/lang/String;
- method setString (Ljava/lang/String;)V

## javax/microedition/lcdui/game/TiledLayer
- field image Ljavax/microedition/lcdui/Image;
- field columns I
- field rows I
- field cellWidth I
- field cellHeight I
- field cells [I
- field animated [I
- field animatedCount I
- method <init> (IILjavax/microedition/lcdui/Image;II)V
- method createAnimatedTile (I)I
- method fillCells (IIIII)V
- method getAnimatedTile (I)I
- method getCell (II)I
- method getCellHeight ()I
- method getCellWidth ()I
- method getColumns ()I
- method getRows ()I
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method setAnimatedTile (II)V
- method setCell (III)V
- method setStaticTileSet (Ljavax/microedition/lcdui/Image;II)V

## javax/microedition/location/Coordinates
- field latitude D
- field longitude D
- field altitude F
- method <init> ()V
- method getLatitude ()D
- method getLongitude ()D
- method getAltitude ()F
- method <init> (DDF)V
- method azimuthTo (Ljavax/microedition/location/Coordinates;)F
- method azimuthTo (Ljavax/microedition/location/Coordinates;)F
- method convert (DI)Ljava/lang/String;
- method convert (DI)Ljava/lang/String;
- method distance (Ljavax/microedition/location/Coordinates;)F
- method distance (Ljavax/microedition/location/Coordinates;)F
- method setAltitude (F)V
- method setAltitude (F)V
- method setLatitude (D)V
- method setLatitude (D)V
- method setLongitude (D)V
- method setLongitude (D)V

## javax/microedition/location/Criteria
- method <init> ()V
- method setAddressInfoRequired (Z)V
- method setAltitudeRequired (Z)V
- method setCostAllowed (Z)V
- method setHorizontalAccuracy (I)V
- method setPreferredPowerConsumption (I)V
- method setPreferredResponseTime (I)V
- method setSpeedAndCourseRequired (Z)V
- method setVerticalAccuracy (I)V

## javax/microedition/location/Location
- field MTA_ASSISTED I
- field MTA_ASSISTED I
- field MTE_CELLID I
- field MTE_CELLID I
- field MTY_NETWORKBASED I
- field MTY_NETWORKBASED I
- method <init> ()V
- method getCourse ()F
- method getQualifiedCoordinates ()Ljavax/microedition/location/QualifiedCoordinates;
- method getSpeed ()F
- method getTimestamp ()J
- method isValid ()Z
- method getAddressInfo ()Ljavax/microedition/location/AddressInfo;
- method getAddressInfo ()Ljavax/microedition/location/AddressInfo;
- method getExtraInfo (Ljava/lang/String;)Ljava/lang/String;
- method getExtraInfo (Ljava/lang/String;)Ljava/lang/String;
- method getLocationMethod ()I
- method getLocationMethod ()I

## javax/microedition/location/LocationException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/location/LocationListener
- method providerStateChanged (Ljavax/microedition/location/LocationProvider;I)V
- method providerStateChanged (Ljavax/microedition/location/LocationProvider;I)V

## javax/microedition/location/LocationProvider
- field AVAILABLE I
- field TEMPORARILY_UNAVAILABLE I
- field OUT_OF_SERVICE I
- method <init> ()V
- method getInstance (Ljavax/microedition/location/Criteria;)Ljavax/microedition/location/LocationProvider;
- method getLastKnownLocation ()Ljavax/microedition/location/Location;
- method getLocation (I)Ljavax/microedition/location/Location;
- method getState ()I
- method reset ()V
- method setLocationListener (Ljavax/microedition/location/LocationListener;III)V

## javax/microedition/location/QualifiedCoordinates
- field horizontalAccuracy F
- field verticalAccuracy F
- method <init> ()V
- method getHorizontalAccuracy ()F
- method getVerticalAccuracy ()F
- method <init> (DDFFF)V
- method azimuthTo (Ljavax/microedition/location/Coordinates;)F
- method azimuthTo (Ljavax/microedition/location/Coordinates;)F
- method distance (Ljavax/microedition/location/Coordinates;)F
- method distance (Ljavax/microedition/location/Coordinates;)F

## javax/microedition/m3g/AnimationController
- field speed F
- field weight F
- field activeIntervalStart I
- field activeIntervalEnd I
- field refSequenceTime F
- field refWorldTime I
- method <init> ()V
- method getActiveIntervalEnd ()I
- method getActiveIntervalStart ()I
- method getPosition (I)F
- method getRefWorldTime ()I
- method getSpeed ()F
- method getWeight ()F
- method setActiveInterval (II)V
- method setPosition (FI)V
- method setSpeed (FI)V
- method setWeight (F)V

## javax/microedition/m3g/AnimationTrack
- field ALPHA I
- field AMBIENT_COLOR I
- field COLOR I
- field CROP I
- field DENSITY I
- field DIFFUSE_COLOR I
- field EMISSIVE_COLOR I
- field FAR_DISTANCE I
- field FIELD_OF_VIEW I
- field INTENSITY I
- field MORPH_WEIGHTS I
- field NEAR_DISTANCE I
- field ORIENTATION I
- field PICKABILITY I
- field SCALE I
- field SHININESS I
- field SPECULAR_COLOR I
- field SPOT_ANGLE I
- field SPOT_EXPONENT I
- field TRANSLATION I
- field VISIBILITY I
- field sequence Ljavax/microedition/m3g/KeyframeSequence;
- field controller Ljavax/microedition/m3g/AnimationController;
- field targetProperty I
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/KeyframeSequence;I)V
- method getController ()Ljavax/microedition/m3g/AnimationController;
- method getKeyframeSequence ()Ljavax/microedition/m3g/KeyframeSequence;
- method getTargetProperty ()I
- method setController (Ljavax/microedition/m3g/AnimationController;)V

## javax/microedition/m3g/Appearance
- field compositingMode Ljavax/microedition/m3g/CompositingMode;
- field fog Ljavax/microedition/m3g/Fog;
- field layer I
- field polygonMode Ljavax/microedition/m3g/PolygonMode;
- field material Ljavax/microedition/m3g/Material;
- field texture0 Ljavax/microedition/m3g/Texture2D;
- field texture1 Ljavax/microedition/m3g/Texture2D;
- method <init> ()V
- method getCompositingMode ()Ljavax/microedition/m3g/CompositingMode;
- method getFog ()Ljavax/microedition/m3g/Fog;
- method getMaterial ()Ljavax/microedition/m3g/Material;
- method getLayer ()I
- method getPolygonMode ()Ljavax/microedition/m3g/PolygonMode;
- method getTexture (I)Ljavax/microedition/m3g/Texture2D;
- method setCompositingMode (Ljavax/microedition/m3g/CompositingMode;)V
- method setFog (Ljavax/microedition/m3g/Fog;)V
- method setLayer (I)V
- method setMaterial (Ljavax/microedition/m3g/Material;)V
- method setPolygonMode (Ljavax/microedition/m3g/PolygonMode;)V
- method setTexture (ILjavax/microedition/m3g/Texture2D;)V

## javax/microedition/m3g/Background
- field BORDER I
- field REPEAT I
- field color I
- field image Ljavax/microedition/m3g/Image2D;
- field colorClear Z
- field depthClear Z
- field imageModeX I
- field imageModeY I
- field cropX I
- field cropY I
- field cropW I
- field cropH I
- method <clinit> ()V
- method <init> ()V
- method getColor ()I
- method getCropHeight ()I
- method getCropWidth ()I
- method getCropX ()I
- method getCropY ()I
- method getImage ()Ljavax/microedition/m3g/Image2D;
- method getImageModeX ()I
- method getImageModeY ()I
- method isColorClearEnabled ()Z
- method isDepthClearEnabled ()Z
- method setColor (I)V
- method setColorClearEnable (Z)V
- method setCrop (IIII)V
- method setDepthClearEnable (Z)V
- method setImage (Ljavax/microedition/m3g/Image2D;)V
- method setImageMode (II)V

## javax/microedition/m3g/Camera
- field GENERIC I
- field PARALLEL I
- field PERSPECTIVE I
- field projectionMode I
- field fovy F
- field parallelHeight F
- field aspect F
- field near F
- field far F
- field genericProjection [F
- method <clinit> ()V
- method <init> ()V
- method getProjection (Ljavax/microedition/m3g/Transform;)I
- method getProjection ([F)I
- method setGeneric (Ljavax/microedition/m3g/Transform;)V
- method setParallel (FFFF)V
- method setPerspective (FFFF)V

## javax/microedition/m3g/CompositingMode
- field ALPHA I
- field ALPHA_ADD I
- field MODULATE I
- field MODULATE_X2 I
- field REPLACE I
- field alphaThreshold F
- field alphaWrite Z
- field blending I
- field colorWrite Z
- field depthTest Z
- field depthWrite Z
- field depthOffsetFactor F
- field depthOffsetUnits F
- method <clinit> ()V
- method <init> ()V
- method getAlphaThreshold ()F
- method getBlending ()I
- method setAlphaThreshold (F)V
- method setAlphaWriteEnable (Z)V
- method setBlending (I)V
- method setColorWriteEnable (Z)V
- method getDepthOffsetFactor ()F
- method getDepthOffsetUnits ()F
- method isAlphaWriteEnabled ()Z
- method isColorWriteEnabled ()Z
- method isDepthTestEnabled ()Z
- method isDepthWriteEnabled ()Z
- method setDepthOffset (FF)V
- method setDepthTestEnable (Z)V
- method setDepthWriteEnable (Z)V

## javax/microedition/m3g/Fog
- field EXPONENTIAL I
- field LINEAR I
- field color I
- field density F
- field mode I
- field near F
- field far F
- method <clinit> ()V
- method <init> ()V
- method getColor ()I
- method getDensity ()F
- method getFarDistance ()F
- method getMode ()I
- method getNearDistance ()F
- method setColor (I)V
- method setDensity (F)V
- method setLinear (FF)V
- method setMode (I)V

## javax/microedition/m3g/Graphics3D
- field ANTIALIAS I
- field DITHER I
- field TRUE_COLOR I
- field OVERWRITE I
- field target Ljava/lang/Object;
- field viewportX I
- field viewportY I
- field viewportW I
- field viewportH I
- field camera Ljavax/microedition/m3g/Camera;
- field cameraTransform [F
- field cameraTransformSet Z
- field colorBuffer [I
- field colorBufferValid Z
- field depthBuffer [F
- field depthEnabled Z
- field depthRangeNear F
- field depthRangeFar F
- field hints I
- field lights [Ljavax/microedition/m3g/Light;
- field lightTransforms [Ljavax/microedition/m3g/Transform;
- field instance Ljavax/microedition/m3g/Graphics3D;
- method <clinit> ()V
- method <init> ()V
- method addLight (Ljavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)I
- method getInstance ()Ljavax/microedition/m3g/Graphics3D;
- method bindTarget (Ljava/lang/Object;)V
- method bindTarget (Ljava/lang/Object;ZI)V
- method clear (Ljavax/microedition/m3g/Background;)V
- method getCamera (Ljavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Camera;
- method getDepthRangeFar ()F
- method getDepthRangeNear ()F
- method getHints ()I
- method getLight (ILjavax/microedition/m3g/Transform;)Ljavax/microedition/m3g/Light;
- method getLightCount ()I
- method getProperties ()Ljava/util/Hashtable;
- method getTarget ()Ljava/lang/Object;
- method getViewportHeight ()I
- method getViewportWidth ()I
- method getViewportX ()I
- method getViewportY ()I
- method isDepthBufferEnabled ()Z
- method releaseTarget ()V
- method render (Ljavax/microedition/m3g/World;)V
- method render (Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)V
- method render (Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Transform;)V
- method render (Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Transform;I)V
- method resetLights ()V
- method setCamera (Ljavax/microedition/m3g/Camera;Ljavax/microedition/m3g/Transform;)V
- method setDepthRange (FF)V
- method setLight (ILjavax/microedition/m3g/Light;Ljavax/microedition/m3g/Transform;)V
- method setViewport (IIII)V

## javax/microedition/m3g/Group
- field children [Ljavax/microedition/m3g/Node;
- field childCount I
- method <init> ()V
- method addChild (Ljavax/microedition/m3g/Node;)V
- method getChild (I)Ljavax/microedition/m3g/Node;
- method getChildCount ()I
- method pick (IFFFFFFLjavax/microedition/m3g/RayIntersection;)Z
- method pick (IFFLjavax/microedition/m3g/Camera;Ljavax/microedition/m3g/RayIntersection;)Z
- method removeChild (Ljavax/microedition/m3g/Node;)V

## javax/microedition/m3g/Image2D
- field ALPHA I
- field LUMINANCE I
- field LUMINANCE_ALPHA I
- field RGB I
- field RGBA I
- field format I
- field width I
- field height I
- field image Ljavax/microedition/lcdui/Image;
- field mutable Z
- method <clinit> ()V
- method <init> ()V
- method <init> (ILjava/lang/Object;)V
- method <init> (ILjavax/microedition/lcdui/Image;)V
- method <init> (III)V
- method <init> (III[B)V
- method <init> (III[B[B)V
- method getFormat ()I
- method getWidth ()I
- method getHeight ()I
- method isMutable ()Z
- method set (IIII[B)V

## javax/microedition/m3g/IndexBuffer
- method <init> ()V
- method getIndexCount ()I
- method getIndices ([I)V

## javax/microedition/m3g/KeyframeSequence
- field LINEAR I
- field SLERP I
- field SPLINE I
- field SQUAD I
- field STEP I
- field CONSTANT I
- field LOOP I
- field interpolationType I
- field repeatMode I
- field duration I
- field validRangeFirst I
- field validRangeLast I
- field componentCount I
- field keyframeCount I
- field keyframeTimes [I
- field keyframeValues [F
- method <clinit> ()V
- method <init> (III)V
- method getComponentCount ()I
- method getDuration ()I
- method getInterpolationType ()I
- method getKeyframe (I[F)I
- method getKeyframeCount ()I
- method getRepeatMode ()I
- method getValidRangeFirst ()I
- method getValidRangeLast ()I
- method setDuration (I)V
- method setKeyframe (II[F)V
- method setRepeatMode (I)V
- method setValidRange (II)V

## javax/microedition/m3g/Light
- field AMBIENT I
- field DIRECTIONAL I
- field OMNI I
- field SPOT I
- field constantAttenuation F
- field linearAttenuation F
- field quadraticAttenuation F
- field color I
- field intensity F
- field mode I
- field spotAngle F
- field spotExponent F
- method <clinit> ()V
- method <init> ()V
- method getColor ()I
- method getConstantAttenuation ()F
- method getIntensity ()F
- method getLinearAttenuation ()F
- method getMode ()I
- method getQuadraticAttenuation ()F
- method getSpotAngle ()F
- method getSpotExponent ()F
- method setAttenuation (FFF)V
- method setColor (I)V
- method setIntensity (F)V
- method setMode (I)V
- method setSpotAngle (F)V
- method setSpotExponent (F)V

## javax/microedition/m3g/Loader
- method load (Ljava/lang/String;)[Ljavax/microedition/m3g/Object3D;
- method load ([BI)[Ljavax/microedition/m3g/Object3D;

## javax/microedition/m3g/Material
- field AMBIENT I
- field DIFFUSE I
- field EMISSIVE I
- field SPECULAR I
- field ambientColor I
- field diffuseColor I
- field emissiveColor I
- field specularColor I
- field shininess F
- field vertexColorTracking Z
- method <clinit> ()V
- method <init> ()V
- method getColor (I)I
- method getShininess ()F
- method isVertexColorTrackingEnabled ()Z
- method setColor (II)V
- method setShininess (F)V
- method setVertexColorTrackingEnable (Z)V

## javax/microedition/m3g/Mesh
- field vertexBuffer Ljavax/microedition/m3g/VertexBuffer;
- field submeshCount I
- field indexBuffers [Ljavax/microedition/m3g/IndexBuffer;
- field appearances [Ljavax/microedition/m3g/Appearance;
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;)V
- method getAppearance (I)Ljavax/microedition/m3g/Appearance;
- method getIndexBuffer (I)Ljavax/microedition/m3g/IndexBuffer;
- method getSubmeshCount ()I
- method getVertexBuffer ()Ljavax/microedition/m3g/VertexBuffer;
- method setAppearance (ILjavax/microedition/m3g/Appearance;)V

## javax/microedition/m3g/MorphingMesh
- field targets [Ljavax/microedition/m3g/VertexBuffer;
- field weights [F
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;)V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;)V
- method getMorphTargetCount ()I
- method getMorphTarget (I)Ljavax/microedition/m3g/VertexBuffer;
- method getWeights ([F)V
- method setWeights ([F)V

## javax/microedition/m3g/Node
- field NONE I
- field ORIGIN I
- field X_AXIS I
- field Y_AXIS I
- field Z_AXIS I
- field parent Ljavax/microedition/m3g/Node;
- field renderingEnabled Z
- field pickingEnabled Z
- field alphaFactor F
- field scope I
- field zReference Ljavax/microedition/m3g/Node;
- field yReference Ljavax/microedition/m3g/Node;
- field zTarget I
- field yTarget I
- method <clinit> ()V
- method <init> ()V
- method align (Ljavax/microedition/m3g/Node;)V
- method getAlignmentReference (I)Ljavax/microedition/m3g/Node;
- method getAlignmentTarget (I)I
- method getAlphaFactor ()F
- method getParent ()Ljavax/microedition/m3g/Node;
- method getScope ()I
- method getTransformTo (Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)Z
- method isPickingEnabled ()Z
- method isRenderingEnabled ()Z
- method setAlignment (Ljavax/microedition/m3g/Node;ILjavax/microedition/m3g/Node;I)V
- method setAlphaFactor (F)V
- method setPickingEnable (Z)V
- method setRenderingEnable (Z)V
- method setScope (I)V

## javax/microedition/m3g/Object3D
- field userID I
- field userObject Ljava/lang/Object;
- field animationTracks [Ljavax/microedition/m3g/AnimationTrack;
- method <init> ()V
- method addAnimationTrack (Ljavax/microedition/m3g/AnimationTrack;)V
- method animate (I)I
- method duplicate ()Ljavax/microedition/m3g/Object3D;
- method find (I)Ljavax/microedition/m3g/Object3D;
- method getAnimationTrack (I)Ljavax/microedition/m3g/AnimationTrack;
- method getAnimationTrackCount ()I
- method getUserID ()I
- method removeAnimationTrack (Ljavax/microedition/m3g/AnimationTrack;)V
- method setUserID (I)V
- method setUserObject (Ljava/lang/Object;)V
- method getUserObject ()Ljava/lang/Object;
- method getReferences ([Ljavax/microedition/m3g/Object3D;)I

## javax/microedition/m3g/PolygonMode
- field CULL_BACK I
- field CULL_FRONT I
- field CULL_NONE I
- field SHADE_FLAT I
- field SHADE_SMOOTH I
- field WINDING_CCW I
- field WINDING_CW I
- field culling I
- field localCameraLighting Z
- field perspectiveCorrection Z
- field shading I
- field twoSidedLighting Z
- field winding I
- method <clinit> ()V
- method <init> ()V
- method getCulling ()I
- method getShading ()I
- method getWinding ()I
- method isLocalCameraLightingEnabled ()Z
- method isPerspectiveCorrectionEnabled ()Z
- method isTwoSidedLightingEnabled ()Z
- method setCulling (I)V
- method setLocalCameraLightingEnable (Z)V
- method setPerspectiveCorrectionEnable (Z)V
- method setShading (I)V
- method setTwoSidedLightingEnable (Z)V
- method setWinding (I)V

## javax/microedition/m3g/RayIntersection
- field intersected Ljavax/microedition/m3g/Node;
- field distance F
- field submeshIndex I
- field textureS [F
- field textureT [F
- field normal [F
- field ray [F
- method <init> ()V
- method getDistance ()F
- method getIntersected ()Ljavax/microedition/m3g/Node;
- method getNormalX ()F
- method getNormalY ()F
- method getNormalZ ()F
- method getRay ([F)V
- method getSubmeshIndex ()I
- method getTextureS (I)F
- method getTextureT (I)F

## javax/microedition/m3g/SkinnedMesh
- field skeleton Ljavax/microedition/m3g/Group;
- field boneNodes [Ljavax/microedition/m3g/Node;
- field boneWeights [I
- field boneFirst [I
- field boneCount [I
- field boneBind [F
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Group;)V
- method <init> (Ljavax/microedition/m3g/VertexBuffer;[Ljavax/microedition/m3g/IndexBuffer;[Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Group;)V
- method getSkeleton ()Ljavax/microedition/m3g/Group;
- method addTransform (Ljavax/microedition/m3g/Node;III)V
- method getBoneTransform (Ljavax/microedition/m3g/Node;Ljavax/microedition/m3g/Transform;)V
- method getBoneVertices (Ljavax/microedition/m3g/Node;[I[F)I

## javax/microedition/m3g/Sprite3D
- field scaled Z
- field image Ljavax/microedition/m3g/Image2D;
- field appearance Ljavax/microedition/m3g/Appearance;
- field cropX I
- field cropY I
- field cropW I
- field cropH I
- method <init> (ZLjavax/microedition/m3g/Image2D;Ljavax/microedition/m3g/Appearance;)V
- method getAppearance ()Ljavax/microedition/m3g/Appearance;
- method getCropHeight ()I
- method getCropWidth ()I
- method getCropX ()I
- method getCropY ()I
- method getImage ()Ljavax/microedition/m3g/Image2D;
- method isScaled ()Z
- method setAppearance (Ljavax/microedition/m3g/Appearance;)V
- method setCrop (IIII)V
- method setImage (Ljavax/microedition/m3g/Image2D;)V

## javax/microedition/m3g/Texture2D
- field FILTER_BASE_LEVEL I
- field FILTER_LINEAR I
- field FILTER_NEAREST I
- field FUNC_ADD I
- field FUNC_BLEND I
- field FUNC_DECAL I
- field FUNC_MODULATE I
- field FUNC_REPLACE I
- field WRAP_CLAMP I
- field WRAP_REPEAT I
- field image Ljavax/microedition/m3g/Image2D;
- field blendColor I
- field blending I
- field wrappingS I
- field wrappingT I
- field levelFilter I
- field imageFilter I
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/Image2D;)V
- method getBlendColor ()I
- method getBlending ()I
- method getImage ()Ljavax/microedition/m3g/Image2D;
- method getImageFilter ()I
- method getLevelFilter ()I
- method getWrappingS ()I
- method getWrappingT ()I
- method setBlendColor (I)V
- method setBlending (I)V
- method setFiltering (II)V
- method setImage (Ljavax/microedition/m3g/Image2D;)V
- method setWrapping (II)V

## javax/microedition/m3g/Transform
- field matrix [F
- method <init> ()V
- method <init> (Ljavax/microedition/m3g/Transform;)V
- method get ([F)V
- method invert ()V
- method postMultiply (Ljavax/microedition/m3g/Transform;)V
- method set ([F)V
- method set (Ljavax/microedition/m3g/Transform;)V
- method setIdentity ()V
- method postRotate (FFFF)V
- method postRotateQuat (FFFF)V
- method postScale (FFF)V
- method postTranslate (FFF)V
- method transform ([F)V
- method transform (Ljavax/microedition/m3g/VertexArray;[FZ)V
- method transpose ()V

## javax/microedition/m3g/Transformable
- field translationX F
- field translationY F
- field translationZ F
- field scaleX F
- field scaleY F
- field scaleZ F
- field orientationAngle F
- field orientationX F
- field orientationY F
- field orientationZ F
- field transform [F
- method <init> ()V
- method getCompositeTransform (Ljavax/microedition/m3g/Transform;)V
- method getTransform (Ljavax/microedition/m3g/Transform;)V
- method getOrientation ([F)V
- method getScale ([F)V
- method getTranslation ([F)V
- method postRotate (FFFF)V
- method preRotate (FFFF)V
- method scale (FFF)V
- method setOrientation (FFFF)V
- method setScale (FFF)V
- method setTransform (Ljavax/microedition/m3g/Transform;)V
- method setTranslation (FFF)V
- method translate (FFF)V

## javax/microedition/m3g/TriangleStripArray
- field indices [I
- field stripLengths [I
- field triangleCache [I
- method <init> ()V
- method <init> (I[I)V
- method <init> ([I[I)V
- method getIndexCount ()I
- method getIndices ([I)V

## javax/microedition/m3g/VertexArray
- field componentSize I
- field componentCount I
- field vertexCount I
- field version I
- field byteData [B
- field shortData [S
- method <init> ()V
- method <init> (III)V
- method get (II[B)V
- method get (II[S)V
- method getComponentCount ()I
- method getComponentType ()I
- method getVertexCount ()I
- method set (II[B)V
- method set (II[S)V

## javax/microedition/m3g/VertexBuffer
- field defaultColor I
- field positions Ljavax/microedition/m3g/VertexArray;
- field normals Ljavax/microedition/m3g/VertexArray;
- field colors Ljavax/microedition/m3g/VertexArray;
- field texCoords0 Ljavax/microedition/m3g/VertexArray;
- field texCoords1 Ljavax/microedition/m3g/VertexArray;
- field positionScale F
- field positionBias [F
- field texScale F
- field texBias [F
- field texScale1 F
- field texBias1 [F
- method <init> ()V
- method getColors ()Ljavax/microedition/m3g/VertexArray;
- method getDefaultColor ()I
- method getNormals ()Ljavax/microedition/m3g/VertexArray;
- method getPositions ([F)Ljavax/microedition/m3g/VertexArray;
- method getTexCoords (I[F)Ljavax/microedition/m3g/VertexArray;
- method getVertexCount ()I
- method setColors (Ljavax/microedition/m3g/VertexArray;)V
- method setDefaultColor (I)V
- method setNormals (Ljavax/microedition/m3g/VertexArray;)V
- method setPositions (Ljavax/microedition/m3g/VertexArray;F[F)V
- method setTexCoords (ILjavax/microedition/m3g/VertexArray;F[F)V

## javax/microedition/m3g/World
- field activeCamera Ljavax/microedition/m3g/Camera;
- field background Ljavax/microedition/m3g/Background;
- method <init> ()V
- method getActiveCamera ()Ljavax/microedition/m3g/Camera;
- method getBackground ()Ljavax/microedition/m3g/Background;
- method setActiveCamera (Ljavax/microedition/m3g/Camera;)V
- method setBackground (Ljavax/microedition/m3g/Background;)V

## javax/microedition/media/Control

## javax/microedition/media/Controllable
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;

## javax/microedition/media/Manager
- field TONE_DEVICE_LOCATOR Ljava/lang/String;
- field MIDI_DEVICE_LOCATOR Ljava/lang/String;
- method <clinit> ()V
- method createPlayer (Ljava/io/InputStream;Ljava/lang/String;)Ljavax/microedition/media/Player;
- method createPlayer (Ljava/lang/String;)Ljavax/microedition/media/Player;
- method playTone (III)V
- method getSupportedContentTypes (Ljava/lang/String;)[Ljava/lang/String;
- method getSupportedProtocols (Ljava/lang/String;)[Ljava/lang/String;
- method createPlayer (Ljava/lang/String;)Lcom/siemens/mp/media/Player;
- method createPlayer (Ljava/lang/String;)Lcom/siemens/mp/media/Player;
- method createPlayer (Ljavax/microedition/media/protocol/DataSource;)Ljavax/microedition/media/Player;
- method createPlayer (Ljavax/microedition/media/protocol/DataSource;)Ljavax/microedition/media/Player;
- method getSystemTimeBase ()Ljavax/microedition/media/TimeBase;
- method getSystemTimeBase ()Ljavax/microedition/media/TimeBase;

## javax/microedition/media/MediaException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/media/control/MIDIControl
- method <init> ()V
- method isBankQuerySupported ()Z
- method getChannelVolume (I)I
- method setChannelVolume (II)V
- method getProgram (I)[I
- method setProgram (III)V
- method shortMidiEvent (III)V
- method longMidiEvent ([BII)I
- method longMidiEvent ([BII)I

## javax/microedition/media/control/MetaDataControl
- method <init> ()V
- method getKeys ()[Ljava/lang/String;
- method getKeyValue (Ljava/lang/String;)Ljava/lang/String;

## javax/microedition/media/Player
- field state I
- field loopCount I
- field mediaTime J
- field UNREALIZED I
- field REALIZED I
- field PREFETCHED I
- field STARTED I
- field CLOSED I
- field TIME_UNKNOWN J
- field listeners Ljava/util/Vector;
- method <clinit> ()V
- method <init> ()V
- method realize ()V
- method prefetch ()V
- method start ()V
- method stop ()V
- method close ()V
- method deallocate ()V
- method addPlayerListener (Ljavax/microedition/media/PlayerListener;)V
- method removePlayerListener (Ljavax/microedition/media/PlayerListener;)V
- method getState ()I
- method setLoopCount (I)V
- method setMediaTime (J)J
- method getMediaTime ()J
- method getDuration ()J
- method getContentType ()Ljava/lang/String;
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;
- method getTimeBase ()Ljavax/microedition/media/TimeBase;
- method getTimeBase ()Ljavax/microedition/media/TimeBase;
- method setTimeBase (Ljavax/microedition/media/TimeBase;)V
- method setTimeBase (Ljavax/microedition/media/TimeBase;)V

## javax/microedition/media/PlayerListener
- field STARTED Ljava/lang/String;
- field END_OF_MEDIA Ljava/lang/String;
- field STOPPED Ljava/lang/String;
- field STOPPED_AT_TIME Ljava/lang/String;
- field CLOSED Ljava/lang/String;
- field ERROR Ljava/lang/String;
- field DEVICE_AVAILABLE Ljava/lang/String;
- field DEVICE_UNAVAILABLE Ljava/lang/String;
- field VOLUME_CHANGED Ljava/lang/String;
- field DURATION_UPDATED Ljava/lang/String;
- method <clinit> ()V
- method playerUpdate (Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V

## javax/microedition/media/control/StopTimeControl
- field stopTime J
- method <init> ()V
- method getStopTime ()J
- method setStopTime (J)V

## javax/microedition/media/control/ToneControl
- method <init> ()V
- method setSequence ([B)V

## javax/microedition/media/control/VideoControl
- method <init> ()V
- method initDisplayMode (ILjava/lang/Object;)Ljava/lang/Object;
- method setDisplaySize (II)V
- method setDisplayLocation (II)V
- method setDisplayFullScreen (Z)V
- method setVisible (Z)V
- method getSourceWidth ()I
- method getSourceHeight ()I
- method getSnapshot (Ljava/lang/String;)[B
- method getDisplayHeight ()I
- method getDisplayHeight ()I
- method getDisplayWidth ()I
- method getDisplayWidth ()I
- method getDisplayX ()I
- method getDisplayX ()I
- method getDisplayY ()I
- method getDisplayY ()I

## javax/microedition/media/control/VolumeControl
- field level I
- field muted Z
- method <init> ()V
- method getLevel ()I
- method isMuted ()Z
- method setLevel (I)I
- method setMute (Z)V

## javax/microedition/midlet/MIDlet
- method <init> ()V
- method startApp ()V
- method pauseApp ()V
- method destroyApp (Z)V
- method notifyDestroyed ()V
- method notifyPaused ()V
- method getAppProperty (Ljava/lang/String;)Ljava/lang/String;
- method platformRequest (Ljava/lang/String;)Z
- method checkPermission (Ljava/lang/String;)I
- method _notifyDestroyedV ()V
- method _notifyDestroyedV ()V
- method _notifyPausedV ()V
- method _notifyPausedV ()V
- method resumeRequest ()V
- method resumeRequest ()V

## javax/microedition/midlet/MIDletStateChangeException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/pim/Contact
- method <init> ()V
- method countValues (I)I
- method getString (II)Ljava/lang/String;
- method addBoolean (IIZ)V
- method addBoolean (IIZ)V
- method addDate (IIJ)V
- method addDate (IIJ)V
- method addInt (III)V
- method addInt (III)V
- method addString (IILjava/lang/String;)V
- method addString (IILjava/lang/String;)V
- method addStringArray (II[Ljava/lang/String;)V
- method addStringArray (II[Ljava/lang/String;)V
- method addToCategory (Ljava/lang/String;)V
- method addToCategory (Ljava/lang/String;)V
- method commit ()V
- method commit ()V
- method getAttributes (II)I
- method getAttributes (II)I
- method getBinary (II)[B
- method getBinary (II)[B
- method getBoolean (II)Z
- method getBoolean (II)Z
- method getCategories ()[Ljava/lang/String;
- method getCategories ()[Ljava/lang/String;
- method getDate (II)J
- method getDate (II)J
- method getFields ()[I
- method getFields ()[I
- method getInt (II)I
- method getInt (II)I
- method getPIMList ()Ljavax/microedition/pim/PIMList;
- method getPIMList ()Ljavax/microedition/pim/PIMList;
- method getPreferredIndex (I)I
- method getPreferredIndex (I)I
- method getStringArray (II)[Ljava/lang/String;
- method getStringArray (II)[Ljava/lang/String;
- method removeValue (II)V
- method removeValue (II)V
- method setBoolean (IIIZ)V
- method setBoolean (IIIZ)V
- method setDate (IIIJ)V
- method setDate (IIIJ)V
- method setInt (IIII)V
- method setInt (IIII)V
- method setString (IIILjava/lang/String;)V
- method setString (IIILjava/lang/String;)V
- method setStringArray (III[Ljava/lang/String;)V
- method setStringArray (III[Ljava/lang/String;)V

## javax/microedition/pim/ContactList
- method <init> ()V
- method close ()V
- method isSupportedField (I)Z
- method items ()Ljava/util/Enumeration;
- method addCategory (Ljava/lang/String;)V
- method addCategory (Ljava/lang/String;)V
- method createContact ()Ljavax/microedition/pim/Contact;
- method createContact ()Ljavax/microedition/pim/Contact;
- method getAttributeLabel (I)Ljava/lang/String;
- method getAttributeLabel (I)Ljava/lang/String;
- method getCategories ()[Ljava/lang/String;
- method getCategories ()[Ljava/lang/String;
- method getFieldDataType (I)I
- method getFieldDataType (I)I
- method getFieldLabel (I)Ljava/lang/String;
- method getFieldLabel (I)Ljava/lang/String;
- method getSupportedAttributes (I)[I
- method getSupportedAttributes (I)[I
- method getSupportedFields ()[I
- method getSupportedFields ()[I
- method importContact (Ljavax/microedition/pim/Contact;)Ljavax/microedition/pim/Contact;
- method importContact (Ljavax/microedition/pim/Contact;)Ljavax/microedition/pim/Contact;
- method isCategory (Ljava/lang/String;)Z
- method isCategory (Ljava/lang/String;)Z
- method isSupportedArrayElement (II)Z
- method isSupportedArrayElement (II)Z
- method isSupportedAttribute (II)Z
- method isSupportedAttribute (II)Z
- method items (Ljava/lang/String;)Ljava/util/Enumeration;
- method items (Ljava/lang/String;)Ljava/util/Enumeration;
- method items (Ljavax/microedition/pim/PIMItem;)Ljava/util/Enumeration;
- method items (Ljavax/microedition/pim/PIMItem;)Ljava/util/Enumeration;
- method maxValues (I)I
- method maxValues (I)I
- method removeContact (Ljavax/microedition/pim/Contact;)V
- method removeContact (Ljavax/microedition/pim/Contact;)V
- method stringArraySize (I)I
- method stringArraySize (I)I

## javax/microedition/pim/PIM
- method <init> ()V
- method getInstance ()Ljavax/microedition/pim/PIM;
- method openPIMList (II)Ljavax/microedition/pim/PIMList;
- method fromSerialFormat (Ljava/io/InputStream;Ljava/lang/String;)[Ljavax/microedition/pim/PIMItem;
- method fromSerialFormat (Ljava/io/InputStream;Ljava/lang/String;)[Ljavax/microedition/pim/PIMItem;
- method listPIMLists (I)[Ljava/lang/String;
- method listPIMLists (I)[Ljava/lang/String;
- method openPIMList (IILjava/lang/String;)Ljavax/microedition/pim/PIMList;
- method openPIMList (IILjava/lang/String;)Ljavax/microedition/pim/PIMList;
- method supportedSerialFormats (I)[Ljava/lang/String;
- method supportedSerialFormats (I)[Ljava/lang/String;
- method toSerialFormat (Ljavax/microedition/pim/PIMItem;Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V
- method toSerialFormat (Ljavax/microedition/pim/PIMItem;Ljava/io/OutputStream;Ljava/lang/String;Ljava/lang/String;)V

## javax/microedition/pim/PIMException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getReason ()I
- method getReason ()I

## javax/microedition/pim/PIMList
- method close ()V
- method close ()V
- method getArrayElementLabel (II)Ljava/lang/String;
- method getArrayElementLabel (II)Ljava/lang/String;
- method getAttributeLabel (I)Ljava/lang/String;
- method getAttributeLabel (I)Ljava/lang/String;
- method getFieldDataType (I)I
- method getFieldDataType (I)I
- method getFieldLabel (I)Ljava/lang/String;
- method getFieldLabel (I)Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getSupportedFields ()[I
- method getSupportedFields ()[I
- method isSupportedArrayElement (II)Z
- method isSupportedArrayElement (II)Z
- method isSupportedAttribute (II)Z
- method isSupportedAttribute (II)Z
- method isSupportedField (I)Z
- method isSupportedField (I)Z
- method items ()Ljava/util/Enumeration;
- method items ()Ljava/util/Enumeration;
- method items (Ljava/lang/String;)Ljava/util/Enumeration;
- method items (Ljava/lang/String;)Ljava/util/Enumeration;
- method items (Ljavax/microedition/pim/PIMItem;)Ljava/util/Enumeration;
- method items (Ljavax/microedition/pim/PIMItem;)Ljava/util/Enumeration;
- method maxValues (I)I
- method maxValues (I)I
- method stringArraySize (I)I
- method stringArraySize (I)I

## javax/microedition/pim/UnsupportedFieldException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/rms/InvalidRecordIDException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/rms/RecordComparator
- method compare ([B[B)I

## javax/microedition/rms/RecordEnumeration
- field store Ljavax/microedition/rms/RecordStore;
- field ids [I
- field position I
- field keepUpdated Z
- field destroyed Z
- method <init> (Ljavax/microedition/rms/RecordStore;[IZ)V
- method destroy ()V
- method hasNextElement ()Z
- method hasPreviousElement ()Z
- method isKeptUpdated ()Z
- method keepUpdated (Z)V
- method nextRecord ()[B
- method nextRecordId ()I
- method numRecords ()I
- method previousRecord ()[B
- method previousRecordId ()I
- method rebuild ()V
- method reset ()V

## javax/microedition/rms/RecordFilter
- method matches ([B)Z

## javax/microedition/rms/RecordListener
- method recordAdded (Ljavax/microedition/rms/RecordStore;I)V
- method recordChanged (Ljavax/microedition/rms/RecordStore;I)V
- method recordDeleted (Ljavax/microedition/rms/RecordStore;I)V

## javax/microedition/rms/RecordStore
- field name Ljava/lang/String;
- method <init> (Ljava/lang/String;)V
- method openRecordStore (Ljava/lang/String;Z)Ljavax/microedition/rms/RecordStore;
- method openRecordStore (Ljava/lang/String;ZIZ)Ljavax/microedition/rms/RecordStore;
- method closeRecordStore ()V
- method getNumRecords ()I
- method getNextRecordID ()I
- method addRecord ([BII)I
- method deleteRecord (I)V
- method deleteRecordStore (Ljava/lang/String;)V
- method enumerateRecords (Ljavax/microedition/rms/RecordFilter;Ljavax/microedition/rms/RecordComparator;Z)Ljavax/microedition/rms/RecordEnumeration;
- method getLastModified ()J
- method getName ()Ljava/lang/String;
- method getRecord (I)[B
- method getRecordSize (I)I
- method setRecord (I[BII)V
- method getRecord (I[BI)I
- method getVersion ()I
- method getSize ()I
- method getSizeAvailable ()I
- method addRecordListener (Ljavax/microedition/rms/RecordListener;)V
- method removeRecordListener (Ljavax/microedition/rms/RecordListener;)V
- method listRecordStores ()[Ljava/lang/String;
- method openRecordStore (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/rms/RecordStore;
- method setMode (IZ)V
- method epenRecordStore (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/rms/RecordStore;
- method epenRecordStore (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/rms/RecordStore;
- method epenRecordStore (Ljava/lang/String;ZIZ)Ljavax/microedition/rms/RecordStore;
- method epenRecordStore (Ljava/lang/String;ZIZ)Ljavax/microedition/rms/RecordStore;

## javax/microedition/rms/RecordStoreException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/rms/RecordStoreFullException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/rms/RecordStoreNotFoundException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/rms/RecordStoreNotOpenException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/sensor/ChannelInfo
- field TYPE_DOUBLE I
- field TYPE_DOUBLE I
- field TYPE_INT I
- field TYPE_INT I
- method <init> ()V
- method getDataType ()I
- method getAccuracy ()F
- method getAccuracy ()F
- method getMeasurementRanges ()[Ljavax/microedition/sensor/MeasurementRange;
- method getMeasurementRanges ()[Ljavax/microedition/sensor/MeasurementRange;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getScale ()I
- method getScale ()I
- method getUnit ()Ljavax/microedition/sensor/Unit;
- method getUnit ()Ljavax/microedition/sensor/Unit;

## javax/microedition/sensor/Data
- method <init> ()V
- method getDoubleValues ()[D
- method getIntValues ()[I
- method getChannelInfo ()Ljavax/microedition/sensor/ChannelInfo;
- method getChannelInfo ()Ljavax/microedition/sensor/ChannelInfo;
- method getObjectValues ()[Ljava/lang/Object;
- method getObjectValues ()[Ljava/lang/Object;

## javax/microedition/sensor/DataListener

## javax/microedition/sensor/SensorConnection
- method <init> ()V
- method getSensorInfo ()Ljavax/microedition/sensor/SensorInfo;
- method setDataListener (Ljavax/microedition/sensor/DataListener;I)V
- method close ()V
- method close ()V
- method getData (I)[Ljavax/microedition/sensor/Data;
- method getData (I)[Ljavax/microedition/sensor/Data;
- method getData (IJZZZ)[Ljavax/microedition/sensor/Data;
- method getData (IJZZZ)[Ljavax/microedition/sensor/Data;
- method getState ()I
- method getState ()I
- method removeDataListener ()V
- method removeDataListener ()V
- method setDataListener (Ljavax/microedition/sensor/DataListener;IJZZZ)V
- method setDataListener (Ljavax/microedition/sensor/DataListener;IJZZZ)V

## javax/microedition/sensor/SensorInfo
- field CONTEXT_TYPE_DEVICE Ljava/lang/String;
- field CONTEXT_TYPE_DEVICE Ljava/lang/String;
- field CONTEXT_TYPE_USER Ljava/lang/String;
- field CONTEXT_TYPE_USER Ljava/lang/String;
- method <init> ()V
- method getChannelInfos ()[Ljavax/microedition/sensor/ChannelInfo;
- method getUrl ()Ljava/lang/String;
- method getConnectionType ()I
- method getConnectionType ()I
- method getContextType ()Ljava/lang/String;
- method getContextType ()Ljava/lang/String;
- method getDescription ()Ljava/lang/String;
- method getDescription ()Ljava/lang/String;
- method getMaxBufferSize ()I
- method getMaxBufferSize ()I
- method getModel ()Ljava/lang/String;
- method getModel ()Ljava/lang/String;
- method getProperty (Ljava/lang/String;)Ljava/lang/Object;
- method getProperty (Ljava/lang/String;)Ljava/lang/Object;
- method getPropertyNames ()[Ljava/lang/String;
- method getPropertyNames ()[Ljava/lang/String;
- method getQuantity ()Ljava/lang/String;
- method getQuantity ()Ljava/lang/String;
- method isAvailabilityPushSupported ()Z
- method isAvailabilityPushSupported ()Z
- method isAvailable ()Z
- method isAvailable ()Z
- method isConditionPushSupported ()Z
- method isConditionPushSupported ()Z

## javax/microedition/sensor/SensorManager
- method findSensors (Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;
- method findSensors (Ljava/lang/String;Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;

## javax/bluetooth/BluetoothConnectionException
- field status I
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getStatus ()I

## javax/bluetooth/BluetoothStateException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/bluetooth/DataElement
- field type I
- field longValue J
- field value Ljava/lang/Object;
- field NULL I
- field U_INT_1 I
- field U_INT_2 I
- field U_INT_4 I
- field INT_1 I
- field INT_2 I
- field INT_4 I
- field INT_8 I
- field URL I
- field UUID I
- field BOOL I
- field STRING I
- field DATSEQ I
- field DATALT I
- method <clinit> ()V
- method <init> ()V
- method <init> (I)V
- method <init> (IJ)V
- method <init> (Ljava/lang/Object;)V
- method <init> (ILjava/lang/Object;)V
- method <init> (Z)V
- method addElement (Ljavax/bluetooth/DataElement;)V
- method getDataType ()I
- method getLong ()J
- method getValue ()Ljava/lang/Object;
- method getBoolean ()Z
- method getBoolean ()Z
- method getSize ()I
- method getSize ()I
- method removeElement (Ljavax/bluetooth/DataElement;)Z
- method removeElement (Ljavax/bluetooth/DataElement;)Z

## javax/bluetooth/DeviceClass
- field cod I
- method <init> ()V
- method <init> (I)V
- method getMajorDeviceClass ()I
- method getMinorDeviceClass ()I
- method getServiceClasses ()I

## javax/bluetooth/DiscoveryAgent
- field GIAC I
- field LIAC I
- field NOT_DISCOVERABLE I
- field CACHED I
- field PREKNOWN I
- method <clinit> ()V
- method <init> ()V
- method cancelInquiry (Ljavax/bluetooth/DiscoveryListener;)Z
- method cancelServiceSearch (I)Z
- method searchServices ([I[Ljavax/bluetooth/UUID;Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DiscoveryListener;)I
- method startInquiry (ILjavax/bluetooth/DiscoveryListener;)Z
- method retrieveDevices (I)[Ljavax/bluetooth/RemoteDevice;
- method selectService (Ljavax/bluetooth/UUID;IZ)Ljava/lang/String;
- method selectService (Ljavax/bluetooth/UUID;IZ)Ljava/lang/String;

## javax/bluetooth/DiscoveryListener
- field INQUIRY_COMPLETED I
- field INQUIRY_TERMINATED I
- field INQUIRY_ERROR I
- field SERVICE_SEARCH_COMPLETED I
- field SERVICE_SEARCH_TERMINATED I
- field SERVICE_SEARCH_ERROR I
- field SERVICE_SEARCH_NO_RECORDS I
- field SERVICE_SEARCH_DEVICE_NOT_REACHABLE I
- method deviceDiscovered (Ljavax/bluetooth/RemoteDevice;Ljavax/bluetooth/DeviceClass;)V
- method inquiryCompleted (I)V
- method servicesDiscovered (I[Ljavax/bluetooth/ServiceRecord;)V
- method serviceSearchCompleted (II)V
- method <clinit> ()V

## javax/bluetooth/L2CAPConnection
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method ready ()Z
- method receive ([B)I
- method send ([B)V
- method getTransmitMTU ()I
- method getReceiveMTU ()I

## javax/bluetooth/L2CAPConnectionNotifier
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method acceptAndOpen ()Ljavax/bluetooth/L2CAPConnection;

## javax/bluetooth/LocalDevice
- field instance Ljavax/bluetooth/LocalDevice;
- field agent Ljavax/bluetooth/DiscoveryAgent;
- method <init> ()V
- method getDiscoveryAgent ()Ljavax/bluetooth/DiscoveryAgent;
- method getLocalDevice ()Ljavax/bluetooth/LocalDevice;
- method setDiscoverable (I)Z
- method getDiscoverable ()I
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method getRecord (Ljavax/microedition/io/Connection;)Ljavax/bluetooth/ServiceRecord;
- method updateRecord (Ljavax/bluetooth/ServiceRecord;)V
- method getBluetoothAddress ()Ljava/lang/String;
- method getFriendlyName ()Ljava/lang/String;
- method getDeviceClass ()Ljavax/bluetooth/DeviceClass;
- method isPowerOn ()Z

## javax/bluetooth/RemoteDevice
- field address Ljava/lang/String;
- field name Ljava/lang/String;
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getBluetoothAddress ()Ljava/lang/String;
- method getFriendlyName (Z)Ljava/lang/String;
- method authenticate ()Z
- method authenticate ()Z
- method getRemoteDevice (Ljavax/microedition/io/Connection;)Ljavax/bluetooth/RemoteDevice;
- method getRemoteDevice (Ljavax/microedition/io/Connection;)Ljavax/bluetooth/RemoteDevice;
- method isAuthenticated ()Z
- method isAuthenticated ()Z
- method isEncrypted ()Z
- method isEncrypted ()Z
- method isTrustedDevice ()Z
- method isTrustedDevice ()Z

## javax/bluetooth/ServiceRecord
- field host Ljavax/bluetooth/RemoteDevice;
- method <init> ()V
- method getAttributeValue (I)Ljavax/bluetooth/DataElement;
- method getConnectionURL (IZ)Ljava/lang/String;
- method getHostDevice ()Ljavax/bluetooth/RemoteDevice;
- method setAttributeValue (ILjavax/bluetooth/DataElement;)Z
- method setDeviceServiceClasses (I)V
- method getAttributeIDs ()[I
- method getAttributeIDs ()[I
- method populateRecord ([I)Z
- method populateRecord ([I)Z

## javax/bluetooth/ServiceRegistrationException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/bluetooth/UUID
- field value Ljava/lang/String;
- method <init> (J)V
- method <init> (Ljava/lang/String;Z)V
- method toString ()Ljava/lang/String;
- method equals (Ljava/lang/Object;)Z

## javax/obex/ClientSession
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method connect (Ljavax/obex/HeaderSet;)Ljavax/obex/HeaderSet;
- method createHeaderSet ()Ljavax/obex/HeaderSet;
- method disconnect (Ljavax/obex/HeaderSet;)Ljavax/obex/HeaderSet;
- method put (Ljavax/obex/HeaderSet;)Ljavax/obex/Operation;
- method setAuthenticator (Ljavax/obex/Authenticator;)V
- method setPath (Ljavax/obex/HeaderSet;ZZ)Ljavax/obex/HeaderSet;

## javax/obex/HeaderSet
- field NAME I
- field TYPE I
- field LENGTH I
- field TIME_ISO_8601 I
- field TIME_4_BYTE I
- field DESCRIPTION I
- field TARGET I
- field HTTP I
- field WHO I
- field OBJECT_CLASS I
- field APPLICATION_PARAMETER I
- method <clinit> ()V
- method <init> ()V
- method getHeader (I)Ljava/lang/Object;
- method getHeaderList ()[I
- method getResponseCode ()I
- method setHeader (ILjava/lang/Object;)V

## javax/obex/Operation
- method <init> ()V
- method abort ()V
- method close ()V
- method getReceivedHeaders ()Ljavax/obex/HeaderSet;
- method getResponseCode ()I
- method getType ()Ljava/lang/String;
- method getEncoding ()Ljava/lang/String;
- method getLength ()J
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;

## javax/wireless/messaging/BinaryMessage
- field payload [B
- method <init> ()V
- method getPayloadData ()[B
- method setPayloadData ([B)V

## javax/wireless/messaging/Message
- field address Ljava/lang/String;
- field timestamp Ljava/util/Date;
- method <init> ()V
- method getAddress ()Ljava/lang/String;
- method setAddress (Ljava/lang/String;)V
- method getTimestamp ()Ljava/util/Date;
- method etAddr (Ljava/lang/String;)V
- method etAddr (Ljava/lang/String;)V

## javax/wireless/messaging/MessageConnection
- field url Ljava/lang/String;
- field listener Ljavax/wireless/messaging/MessageListener;
- field TEXT_MESSAGE Ljava/lang/String;
- field BINARY_MESSAGE Ljava/lang/String;
- field MULTIPART_MESSAGE Ljava/lang/String;
- method <clinit> ()V
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method newMessage (Ljava/lang/String;)Ljavax/wireless/messaging/Message;
- method newMessage (Ljava/lang/String;Ljava/lang/String;)Ljavax/wireless/messaging/Message;
- method receive ()Ljavax/wireless/messaging/Message;
- method send (Ljavax/wireless/messaging/Message;)V
- method setMessageListener (Ljavax/wireless/messaging/MessageListener;)V
- method numberOfSegments (Ljavax/wireless/messaging/Message;)I

## javax/wireless/messaging/MessageListener
- method notifyIncomingMessage (Ljavax/wireless/messaging/MessageConnection;)V

## javax/wireless/messaging/TextMessage
- field payload Ljava/lang/String;
- method <init> ()V
- method getPayloadText ()Ljava/lang/String;
- method setPayloadText (Ljava/lang/String;)V

## javax/xml/parsers/ParserConfigurationException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/xml/parsers/SAXParser
- method <init> ()V
- method parse (Lorg/xml/sax/InputSource;Lorg/xml/sax/helpers/DefaultHandler;)V
- method parse (Ljava/io/InputStream;Lorg/xml/sax/helpers/DefaultHandler;)V

## javax/xml/parsers/SAXParserFactory
- method <init> ()V
- method newInstance ()Ljavax/xml/parsers/SAXParserFactory;
- method newSAXParser ()Ljavax/xml/parsers/SAXParser;
- method setNamespaceAware (Z)V
- method setValidating (Z)V

## org/rustjava/net/FileURLConnection
- field file Ljava/io/File;
- method <init> (Ljava/net/URL;Ljava/io/File;)V
- method getInputStream ()Ljava/io/InputStream;

## org/rustjava/net/FileURLHandler
- method <init> ()V
- method openConnection (Ljava/net/URL;)Ljava/net/URLConnection;

## org/rustjava/net/JarURLConnection
- field openedFiles Ljava/util/Hashtable;
- method <clinit> ()V
- method <init> (Ljava/net/URL;)V
- method getJarFile ()Ljava/util/jar/JarFile;
- method getInputStream ()Ljava/io/InputStream;

## org/rustjava/net/JarURLHandler
- method <init> ()V
- method openConnection (Ljava/net/URL;)Ljava/net/URLConnection;

## org/xml/sax/Attributes
- method <init> ()V

## org/xml/sax/helpers/DefaultHandler
- method <init> ()V
- method startDocument ()V
- method endDocument ()V
- method startElement (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Lorg/xml/sax/Attributes;)V
- method endElement (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method characters ([CII)V
- method ignorableWhitespace ([CII)V
- method processingInstruction (Ljava/lang/String;Ljava/lang/String;)V
- method skippedEntity (Ljava/lang/String;)V
- method error (Lorg/xml/sax/SAXParseException;)V
- method fatalError (Lorg/xml/sax/SAXParseException;)V
- method warning (Lorg/xml/sax/SAXParseException;)V

## org/xml/sax/InputSource
- field byteStream Ljava/io/InputStream;
- field characterStream Ljava/io/Reader;
- field systemId Ljava/lang/String;
- field encoding Ljava/lang/String;
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V
- method <init> (Ljava/io/Reader;)V
- method <init> (Ljava/lang/String;)V
- method setByteStream (Ljava/io/InputStream;)V
- method getByteStream ()Ljava/io/InputStream;
- method setCharacterStream (Ljava/io/Reader;)V
- method getCharacterStream ()Ljava/io/Reader;
- method setSystemId (Ljava/lang/String;)V
- method getSystemId ()Ljava/lang/String;
- method setEncoding (Ljava/lang/String;)V
- method getEncoding ()Ljava/lang/String;

## org/xml/sax/SAXException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## org/xml/sax/SAXParseException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## ConstsDefines
- method <init> ()V

## ConstsDefines$HUD
- method <init> ()V

## ConstsDefines$TitanCinematicSettings
- method <init> ()V

## MIDPCanvas
- method <init> ()V
- method pointerPressed (II)V
- method pointerPressed (II)V

## com/motorola/extensions/RemoteControl
- method <init> ()V
- method gainControl ()V
- method gainControl ()V

## com/motorola/extensions/ScalableImage
- method <init> ()V
- method createImage ([BIIIII)Lcom/motorola/extensions/ScalableImage;
- method createImage ([BIIIII)Lcom/motorola/extensions/ScalableImage;
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getOrigHeight ()I
- method getOrigHeight ()I
- method getOrigWidth ()I
- method getOrigWidth ()I

## com/motorola/extensions/ScalableJPGImage
- method <init> ()V
- method createImage ([BIIII)Lcom/motorola/extensions/ScalableJPGImage;
- method createImage ([BIIII)Lcom/motorola/extensions/ScalableJPGImage;
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getImage ()Ljavax/microedition/lcdui/Image;
- method getOrigHeight ()I
- method getOrigHeight ()I
- method getOrigWidth ()I
- method getOrigWidth ()I

## com/motorola/funlight/Region
- method <init> ()V
- method getColor ()I
- method getColor ()I
- method getControl ()I
- method getControl ()I
- method getID ()I
- method getID ()I
- method releaseControl ()V
- method releaseControl ()V
- method setColor (BBB)I
- method setColor (BBB)I
- method setColor (I)I
- method setColor (I)I
- method setColor (I)V
- method setColor (I)V

## com/motorola/iden/lcdui/ExternalDisplay
- method <init> ()V
- method getDisplay (Ljavax/microedition/midlet/MIDlet;)Lcom/motorola/iden/lcdui/ExternalDisplay;
- method getDisplay (Ljavax/microedition/midlet/MIDlet;)Lcom/motorola/iden/lcdui/ExternalDisplay;
- method getFlipState ()Z
- method getFlipState ()Z
- method releaseDisplay ()V
- method releaseDisplay ()V
- method requestDisplay ()V
- method requestDisplay ()V
- method setCurrent (Ljavax/microedition/lcdui/Displayable;)V
- method setCurrent (Ljavax/microedition/lcdui/Displayable;)V

## com/motorola/iden/lcdui/ExternalDisplayCanvas
- method <init> ()V
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I

## com/motorola/iden/position/AggregatePosition
- method <init> ()V
- method getAltitude ()I
- method getAltitude ()I
- method getAltitudeUncertainty ()I
- method getAltitudeUncertainty ()I
- method getResponseCode ()I
- method getResponseCode ()I
- method getSpeed ()I
- method getSpeed ()I
- method getSpeedUncertainty ()I
- method getSpeedUncertainty ()I
- method getTravelDirection ()I
- method getTravelDirection ()I
- method hasAltitude ()Z
- method hasAltitude ()Z
- method hasAltitudeUncertainty ()Z
- method hasAltitudeUncertainty ()Z
- method hasSpeed ()Z
- method hasSpeed ()Z
- method hasSpeedUncertainty ()Z
- method hasSpeedUncertainty ()Z
- method hasTravelDirection ()Z
- method hasTravelDirection ()Z

## com/motorola/iden/position/Position2D
- method <init> ()V
- method getLatLonAccuracy ()I
- method getLatLonAccuracy ()I
- method getLatitude (I)Ljava/lang/String;
- method getLatitude (I)Ljava/lang/String;
- method getLongitude (I)Ljava/lang/String;
- method getLongitude (I)Ljava/lang/String;
- method getServingCellLatitude (I)Ljava/lang/String;
- method getServingCellLatitude (I)Ljava/lang/String;
- method getServingCellLongitude (I)Ljava/lang/String;
- method getServingCellLongitude (I)Ljava/lang/String;
- method getTimeStamp ()J
- method getTimeStamp ()J
- method hasLatLon ()Z
- method hasLatLon ()Z
- method hasLatLonAccuracy ()Z
- method hasLatLonAccuracy ()Z
- method hasServingCellLatLon ()Z
- method hasServingCellLatLon ()Z
- method hasTimeStamp ()Z
- method hasTimeStamp ()Z

## com/motorola/iden/position/PositionConnection
- method <init> ()V
- method getPosition ()Lcom/motorola/iden/position/AggregatePosition;
- method getPosition ()Lcom/motorola/iden/position/AggregatePosition;
- method getPosition (Ljava/lang/String;)Lcom/motorola/iden/position/AggregatePosition;
- method getPosition (Ljava/lang/String;)Lcom/motorola/iden/position/AggregatePosition;
- method getStatus ()I
- method getStatus ()I

## com/motorola/io/FileConnection
- method <init> ()V
- method availableSize ()J
- method availableSize ()J
- method canRead ()Z
- method canRead ()Z
- method canWrite ()Z
- method canWrite ()Z
- method close ()V
- method close ()V
- method create ()V
- method create ()V
- method create ()Z
- method create ()Z
- method delete ()V
- method delete ()V
- method delete ()Z
- method delete ()Z
- method directorySize (Z)J
- method directorySize (Z)J
- method exists ()Z
- method exists ()Z
- method fileSize ()J
- method fileSize ()J
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method isDirectory ()Z
- method isDirectory ()Z
- method isHidden ()Z
- method isHidden ()Z
- method lastModified ()J
- method lastModified ()J
- method list ()Ljava/util/Enumeration;
- method list ()Ljava/util/Enumeration;
- method list ()[Ljava/lang/String;
- method list ()[Ljava/lang/String;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method mkdir ()V
- method mkdir ()V
- method mkdir ()Z
- method mkdir ()Z
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method rename (Ljava/lang/String;)V
- method rename (Ljava/lang/String;)V
- method rename (Ljava/lang/String;)Z
- method rename (Ljava/lang/String;)Z
- method setHidden (Z)V
- method setHidden (Z)V
- method setReadable (Z)V
- method setReadable (Z)V
- method setWritable (Z)V
- method setWritable (Z)V
- method setWriteable (Z)V
- method setWriteable (Z)V
- method totalSize ()J
- method totalSize ()J
- method truncate (J)V
- method truncate (J)V
- method usedSize ()J
- method usedSize ()J

## com/motorola/io/FileSystemRegistry
- method <init> ()V
- method listRoots ()Ljava/util/Enumeration;
- method listRoots ()Ljava/util/Enumeration;
- method listRoots ()[Ljava/lang/String;
- method listRoots ()[Ljava/lang/String;

## com/motorola/io/drm/DrmFileConnection
- method <init> ()V
- method checkRights (I)Z
- method checkRights (I)Z
- method close ()V
- method close ()V
- method getContentType ()Ljava/lang/String;
- method getContentType ()Ljava/lang/String;
- method openInputStream ()Ljava/io/InputStream;
- method openInputStream ()Ljava/io/InputStream;

## com/motorola/io/file/FileConnection
- method <init> ()V
- method availableSize ()J
- method availableSize ()J
- method canRead ()Z
- method canRead ()Z
- method canWrite ()Z
- method canWrite ()Z
- method close ()V
- method close ()V
- method create ()V
- method create ()V
- method create ()Z
- method create ()Z
- method delete ()V
- method delete ()V
- method delete ()Z
- method delete ()Z
- method directorySize (Z)J
- method directorySize (Z)J
- method exists ()Z
- method exists ()Z
- method fileSize ()J
- method fileSize ()J
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method isDirectory ()Z
- method isDirectory ()Z
- method isHidden ()Z
- method isHidden ()Z
- method isOpen ()Z
- method isOpen ()Z
- method lastModified ()J
- method lastModified ()J
- method list ()Ljava/util/Enumeration;
- method list ()Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method list (Ljava/lang/String;Z)Ljava/util/Enumeration;
- method mkdir ()V
- method mkdir ()V
- method mkdir ()Z
- method mkdir ()Z
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataInputStream ()Ljava/io/DataInputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openDataOutputStream ()Ljava/io/DataOutputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openInputStream ()Ljava/io/InputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream ()Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method openOutputStream (J)Ljava/io/OutputStream;
- method rename (Ljava/lang/String;)V
- method rename (Ljava/lang/String;)V
- method setFileConnection (Ljava/lang/String;)V
- method setFileConnection (Ljava/lang/String;)V
- method setHidden (Z)V
- method setHidden (Z)V
- method setReadable (Z)V
- method setReadable (Z)V
- method setWritable (Z)V
- method setWritable (Z)V
- method setWriteable (Z)V
- method setWriteable (Z)V
- method totalSize ()J
- method totalSize ()J
- method truncate (J)V
- method truncate (J)V
- method usedSize ()J
- method usedSize ()J

## com/motorola/io/file/FileSystemRegistry
- method <init> ()V
- method listRoots ()Ljava/util/Enumeration;
- method listRoots ()Ljava/util/Enumeration;

## com/motorola/itunes/DodStatus
- method <init> ()V
- method dodCompleted (I)Z
- method dodCompleted (I)Z

## com/motorola/itunes/StatusArea
- method <init> ()V
- method setStatus (I)Z
- method setStatus (I)Z

## com/motorola/itunes/Utils
- method <init> ()V
- method checkArraySignature ([BIILjava/lang/String;Ljava/lang/String;)Z
- method checkArraySignature ([BIILjava/lang/String;Ljava/lang/String;)Z

## com/motorola/location/AggregatePosition
- method <init> ()V
- method getAltitude ()I
- method getAltitude ()I
- method getSpeed ()I
- method getSpeed ()I
- method getTimeStamp ()J
- method getTimeStamp ()J
- method getTravelDirection ()I
- method getTravelDirection ()I
- method hasAltitude ()Z
- method hasAltitude ()Z
- method hasSpeed ()Z
- method hasSpeed ()Z
- method hasTravelDirection ()Z
- method hasTravelDirection ()Z

## com/motorola/location/Position2D
- method <init> ()V
- method getLatLonAccuracy ()I
- method getLatLonAccuracy ()I
- method getLatitude ()I
- method getLatitude ()I
- method getLongitude ()I
- method getLongitude ()I
- method hasLatLon ()Z
- method hasLatLon ()Z

## com/motorola/location/PositionListener
- method <init> ()V

## com/motorola/location/PositionSource
- method <init> ()V
- method addPositionListener (Lcom/motorola/location/PositionListener;)V
- method addPositionListener (Lcom/motorola/location/PositionListener;)V
- method close ()V
- method close ()V
- method generatePosition (III)V
- method generatePosition (III)V
- method removePositionListener (Lcom/motorola/location/PositionListener;)V
- method removePositionListener (Lcom/motorola/location/PositionListener;)V

## com/motorola/multimedia/Lighting
- method <init> ()V
- method backlightOff ()V
- method backlightOff ()V
- method backlightOn ()V
- method backlightOn ()V

## com/motorola/phonebook/PhoneBookRecord
- field ALL_MEMORY I
- field ALL_MEMORY I
- field SORT_BY_NAME I
- field SORT_BY_NAME I
- field name Ljava/lang/String;
- field name Ljava/lang/String;
- field telNo Ljava/lang/String;
- field telNo Ljava/lang/String;
- field type I
- field type I
- method <init> ()V
- method getAvailableRecords (I)I
- method getAvailableRecords (I)I
- method getNumberRecords (I)I
- method getNumberRecords (I)I
- method getRecord (II)V
- method getRecord (II)V
- method getUsedRecords (II)I
- method getUsedRecords (II)I

## com/motorola/smsaccess/SMSFolder
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method append ([B)V
- method append ([B)V
- method getList ()[I
- method getList ()[I
- method getMessage (I)[B
- method getMessage (I)[B

## com/motorola/smsaccess/SMSMessage
- method <init> ()V
- method <init> ([B)V
- method getFolder ()Ljava/lang/String;
- method getFolder ()Ljava/lang/String;
- method getTimestamp ()Ljava/util/Date;
- method getTimestamp ()Ljava/util/Date;
- method getType ()B
- method getType ()B
- method isUnread ()Z
- method isUnread ()Z

## com/motorola/synerj/apps/browser/Browser
- method <init> ()V
- method getBrowser ()Lcom/motorola/synerj/apps/browser/Browser;
- method getBrowser ()Lcom/motorola/synerj/apps/browser/Browser;
- method launch (Ljava/lang/String;)V
- method launch (Ljava/lang/String;)V

## com/motorola/synerj/apps/dbk/DateBook
- method <init> ()V
- method getDateBook ()Lcom/motorola/synerj/apps/dbk/DateBook;
- method getDateBook ()Lcom/motorola/synerj/apps/dbk/DateBook;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/dial/DialEditor
- method <init> ()V
- method getDialEditor ()Lcom/motorola/synerj/apps/dial/DialEditor;
- method getDialEditor ()Lcom/motorola/synerj/apps/dial/DialEditor;
- method launch (I)Z
- method launch (I)Z
- method launchSpeedNo (I)Z
- method launchSpeedNo (I)Z

## com/motorola/synerj/apps/messages/Messages
- method <init> ()V
- method getMessages ()Lcom/motorola/synerj/apps/messages/Messages;
- method getMessages ()Lcom/motorola/synerj/apps/messages/Messages;
- method launch ()V
- method launch ()V
- method launchEditor ()V
- method launchEditor ()V
- method launchInbox ()V
- method launchInbox ()V

## com/motorola/synerj/apps/mmedia/Camera
- method <init> ()V
- method getCameraApplication ()Lcom/motorola/synerj/apps/mmedia/Camera;
- method getCameraApplication ()Lcom/motorola/synerj/apps/mmedia/Camera;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/mmedia/MediaPlayer
- method <init> ()V
- method getMediaPlayerApplication ()Lcom/motorola/synerj/apps/mmedia/MediaPlayer;
- method getMediaPlayerApplication ()Lcom/motorola/synerj/apps/mmedia/MediaPlayer;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/mmedia/Pictures
- method <init> ()V
- method getPicturesApplication ()Lcom/motorola/synerj/apps/mmedia/Pictures;
- method getPicturesApplication ()Lcom/motorola/synerj/apps/mmedia/Pictures;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/mmenu/MainMenu
- method <init> ()V
- method getMainMenu ()Lcom/motorola/synerj/apps/mmenu/MainMenu;
- method getMainMenu ()Lcom/motorola/synerj/apps/mmenu/MainMenu;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/pbk/PhoneBook
- method <init> ()V
- method getPhonebook ()Lcom/motorola/synerj/apps/pbk/PhoneBook;
- method getPhonebook ()Lcom/motorola/synerj/apps/pbk/PhoneBook;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/rc/RecentCalls
- method <init> ()V
- method getRecentCalls ()Lcom/motorola/synerj/apps/rc/RecentCalls;
- method getRecentCalls ()Lcom/motorola/synerj/apps/rc/RecentCalls;
- method launch (I)V
- method launch (I)V

## com/motorola/synerj/apps/voicenote/VoiceNote
- method <init> ()V
- method getVoiceNote ()Lcom/motorola/synerj/apps/voicenote/VoiceNote;
- method getVoiceNote ()Lcom/motorola/synerj/apps/voicenote/VoiceNote;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/voicerec/VoiceRecognition
- method <init> ()V
- method getVoiceRecognition ()Lcom/motorola/synerj/apps/voicerec/VoiceRecognition;
- method getVoiceRecognition ()Lcom/motorola/synerj/apps/voicerec/VoiceRecognition;
- method launch ()V
- method launch ()V

## com/motorola/synerj/apps/volume/Volume
- method <init> ()V
- method getVolume ()Lcom/motorola/synerj/apps/volume/Volume;
- method getVolume ()Lcom/motorola/synerj/apps/volume/Volume;
- method launch (I)V
- method launch (I)V

## com/motorola/synerj/fw/EventHandler
- method <init> ()V

## com/motorola/synerj/fw/EventManager
- method <init> ()V
- method postEvent (Ljava/lang/Object;I)V
- method postEvent (Ljava/lang/Object;I)V
- method postEvent (Ljava/lang/Object;ILjava/lang/Object;)V
- method postEvent (Ljava/lang/Object;ILjava/lang/Object;)V
- method registerEventHandler (Ljava/lang/Object;Lcom/motorola/synerj/fw/EventHandler;)V
- method registerEventHandler (Ljava/lang/Object;Lcom/motorola/synerj/fw/EventHandler;)V
- method sendEvent (Ljava/lang/Object;I)V
- method sendEvent (Ljava/lang/Object;I)V
- method unregisterEventHandler (Lcom/motorola/synerj/fw/EventHandler;)V
- method unregisterEventHandler (Lcom/motorola/synerj/fw/EventHandler;)V
- method unregisterEventHandler (Ljava/lang/Object;Lcom/motorola/synerj/fw/EventHandler;)V
- method unregisterEventHandler (Ljava/lang/Object;Lcom/motorola/synerj/fw/EventHandler;)V

## com/motorola/synerj/fw/Framework
- field context Lcom/motorola/synerj/fw/FrameworkContext;
- field context Lcom/motorola/synerj/fw/FrameworkContext;
- method <init> ()V
- method addResumeListener (Lcom/motorola/synerj/fw/FrameworkResumeListener;)V
- method addResumeListener (Lcom/motorola/synerj/fw/FrameworkResumeListener;)V
- method addStopListener (Lcom/motorola/synerj/fw/FrameworkStopListener;)V
- method addStopListener (Lcom/motorola/synerj/fw/FrameworkStopListener;)V
- method addSuspendListener (Lcom/motorola/synerj/fw/FrameworkSuspendListener;)V
- method addSuspendListener (Lcom/motorola/synerj/fw/FrameworkSuspendListener;)V
- method removeStopListener (Lcom/motorola/synerj/fw/FrameworkStopListener;)V
- method removeStopListener (Lcom/motorola/synerj/fw/FrameworkStopListener;)V

## com/motorola/synerj/fw/FrameworkContext
- method <init> ()V
- method requestShutdown ()V
- method requestShutdown ()V

## com/motorola/synerj/fw/FrameworkMIDlet
- method <init> ()V

## com/motorola/synerj/fw/FrameworkResumeListener
- method <init> ()V

## com/motorola/synerj/fw/FrameworkStopListener
- method <init> ()V

## com/motorola/synerj/fw/FrameworkSuspendListener
- method <init> ()V

## com/motorola/synerj/svc/auf/FileSystem
- method <init> ()V
- method isSystem (Ljava/lang/String;)Z
- method isSystem (Ljava/lang/String;)Z
- method setSystem (Ljava/lang/String;Z)V
- method setSystem (Ljava/lang/String;Z)V

## com/motorola/synerj/svc/auf/Messages
- method <init> ()V
- method addListener (Lcom/motorola/synerj/svc/auf/MessagesListener;)V
- method addListener (Lcom/motorola/synerj/svc/auf/MessagesListener;)V
- method getInstance ()Lcom/motorola/synerj/svc/auf/Messages;
- method getInstance ()Lcom/motorola/synerj/svc/auf/Messages;
- method removeListener (Lcom/motorola/synerj/svc/auf/MessagesListener;)V
- method removeListener (Lcom/motorola/synerj/svc/auf/MessagesListener;)V

## com/motorola/synerj/svc/auf/MessagesListener
- method <init> ()V

## com/motorola/synerj/svc/auf/ResourceManager
- method <init> ()V
- method getImageFromID (I)Ljavax/microedition/lcdui/Image;
- method getImageFromID (I)Ljavax/microedition/lcdui/Image;
- method getResourceManager (Ljava/lang/String;)Lcom/motorola/synerj/svc/auf/ResourceManager;
- method getResourceManager (Ljava/lang/String;)Lcom/motorola/synerj/svc/auf/ResourceManager;
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;

## com/motorola/synerj/svc/device/Bluetooth
- method <init> ()V
- method getBtStatus ()I
- method getBtStatus ()I

## com/motorola/synerj/svc/device/Case
- method <init> ()V
- method getFlipState ()I
- method getFlipState ()I

## com/motorola/synerj/svc/device/Display
- method <init> ()V
- method sendMsgChangeModeEventToSynergy ()Z
- method sendMsgChangeModeEventToSynergy ()Z

## com/motorola/synerj/svc/device/Funlights
- method <init> ()V
- method switchRythmLights (Z)Z
- method switchRythmLights (Z)Z

## com/motorola/synerj/svc/device/Keyboard
- method <init> ()V
- method getLockState ()I
- method getLockState ()I

## com/motorola/synerj/svc/device/Network
- method <init> ()V
- method getInstance ()Lcom/motorola/synerj/svc/device/Network;
- method getInstance ()Lcom/motorola/synerj/svc/device/Network;
- method getNetworkName ()Ljava/lang/String;
- method getNetworkName ()Ljava/lang/String;
- method getServiceStatus ()I
- method getServiceStatus ()I

## com/motorola/synerj/svc/device/Power
- method <init> ()V
- method getBatteryState ()I
- method getBatteryState ()I

## com/motorola/synerj/svc/device/Transflash
- method <init> ()V
- method getTransflashStatus ()I
- method getTransflashStatus ()I

## com/motorola/synerj/svc/net/SessionManager
- method <init> ()V
- method closeSession ()V
- method closeSession ()V
- method getInstance ()Lcom/motorola/synerj/svc/net/SessionManager;
- method getInstance ()Lcom/motorola/synerj/svc/net/SessionManager;

## com/motorola/synerj/svc/user/AppManager
- method <init> ()V
- method getInstance ()Lcom/motorola/synerj/svc/user/AppManager;
- method getInstance ()Lcom/motorola/synerj/svc/user/AppManager;
- method getSuitesList ()[Lcom/motorola/synerj/svc/user/SuiteEntry;
- method getSuitesList ()[Lcom/motorola/synerj/svc/user/SuiteEntry;

## com/motorola/synerj/svc/user/HomeScreen
- method <init> ()V
- method addListener (Lcom/motorola/synerj/svc/user/HomeScreenListener;)V
- method addListener (Lcom/motorola/synerj/svc/user/HomeScreenListener;)V
- method getAppLargeIconResId (I)I
- method getAppLargeIconResId (I)I
- method getClockAppearance ()I
- method getClockAppearance ()I
- method getIconsMode ()I
- method getIconsMode ()I
- method getInstance ()Lcom/motorola/synerj/svc/user/HomeScreen;
- method getInstance ()Lcom/motorola/synerj/svc/user/HomeScreen;
- method getLayout ()I
- method getLayout ()I
- method getName (I)Ljava/lang/String;
- method getName (I)Ljava/lang/String;
- method launch (I)V
- method launch (I)V

## com/motorola/synerj/svc/user/HomeScreenListener
- method <init> ()V

## com/motorola/synerj/svc/user/Ring
- method <init> ()V
- method getMode ()I
- method getMode ()I

## com/motorola/synerj/svc/user/Settings
- method <init> ()V
- method addListener (Lcom/motorola/synerj/svc/user/SettingsListener;)V
- method addListener (Lcom/motorola/synerj/svc/user/SettingsListener;)V
- method getDateFormat ()I
- method getDateFormat ()I
- method getInstance ()Lcom/motorola/synerj/svc/user/Settings;
- method getInstance ()Lcom/motorola/synerj/svc/user/Settings;
- method getLanguage ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;
- method getSilentModeHashKeyAvaliable ()Z
- method getSilentModeHashKeyAvaliable ()Z
- method getTimeFormat ()I
- method getTimeFormat ()I
- method removeListener (Lcom/motorola/synerj/svc/user/SettingsListener;)V
- method removeListener (Lcom/motorola/synerj/svc/user/SettingsListener;)V

## com/motorola/synerj/svc/user/SettingsListener
- method <init> ()V

## com/motorola/synerj/svc/user/SuiteEntry
- method <init> ()V
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method launch ()Z
- method launch ()Z

## com/motorola/synerj/ui/Adjuster
- method <init> ()V
- method getDecorator (I)Lcom/motorola/synerj/ui/PrimaryViewDecorator;
- method getDecorator (I)Lcom/motorola/synerj/ui/PrimaryViewDecorator;

## com/motorola/synerj/ui/Colors
- method <init> ()V
- method get (I)I
- method get (I)I

## com/motorola/synerj/ui/Fonts
- method <init> ()V
- method get (I)Ljavax/microedition/lcdui/Font;
- method get (I)Ljavax/microedition/lcdui/Font;

## com/motorola/synerj/ui/GifAnimation
- method <init> ()V
- method createAnimation ([BII)Lcom/motorola/synerj/ui/GifAnimation;
- method createAnimation ([BII)Lcom/motorola/synerj/ui/GifAnimation;
- method getDuration ()I
- method getDuration ()I
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I
- method paint (Lcom/motorola/synerj/ui/UIGraphics;III)V
- method paint (Lcom/motorola/synerj/ui/UIGraphics;III)V
- method prepareNextFrame ()V
- method prepareNextFrame ()V
- method setInfinite (Z)V
- method setInfinite (Z)V

## com/motorola/synerj/ui/PrimaryDisplay
- method <init> ()V
- method getHeight ()I
- method getHeight ()I
- method getIdleDisplay ()Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getIdleDisplay ()Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getPrimaryDisplay ()Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getPrimaryDisplay ()Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getPrimaryDisplay (Z)Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getPrimaryDisplay (Z)Lcom/motorola/synerj/ui/PrimaryDisplay;
- method getStatusAreaHeight ()I
- method getStatusAreaHeight ()I
- method getWidth ()I
- method getWidth ()I
- method hasFocus ()Z
- method hasFocus ()Z
- method hasProxy ()Z
- method hasProxy ()Z
- method popView ()V
- method popView ()V
- method popView (Lcom/motorola/synerj/ui/PrimaryView;)V
- method popView (Lcom/motorola/synerj/ui/PrimaryView;)V
- method pushView (Lcom/motorola/synerj/ui/PrimaryViewBase;)V
- method pushView (Lcom/motorola/synerj/ui/PrimaryViewBase;)V
- method replaceView (Lcom/motorola/synerj/ui/PrimaryView;)V
- method replaceView (Lcom/motorola/synerj/ui/PrimaryView;)V
- method setListener (Lcom/motorola/synerj/ui/PrimaryDisplayListener;)V
- method setListener (Lcom/motorola/synerj/ui/PrimaryDisplayListener;)V
- method setStatusWallpaper (Ljavax/microedition/lcdui/Image;II)V
- method setStatusWallpaper (Ljavax/microedition/lcdui/Image;II)V
- method start ()V
- method start ()V
- method stop ()V
- method stop ()V

## com/motorola/synerj/ui/PrimaryDisplayListener
- method <init> ()V

## com/motorola/synerj/ui/PrimaryView
- method <init> ()V
- method <init> (Lcom/motorola/synerj/ui/PrimaryViewDecorator;)V
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I
- method repaint ()V
- method repaint ()V
- method repaint (IIII)V
- method repaint (IIII)V
- method setBox (IIIIIII)V
- method setBox (IIIIIII)V
- method setKeyboardListener (Lcom/motorola/synerj/ui/UIKeyboardListener;)V
- method setKeyboardListener (Lcom/motorola/synerj/ui/UIKeyboardListener;)V

## com/motorola/synerj/ui/PrimaryViewDecorator
- method <init> ()V
- method createScrollbar (Lcom/motorola/synerj/ui/PrimaryView;)Lcom/motorola/synerj/ui/widget/Scrollbar;
- method createScrollbar (Lcom/motorola/synerj/ui/PrimaryView;)Lcom/motorola/synerj/ui/widget/Scrollbar;
- method createSoftkeys (Lcom/motorola/synerj/ui/PrimaryView;)Lcom/motorola/synerj/ui/widget/Softkeys;
- method createSoftkeys (Lcom/motorola/synerj/ui/PrimaryView;)Lcom/motorola/synerj/ui/widget/Softkeys;
- method getMaxViewHeight ()I
- method getMaxViewHeight ()I
- method getMaxViewWidth ()I
- method getMaxViewWidth ()I
- method getMinViewHeight ()I
- method getMinViewHeight ()I
- method getMinViewWidth ()I
- method getMinViewWidth ()I
- method getSkinImages ()Lcom/motorola/synerj/ui/SkinImages;
- method getSkinImages ()Lcom/motorola/synerj/ui/SkinImages;
- method setBox (Lcom/motorola/synerj/ui/PrimaryView;II)V
- method setBox (Lcom/motorola/synerj/ui/PrimaryView;II)V

## com/motorola/synerj/ui/Skin
- method <init> ()V
- method getAreaProperty (I)Lcom/motorola/synerj/ui/widget/Area;
- method getAreaProperty (I)Lcom/motorola/synerj/ui/widget/Area;
- method getBooleanProperty (I)Z
- method getBooleanProperty (I)Z
- method getColorProperty (I)I
- method getColorProperty (I)I
- method getImageProperty (I)Ljavax/microedition/lcdui/Image;
- method getImageProperty (I)Ljavax/microedition/lcdui/Image;
- method getIntProperty (I)I
- method getIntProperty (I)I

## com/motorola/synerj/ui/UICommandListener
- method <init> ()V

## com/motorola/synerj/ui/UIGraphics
- method <init> ()V
- method clipRect (IIII)V
- method clipRect (IIII)V
- method drawChar (CIII)V
- method drawChar (CIII)V
- method drawChars ([CIIIII)V
- method drawChars ([CIIIII)V
- method drawImage (Ljavax/microedition/lcdui/Image;III)V
- method drawImage (Ljavax/microedition/lcdui/Image;III)V
- method drawLine (IIII)V
- method drawLine (IIII)V
- method drawRect (IIII)V
- method drawRect (IIII)V
- method drawString (Ljava/lang/String;III)V
- method drawString (Ljava/lang/String;III)V
- method drawStringOutline (Ljava/lang/String;II[I)V
- method drawStringOutline (Ljava/lang/String;II[I)V
- method fillBackground (II)V
- method fillBackground (II)V
- method fillRect (IIII)V
- method fillRect (IIII)V
- method fillTriangle (IIIIII)V
- method fillTriangle (IIIIII)V
- method getClipHeight ()I
- method getClipHeight ()I
- method getClipWidth ()I
- method getClipWidth ()I
- method getClipX ()I
- method getClipX ()I
- method getClipY ()I
- method getClipY ()I
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getImageGraphics (Ljavax/microedition/lcdui/Image;Lcom/motorola/synerj/ui/UIGraphics;)Lcom/motorola/synerj/ui/UIGraphics;
- method getImageGraphics (Ljavax/microedition/lcdui/Image;Lcom/motorola/synerj/ui/UIGraphics;)Lcom/motorola/synerj/ui/UIGraphics;
- method getTranslateX ()I
- method getTranslateX ()I
- method getTranslateY ()I
- method getTranslateY ()I
- method setClip (IIII)V
- method setClip (IIII)V
- method setColor (I)V
- method setColor (I)V
- method setColor (III)V
- method setColor (III)V
- method setDrawMode (Z)V
- method setDrawMode (Z)V
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method translate (II)V
- method translate (II)V

## com/motorola/synerj/ui/UIKeyboardListener
- method <init> ()V

## com/motorola/synerj/ui/dialog/CharEditorDialog
- method <init> ()V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method setCommandListener (Lcom/motorola/synerj/ui/UICommandListener;)V
- method setCommandListener (Lcom/motorola/synerj/ui/UICommandListener;)V
- method setConstraints (I)V
- method setConstraints (I)V
- method setIsPassword (Z)V
- method setIsPassword (Z)V
- method setMaxSize (I)V
- method setMaxSize (I)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V

## com/motorola/synerj/ui/settings/Settings
- method <init> ()V
- method addListener (Lcom/motorola/synerj/ui/settings/SettingsListener;)V
- method addListener (Lcom/motorola/synerj/ui/settings/SettingsListener;)V
- method getInstance ()Lcom/motorola/synerj/ui/settings/Settings;
- method getInstance ()Lcom/motorola/synerj/ui/settings/Settings;
- method setWallpaper (Ljava/lang/String;)Z
- method setWallpaper (Ljava/lang/String;)Z

## com/motorola/synerj/ui/settings/SettingsListener
- method <init> ()V

## com/motorola/synerj/ui/widget/Area
- field height I
- field height I
- field width I
- field width I
- field x I
- field x I
- field y I
- field y I
- method <init> ()V
- method <init> (IIII)V

## com/nokia/mid/imagescale/ImageScaler
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method addListener (Lcom/nokia/mid/imagescale/ImageScalerListener;)V
- method addListener (Lcom/nokia/mid/imagescale/ImageScalerListener;)V
- method removeDestFile ()V
- method removeDestFile ()V
- method removeListener (Lcom/nokia/mid/imagescale/ImageScalerListener;)V
- method removeListener (Lcom/nokia/mid/imagescale/ImageScalerListener;)V
- method scaleImage (I)I
- method scaleImage (I)I
- method scaleImage (IIZ)I
- method scaleImage (IIZ)I
- method setAutoOrientation (Z)V
- method setAutoOrientation (Z)V
- method setJpegQuality (I)V
- method setJpegQuality (I)V

## com/nokia/mid/imagescale/ImageScalerException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getReason ()I
- method getReason ()I

## com/nokia/mid/imagescale/ImageScalerListener
- method <init> ()V

## com/nokia/mid/impl/jms/core/Explorer
- method <init> ()V
- method getExplorer ()Lcom/nokia/mid/impl/jms/core/Explorer;
- method getExplorer ()Lcom/nokia/mid/impl/jms/core/Explorer;
- method listContents (Ljava/lang/String;)[Lcom/nokia/mid/impl/jms/file/File;
- method listContents (Ljava/lang/String;)[Lcom/nokia/mid/impl/jms/file/File;

## com/nokia/mid/impl/jms/core/Installer
- method <init> ()V
- method getInstaller ()Lcom/nokia/mid/impl/jms/core/Installer;
- method getInstaller ()Lcom/nokia/mid/impl/jms/core/Installer;
- method install (Ljava/lang/String;Lcom/nokia/mid/impl/jms/core/JADFile;Lcom/nokia/mid/impl/jms/core/JARFile;Z)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method install (Ljava/lang/String;Lcom/nokia/mid/impl/jms/core/JADFile;Lcom/nokia/mid/impl/jms/core/JARFile;Z)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method saveContent (Ljava/io/InputStream;Ljava/lang/String;Z)Z
- method saveContent (Ljava/io/InputStream;Ljava/lang/String;Z)Z
- method uninstall (I)Z
- method uninstall (I)Z

## com/nokia/mid/impl/jms/core/InstallerException
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method getErrorCode ()I
- method getErrorCode ()I

## com/nokia/mid/impl/jms/core/JADFile
- method <init> ()V
- method <init> ([BLjava/lang/String;)V
- method getErrorCode ()I
- method getErrorCode ()I
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method validate ()Z
- method validate ()Z

## com/nokia/mid/impl/jms/core/JARFile
- method <init> ()V
- method <init> (Ljava/io/InputStream;Ljava/lang/String;)V
- method <init> ([BLjava/lang/String;)V
- method createJAD ()Lcom/nokia/mid/impl/jms/core/JADFile;
- method createJAD ()Lcom/nokia/mid/impl/jms/core/JADFile;
- method setMimeType (Ljava/lang/String;)Z
- method setMimeType (Ljava/lang/String;)Z

## com/nokia/mid/impl/jms/core/Launcher
- method <init> ()V
- method getLauncher ()Lcom/nokia/mid/impl/jms/core/Launcher;
- method getLauncher ()Lcom/nokia/mid/impl/jms/core/Launcher;
- method handleContent (Ljava/lang/String;)V
- method handleContent (Ljava/lang/String;)V
- method launchMIDlet (IILjava/util/Hashtable;Ljava/lang/String;)V
- method launchMIDlet (IILjava/util/Hashtable;Ljava/lang/String;)V
- method launchMIDlet (ILjava/util/Hashtable;Ljava/lang/String;)V
- method launchMIDlet (ILjava/util/Hashtable;Ljava/lang/String;)V

## com/nokia/mid/impl/jms/core/LauncherException
- method <init> ()V

## com/nokia/mid/impl/jms/core/MIDletRegistry
- method <init> ()V
- method findMIDletSuite (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method findMIDletSuite (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method getJADProperty (ILjava/lang/String;)Ljava/lang/String;
- method getJADProperty (ILjava/lang/String;)Ljava/lang/String;
- method getMIDletLocation (I)Ljava/lang/String;
- method getMIDletLocation (I)Ljava/lang/String;
- method getMIDletRegistry ()Lcom/nokia/mid/impl/jms/core/MIDletRegistry;
- method getMIDletRegistry ()Lcom/nokia/mid/impl/jms/core/MIDletRegistry;
- method getMIDletType (I)I
- method getMIDletType (I)I

## com/nokia/mid/impl/jms/core/MIDletSuite
- method <init> ()V
- method delete ()Z
- method delete ()Z
- method exists ()Z
- method exists ()Z
- method getDefaultIcon ()Ljavax/microedition/lcdui/Image;
- method getDefaultIcon ()Ljavax/microedition/lcdui/Image;
- method getIcon ()[B
- method getIcon ()[B
- method getJADFilePath ()Ljava/lang/String;
- method getJADFilePath ()Ljava/lang/String;
- method getJADProperty (Ljava/lang/String;)Ljava/lang/String;
- method getJADProperty (Ljava/lang/String;)Ljava/lang/String;
- method getJARFilePath ()Ljava/lang/String;
- method getJARFilePath ()Ljava/lang/String;
- method getMIDletAttribute (I)[B
- method getMIDletAttribute (I)[B
- method getMIDletId ()I
- method getMIDletId ()I
- method getMIDletStatus ()I
- method getMIDletStatus ()I
- method getMIDletSuite (I)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method getMIDletSuite (I)Lcom/nokia/mid/impl/jms/core/MIDletSuite;
- method getResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method getResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method install (I)I
- method install (I)I
- method moveTo (Lcom/nokia/mid/impl/jms/file/File;)Z
- method moveTo (Lcom/nokia/mid/impl/jms/file/File;)Z

## com/nokia/mid/impl/jms/file/File
- method <init> ()V
- method delete ()Z
- method delete ()Z
- method exists ()Z
- method exists ()Z
- method getAttributes ()I
- method getAttributes ()I
- method getFile (Lcom/nokia/mid/impl/jms/file/File;Ljava/lang/String;)Lcom/nokia/mid/impl/jms/file/File;
- method getFile (Lcom/nokia/mid/impl/jms/file/File;Ljava/lang/String;)Lcom/nokia/mid/impl/jms/file/File;
- method getFile (Ljava/lang/String;)Lcom/nokia/mid/impl/jms/file/File;
- method getFile (Ljava/lang/String;)Lcom/nokia/mid/impl/jms/file/File;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getParent ()Ljava/lang/String;
- method getParent ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getPath ()Ljava/lang/String;
- method getSize ()J
- method getSize ()J
- method getSize (Z)J
- method getSize (Z)J
- method isDirectory ()Z
- method isDirectory ()Z
- method listContents (ZLjava/lang/String;)[Lcom/nokia/mid/impl/jms/file/File;
- method listContents (ZLjava/lang/String;)[Lcom/nokia/mid/impl/jms/file/File;
- method rename (Ljava/lang/String;)Z
- method rename (Ljava/lang/String;)Z
- method setAttributes (I)V
- method setAttributes (I)V

## com/nokia/mid/impl/jms/file/FileInputStream
- method <init> ()V
- method <init> (Lcom/nokia/mid/impl/jms/file/File;)V
- method <init> (Ljava/lang/String;)V
- method close ()V
- method close ()V
- method read ([BII)I
- method read ([BII)I

## com/nokia/mid/impl/jms/file/FileOutputStream
- method <init> ()V
- method <init> (Lcom/nokia/mid/impl/jms/file/File;ZZ)V
- method <init> (Ljava/lang/String;ZZ)V
- method close ()V
- method close ()V
- method seek (I)V
- method seek (I)V
- method truncate (I)V
- method truncate (I)V
- method write ([BII)V
- method write ([BII)V

## com/nokia/mid/impl/jms/file/FileSystem
- method <init> ()V
- method createFile (Ljava/lang/String;[BII)Z
- method createFile (Ljava/lang/String;[BII)Z
- method getFileSystem ()Lcom/nokia/mid/impl/jms/file/FileSystem;
- method getFileSystem ()Lcom/nokia/mid/impl/jms/file/FileSystem;
- method getFreeSpaceAvailable ()J
- method getFreeSpaceAvailable ()J
- method getSystemFilePath (I)Ljava/lang/String;
- method getSystemFilePath (I)Ljava/lang/String;
- method getTotalSpace ()J
- method getTotalSpace ()J
- method mkdir (Ljava/lang/String;)Z
- method mkdir (Ljava/lang/String;)Z
- method pathEndsWithSeparator (Ljava/lang/String;)Z
- method pathEndsWithSeparator (Ljava/lang/String;)Z

## com/nokia/mid/location/LocationUtil
- method <init> ()V
- method getLocationProvider ([ILjava/lang/String;)Ljavax/microedition/location/LocationProvider;
- method getLocationProvider ([ILjava/lang/String;)Ljavax/microedition/location/LocationProvider;

## com/nokia/mid/media/AudioOutput
- method <init> ()V
- method getActiveOutputMode ()I
- method getActiveOutputMode ()I

## com/nokia/mid/media/AudioOutputControl
- method <init> ()V
- method getCurrent ()Lcom/nokia/mid/media/AudioOutput;
- method getCurrent ()Lcom/nokia/mid/media/AudioOutput;
- method setOutputMode (I)I
- method setOutputMode (I)I

## com/nokia/mid/network/NetworkState
- method <init> ()V
- method getState (I)I
- method getState (I)I
- method subscribeListener (Lcom/nokia/mid/network/NetworkStateListener;)V
- method subscribeListener (Lcom/nokia/mid/network/NetworkStateListener;)V

## com/nokia/mid/network/NetworkStateListener
- method <init> ()V

## com/nokia/mid/network/SIMStateListener
- method <init> ()V

## com/nokia/mid/network/WLANState
- method <init> ()V
- method getState ()I
- method getState ()I
- method subscribeListener (Lcom/nokia/mid/network/WLANStateListener;)V
- method subscribeListener (Lcom/nokia/mid/network/WLANStateListener;)V

## com/nokia/mid/network/WLANStateListener
- method <init> ()V

## com/nokia/mid/payment/IAPClientPaymentException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## com/nokia/mid/payment/IAPClientPaymentListener
- method <init> ()V

## com/nokia/mid/payment/IAPClientPaymentManager
- method <init> ()V
- method getDRMResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method getDRMResourceAsStream (Ljava/lang/String;)Ljava/io/InputStream;
- method getIAPClientPaymentManager ()Lcom/nokia/mid/payment/IAPClientPaymentManager;
- method getIAPClientPaymentManager ()Lcom/nokia/mid/payment/IAPClientPaymentManager;
- method getProductData (Ljava/lang/String;)I
- method getProductData (Ljava/lang/String;)I
- method getProductData ([Ljava/lang/String;)I
- method getProductData ([Ljava/lang/String;)I
- method getRestorableProducts (I)I
- method getRestorableProducts (I)I
- method getUserAndDeviceId (I)I
- method getUserAndDeviceId (I)I
- method purchaseProduct (Ljava/lang/String;I)I
- method purchaseProduct (Ljava/lang/String;I)I
- method restoreProduct (Ljava/lang/String;I)I
- method restoreProduct (Ljava/lang/String;I)I
- method setIAPClientPaymentListener (Lcom/nokia/mid/payment/IAPClientPaymentListener;)V
- method setIAPClientPaymentListener (Lcom/nokia/mid/payment/IAPClientPaymentListener;)V

## com/nokia/mid/payment/IAPClientProductData
- method <init> ()V
- method getPrice ()Ljava/lang/String;
- method getPrice ()Ljava/lang/String;
- method getProductId ()Ljava/lang/String;
- method getProductId ()Ljava/lang/String;
- method getShortDescription ()Ljava/lang/String;
- method getShortDescription ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;

## com/nokia/mid/payment/IAPClientUserAndDeviceData
- method <init> ()V
- method getAccount ()Ljava/lang/String;
- method getAccount ()Ljava/lang/String;
- method getCountry ()Ljava/lang/String;
- method getCountry ()Ljava/lang/String;
- method getDeviceModel ()Ljava/lang/String;
- method getDeviceModel ()Ljava/lang/String;
- method getImei ()Ljava/lang/String;
- method getImei ()Ljava/lang/String;
- method getImsi ()Ljava/lang/String;
- method getImsi ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;

## com/nokia/mid/pim/ContactChange
- field changeType I
- field changeType I
- field contact Ljavax/microedition/pim/Contact;
- field contact Ljavax/microedition/pim/Contact;
- field contactUID Ljava/lang/String;
- field contactUID Ljava/lang/String;
- method <init> ()V

## com/nokia/mid/pim/ContactChangeListener
- method <init> ()V

## com/nokia/mid/pim/ContactChangeManager
- method <init> ()V
- method addChangeListener (Lcom/nokia/mid/pim/ContactChangeListener;)V
- method addChangeListener (Lcom/nokia/mid/pim/ContactChangeListener;)V
- method removeChangeListener (Lcom/nokia/mid/pim/ContactChangeListener;)V
- method removeChangeListener (Lcom/nokia/mid/pim/ContactChangeListener;)V

## com/nokia/mid/pim/PIMUtils
- method <init> ()V
- method compareContactName (Ljava/lang/String;Ljava/lang/String;)I
- method compareContactName (Ljava/lang/String;Ljava/lang/String;)I

## com/nokia/mid/s40/MVMMonitor
- method <init> ()V
- method number_of_running_apps ()I
- method number_of_running_apps ()I

## com/nokia/mid/s40/bg/BGUtils
- method <init> ()V
- method getArgs ()Ljava/lang/String;
- method getArgs ()Ljava/lang/String;
- method getRunningMIDlets (Ljava/lang/String;Ljava/lang/String;)[I
- method getRunningMIDlets (Ljava/lang/String;Ljava/lang/String;)[I
- method launchIEMIDlet (Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)Z
- method launchIEMIDlet (Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)Z
- method setBGMIDletResident (Z)V
- method setBGMIDletResident (Z)V
- method setIESuiteLifecycleListener (Ljava/lang/String;Ljava/lang/String;Lcom/nokia/mid/s40/bg/IESuiteLifecycleListener;)V
- method setIESuiteLifecycleListener (Ljava/lang/String;Ljava/lang/String;Lcom/nokia/mid/s40/bg/IESuiteLifecycleListener;)V
- method setMIDletProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method setMIDletProperty (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;

## com/nokia/mid/s40/bg/IESuiteLifecycleListener
- method <init> ()V

## com/nokia/mid/s40/codec/DataDecoder
- method <init> ()V
- method <init> (Ljava/lang/String;[BII)V
- method getBoolean ()Z
- method getBoolean ()Z
- method getByteArray ()[B
- method getByteArray ()[B
- method getEnd (I)V
- method getEnd (I)V
- method getFloat (I)D
- method getFloat (I)D
- method getInteger (I)J
- method getInteger (I)J
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getStart (I)V
- method getStart (I)V
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;
- method getType ()I
- method getType ()I
- method listHasMoreItems ()Z
- method listHasMoreItems ()Z

## com/nokia/mid/s40/codec/DataEncoder
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getData ()[B
- method getData ()[B
- method put (ILjava/lang/String;J)V
- method put (ILjava/lang/String;J)V
- method put (ILjava/lang/String;Ljava/lang/String;)V
- method put (ILjava/lang/String;Ljava/lang/String;)V
- method put (ILjava/lang/String;Z)V
- method put (ILjava/lang/String;Z)V
- method put (Ljava/lang/String;[BI)V
- method put (Ljava/lang/String;[BI)V
- method putEnd (ILjava/lang/String;)V
- method putEnd (ILjava/lang/String;)V
- method putStart (ILjava/lang/String;)V
- method putStart (ILjava/lang/String;)V

## com/nokia/mid/s40/codec/DataType
- method <init> ()V
- method getTypeGroup (I)I
- method getTypeGroup (I)I

## com/nokia/mid/s40/io/LocalMessageProtocolConnection
- method <init> ()V
- method close ()V
- method close ()V
- method newMessage ([B)Lcom/nokia/mid/s40/io/LocalMessageProtocolMessage;
- method newMessage ([B)Lcom/nokia/mid/s40/io/LocalMessageProtocolMessage;
- method receive (Lcom/nokia/mid/s40/io/LocalMessageProtocolMessage;)V
- method receive (Lcom/nokia/mid/s40/io/LocalMessageProtocolMessage;)V
- method receive ([B)I
- method receive ([B)I
- method send ([BII)V
- method send ([BII)V

## com/nokia/mid/s40/io/LocalMessageProtocolMessage
- method <init> ()V
- method getData ()[B
- method getData ()[B
- method getLength ()I
- method getLength ()I
- method setData ([B)V
- method setData ([B)V

## com/nokia/mid/s40/io/LocalMessageProtocolServerConnection
- method <init> ()V
- method acceptAndOpen ()Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;
- method acceptAndOpen ()Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;

## com/nokia/mid/s40/io/LocalProtocolConnection
- method <init> ()V
- method getLocalName ()Ljava/lang/String;
- method getLocalName ()Ljava/lang/String;

## com/nokia/mid/s40/io/LocalProtocolServerConnection
- method <init> ()V

## com/nokia/mid/s40/io/LocalStreamProtocolConnection
- method <init> ()V

## com/nokia/mid/s40/io/LocalStreamProtocolServerConnection
- method <init> ()V
- method acceptAndOpen ()Lcom/nokia/mid/s40/io/LocalStreamProtocolConnection;
- method acceptAndOpen ()Lcom/nokia/mid/s40/io/LocalStreamProtocolConnection;

## com/nokia/mid/s40/io/ServiceArchitectureServerConnection
- method <init> ()V
- method registerWithServiceRegistry (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[B)V
- method registerWithServiceRegistry (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;[B)V

## com/nokia/mid/s40/io/SharedLocalProtocolMemory
- method <init> ()V
- method attach (Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;I)V
- method attach (Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;I)V
- method close ()V
- method close ()V
- method create (I)Lcom/nokia/mid/s40/io/SharedLocalProtocolMemory;
- method create (I)Lcom/nokia/mid/s40/io/SharedLocalProtocolMemory;
- method getSize ()I
- method getSize ()I
- method getURI ()Ljava/lang/String;
- method getURI ()Ljava/lang/String;
- method open (Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;Ljava/lang/String;I)Lcom/nokia/mid/s40/io/SharedLocalProtocolMemory;
- method open (Lcom/nokia/mid/s40/io/LocalMessageProtocolConnection;Ljava/lang/String;I)Lcom/nokia/mid/s40/io/SharedLocalProtocolMemory;
- method read (II)[B
- method read (II)[B
- method write ([BII)V
- method write ([BII)V

## com/nokia/mid/s40/lcdui/SharedLocalProtocolImage
- method <init> ()V
- method create (Ljavax/microedition/io/Connection;II)Lcom/nokia/mid/s40/lcdui/SharedLocalProtocolImage;
- method create (Ljavax/microedition/io/Connection;II)Lcom/nokia/mid/s40/lcdui/SharedLocalProtocolImage;
- method dispose ()V
- method dispose ()V
- method drawImage (Ljavax/microedition/lcdui/Graphics;III)V
- method drawImage (Ljavax/microedition/lcdui/Graphics;III)V
- method getURI ()Ljava/lang/String;
- method getURI ()Ljava/lang/String;

## com/nokia/mid/s40/service_api_utils/ProtocolVersion
- method <init> ()V
- method getMajorVersion ()J
- method getMajorVersion ()J
- method getMinorVersion ()J
- method getMinorVersion ()J

## com/nokia/mid/s40/service_api_utils/ServiceAPI
- method <init> ()V
- method matchVersion (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/s40/service_api_utils/ProtocolVersion;
- method matchVersion (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/s40/service_api_utils/ProtocolVersion;

## com/nokia/mid/s40/sim/MultiSIMUtils
- method <init> ()V
- method getActiveSIMSlot (I)I
- method getActiveSIMSlot (I)I
- method setActiveSIMSlot (IIZ)I
- method setActiveSIMSlot (IIZ)I

## com/nokia/mid/setting/Setting
- method <init> ()V
- method getSetting (I)I
- method getSetting (I)I
- method subscribeListener (Lcom/nokia/mid/setting/SettingListener;)V
- method subscribeListener (Lcom/nokia/mid/setting/SettingListener;)V
- method unSubscribeListener (Lcom/nokia/mid/setting/SettingListener;)V
- method unSubscribeListener (Lcom/nokia/mid/setting/SettingListener;)V

## com/nokia/mid/setting/SettingListener
- method <init> ()V

## com/nokia/mid/setting/ToneSetting
- method <init> ()V
- method getToneSetting (I)Ljava/lang/String;
- method getToneSetting (I)Ljava/lang/String;
- method getVolumeSetting (I)I
- method getVolumeSetting (I)I
- method subscribeListener (Lcom/nokia/mid/setting/ToneSettingListener;)V
- method subscribeListener (Lcom/nokia/mid/setting/ToneSettingListener;)V

## com/nokia/mid/setting/ToneSettingListener
- method <init> ()V

## com/nokia/mid/sms/SMSRegistrationListener
- method <init> ()V

## com/nokia/mid/sms/SMSRegistrationManager
- method <init> ()V
- method closeConnection (Ljavax/microedition/midlet/MIDlet;)V
- method closeConnection (Ljavax/microedition/midlet/MIDlet;)V
- method connect (Ljavax/microedition/midlet/MIDlet;Lcom/nokia/mid/sms/SMSRegistrationListener;)V
- method connect (Ljavax/microedition/midlet/MIDlet;Lcom/nokia/mid/sms/SMSRegistrationListener;)V

## com/nokia/mid/sx/SxFunction
- method <init> ()V

## com/nokia/mid/sx/SxObject
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method send (Ljava/lang/String;)Lcom/nokia/mid/sx/SxValue;
- method send (Ljava/lang/String;)Lcom/nokia/mid/sx/SxValue;

## com/nokia/mid/sx/SxString
- method <init> ()V
- method getString ()Ljava/lang/String;
- method getString ()Ljava/lang/String;

## com/nokia/mid/sx/ui/SxConfig
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getBoolean ()Z
- method getBoolean ()Z
- method getInteger ()I
- method getInteger ()I
- method subscribe (Lcom/nokia/mid/sx/SxFunction;)Lcom/nokia/mid/sx/SxValue;
- method subscribe (Lcom/nokia/mid/sx/SxFunction;)Lcom/nokia/mid/sx/SxValue;

## com/nokia/mid/sx/ui/SxMisc
- method <init> ()V
- method setSwipeAwayAllowed (Z)V
- method setSwipeAwayAllowed (Z)V

## com/nokia/mid/ui/CanvasGraphicsItem
- method <init> ()V
- method <init> (II)V
- method repaint ()V
- method repaint ()V
- method setParent (Ljava/lang/Object;)V
- method setParent (Ljava/lang/Object;)V
- method setPosition (II)V
- method setPosition (II)V
- method setSize (II)V
- method setSize (II)V
- method setVisible (Z)V
- method setVisible (Z)V
- method setZPosition (I)V
- method setZPosition (I)V

## com/nokia/mid/ui/CanvasItem
- method <init> ()V
- method getHeight ()I
- method getHeight ()I
- method getParent ()Ljava/lang/Object;
- method getParent ()Ljava/lang/Object;
- method getPositionX ()I
- method getPositionX ()I
- method getPositionY ()I
- method getPositionY ()I
- method getWidth ()I
- method getWidth ()I
- method getZPosition ()I
- method getZPosition ()I
- method isVisible ()Z
- method isVisible ()Z

## com/nokia/mid/ui/CategoryBar
- method <init> ()V
- method <init> ([Lcom/nokia/mid/ui/IconCommand;Z)V
- method <init> ([Ljavax/microedition/lcdui/Image;[Ljavax/microedition/lcdui/Image;[Ljava/lang/String;)V
- method getBestImageHeight (I)I
- method getBestImageHeight (I)I
- method getBestImageWidth (I)I
- method getBestImageWidth (I)I
- method getDefaultBoundingBox ()[I
- method getDefaultBoundingBox ()[I
- method getMaxElements ()I
- method getMaxElements ()I
- method getMode ()I
- method getMode ()I
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getVisibility ()Z
- method getVisibility ()Z
- method setBackgroundColour (I)V
- method setBackgroundColour (I)V
- method setBackgroundImage (Ljavax/microedition/lcdui/Image;)V
- method setBackgroundImage (Ljavax/microedition/lcdui/Image;)V
- method setElementListener (Lcom/nokia/mid/ui/ElementListener;)V
- method setElementListener (Lcom/nokia/mid/ui/ElementListener;)V
- method setElementProperties (ILcom/nokia/mid/ui/IconCommand;Z)V
- method setElementProperties (ILcom/nokia/mid/ui/IconCommand;Z)V
- method setElementProperties (ILjavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/Image;Ljava/lang/String;)V
- method setElementProperties (ILjavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/Image;Ljava/lang/String;)V
- method setHighlightColour (I)V
- method setHighlightColour (I)V
- method setMode (I)V
- method setMode (I)V
- method setSelectedIndex (I)V
- method setSelectedIndex (I)V
- method setTransitionSupport (Z)V
- method setTransitionSupport (Z)V
- method setVisibility (Z)V
- method setVisibility (Z)V
- method suppressSizeChanged (Z)V
- method suppressSizeChanged (Z)V

## com/nokia/mid/ui/Clipboard
- method <init> ()V
- method copyToClipboard (Ljava/lang/String;)V
- method copyToClipboard (Ljava/lang/String;)V

## com/nokia/mid/ui/CustomKeyboardControl
- method <init> ()V
- method dismiss ()V
- method dismiss ()V
- method launch (II)V
- method launch (II)V

## com/nokia/mid/ui/ElementListener
- method <init> ()V

## com/nokia/mid/ui/FileSelect
- method <init> ()V
- method launch (Ljava/lang/String;IZ)[Lcom/nokia/mid/ui/FileSelectDetail;
- method launch (Ljava/lang/String;IZ)[Lcom/nokia/mid/ui/FileSelectDetail;

## com/nokia/mid/ui/FileSelectDetail
- field displayName Ljava/lang/String;
- field displayName Ljava/lang/String;
- field mimeType Ljava/lang/String;
- field mimeType Ljava/lang/String;
- field size J
- field size J
- field url Ljava/lang/String;
- field url Ljava/lang/String;
- method <init> ()V

## com/nokia/mid/ui/IconCommand
- method <init> ()V
- method <init> (Ljava/lang/String;III)V
- method <init> (Ljava/lang/String;Ljavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/Image;II)V

## com/nokia/mid/ui/IdleEventListener
- method <init> ()V

## com/nokia/mid/ui/IdleItem
- method <init> ()V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method getActiveIdleItemState ()I
- method getActiveIdleItemState ()I
- method getIdleItemEventListener ()Lcom/nokia/mid/ui/IdleEventListener;
- method getIdleItemEventListener ()Lcom/nokia/mid/ui/IdleEventListener;
- method getInteractionModes ()I
- method getInteractionModes ()I
- method getPreferredHeight ()I
- method getPreferredHeight ()I
- method getPreferredWidth ()I
- method getPreferredWidth ()I
- method pointerPressed (II)V
- method pointerPressed (II)V
- method pointerReleased (II)V
- method pointerReleased (II)V
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method repaint ()V
- method repaint ()V
- method setCurrentIdleItem (Lcom/nokia/mid/ui/IdleItem;)Z
- method setCurrentIdleItem (Lcom/nokia/mid/ui/IdleItem;)Z
- method setIdleItemCommandListener (Lcom/nokia/mid/ui/IdleItemCommandListener;)V
- method setIdleItemCommandListener (Lcom/nokia/mid/ui/IdleItemCommandListener;)V
- method setIdleItemEventListener (Lcom/nokia/mid/ui/IdleEventListener;)V
- method setIdleItemEventListener (Lcom/nokia/mid/ui/IdleEventListener;)V

## com/nokia/mid/ui/IdleItemCommandListener
- method <init> ()V

## com/nokia/mid/ui/IdleStyle
- method <init> ()V
- method getIdleScreenColour (I)I
- method getIdleScreenColour (I)I
- method getIdleScreenFont (I)Ljavax/microedition/lcdui/Font;
- method getIdleScreenFont (I)Ljavax/microedition/lcdui/Font;

## com/nokia/mid/ui/KeyboardVisibilityListener
- method <init> ()V

## com/nokia/mid/ui/LCDUIUtil
- method <init> ()V
- method getObjectTrait (Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
- method getObjectTrait (Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
- method setObjectTrait (Ljava/lang/Object;Ljava/lang/String;Ljava/lang/Object;)Z
- method setObjectTrait (Ljava/lang/Object;Ljava/lang/String;Ljava/lang/Object;)Z
- method setStatusZoneVisibility (Ljava/lang/Object;Ljava/lang/Object;)V
- method setStatusZoneVisibility (Ljava/lang/Object;Ljava/lang/Object;)V

## com/nokia/mid/ui/PopupList
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;I)V
- method appendItem (Lcom/nokia/mid/ui/PopupListItem;)V
- method appendItem (Lcom/nokia/mid/ui/PopupListItem;)V
- method getItem (I)Lcom/nokia/mid/ui/PopupListItem;
- method getItem (I)Lcom/nokia/mid/ui/PopupListItem;
- method insertItem (Lcom/nokia/mid/ui/PopupListItem;I)V
- method insertItem (Lcom/nokia/mid/ui/PopupListItem;I)V
- method removeItemAt (I)V
- method removeItemAt (I)V
- method setListener (Lcom/nokia/mid/ui/PopupListListener;)V
- method setListener (Lcom/nokia/mid/ui/PopupListListener;)V
- method setVisible (Z)V
- method setVisible (Z)V
- method size ()I
- method size ()I

## com/nokia/mid/ui/PopupListItem
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;

## com/nokia/mid/ui/PopupListListener
- method <init> ()V

## com/nokia/mid/ui/S40TextEditor
- method <init> ()V
- method getIndicatorIcons ()[Ljavax/microedition/lcdui/Image;
- method getIndicatorIcons ()[Ljavax/microedition/lcdui/Image;
- method getTextEditorCommands ()[Ljavax/microedition/lcdui/Command;
- method getTextEditorCommands ()[Ljavax/microedition/lcdui/Command;
- method isCommandKeyWanted (Ljavax/microedition/lcdui/Command;)Z
- method isCommandKeyWanted (Ljavax/microedition/lcdui/Command;)Z
- method isMenuCommand (Ljavax/microedition/lcdui/Command;)Z
- method isMenuCommand (Ljavax/microedition/lcdui/Command;)Z
- method launchTextEditorCommand (Ljavax/microedition/lcdui/Command;I)Z
- method launchTextEditorCommand (Ljavax/microedition/lcdui/Command;I)Z
- method setCursorWrap (I)V
- method setCursorWrap (I)V

## com/nokia/mid/ui/S60TextEditor
- method <init> ()V
- method setTouchEnabled (Z)V
- method setTouchEnabled (Z)V

## com/nokia/mid/ui/SoftNotification
- method <init> ()V
- method getId ()I
- method getId ()I
- method newInstance ()Lcom/nokia/mid/ui/SoftNotification;
- method newInstance ()Lcom/nokia/mid/ui/SoftNotification;
- method post ()V
- method post ()V
- method remove ()V
- method remove ()V
- method setImage ([B)V
- method setImage ([B)V
- method setListener (Lcom/nokia/mid/ui/SoftNotificationListener;)V
- method setListener (Lcom/nokia/mid/ui/SoftNotificationListener;)V
- method setSoftkeyLabels (Ljava/lang/String;Ljava/lang/String;)V
- method setSoftkeyLabels (Ljava/lang/String;Ljava/lang/String;)V
- method setText (Ljava/lang/String;Ljava/lang/String;)V
- method setText (Ljava/lang/String;Ljava/lang/String;)V

## com/nokia/mid/ui/SoftNotificationListener
- method <init> ()V

## com/nokia/mid/ui/TactileFeedback
- method <init> ()V
- method directFeedback (I)V
- method directFeedback (I)V

## com/nokia/mid/ui/TextEditor
- method <init> ()V
- method createTextEditor (IIII)Lcom/nokia/mid/ui/TextEditor;
- method createTextEditor (IIII)Lcom/nokia/mid/ui/TextEditor;
- method createTextEditor (Ljava/lang/String;IIII)Lcom/nokia/mid/ui/TextEditor;
- method createTextEditor (Ljava/lang/String;IIII)Lcom/nokia/mid/ui/TextEditor;
- method delete (II)V
- method delete (II)V
- method getBackgroundColor ()I
- method getBackgroundColor ()I
- method getCaretPosition ()I
- method getCaretPosition ()I
- method getConstraints ()I
- method getConstraints ()I
- method getContent ()Ljava/lang/String;
- method getContent ()Ljava/lang/String;
- method getContentHeight ()I
- method getContentHeight ()I
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getFont ()Ljavax/microedition/lcdui/Font;
- method getForegroundColor ()I
- method getForegroundColor ()I
- method getHeight ()I
- method getHeight ()I
- method getInitialInputMode ()Ljava/lang/String;
- method getInitialInputMode ()Ljava/lang/String;
- method getLineMarginHeight ()I
- method getLineMarginHeight ()I
- method getMaxSize ()I
- method getMaxSize ()I
- method getParent ()Ljava/lang/Object;
- method getParent ()Ljava/lang/Object;
- method getPositionX ()I
- method getPositionX ()I
- method getPositionY ()I
- method getPositionY ()I
- method getSelection ()Ljava/lang/String;
- method getSelection ()Ljava/lang/String;
- method getVisibleContentPosition ()I
- method getVisibleContentPosition ()I
- method getWidth ()I
- method getWidth ()I
- method getZPosition ()I
- method getZPosition ()I
- method hasFocus ()Z
- method hasFocus ()Z
- method insert (Ljava/lang/String;I)V
- method insert (Ljava/lang/String;I)V
- method isMultiline ()Z
- method isMultiline ()Z
- method isVisible ()Z
- method isVisible ()Z
- method setBackgroundColor (I)V
- method setBackgroundColor (I)V
- method setCaret (I)V
- method setCaret (I)V
- method setConstraints (I)V
- method setConstraints (I)V
- method setContent (Ljava/lang/String;)V
- method setContent (Ljava/lang/String;)V
- method setDefaultIndicators ()V
- method setDefaultIndicators ()V
- method setFocus (Z)V
- method setFocus (Z)V
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method setFont (Ljavax/microedition/lcdui/Font;)V
- method setForegroundColor (I)V
- method setForegroundColor (I)V
- method setHighlightBackgroundColor (I)V
- method setHighlightBackgroundColor (I)V
- method setHighlightForegroundColor (I)V
- method setHighlightForegroundColor (I)V
- method setInitialInputMode (Ljava/lang/String;)V
- method setInitialInputMode (Ljava/lang/String;)V
- method setMaxSize (I)I
- method setMaxSize (I)I
- method setMultiline (Z)V
- method setMultiline (Z)V
- method setParent (Ljava/lang/Object;)V
- method setParent (Ljava/lang/Object;)V
- method setPosition (II)V
- method setPosition (II)V
- method setSelection (II)V
- method setSelection (II)V
- method setSize (II)V
- method setSize (II)V
- method setTextEditorListener (Lcom/nokia/mid/ui/TextEditorListener;)V
- method setTextEditorListener (Lcom/nokia/mid/ui/TextEditorListener;)V
- method setTouchEnabled (Z)V
- method setTouchEnabled (Z)V
- method setVisible (Z)V
- method setVisible (Z)V
- method setZPosition (I)V
- method setZPosition (I)V
- method size ()I
- method size ()I

## com/nokia/mid/ui/TextEditorExtensionAccess
- method <init> ()V
- method getTouchControl ()Lcom/nokia/mid/ui/TextEditorTouchControl;
- method getTouchControl ()Lcom/nokia/mid/ui/TextEditorTouchControl;

## com/nokia/mid/ui/TextEditorListener
- method <init> ()V
- method inputAction (Lcom/nokia/mid/ui/TextEditor;I)V
- method inputAction (Lcom/nokia/mid/ui/TextEditor;I)V

## com/nokia/mid/ui/TextEditorTouchControl
- method <init> ()V
- method getPanelHeight ()I
- method getPanelHeight ()I
- method setPanelMode (I)V
- method setPanelMode (I)V
- method setPanelPosition (II)V
- method setPanelPosition (II)V

## com/nokia/mid/ui/VirtualKeyboard
- method <init> ()V
- method getCustomKeyboardControl ()Lcom/nokia/mid/ui/CustomKeyboardControl;
- method getCustomKeyboardControl ()Lcom/nokia/mid/ui/CustomKeyboardControl;
- method getHeight ()I
- method getHeight ()I
- method getYPosition ()I
- method getYPosition ()I
- method hideOpenKeypadCommand (Z)V
- method hideOpenKeypadCommand (Z)V
- method isVisible ()Z
- method isVisible ()Z
- method setVisibilityListener (Lcom/nokia/mid/ui/KeyboardVisibilityListener;)V
- method setVisibilityListener (Lcom/nokia/mid/ui/KeyboardVisibilityListener;)V
- method suppressSizeChanged (Z)V
- method suppressSizeChanged (Z)V

## com/nokia/mid/ui/frameanimator/FrameAnimator
- method <init> ()V
- method drag (II)V
- method drag (II)V
- method isRegistered ()Z
- method isRegistered ()Z
- method kineticScroll (IIIF)V
- method kineticScroll (IIIF)V
- method register (IISSLcom/nokia/mid/ui/frameanimator/FrameAnimatorListener;)Z
- method register (IISSLcom/nokia/mid/ui/frameanimator/FrameAnimatorListener;)Z
- method stop ()V
- method stop ()V
- method unregister ()V
- method unregister ()V

## com/nokia/mid/ui/frameanimator/FrameAnimatorListener
- method <init> ()V

## com/nokia/mid/ui/gestures/GestureEvent
- method <init> ()V
- method getDragDistanceX ()I
- method getDragDistanceX ()I
- method getDragDistanceY ()I
- method getDragDistanceY ()I
- method getFlickDirection ()F
- method getFlickDirection ()F
- method getFlickSpeed ()I
- method getFlickSpeed ()I
- method getFlickSpeedX ()I
- method getFlickSpeedX ()I
- method getFlickSpeedY ()I
- method getFlickSpeedY ()I
- method getPinchCenterChangeX ()I
- method getPinchCenterChangeX ()I
- method getPinchCenterChangeY ()I
- method getPinchCenterChangeY ()I
- method getPinchCenterX ()I
- method getPinchCenterX ()I
- method getPinchCenterY ()I
- method getPinchCenterY ()I
- method getPinchDistanceChange ()I
- method getPinchDistanceChange ()I
- method getPinchDistanceCurrent ()I
- method getPinchDistanceCurrent ()I
- method getPinchDistanceStarting ()I
- method getPinchDistanceStarting ()I
- method getStartX ()I
- method getStartX ()I
- method getStartY ()I
- method getStartY ()I
- method getType ()I
- method getType ()I

## com/nokia/mid/ui/gestures/GestureInteractiveZone
- method <init> ()V
- method <init> (I)V
- method getGestures ()I
- method getGestures ()I
- method getHeight ()I
- method getHeight ()I
- method getLongPressTimeInterval ()I
- method getLongPressTimeInterval ()I
- method getWidth ()I
- method getWidth ()I
- method getX ()I
- method getX ()I
- method getY ()I
- method getY ()I
- method isSupported (I)Z
- method isSupported (I)Z
- method setRectangle (IIII)V
- method setRectangle (IIII)V

## com/nokia/mid/ui/gestures/GestureListener
- method <init> ()V

## com/nokia/mid/ui/gestures/GestureRegistrationManager
- method <init> ()V
- method register (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureInteractiveZone;)Z
- method register (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureInteractiveZone;)Z
- method setListener (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureListener;)V
- method setListener (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureListener;)V
- method unregister (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureInteractiveZone;)V
- method unregister (Ljava/lang/Object;Lcom/nokia/mid/ui/gestures/GestureInteractiveZone;)V
- method unregisterAll (Ljava/lang/Object;)V
- method unregisterAll (Ljava/lang/Object;)V

## com/nokia/mid/ui/lcdui/DisplayStateListener
- method <init> ()V

## com/nokia/mid/ui/lcdui/ForegroundUnavailableException
- method <init> ()V

## com/nokia/mid/ui/lcdui/Indicator
- method <init> ()V
- method <init> (ILjavax/microedition/lcdui/Image;)V
- method setActive (Z)V
- method setActive (Z)V

## com/nokia/mid/ui/lcdui/IndicatorManager
- method <init> ()V
- method appendIndicator (Lcom/nokia/mid/ui/lcdui/Indicator;Z)I
- method appendIndicator (Lcom/nokia/mid/ui/lcdui/Indicator;Z)I
- method getIndicatorManager ()Lcom/nokia/mid/ui/lcdui/IndicatorManager;
- method getIndicatorManager ()Lcom/nokia/mid/ui/lcdui/IndicatorManager;
- method shutdownIndicatorManager ()V
- method shutdownIndicatorManager ()V

## com/nokia/mid/ui/lcdui/LCDUIUtils
- method <init> ()V
- method getBestImageHeight (I)I
- method getBestImageHeight (I)I
- method getBestImageWidth (I)I
- method getBestImageWidth (I)I
- method isDisplayActive (Ljavax/microedition/lcdui/Display;)Z
- method isDisplayActive (Ljavax/microedition/lcdui/Display;)Z
- method setCurrent (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;)V
- method setCurrent (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;)V
- method setCurrent (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setCurrent (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setCurrentMIDlet (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setCurrentMIDlet (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;)V
- method setCurrentNoWaitForForeground (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;)V
- method setCurrentNoWaitForForeground (Ljavax/microedition/lcdui/Display;Ljavax/microedition/lcdui/Displayable;)V
- method setDisplayStateListener (Ljavax/microedition/lcdui/Display;Lcom/nokia/mid/ui/lcdui/DisplayStateListener;)V
- method setDisplayStateListener (Ljavax/microedition/lcdui/Display;Lcom/nokia/mid/ui/lcdui/DisplayStateListener;)V
- method setVisibilityListener (Ljavax/microedition/lcdui/Displayable;Lcom/nokia/mid/ui/lcdui/VisibilityListener;)V
- method setVisibilityListener (Ljavax/microedition/lcdui/Displayable;Lcom/nokia/mid/ui/lcdui/VisibilityListener;)V

## com/nokia/mid/ui/lcdui/VisibilityListener
- method <init> ()V

## com/nokia/mid/ui/locale/Locale
- method <init> ()V
- method getCountry ()Ljava/lang/String;
- method getCountry ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;

## com/nokia/mid/ui/locale/LocaleManager
- method <init> ()V
- method getAvailableLocales ()[Lcom/nokia/mid/ui/locale/Locale;
- method getAvailableLocales ()[Lcom/nokia/mid/ui/locale/Locale;
- method getInstance ()Lcom/nokia/mid/ui/locale/LocaleManager;
- method getInstance ()Lcom/nokia/mid/ui/locale/LocaleManager;

## com/nokia/mid/ui/multipointtouch/MultipointTouch
- method <init> ()V
- method addMultipointTouchListener (Lcom/nokia/mid/ui/multipointtouch/MultipointTouchListener;)V
- method addMultipointTouchListener (Lcom/nokia/mid/ui/multipointtouch/MultipointTouchListener;)V
- method getInstance ()Lcom/nokia/mid/ui/multipointtouch/MultipointTouch;
- method getInstance ()Lcom/nokia/mid/ui/multipointtouch/MultipointTouch;
- method getMaxPointers ()I
- method getMaxPointers ()I
- method getState (I)I
- method getState (I)I
- method getX (I)I
- method getX (I)I
- method getY (I)I
- method getY (I)I
- method removeMultipointTouchListener (Lcom/nokia/mid/ui/multipointtouch/MultipointTouchListener;)V
- method removeMultipointTouchListener (Lcom/nokia/mid/ui/multipointtouch/MultipointTouchListener;)V

## com/nokia/mid/ui/multipointtouch/MultipointTouchListener
- method <init> ()V

## com/nokia/mid/ui/orientation/Orientation
- method <init> ()V
- method addOrientationListener (Lcom/nokia/mid/ui/orientation/OrientationListener;)V
- method addOrientationListener (Lcom/nokia/mid/ui/orientation/OrientationListener;)V
- method getAppOrientation ()I
- method getAppOrientation ()I
- method getDisplayOrientation ()I
- method getDisplayOrientation ()I
- method removeOrientationListener (Lcom/nokia/mid/ui/orientation/OrientationListener;)V
- method removeOrientationListener (Lcom/nokia/mid/ui/orientation/OrientationListener;)V
- method setAppOrientation (I)V
- method setAppOrientation (I)V

## com/nokia/mid/ui/orientation/OrientationListener
- method <init> ()V

## com/nokia/mid/ui/s40/TextEditor
- method <init> ()V
- method getIndicatorIcons ()[Ljavax/microedition/lcdui/Image;
- method getIndicatorIcons ()[Ljavax/microedition/lcdui/Image;
- method getTextEditorCommands ()[Ljavax/microedition/lcdui/Command;
- method getTextEditorCommands ()[Ljavax/microedition/lcdui/Command;
- method isCommandKeyWanted (Ljavax/microedition/lcdui/Command;)Z
- method isCommandKeyWanted (Ljavax/microedition/lcdui/Command;)Z
- method isMenuCommand (Ljavax/microedition/lcdui/Command;)Z
- method isMenuCommand (Ljavax/microedition/lcdui/Command;)Z
- method launchTextEditorCommand (Ljavax/microedition/lcdui/Command;I)Z
- method launchTextEditorCommand (Ljavax/microedition/lcdui/Command;I)Z
- method setCursorWrap (I)V
- method setCursorWrap (I)V

## com/nokia/mid/ui/s40/TextEditorUtils
- method <init> ()V
- method copyToClipboard (Ljava/lang/String;)V
- method copyToClipboard (Ljava/lang/String;)V

## com/nokia/mid/ui/theme/Element
- method <init> ()V
- method getContent ()Ljava/lang/Object;
- method getContent ()Ljava/lang/Object;
- method getKind ()Ljava/lang/String;
- method getKind ()Ljava/lang/String;

## com/nokia/mid/ui/theme/Theme
- method <init> ()V
- method getElement (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/ui/theme/Element;
- method getElement (Ljava/lang/String;Ljava/lang/String;)Lcom/nokia/mid/ui/theme/Element;

## com/nokia/mid/ui/theme/ThemeManager
- method <init> ()V
- method getActiveTheme ()Lcom/nokia/mid/ui/theme/Theme;
- method getActiveTheme ()Lcom/nokia/mid/ui/theme/Theme;
- method getInstance ()Lcom/nokia/mid/ui/theme/ThemeManager;
- method getInstance ()Lcom/nokia/mid/ui/theme/ThemeManager;

## com/nokia/mj/impl/sensor/SensorManagerImpl
- method <init> ()V
- method findSensors (Ljava/lang/String;Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;
- method findSensors (Ljava/lang/String;Ljava/lang/String;)[Ljavax/microedition/sensor/SensorInfo;
- method getInstance ()Lcom/nokia/mj/impl/sensor/SensorManagerImpl;
- method getInstance ()Lcom/nokia/mj/impl/sensor/SensorManagerImpl;

## com/nokia/notifications/NotificationException
- method <init> ()V
- method getReason ()I
- method getReason ()I

## com/nokia/notifications/NotificationInfo
- method <init> ()V
- method getNotificationId ()Ljava/lang/String;
- method getNotificationId ()Ljava/lang/String;

## com/nokia/notifications/NotificationMessage
- method <init> ()V
- method getFrom ()Ljava/lang/String;
- method getFrom ()Ljava/lang/String;
- method getPayload ()Lcom/nokia/notifications/NotificationPayload;
- method getPayload ()Lcom/nokia/notifications/NotificationPayload;
- method getTimestamp ()Ljava/util/Date;
- method getTimestamp ()Ljava/util/Date;

## com/nokia/notifications/NotificationPayload
- method <init> ()V
- method getData ()Ljava/lang/String;
- method getData ()Ljava/lang/String;

## com/nokia/notifications/NotificationSession
- method <init> ()V
- method close ()V
- method close ()V
- method getNotificationInformation ()V
- method getNotificationInformation ()V
- method registerApplication ()V
- method registerApplication ()V
- method setWakeUp (Z)V
- method setWakeUp (Z)V
- method unregisterApplication ()V
- method unregisterApplication ()V

## com/nokia/notifications/NotificationSessionFactory
- method <init> ()V
- method getNotificationsEnablerVersion ()Ljava/lang/String;
- method getNotificationsEnablerVersion ()Ljava/lang/String;
- method openSession (Ljavax/microedition/midlet/MIDlet;Ljava/lang/String;Ljava/lang/String;Lcom/nokia/notifications/NotificationSessionListener;)Lcom/nokia/notifications/NotificationSession;
- method openSession (Ljavax/microedition/midlet/MIDlet;Ljava/lang/String;Ljava/lang/String;Lcom/nokia/notifications/NotificationSessionListener;)Lcom/nokia/notifications/NotificationSession;

## com/nokia/notifications/NotificationSessionListener
- method <init> ()V

## com/nokia/notifications/NotificationState
- method <init> ()V
- method getSessionError ()I
- method getSessionError ()I
- method getSessionState ()I
- method getSessionState ()I

## com/nokia/notifications/installer/InstallListener
- method <init> ()V

## com/nokia/notifications/installer/InstallerFactory
- method <init> ()V
- method getInstaller ()Lcom/nokia/notifications/installer/NotificationsEnablerInstaller;
- method getInstaller ()Lcom/nokia/notifications/installer/NotificationsEnablerInstaller;

## com/nokia/notifications/installer/NotificationsEnablerInstaller
- method <init> ()V
- method checkAndUpdateNapiEnabler (Lcom/nokia/notifications/installer/InstallListener;Ljavax/microedition/lcdui/Displayable;Ljavax/microedition/midlet/MIDlet;)V
- method checkAndUpdateNapiEnabler (Lcom/nokia/notifications/installer/InstallListener;Ljavax/microedition/lcdui/Displayable;Ljavax/microedition/midlet/MIDlet;)V
- method checkNapiEnabler (Lcom/nokia/notifications/installer/InstallListener;Ljavax/microedition/midlet/MIDlet;)V
- method checkNapiEnabler (Lcom/nokia/notifications/installer/InstallListener;Ljavax/microedition/midlet/MIDlet;)V

## com/nokia/phone/sdk/concept/exec/modules/push/PushEntryValidator
- method <init> ()V

## com/nokia/phone/sdk/concept/exec/modules/push/PushRegistryExtension
- method <init> ()V
- method getInstance ()Lcom/nokia/phone/sdk/concept/exec/modules/push/PushRegistryExtension;
- method getInstance ()Lcom/nokia/phone/sdk/concept/exec/modules/push/PushRegistryExtension;
- method getWakeUpConnection (Ljava/lang/String;)Ljavax/microedition/io/Connection;
- method getWakeUpConnection (Ljava/lang/String;)Ljavax/microedition/io/Connection;

## com/nokia/phone/sdk/concept/exec/modules/push/PushSecurityTicket
- method <init> ()V

## com/nokia/phone/sdk/concept/exec/modules/push/PushStreamValidator
- method <init> ()V

## com/nokia/phone/sdk/concept/exec/modules/push/PushableConnection
- method <init> ()V

## com/nokia/phone/sdk/concept/gateway/MEMirrorConnectionPool
- method <init> ()V
- method getConnection (I)Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorConnection;
- method getConnection (I)Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorConnection;
- method getInstance ()Lcom/nokia/phone/sdk/concept/gateway/MEMirrorConnectionPool;
- method getInstance ()Lcom/nokia/phone/sdk/concept/gateway/MEMirrorConnectionPool;

## com/nokia/phone/sdk/concept/gateway/mirror/MirrorConnection
- method <init> ()V
- method getMirrorInput ()Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorInputStream;
- method getMirrorInput ()Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorInputStream;
- method getMirrorOutput ()Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorOutputStream;
- method getMirrorOutput ()Lcom/nokia/phone/sdk/concept/gateway/mirror/MirrorOutputStream;
- method release ()V
- method release ()V
- method reserve ()V
- method reserve ()V

## com/nokia/phone/sdk/concept/gateway/mirror/MirrorInputStream
- method <init> ()V
- method readIntArray ()[I
- method readIntArray ()[I
- method readResponseHeader ()V
- method readResponseHeader ()V
- method readStringArray ()[Ljava/lang/String;
- method readStringArray ()[Ljava/lang/String;

## com/nokia/phone/sdk/concept/gateway/mirror/MirrorOutputStream
- method <init> ()V
- method flush ()V
- method flush ()V
- method mirrorTargetToBytes (Ljava/lang/String;Ljava/lang/String;Z)[B
- method mirrorTargetToBytes (Ljava/lang/String;Ljava/lang/String;Z)[B
- method write ([B)V
- method write ([B)V
- method writeUTF (Ljava/lang/String;)V
- method writeUTF (Ljava/lang/String;)V

## com/nokia/phone/sdk/concept/midlet/MIDletSuite
- method <init> ()V
- method checkPermission (Ljava/lang/String;)I
- method checkPermission (Ljava/lang/String;)I

## com/nokia/phone/sdk/concept/midlet/Scheduler
- method <init> ()V
- method getMIDletSuite ()Lcom/nokia/phone/sdk/concept/midlet/MIDletSuite;
- method getMIDletSuite ()Lcom/nokia/phone/sdk/concept/midlet/MIDletSuite;
- method getScheduler ()Lcom/nokia/phone/sdk/concept/midlet/Scheduler;
- method getScheduler ()Lcom/nokia/phone/sdk/concept/midlet/Scheduler;

## com/samsung/mpns/PushReceiver
- method <init> ()V
- method deregister (Ljava/lang/String;)Z
- method deregister (Ljava/lang/String;)Z
- method getInstance ()Lcom/samsung/mpns/PushReceiver;
- method getInstance ()Lcom/samsung/mpns/PushReceiver;
- method getToken ()Ljava/lang/String;
- method getToken ()Ljava/lang/String;
- method isAppRegistered (Ljava/lang/String;)Z
- method isAppRegistered (Ljava/lang/String;)Z
- method registerWithUser (Ljava/lang/String;Ljava/lang/String;)Z
- method registerWithUser (Ljava/lang/String;Ljava/lang/String;)Z

## com/samsung/util/LCDLight
- method <init> ()V
- method isSupported ()Z
- method isSupported ()Z
- method off ()V
- method off ()V
- method on (I)V
- method on (I)V

## com/samsung/util/SM
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method setData (Ljava/lang/String;)V
- method setData (Ljava/lang/String;)V
- method setDestAddress (Ljava/lang/String;)Lcom/samsung/util/SM;
- method setDestAddress (Ljava/lang/String;)Lcom/samsung/util/SM;
- method setDestAddress (Ljava/lang/String;)V
- method setDestAddress (Ljava/lang/String;)V

## com/samsung/util/SMS
- method <init> ()V
- method isSupported ()Z
- method isSupported ()Z
- method send (Lcom/samsung/util/SM;)V
- method send (Lcom/samsung/util/SM;)V

## com/siemens/mp/MIDlet
- method <init> ()V
- method getCommandLine ()[Ljava/lang/String;
- method getCommandLine ()[Ljava/lang/String;
- method getSupportedProtocols ()[Ljava/lang/String;
- method getSupportedProtocols ()[Ljava/lang/String;
- method platformRequest (Ljava/lang/String;)Z
- method platformRequest (Ljava/lang/String;)Z

## com/siemens/mp/NotAllowedException
- method <init> ()V

## com/siemens/mp/color_game/Layer
- method <init> ()V
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I
- method getX ()I
- method getX ()I
- method getY ()I
- method getY ()I
- method isVisible ()Z
- method isVisible ()Z
- method move (II)V
- method move (II)V
- method setHeight (I)V
- method setHeight (I)V
- method setPosition (II)V
- method setPosition (II)V
- method setVisible (Z)V
- method setVisible (Z)V
- method setWidth (I)V
- method setWidth (I)V

## com/siemens/mp/color_game/LayerManager
- method <init> ()V
- method append (Lcom/siemens/mp/color_game/Layer;)V
- method append (Lcom/siemens/mp/color_game/Layer;)V
- method getLayerAt (I)Lcom/siemens/mp/color_game/Layer;
- method getLayerAt (I)Lcom/siemens/mp/color_game/Layer;
- method getSize ()I
- method getSize ()I
- method insert (Lcom/siemens/mp/color_game/Layer;I)V
- method insert (Lcom/siemens/mp/color_game/Layer;I)V
- method paint (Ljavax/microedition/lcdui/Graphics;II)V
- method paint (Ljavax/microedition/lcdui/Graphics;II)V
- method remove (Lcom/siemens/mp/color_game/Layer;)V
- method remove (Lcom/siemens/mp/color_game/Layer;)V
- method setViewWindow (IIII)V
- method setViewWindow (IIII)V

## com/siemens/mp/color_game/Sprite
- method <init> ()V
- method <init> (Lcom/siemens/mp/color_game/Sprite;)V
- method <init> (Ljavax/microedition/lcdui/Image;)V
- method <init> (Ljavax/microedition/lcdui/Image;II)V
- method collidesWith (Lcom/siemens/mp/color_game/Sprite;Z)Z
- method collidesWith (Lcom/siemens/mp/color_game/Sprite;Z)Z
- method collidesWith (Ljavax/microedition/lcdui/Image;IIZ)Z
- method collidesWith (Ljavax/microedition/lcdui/Image;IIZ)Z
- method getFrame ()I
- method getFrame ()I
- method getFrameSequenceLength ()I
- method getFrameSequenceLength ()I
- method getRawFrameCount ()I
- method getRawFrameCount ()I
- method nextFrame ()V
- method nextFrame ()V
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method prevFrame ()V
- method prevFrame ()V
- method setCollisionRectangle (IIII)V
- method setCollisionRectangle (IIII)V
- method setFrame (I)V
- method setFrame (I)V
- method setFrameSequence ([I)V
- method setFrameSequence ([I)V
- method setImage (Ljavax/microedition/lcdui/Image;II)V
- method setImage (Ljavax/microedition/lcdui/Image;II)V

## com/siemens/mp/color_game/TiledLayer
- method <init> ()V
- method <init> (IILjavax/microedition/lcdui/Image;II)V
- method createAnimatedTile (I)I
- method createAnimatedTile (I)I
- method fillCells (IIIII)V
- method fillCells (IIIII)V
- method getAnimatedTile (I)I
- method getAnimatedTile (I)I
- method getCell (II)I
- method getCell (II)I
- method getCellHeight ()I
- method getCellHeight ()I
- method getCellWidth ()I
- method getCellWidth ()I
- method getColumns ()I
- method getColumns ()I
- method getRows ()I
- method getRows ()I
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method paint (Ljavax/microedition/lcdui/Graphics;)V
- method setAnimatedTile (II)V
- method setAnimatedTile (II)V
- method setCell (III)V
- method setCell (III)V
- method setStaticTileSet (Ljavax/microedition/lcdui/Image;II)V
- method setStaticTileSet (Ljavax/microedition/lcdui/Image;II)V

## com/siemens/mp/game/ExtendedImage
- method <init> ()V
- method <init> (Ljavax/microedition/lcdui/Image;)V
- method blitToScreen (II)V
- method blitToScreen (II)V
- method getPixel (II)I
- method getPixel (II)I
- method setPixel (IIB)V
- method setPixel (IIB)V
- method setPixels ([BIIII)V
- method setPixels ([BIIII)V

## com/siemens/mp/game/MelodyComposer
- method <init> ()V
- method appendNote (II)V
- method appendNote (II)V
- method getMelody ()Lcom/siemens/mp/game/Melody;
- method getMelody ()Lcom/siemens/mp/game/Melody;
- method resetMelody ()V
- method resetMelody ()V
- method setBPM (I)V
- method setBPM (I)V

## com/siemens/mp/gsm/Call
- method <init> ()V
- method start (Ljava/lang/String;)V
- method start (Ljava/lang/String;)V

## com/siemens/mp/gsm/PhoneBook
- method <init> ()V
- method getMDN ()[Ljava/lang/String;
- method getMDN ()[Ljava/lang/String;

## com/siemens/mp/gsm/SMS
- method <init> ()V
- method send (Ljava/lang/String;Ljava/lang/String;)I
- method send (Ljava/lang/String;Ljava/lang/String;)I
- method send (Ljava/lang/String;[B)I
- method send (Ljava/lang/String;[B)I

## com/siemens/mp/io/Connection
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method send ([B)V
- method send ([B)V

## com/siemens/mp/io/File
- method <init> ()V
- method buildPath (Ljava/lang/String;)Ljava/lang/String;
- method buildPath (Ljava/lang/String;)Ljava/lang/String;
- method checkFileName (Ljava/lang/String;)I
- method checkFileName (Ljava/lang/String;)I
- method close (I)I
- method close (I)I
- method copy (Ljava/lang/String;Ljava/lang/String;)I
- method copy (Ljava/lang/String;Ljava/lang/String;)I
- method debugWrite (Ljava/lang/String;Ljava/lang/String;)I
- method debugWrite (Ljava/lang/String;Ljava/lang/String;)I
- method delete (Ljava/lang/String;)I
- method delete (Ljava/lang/String;)I
- method exists (Ljava/lang/String;)I
- method exists (Ljava/lang/String;)I
- method getDirectorySize (Ljava/lang/String;)J
- method getDirectorySize (Ljava/lang/String;)J
- method getIsHidden (Ljava/lang/String;)Z
- method getIsHidden (Ljava/lang/String;)Z
- method getLastModified (Ljava/lang/String;)J
- method getLastModified (Ljava/lang/String;)J
- method isDirectory (Ljava/lang/String;)Z
- method isDirectory (Ljava/lang/String;)Z
- method length (I)I
- method length (I)I
- method list (Ljava/lang/String;)[Ljava/lang/String;
- method list (Ljava/lang/String;)[Ljava/lang/String;
- method list (Ljava/lang/String;Z)[Ljava/lang/String;
- method list (Ljava/lang/String;Z)[Ljava/lang/String;
- method mkdir (Ljava/lang/String;)V
- method mkdir (Ljava/lang/String;)V
- method mkdir (Ljava/lang/String;)Z
- method mkdir (Ljava/lang/String;)Z
- method open (Ljava/lang/String;)I
- method open (Ljava/lang/String;)I
- method read (I[BII)I
- method read (I[BII)I
- method rename (Ljava/lang/String;Ljava/lang/String;)I
- method rename (Ljava/lang/String;Ljava/lang/String;)I
- method seek (II)I
- method seek (II)I
- method spaceAvailable ()I
- method spaceAvailable ()I
- method truncate (II)V
- method truncate (II)V
- method write (I[BII)I
- method write (I[BII)I

## com/siemens/mp/io/file/FileSystemListener
- method <init> ()V

## com/siemens/mp/io/file/FileSystemRegistry
- method <init> ()V
- method listRoots ()Ljava/util/Enumeration;
- method listRoots ()Ljava/util/Enumeration;

## com/siemens/mp/lcdui/Canvas
- method <init> ()V
- method keyPressed (I)V
- method keyPressed (I)V
- method keyRepeated (I)V
- method keyRepeated (I)V
- method setCenterKeyIcon (C)V
- method setCenterKeyIcon (C)V
- method setCenterKeyIcon (Ljavax/microedition/lcdui/Canvas;C)V
- method setCenterKeyIcon (Ljavax/microedition/lcdui/Canvas;C)V
- method setFullScreenMode (Z)V
- method setFullScreenMode (Z)V

## com/siemens/mp/lcdui/Display
- method <init> ()V
- method getColor (I)I
- method getColor (I)I

## com/siemens/mp/lcdui/Displayable
- method <init> ()V
- method setHeadlineIcon (Ljavax/microedition/lcdui/Displayable;Ljavax/microedition/lcdui/Image;)V
- method setHeadlineIcon (Ljavax/microedition/lcdui/Displayable;Ljavax/microedition/lcdui/Image;)V
- method setHeadlineRightText (Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;)V
- method setHeadlineRightText (Ljavax/microedition/lcdui/Displayable;Ljava/lang/String;)V
- method setPopupFlag (Ljavax/microedition/lcdui/Displayable;Z)V
- method setPopupFlag (Ljavax/microedition/lcdui/Displayable;Z)V
- method setSoftkeyCommand (Ljavax/microedition/lcdui/Displayable;ILjavax/microedition/lcdui/Command;)V
- method setSoftkeyCommand (Ljavax/microedition/lcdui/Displayable;ILjavax/microedition/lcdui/Command;)V

## com/siemens/mp/lcdui/Form
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method append (Ljava/lang/String;)I
- method append (Ljava/lang/String;)I
- method append (Ljavax/microedition/lcdui/Item;)I
- method append (Ljavax/microedition/lcdui/Item;)I
- method appendStaticItem (Ljavax/microedition/lcdui/Item;)I
- method appendStaticItem (Ljavax/microedition/lcdui/Item;)I
- method deleteAll ()V
- method deleteAll ()V
- method getKeyAction (I)I
- method getKeyAction (I)I
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method set (ILjavax/microedition/lcdui/Item;)V
- method set (ILjavax/microedition/lcdui/Item;)V
- method setCommandListener (Ljavax/microedition/lcdui/CommandListener;)V
- method setCommandListener (Ljavax/microedition/lcdui/CommandListener;)V
- method setItemStateListener (Ljavax/microedition/lcdui/ItemStateListener;)V
- method setItemStateListener (Ljavax/microedition/lcdui/ItemStateListener;)V
- method setKeyDispatcher (Lcom/siemens/mp/lcdui/KeyDispatcher;)V
- method setKeyDispatcher (Lcom/siemens/mp/lcdui/KeyDispatcher;)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V

## com/siemens/mp/lcdui/Graphics
- method <init> ()V
- method setAreaColor (Ljavax/microedition/lcdui/Graphics;IIIII)V
- method setAreaColor (Ljavax/microedition/lcdui/Graphics;IIIII)V
- method setLightOff ()V
- method setLightOff ()V
- method setLightOn ()V
- method setLightOn ()V

## com/siemens/mp/lcdui/KeyDispatcher
- method <init> ()V

## com/siemens/mp/lcdui/Menu
- method <init> ()V
- method getMenuBackgroundImage ()Ljavax/microedition/lcdui/Image;
- method getMenuBackgroundImage ()Ljavax/microedition/lcdui/Image;

## com/siemens/mp/lcdui/PleaseWait
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V

## com/siemens/mp/lcdui/XList
- method <init> ()V
- method <init> (Ljava/lang/String;ILcom/siemens/mp/lcdui/XListConfig;)V
- method <init> (Ljava/lang/String;I[Ljava/lang/String;[Ljavax/microedition/lcdui/Image;Lcom/siemens/mp/lcdui/XListConfig;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method addCommand (Ljavax/microedition/lcdui/Command;)V
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method append (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method deleteAll ()V
- method deleteAll ()V
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method removeCommand (Ljavax/microedition/lcdui/Command;)V
- method setCommandListener (Ljavax/microedition/lcdui/CommandListener;)V
- method setCommandListener (Ljavax/microedition/lcdui/CommandListener;)V
- method setSelectedIndex (IZ)V
- method setSelectedIndex (IZ)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method setXListStateListener (Lcom/siemens/mp/lcdui/XListStateListener;)V
- method setXListStateListener (Lcom/siemens/mp/lcdui/XListStateListener;)V

## com/siemens/mp/lcdui/XListConfig
- method <init> ()V
- method setFlag (IZ)V
- method setFlag (IZ)V

## com/siemens/mp/lcdui/XListStateListener
- method <init> ()V

## com/siemens/mp/media/Controllable
- method <init> ()V
- method getControl (Ljava/lang/String;)Lcom/siemens/mp/media/Control;
- method getControl (Ljava/lang/String;)Lcom/siemens/mp/media/Control;
- method getControls ()[Lcom/siemens/mp/media/Control;
- method getControls ()[Lcom/siemens/mp/media/Control;

## com/siemens/mp/media/NativePlayerListener
- method <init> ()V

## com/siemens/mp/media/PlayerListener
- method <init> ()V

## com/siemens/mp/media/TimeBase
- method <init> ()V

## com/siemens/mp/media/control/VolumeControl
- method <init> ()V
- method getLevel ()I
- method getLevel ()I
- method isMuted ()Z
- method isMuted ()Z
- method setLevel (I)I
- method setLevel (I)I
- method setMute (Z)V
- method setMute (Z)V

## com/siemens/mp/misc/NativeMem
- method <init> ()V

## com/siemens/mp/misc/Security
- method <init> ()V
- method ConfirmationRequest (Ljava/lang/String;ILjava/lang/Object;)Z
- method ConfirmationRequest (Ljava/lang/String;ILjava/lang/Object;)Z

## com/siemens/mp/pim/Contact
- method <init> ()V

## com/siemens/mp/pim/ContactList
- method <init> ()V

## com/siemens/mp/pim/PIM
- method <init> ()V
- method getInstance ()Lcom/siemens/mp/pim/PIM;
- method getInstance ()Lcom/siemens/mp/pim/PIM;
- method openPIMList (II)Lcom/siemens/mp/pim/PIMList;
- method openPIMList (II)Lcom/siemens/mp/pim/PIMList;

## com/siemens/mp/pim/PIMItem
- method <init> ()V
- method getString (II)Ljava/lang/String;
- method getString (II)Ljava/lang/String;

## com/siemens/mp/pim/PIMList
- method <init> ()V
- method items ()Ljava/util/Enumeration;
- method items ()Ljava/util/Enumeration;

## com/siemens/mp/util/HelpSystem
- method <init> ()V
- method openHelpDialog (Ljava/lang/String;)V
- method openHelpDialog (Ljava/lang/String;)V

## com/siemens/mp/util/zip/ZipEntry
- method <init> ()V
- method getCompressedSize ()J
- method getCompressedSize ()J
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getSize ()J
- method getSize ()J
- method isDirectory ()Z
- method isDirectory ()Z

## com/siemens/mp/util/zip/ZipFile
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method close ()V
- method close ()V
- method entries ()Ljava/util/Enumeration;
- method entries ()Ljava/util/Enumeration;
- method getEntry (Ljava/lang/String;)Lcom/siemens/mp/util/zip/ZipEntry;
- method getEntry (Ljava/lang/String;)Lcom/siemens/mp/util/zip/ZipEntry;
- method getInputStream (Lcom/siemens/mp/util/zip/ZipEntry;)Ljava/io/InputStream;
- method getInputStream (Lcom/siemens/mp/util/zip/ZipEntry;)Ljava/io/InputStream;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method size ()I
- method size ()I

## com/siemens/mp/xml/Parser
- method <init> ()V
- method parseFileXML (Ljava/lang/String;)V
- method parseFileXML (Ljava/lang/String;)V

## com/sonyericsson/ams/Application
- method <init> ()V
- method getStatus ()I
- method getStatus ()I
- method stop ()V
- method stop ()V

## com/sonyericsson/ams/ApplicationManager
- method <init> ()V
- method getApplication (Ljava/lang/String;Ljava/lang/String;)Lcom/sonyericsson/ams/Application;
- method getApplication (Ljava/lang/String;Ljava/lang/String;)Lcom/sonyericsson/ams/Application;
- method getApplicationManager ()Lcom/sonyericsson/ams/ApplicationManager;
- method getApplicationManager ()Lcom/sonyericsson/ams/ApplicationManager;
- method installApplication (Ljava/lang/String;Ljava/lang/String;Z)V
- method installApplication (Ljava/lang/String;Ljava/lang/String;Z)V
- method startApplication (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Lcom/sonyericsson/ams/Application;
- method startApplication (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)Lcom/sonyericsson/ams/Application;

## com/sonyericsson/ams/NoSuchApplicationException
- method <init> ()V

## com/sonyericsson/app/waterlevel/view/factory/FlashCanvasFactory
- method <init> ()V

## com/sonyericsson/app/waterlevel/view/factory/GameCanvasFactory
- method <init> ()V

## com/sonyericsson/app/waterlevel/view/gamecanvas/ReferenceAngleCanvas
- method <init> ()V
- method paint ()V
- method paint ()V

## com/sonyericsson/capuchin/FlashCanvas
- method <init> ()V
- method <init> (Lcom/sonyericsson/capuchin/FlashImage;)V
- method keyPressed (I)V
- method keyPressed (I)V

## com/sonyericsson/capuchin/FlashDataRequest
- method <init> ()V
- method complete ()V
- method complete ()V
- method getArgs ()[Ljava/lang/String;
- method getArgs ()[Ljava/lang/String;
- method setProperty (Ljava/lang/String;I)V
- method setProperty (Ljava/lang/String;I)V
- method setProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setProperty (Ljava/lang/String;Ljava/lang/String;)V

## com/sonyericsson/capuchin/FlashDataRequestListener
- method <init> ()V

## com/sonyericsson/capuchin/FlashEventListener
- method <init> ()V
- method handleEvent ([Ljava/lang/String;)V
- method handleEvent ([Ljava/lang/String;)V

## com/sonyericsson/capuchin/FlashEventManager
- method <init> ()V

## com/sonyericsson/capuchin/FlashImage
- method <init> ()V
- method createImage (Ljava/io/InputStream;Lcom/sonyericsson/capuchin/ExternalResourceHandler;)Lcom/sonyericsson/capuchin/FlashImage;
- method createImage (Ljava/io/InputStream;Lcom/sonyericsson/capuchin/ExternalResourceHandler;)Lcom/sonyericsson/capuchin/FlashImage;
- method render (Ljavax/microedition/lcdui/Graphics;IIII)I
- method render (Ljavax/microedition/lcdui/Graphics;IIII)I
- method setFlashDataRequestListener (Lcom/sonyericsson/capuchin/FlashDataRequestListener;)V
- method setFlashDataRequestListener (Lcom/sonyericsson/capuchin/FlashDataRequestListener;)V
- method setFlashEventManager (Lcom/sonyericsson/capuchin/FlashEventManager;)V
- method setFlashEventManager (Lcom/sonyericsson/capuchin/FlashEventManager;)V

## com/sonyericsson/capuchin/FlashPlayer
- method <init> ()V
- method createFlashPlayer (Lcom/sonyericsson/capuchin/FlashImage;Lcom/sonyericsson/capuchin/FlashCanvas;)Lcom/sonyericsson/capuchin/FlashPlayer;
- method createFlashPlayer (Lcom/sonyericsson/capuchin/FlashImage;Lcom/sonyericsson/capuchin/FlashCanvas;)Lcom/sonyericsson/capuchin/FlashPlayer;
- method getDisplayable ()Ljavax/microedition/lcdui/Displayable;
- method getDisplayable ()Ljavax/microedition/lcdui/Displayable;

## com/sonyericsson/homescreen/Homescreen
- method <init> ()V
- method addKeyListener (Lcom/sonyericsson/homescreen/KeyListener;)V
- method addKeyListener (Lcom/sonyericsson/homescreen/KeyListener;)V
- method enterStandby ()V
- method enterStandby ()V
- method leaveStandby ()V
- method leaveStandby ()V
- method setKey (I)V
- method setKey (I)V
- method setKey (ILjava/lang/String;)V
- method setKey (ILjava/lang/String;)V
- method setStandbyInformation (Z)V
- method setStandbyInformation (Z)V

## com/sonyericsson/homescreen/HomescreenMIDlet
- method <init> ()V
- method getHomescreen ()Lcom/sonyericsson/homescreen/Homescreen;
- method getHomescreen ()Lcom/sonyericsson/homescreen/Homescreen;

## com/sonyericsson/homescreen/KeyListener
- method <init> ()V

## com/sonyericsson/media/control/DisplayModeControl
- method <init> ()V
- method setDisplayMode (I)V
- method setDisplayMode (I)V

## com/sonyericsson/multimedia/ControlEvent
- method <init> ()V
- method getData ()Ljava/lang/Object;
- method getData ()Ljava/lang/Object;

## com/sonyericsson/multimedia/ControlException
- method <init> ()V

## com/sonyericsson/multimedia/Media
- method <init> ()V
- method getDuration ()I
- method getDuration ()I
- method getMediaTime ()I
- method getMediaTime ()I
- method getMetaData ()Lcom/sonyericsson/multimedia/MetaData;
- method getMetaData ()Lcom/sonyericsson/multimedia/MetaData;

## com/sonyericsson/multimedia/MetaData
- method <init> ()V
- method getAlbumArt ()[B
- method getAlbumArt ()[B
- method getValue (Ljava/lang/String;)Ljava/lang/String;
- method getValue (Ljava/lang/String;)Ljava/lang/String;

## com/sonyericsson/multimedia/MultimediaService
- method <init> ()V
- method getControl (Ljava/lang/String;)Lcom/sonyericsson/multimedia/Control;
- method getControl (Ljava/lang/String;)Lcom/sonyericsson/multimedia/Control;

## com/sonyericsson/multimedia/MultimediaServiceManager
- method <init> ()V
- method getMultimediaService (Ljava/lang/String;)Lcom/sonyericsson/multimedia/MultimediaService;
- method getMultimediaService (Ljava/lang/String;)Lcom/sonyericsson/multimedia/MultimediaService;

## com/sonyericsson/multimedia/control/MediaControl
- method <init> ()V
- method addMediaControlListener (Lcom/sonyericsson/multimedia/control/MediaControlListener;)V
- method addMediaControlListener (Lcom/sonyericsson/multimedia/control/MediaControlListener;)V
- method fastForward ()V
- method fastForward ()V
- method getState ()I
- method getState ()I
- method next ()V
- method next ()V
- method pause ()V
- method pause ()V
- method play ()V
- method play ()V
- method prev ()V
- method prev ()V
- method rewind ()V
- method rewind ()V
- method skip (I)V
- method skip (I)V

## com/sonyericsson/multimedia/control/MediaControlListener
- method <init> ()V

## com/sonyericsson/multimedia/service/WalkmanService
- method <init> ()V
- method getServiceName ()Ljava/lang/String;
- method getServiceName ()Ljava/lang/String;

## com/sonyericsson/ui/UIActivityMenu
- method <init> ()V
- method addEvent (Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/Image;)I
- method addEvent (Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/lcdui/Image;Ljavax/microedition/lcdui/Image;)I
- method addEvent (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method addEvent (Ljava/lang/String;Ljavax/microedition/lcdui/Image;)I
- method getInstance (Ljavax/microedition/midlet/MIDlet;)Lcom/sonyericsson/ui/UIActivityMenu;
- method getInstance (Ljavax/microedition/midlet/MIDlet;)Lcom/sonyericsson/ui/UIActivityMenu;
- method getInstance (Ljavay/microedition/lcdui/MIDhack;)Lcom/sonyericsson/ui/UIActivityMenu;
- method getInstance (Ljavay/microedition/lcdui/MIDhack;)Lcom/sonyericsson/ui/UIActivityMenu;
- method getInstance (Llib/MIDlet;)Lcom/sonyericsson/ui/UIActivityMenu;
- method getInstance (Llib/MIDlet;)Lcom/sonyericsson/ui/UIActivityMenu;
- method setEventListener (Lcom/sonyericsson/ui/UIEventListener;)V
- method setEventListener (Lcom/sonyericsson/ui/UIEventListener;)V

## com/sonyericsson/ui/UIEventListener
- method <init> ()V

## com/sprintpcs/media/DualTone
- method <init> ()V
- method <init> ([I[I[III)V

## com/sprintpcs/util/Location
- method <init> ()V
- method setServerIPAddress (Ljava/lang/String;)V
- method setServerIPAddress (Ljava/lang/String;)V
- method setServerIPPort (Ljava/lang/String;)V
- method setServerIPPort (Ljava/lang/String;)V

## com/vodafone/midlet/ResidentMIDlet
- method <init> ()V

## com/vodafone/system/DeviceControl
- method <init> ()V
- method getDefaultDeviceControl ()Lcom/vodafone/system/DeviceControl;
- method getDefaultDeviceControl ()Lcom/vodafone/system/DeviceControl;
- method getDeviceState (I)I
- method getDeviceState (I)I

## com/vodafone/v10/midlet/ResidentMIDlet
- method <init> ()V

## com/vodafone/v10/sound/Sound
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## com/vodafone/v10/sound/SoundPlayer
- method <init> ()V
- method disposeTrack (Lcom/vodafone/v10/sound/SoundTrack;)V
- method disposeTrack (Lcom/vodafone/v10/sound/SoundTrack;)V
- method getPlayer ()Lcom/vodafone/v10/sound/SoundPlayer;
- method getPlayer ()Lcom/vodafone/v10/sound/SoundPlayer;
- method getTrack ()Lcom/vodafone/v10/sound/SoundTrack;
- method getTrack ()Lcom/vodafone/v10/sound/SoundTrack;

## com/vodafone/v10/sound/SoundTrack
- method <init> ()V
- method play (I)V
- method play (I)V
- method removeSound ()V
- method removeSound ()V
- method setSound (Lcom/vodafone/v10/sound/Sound;)V
- method setSound (Lcom/vodafone/v10/sound/Sound;)V
- method stop ()V
- method stop ()V

## com/vodafone/v10/system/device/DeviceControl
- method <init> ()V
- method getDefaultDeviceControl ()Lcom/vodafone/v10/system/device/DeviceControl;
- method getDefaultDeviceControl ()Lcom/vodafone/v10/system/device/DeviceControl;
- method setDeviceActive (IZ)V
- method setDeviceActive (IZ)V

## java/awt/AWTEvent
- method <init> ()V

## java/awt/Adjustable
- method <init> ()V
- method setBlockIncrement (I)V
- method setBlockIncrement (I)V
- method setUnitIncrement (I)V
- method setUnitIncrement (I)V

## java/awt/AlphaComposite
- method <init> ()V
- method getInstance (IF)Ljava/awt/AlphaComposite;
- method getInstance (IF)Ljava/awt/AlphaComposite;

## java/awt/BasicStroke
- method <init> ()V
- method <init> (F)V
- method <init> (FII)V
- method <init> (FIIF[FF)V

## java/awt/BorderLayout
- method <init> ()V
- method <init> (II)V
- method getHgap ()I
- method getHgap ()I
- method getVgap ()I
- method getVgap ()I
- method setHgap (I)V
- method setHgap (I)V
- method setVgap (I)V
- method setVgap (I)V

## java/awt/Button
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V

## java/awt/Canvas
- method <init> ()V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method getHeight ()I
- method getHeight ()I
- method getSize ()Ljava/awt/Dimension;
- method getSize ()Ljava/awt/Dimension;
- method getWidth ()I
- method getWidth ()I
- method requestFocusInWindow ()Z
- method requestFocusInWindow ()Z
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setVisible (Z)V
- method setVisible (Z)V

## java/awt/Choice
- method <init> ()V
- method addItem (Ljava/lang/String;)V
- method addItem (Ljava/lang/String;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getSelectedItem ()Ljava/lang/String;
- method getSelectedItem ()Ljava/lang/String;
- method select (I)V
- method select (I)V
- method select (Ljava/lang/String;)V
- method select (Ljava/lang/String;)V

## java/awt/Color
- field BLACK Ljava/awt/Color;
- field BLACK Ljava/awt/Color;
- field BLUE Ljava/awt/Color;
- field BLUE Ljava/awt/Color;
- field DARK_GRAY Ljava/awt/Color;
- field DARK_GRAY Ljava/awt/Color;
- field RED Ljava/awt/Color;
- field RED Ljava/awt/Color;
- field WHITE Ljava/awt/Color;
- field WHITE Ljava/awt/Color;
- field black Ljava/awt/Color;
- field black Ljava/awt/Color;
- field blue Ljava/awt/Color;
- field blue Ljava/awt/Color;
- field darkGray Ljava/awt/Color;
- field darkGray Ljava/awt/Color;
- field gray Ljava/awt/Color;
- field gray Ljava/awt/Color;
- field lightGray Ljava/awt/Color;
- field lightGray Ljava/awt/Color;
- field pink Ljava/awt/Color;
- field pink Ljava/awt/Color;
- field red Ljava/awt/Color;
- field red Ljava/awt/Color;
- field white Ljava/awt/Color;
- field white Ljava/awt/Color;
- field yellow Ljava/awt/Color;
- field yellow Ljava/awt/Color;
- method <init> ()V
- method <init> (FFFF)V
- method <init> (I)V
- method <init> (III)V
- method <init> (IIII)V
- method <init> (IZ)V
- method RGBtoHSB (III[F)[F
- method RGBtoHSB (III[F)[F
- method decode (Ljava/lang/String;)Ljava/awt/Color;
- method decode (Ljava/lang/String;)Ljava/awt/Color;
- method getAlpha ()I
- method getAlpha ()I
- method getBlue ()I
- method getBlue ()I
- method getGreen ()I
- method getGreen ()I
- method getHSBColor (FFF)Ljava/awt/Color;
- method getHSBColor (FFF)Ljava/awt/Color;
- method getRGB ()I
- method getRGB ()I
- method getRed ()I
- method getRed ()I

## java/awt/Component
- method <init> ()V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addFocusListener (Ljava/awt/event/FocusListener;)V
- method addFocusListener (Ljava/awt/event/FocusListener;)V
- method addHierarchyListener (Ljava/awt/event/HierarchyListener;)V
- method addHierarchyListener (Ljava/awt/event/HierarchyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method enableInputMethods (Z)V
- method enableInputMethods (Z)V
- method getBounds ()Ljava/awt/Rectangle;
- method getBounds ()Ljava/awt/Rectangle;
- method getFontMetrics (Ljava/awt/Font;)Ljava/awt/FontMetrics;
- method getFontMetrics (Ljava/awt/Font;)Ljava/awt/FontMetrics;
- method getHeight ()I
- method getHeight ()I
- method getLocation ()Ljava/awt/Point;
- method getLocation ()Ljava/awt/Point;
- method getLocationOnScreen ()Ljava/awt/Point;
- method getLocationOnScreen ()Ljava/awt/Point;
- method getMaximumSize ()Ljava/awt/Dimension;
- method getMaximumSize ()Ljava/awt/Dimension;
- method getMinimumSize ()Ljava/awt/Dimension;
- method getMinimumSize ()Ljava/awt/Dimension;
- method getMouseListeners ()[Ljava/awt/event/MouseListener;
- method getMouseListeners ()[Ljava/awt/event/MouseListener;
- method getParent ()Ljava/awt/Container;
- method getParent ()Ljava/awt/Container;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getWidth ()I
- method getWidth ()I
- method isDisplayable ()Z
- method isDisplayable ()Z
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method requestFocus ()V
- method requestFocus ()V
- method requestFocusInWindow ()Z
- method requestFocusInWindow ()Z
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setBounds (Ljava/awt/Rectangle;)V
- method setBounds (Ljava/awt/Rectangle;)V
- method setDropTarget (Ljava/awt/dnd/DropTarget;)V
- method setDropTarget (Ljava/awt/dnd/DropTarget;)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFocusTraversalKeysEnabled (Z)V
- method setFocusTraversalKeysEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setForeground (Ljava/awt/Color;)V
- method setForeground (Ljava/awt/Color;)V
- method setLocation (II)V
- method setLocation (II)V
- method setLocation (Ljava/awt/Point;)V
- method setLocation (Ljava/awt/Point;)V
- method setSize (II)V
- method setSize (II)V
- method setSize (Ljava/awt/Dimension;)V
- method setSize (Ljava/awt/Dimension;)V
- method setVisible (Z)V
- method setVisible (Z)V

## java/awt/Container
- method <init> ()V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;I)Ljava/awt/Component;
- method add (Ljava/awt/Component;I)Ljava/awt/Component;
- method add (Ljava/awt/Component;Ljava/lang/Object;)V
- method add (Ljava/awt/Component;Ljava/lang/Object;)V
- method add (Ljava/awt/Component;Ljava/lang/Object;I)V
- method add (Ljava/awt/Component;Ljava/lang/Object;I)V
- method add (Ljava/lang/String;Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/lang/String;Ljava/awt/Component;)Ljava/awt/Component;
- method getComponent (I)Ljava/awt/Component;
- method getComponent (I)Ljava/awt/Component;
- method getComponentCount ()I
- method getComponentCount ()I
- method getComponents ()[Ljava/awt/Component;
- method getComponents ()[Ljava/awt/Component;
- method getInsets ()Ljava/awt/Insets;
- method getInsets ()Ljava/awt/Insets;
- method getSize ()Ljava/awt/Dimension;
- method getSize ()Ljava/awt/Dimension;
- method getTreeLock ()Ljava/lang/Object;
- method getTreeLock ()Ljava/lang/Object;
- method isAncestorOf (Ljava/awt/Component;)Z
- method isAncestorOf (Ljava/awt/Component;)Z
- method isEnabled ()Z
- method isEnabled ()Z
- method remove (Ljava/awt/Component;)V
- method remove (Ljava/awt/Component;)V
- method repaint (JIIII)V
- method repaint (JIIII)V
- method requestFocus ()V
- method requestFocus ()V
- method setLayout (Ljava/awt/LayoutManager;)V
- method setLayout (Ljava/awt/LayoutManager;)V

## java/awt/Cursor
- method <init> ()V
- method <init> (I)V
- method getDefaultCursor ()Ljava/awt/Cursor;
- method getDefaultCursor ()Ljava/awt/Cursor;
- method getPredefinedCursor (I)Ljava/awt/Cursor;
- method getPredefinedCursor (I)Ljava/awt/Cursor;
- method getType ()I
- method getType ()I

## java/awt/Desktop
- method <init> ()V
- method browse (Ljava/net/URI;)V
- method browse (Ljava/net/URI;)V
- method getDesktop ()Ljava/awt/Desktop;
- method getDesktop ()Ljava/awt/Desktop;
- method isDesktopSupported ()Z
- method isDesktopSupported ()Z
- method isSupported (Ljava/awt/Desktop$Action;)Z
- method isSupported (Ljava/awt/Desktop$Action;)Z
- method open (Ljava/io/File;)V
- method open (Ljava/io/File;)V

## java/awt/Desktop$Action
- field OPEN Ljava/awt/Desktop$Action;
- field OPEN Ljava/awt/Desktop$Action;
- method <init> ()V

## java/awt/Dialog
- method <init> ()V
- method <init> (Ljava/awt/Frame;Ljava/lang/String;Z)V
- method dispose ()V
- method dispose ()V
- method show ()V
- method show ()V

## java/awt/Dimension
- field height I
- field height I
- field width I
- field width I
- method <init> ()V
- method <init> (II)V
- method <init> (Ljava/awt/Dimension;)V
- method getHeight ()D
- method getHeight ()D
- method getWidth ()D
- method getWidth ()D
- method setSize (Ljava/awt/Dimension;)V
- method setSize (Ljava/awt/Dimension;)V

## java/awt/DisplayMode
- method <init> ()V
- method <init> (IIII)V
- method getBitDepth ()I
- method getBitDepth ()I
- method getHeight ()I
- method getHeight ()I
- method getRefreshRate ()I
- method getRefreshRate ()I
- method getWidth ()I
- method getWidth ()I

## java/awt/EventQueue
- method <init> ()V
- method invokeLater (Ljava/lang/Runnable;)V
- method invokeLater (Ljava/lang/Runnable;)V

## java/awt/FileDialog
- method <init> ()V
- method <init> (Ljava/awt/Frame;Ljava/lang/String;I)V
- method getDirectory ()Ljava/lang/String;
- method getDirectory ()Ljava/lang/String;
- method getFile ()Ljava/lang/String;
- method getFile ()Ljava/lang/String;
- method setFile (Ljava/lang/String;)V
- method setFile (Ljava/lang/String;)V

## java/awt/FlowLayout
- method <init> ()V
- method <init> (I)V

## java/awt/Font
- method <init> ()V
- method <init> (Ljava/lang/String;II)V
- method deriveFont (F)Ljava/awt/Font;
- method deriveFont (F)Ljava/awt/Font;
- method deriveFont (I)Ljava/awt/Font;
- method deriveFont (I)Ljava/awt/Font;
- method getFamily ()Ljava/lang/String;
- method getFamily ()Ljava/lang/String;
- method getFontName ()Ljava/lang/String;
- method getFontName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getSize ()I
- method getSize ()I
- method getStringBounds (Ljava/lang/String;Ljava/awt/font/FontRenderContext;)Ljava/awt/geom/Rectangle2D;
- method getStringBounds (Ljava/lang/String;Ljava/awt/font/FontRenderContext;)Ljava/awt/geom/Rectangle2D;
- method getStyle ()I
- method getStyle ()I

## java/awt/FontMetrics
- method <init> ()V
- method charWidth (C)I
- method charWidth (C)I
- method getAscent ()I
- method getAscent ()I
- method getDescent ()I
- method getDescent ()I
- method getHeight ()I
- method getHeight ()I
- method stringWidth (Ljava/lang/String;)I
- method stringWidth (Ljava/lang/String;)I

## java/awt/Frame
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addNotify ()V
- method addNotify ()V
- method dispatchEvent (Ljava/awt/AWTEvent;)V
- method dispatchEvent (Ljava/awt/AWTEvent;)V
- method dispose ()V
- method dispose ()V
- method getGraphicsConfiguration ()Ljava/awt/GraphicsConfiguration;
- method getGraphicsConfiguration ()Ljava/awt/GraphicsConfiguration;
- method getPeer ()Ljava/awt/peer/ComponentPeer;
- method getPeer ()Ljava/awt/peer/ComponentPeer;
- method setMenuBar (Ljava/awt/MenuBar;)V
- method setMenuBar (Ljava/awt/MenuBar;)V
- method setResizable (Z)V
- method setResizable (Z)V
- method setSize (II)V
- method setSize (II)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method setUndecorated (Z)V
- method setUndecorated (Z)V
- method validate ()V
- method validate ()V

## java/awt/GradientPaint
- method <init> ()V
- method <init> (FFLjava/awt/Color;FFLjava/awt/Color;)V

## java/awt/Graphics
- method <init> ()V
- method clearRect (IIII)V
- method clearRect (IIII)V
- method clipRect (IIII)V
- method clipRect (IIII)V
- method create ()Ljava/awt/Graphics;
- method create ()Ljava/awt/Graphics;
- method dispose ()V
- method dispose ()V
- method drawArc (IIIIII)V
- method drawArc (IIIIII)V
- method drawImage (Ljava/awt/Image;IIIIIIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIIIIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method drawLine (IIII)V
- method drawLine (IIII)V
- method drawPolygon ([I[II)V
- method drawPolygon ([I[II)V
- method drawRect (IIII)V
- method drawRect (IIII)V
- method drawRoundRect (IIIIII)V
- method drawRoundRect (IIIIII)V
- method drawString (Ljava/lang/String;II)V
- method drawString (Ljava/lang/String;II)V
- method fillArc (IIIIII)V
- method fillArc (IIIIII)V
- method fillPolygon ([I[II)V
- method fillPolygon ([I[II)V
- method fillRect (IIII)V
- method fillRect (IIII)V
- method fillRoundRect (IIIIII)V
- method fillRoundRect (IIIIII)V
- method getClipBounds ()Ljava/awt/Rectangle;
- method getClipBounds ()Ljava/awt/Rectangle;
- method getColor ()Ljava/awt/Color;
- method getColor ()Ljava/awt/Color;
- method getFont ()Ljava/awt/Font;
- method getFont ()Ljava/awt/Font;
- method getFontMetrics ()Ljava/awt/FontMetrics;
- method getFontMetrics ()Ljava/awt/FontMetrics;
- method getFontMetrics (Ljava/awt/Font;)Ljava/awt/FontMetrics;
- method getFontMetrics (Ljava/awt/Font;)Ljava/awt/FontMetrics;
- method setClip (IIII)V
- method setClip (IIII)V
- method setColor (Ljava/awt/Color;)V
- method setColor (Ljava/awt/Color;)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V

## java/awt/Graphics2D
- method <init> ()V
- method clearRect (IIII)V
- method clearRect (IIII)V
- method clip (Ljava/awt/Shape;)V
- method clip (Ljava/awt/Shape;)V
- method dispose ()V
- method dispose ()V
- method draw (Ljava/awt/Shape;)V
- method draw (Ljava/awt/Shape;)V
- method drawArc (IIIIII)V
- method drawArc (IIIIII)V
- method drawImage (Ljava/awt/Image;IIIIIIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIIIIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IIIILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;Ljava/awt/geom/AffineTransform;Ljava/awt/image/ImageObserver;)Z
- method drawImage (Ljava/awt/Image;Ljava/awt/geom/AffineTransform;Ljava/awt/image/ImageObserver;)Z
- method drawLine (IIII)V
- method drawLine (IIII)V
- method drawOval (IIII)V
- method drawOval (IIII)V
- method drawPolygon ([I[II)V
- method drawPolygon ([I[II)V
- method drawRect (IIII)V
- method drawRect (IIII)V
- method drawRoundRect (IIIIII)V
- method drawRoundRect (IIIIII)V
- method drawString (Ljava/lang/String;FF)V
- method drawString (Ljava/lang/String;FF)V
- method drawString (Ljava/lang/String;II)V
- method drawString (Ljava/lang/String;II)V
- method fill (Ljava/awt/Shape;)V
- method fill (Ljava/awt/Shape;)V
- method fillArc (IIIIII)V
- method fillArc (IIIIII)V
- method fillOval (IIII)V
- method fillOval (IIII)V
- method fillPolygon ([I[II)V
- method fillPolygon ([I[II)V
- method fillRect (IIII)V
- method fillRect (IIII)V
- method fillRoundRect (IIIIII)V
- method fillRoundRect (IIIIII)V
- method getComposite ()Ljava/awt/Composite;
- method getComposite ()Ljava/awt/Composite;
- method getFontMetrics ()Ljava/awt/FontMetrics;
- method getFontMetrics ()Ljava/awt/FontMetrics;
- method getFontRenderContext ()Ljava/awt/font/FontRenderContext;
- method getFontRenderContext ()Ljava/awt/font/FontRenderContext;
- method getTransform ()Ljava/awt/geom/AffineTransform;
- method getTransform ()Ljava/awt/geom/AffineTransform;
- method rotate (D)V
- method rotate (D)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setClip (IIII)V
- method setClip (IIII)V
- method setColor (Ljava/awt/Color;)V
- method setColor (Ljava/awt/Color;)V
- method setComposite (Ljava/awt/Composite;)V
- method setComposite (Ljava/awt/Composite;)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setPaint (Ljava/awt/Paint;)V
- method setPaint (Ljava/awt/Paint;)V
- method setPaintMode ()V
- method setPaintMode ()V
- method setRenderingHint (Ljava/awt/RenderingHints$Key;Ljava/lang/Object;)V
- method setRenderingHint (Ljava/awt/RenderingHints$Key;Ljava/lang/Object;)V
- method setStroke (Ljava/awt/Stroke;)V
- method setStroke (Ljava/awt/Stroke;)V
- method setTransform (Ljava/awt/geom/AffineTransform;)V
- method setTransform (Ljava/awt/geom/AffineTransform;)V
- method setXORMode (Ljava/awt/Color;)V
- method setXORMode (Ljava/awt/Color;)V
- method transform (Ljava/awt/geom/AffineTransform;)V
- method transform (Ljava/awt/geom/AffineTransform;)V
- method translate (DD)V
- method translate (DD)V
- method translate (II)V
- method translate (II)V

## java/awt/GraphicsConfiguration
- method <init> ()V
- method getDevice ()Ljava/awt/GraphicsDevice;
- method getDevice ()Ljava/awt/GraphicsDevice;

## java/awt/GraphicsDevice
- method <init> ()V
- method getDisplayMode ()Ljava/awt/DisplayMode;
- method getDisplayMode ()Ljava/awt/DisplayMode;
- method getDisplayModes ()[Ljava/awt/DisplayMode;
- method getDisplayModes ()[Ljava/awt/DisplayMode;
- method getFullScreenWindow ()Ljava/awt/Window;
- method getFullScreenWindow ()Ljava/awt/Window;
- method setDisplayMode (Ljava/awt/DisplayMode;)V
- method setDisplayMode (Ljava/awt/DisplayMode;)V
- method setFullScreenWindow (Ljava/awt/Window;)V
- method setFullScreenWindow (Ljava/awt/Window;)V

## java/awt/GraphicsEnvironment
- method <init> ()V
- method getAvailableFontFamilyNames ()[Ljava/lang/String;
- method getAvailableFontFamilyNames ()[Ljava/lang/String;
- method getDefaultScreenDevice ()Ljava/awt/GraphicsDevice;
- method getDefaultScreenDevice ()Ljava/awt/GraphicsDevice;
- method getLocalGraphicsEnvironment ()Ljava/awt/GraphicsEnvironment;
- method getLocalGraphicsEnvironment ()Ljava/awt/GraphicsEnvironment;

## java/awt/GridBagConstraints
- field anchor I
- field anchor I
- field fill I
- field fill I
- field gridwidth I
- field gridwidth I
- field gridx I
- field gridx I
- field gridy I
- field gridy I
- field insets Ljava/awt/Insets;
- field insets Ljava/awt/Insets;
- field weightx D
- field weightx D
- field weighty D
- field weighty D
- method <init> ()V

## java/awt/GridBagLayout
- method <init> ()V
- method setConstraints (Ljava/awt/Component;Ljava/awt/GridBagConstraints;)V
- method setConstraints (Ljava/awt/Component;Ljava/awt/GridBagConstraints;)V

## java/awt/GridLayout
- method <init> ()V
- method <init> (II)V
- method setHgap (I)V
- method setHgap (I)V
- method setVgap (I)V
- method setVgap (I)V

## java/awt/Image
- method <init> ()V
- method getGraphics ()Ljava/awt/Graphics;
- method getGraphics ()Ljava/awt/Graphics;
- method getHeight (Ljava/awt/image/ImageObserver;)I
- method getHeight (Ljava/awt/image/ImageObserver;)I
- method getScaledInstance (III)Ljava/awt/Image;
- method getScaledInstance (III)Ljava/awt/Image;
- method getWidth (Ljava/awt/image/ImageObserver;)I
- method getWidth (Ljava/awt/image/ImageObserver;)I
- method setAccelerationPriority (F)V
- method setAccelerationPriority (F)V

## java/awt/Insets
- field bottom I
- field bottom I
- field left I
- field left I
- field right I
- field right I
- field top I
- field top I
- method <init> ()V
- method <init> (IIII)V
- method set (IIII)V
- method set (IIII)V

## java/awt/Label
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;I)V
- method setAlignment (I)V
- method setAlignment (I)V

## java/awt/LayoutManager
- method <init> ()V

## java/awt/LayoutManager2
- method <init> ()V

## java/awt/List
- method <init> ()V
- method add (Ljava/lang/String;)V
- method add (Ljava/lang/String;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method deselect (I)V
- method deselect (I)V
- method getItemCount ()I
- method getItemCount ()I
- method isIndexSelected (I)Z
- method isIndexSelected (I)Z
- method removeAll ()V
- method removeAll ()V
- method replaceItem (Ljava/lang/String;I)V
- method replaceItem (Ljava/lang/String;I)V
- method select (I)V
- method select (I)V
- method setMultipleMode (Z)V
- method setMultipleMode (Z)V

## java/awt/MediaTracker
- method <init> ()V
- method <init> (Ljava/awt/Component;)V
- method addImage (Ljava/awt/Image;I)V
- method addImage (Ljava/awt/Image;I)V
- method waitForAll ()V
- method waitForAll ()V
- method waitForID (I)V
- method waitForID (I)V

## java/awt/Menu
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method add (Ljava/awt/MenuItem;)Ljava/awt/MenuItem;
- method add (Ljava/awt/MenuItem;)Ljava/awt/MenuItem;
- method insert (Ljava/awt/MenuItem;I)V
- method insert (Ljava/awt/MenuItem;I)V

## java/awt/MenuBar
- method <init> ()V
- method add (Ljava/awt/Menu;)Ljava/awt/Menu;
- method add (Ljava/awt/Menu;)Ljava/awt/Menu;

## java/awt/MenuItem
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/awt/MenuShortcut;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method getLabel ()Ljava/lang/String;
- method getLabel ()Ljava/lang/String;

## java/awt/MenuShortcut
- method <init> ()V
- method <init> (I)V

## java/awt/Panel
- method <init> ()V

## java/awt/Point
- field x I
- field x I
- field y I
- field y I
- method <init> ()V
- method <init> (II)V
- method <init> (Ljava/awt/Point;)V

## java/awt/Rectangle
- field height I
- field height I
- field width I
- field width I
- field x I
- field x I
- field y I
- field y I
- method <init> ()V
- method <init> (IIII)V
- method getHeight ()D
- method getHeight ()D
- method getLocation ()Ljava/awt/Point;
- method getLocation ()Ljava/awt/Point;
- method getWidth ()D
- method getWidth ()D
- method getX ()D
- method getX ()D
- method getY ()D
- method getY ()D
- method setBounds (Ljava/awt/Rectangle;)V
- method setBounds (Ljava/awt/Rectangle;)V

## java/awt/RenderingHints
- field KEY_ANTIALIASING Ljava/awt/RenderingHints$Key;
- field KEY_ANTIALIASING Ljava/awt/RenderingHints$Key;
- field KEY_TEXT_ANTIALIASING Ljava/awt/RenderingHints$Key;
- field KEY_TEXT_ANTIALIASING Ljava/awt/RenderingHints$Key;
- field VALUE_ANTIALIAS_OFF Ljava/lang/Object;
- field VALUE_ANTIALIAS_OFF Ljava/lang/Object;
- field VALUE_ANTIALIAS_ON Ljava/lang/Object;
- field VALUE_ANTIALIAS_ON Ljava/lang/Object;
- field VALUE_TEXT_ANTIALIAS_ON Ljava/lang/Object;
- field VALUE_TEXT_ANTIALIAS_ON Ljava/lang/Object;
- method <init> ()V

## java/awt/RenderingHints$Key
- method <init> ()V

## java/awt/ScrollPane
- method <init> ()V
- method getHAdjustable ()Ljava/awt/Adjustable;
- method getHAdjustable ()Ljava/awt/Adjustable;
- method getScrollPosition ()Ljava/awt/Point;
- method getScrollPosition ()Ljava/awt/Point;
- method getVAdjustable ()Ljava/awt/Adjustable;
- method getVAdjustable ()Ljava/awt/Adjustable;
- method getVScrollbarWidth ()I
- method getVScrollbarWidth ()I
- method getViewportSize ()Ljava/awt/Dimension;
- method getViewportSize ()Ljava/awt/Dimension;
- method setScrollPosition (Ljava/awt/Point;)V
- method setScrollPosition (Ljava/awt/Point;)V

## java/awt/Shape
- method <init> ()V
- method getBounds ()Ljava/awt/Rectangle;
- method getBounds ()Ljava/awt/Rectangle;
- method getBounds2D ()Ljava/awt/geom/Rectangle2D;
- method getBounds2D ()Ljava/awt/geom/Rectangle2D;

## java/awt/SystemColor
- field activeCaption Ljava/awt/SystemColor;
- field activeCaption Ljava/awt/SystemColor;
- field activeCaptionText Ljava/awt/SystemColor;
- field activeCaptionText Ljava/awt/SystemColor;
- method <init> ()V

## java/awt/TextArea
- method <init> ()V
- method <init> (Ljava/lang/String;III)V
- method append (Ljava/lang/String;)V
- method append (Ljava/lang/String;)V

## java/awt/TextComponent
- method <init> ()V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method setEditable (Z)V
- method setEditable (Z)V

## java/awt/TextField
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## java/awt/TexturePaint
- method <init> ()V
- method <init> (Ljava/awt/image/BufferedImage;Ljava/awt/geom/Rectangle2D;)V

## java/awt/Toolkit
- method <init> ()V
- method beep ()V
- method beep ()V
- method createImage (Ljava/awt/image/ImageProducer;)Ljava/awt/Image;
- method createImage (Ljava/awt/image/ImageProducer;)Ljava/awt/Image;
- method createImage (Ljava/net/URL;)Ljava/awt/Image;
- method createImage (Ljava/net/URL;)Ljava/awt/Image;
- method createImage ([B)Ljava/awt/Image;
- method createImage ([B)Ljava/awt/Image;
- method getDefaultToolkit ()Ljava/awt/Toolkit;
- method getDefaultToolkit ()Ljava/awt/Toolkit;
- method getImage (Ljava/lang/String;)Ljava/awt/Image;
- method getImage (Ljava/lang/String;)Ljava/awt/Image;
- method getImage (Ljava/net/URL;)Ljava/awt/Image;
- method getImage (Ljava/net/URL;)Ljava/awt/Image;
- method getScreenResolution ()I
- method getScreenResolution ()I
- method getScreenSize ()Ljava/awt/Dimension;
- method getScreenSize ()Ljava/awt/Dimension;
- method getSystemClipboard ()Ljava/awt/datatransfer/Clipboard;
- method getSystemClipboard ()Ljava/awt/datatransfer/Clipboard;
- method prepareImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method prepareImage (Ljava/awt/Image;IILjava/awt/image/ImageObserver;)Z
- method sync ()V
- method sync ()V

## java/awt/Window
- method <init> ()V
- method addWindowListener (Ljava/awt/event/WindowListener;)V
- method addWindowListener (Ljava/awt/event/WindowListener;)V
- method dispose ()V
- method dispose ()V
- method pack ()V
- method pack ()V
- method show ()V
- method show ()V

## java/awt/Window$Type
- field UTILITY Ljava/awt/Window$Type;
- field UTILITY Ljava/awt/Window$Type;
- method <init> ()V

## java/awt/datatransfer/Clipboard
- method <init> ()V
- method getContents (Ljava/lang/Object;)Ljava/awt/datatransfer/Transferable;
- method getContents (Ljava/lang/Object;)Ljava/awt/datatransfer/Transferable;
- method setContents (Ljava/awt/datatransfer/Transferable;Ljava/awt/datatransfer/ClipboardOwner;)V
- method setContents (Ljava/awt/datatransfer/Transferable;Ljava/awt/datatransfer/ClipboardOwner;)V

## java/awt/datatransfer/DataFlavor
- field imageFlavor Ljava/awt/datatransfer/DataFlavor;
- field imageFlavor Ljava/awt/datatransfer/DataFlavor;
- field javaFileListFlavor Ljava/awt/datatransfer/DataFlavor;
- field javaFileListFlavor Ljava/awt/datatransfer/DataFlavor;
- field stringFlavor Ljava/awt/datatransfer/DataFlavor;
- field stringFlavor Ljava/awt/datatransfer/DataFlavor;
- method <init> ()V
- method <init> (Ljava/lang/Class;Ljava/lang/String;)V
- method equals (Ljava/awt/datatransfer/DataFlavor;)Z
- method equals (Ljava/awt/datatransfer/DataFlavor;)Z
- method getReaderForText (Ljava/awt/datatransfer/Transferable;)Ljava/io/Reader;
- method getReaderForText (Ljava/awt/datatransfer/Transferable;)Ljava/io/Reader;
- method isRepresentationClassReader ()Z
- method isRepresentationClassReader ()Z

## java/awt/datatransfer/StringSelection
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/awt/datatransfer/Transferable
- method <init> ()V
- method getTransferData (Ljava/awt/datatransfer/DataFlavor;)Ljava/lang/Object;
- method getTransferData (Ljava/awt/datatransfer/DataFlavor;)Ljava/lang/Object;
- method getTransferDataFlavors ()[Ljava/awt/datatransfer/DataFlavor;
- method getTransferDataFlavors ()[Ljava/awt/datatransfer/DataFlavor;
- method isDataFlavorSupported (Ljava/awt/datatransfer/DataFlavor;)Z
- method isDataFlavorSupported (Ljava/awt/datatransfer/DataFlavor;)Z

## java/awt/datatransfer/UnsupportedFlavorException
- method <init> ()V
- method <init> (Ljava/awt/datatransfer/DataFlavor;)V

## java/awt/dnd/DropTarget
- method <init> ()V
- method <init> (Ljava/awt/Component;Ljava/awt/dnd/DropTargetListener;)V
- method addDropTargetListener (Ljava/awt/dnd/DropTargetListener;)V
- method addDropTargetListener (Ljava/awt/dnd/DropTargetListener;)V

## java/awt/dnd/DropTargetContext
- method <init> ()V
- method dropComplete (Z)V
- method dropComplete (Z)V

## java/awt/dnd/DropTargetDragEvent
- method <init> ()V
- method acceptDrag (I)V
- method acceptDrag (I)V
- method getCurrentDataFlavors ()[Ljava/awt/datatransfer/DataFlavor;
- method getCurrentDataFlavors ()[Ljava/awt/datatransfer/DataFlavor;
- method rejectDrag ()V
- method rejectDrag ()V

## java/awt/dnd/DropTargetDropEvent
- method <init> ()V
- method acceptDrop (I)V
- method acceptDrop (I)V
- method getDropTargetContext ()Ljava/awt/dnd/DropTargetContext;
- method getDropTargetContext ()Ljava/awt/dnd/DropTargetContext;
- method getTransferable ()Ljava/awt/datatransfer/Transferable;
- method getTransferable ()Ljava/awt/datatransfer/Transferable;
- method rejectDrop ()V
- method rejectDrop ()V

## java/awt/dnd/DropTargetListener
- method <init> ()V

## java/awt/event/ActionEvent
- method <init> ()V
- method <init> (Ljava/lang/Object;ILjava/lang/String;)V
- method getActionCommand ()Ljava/lang/String;
- method getActionCommand ()Ljava/lang/String;
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;

## java/awt/event/ActionListener
- method <init> ()V

## java/awt/event/AdjustmentEvent
- method <init> ()V
- method getValue ()I
- method getValue ()I

## java/awt/event/AdjustmentListener
- method <init> ()V

## java/awt/event/ComponentAdapter
- method <init> ()V

## java/awt/event/ComponentEvent
- method <init> ()V
- method getComponent ()Ljava/awt/Component;
- method getComponent ()Ljava/awt/Component;

## java/awt/event/ComponentListener
- method <init> ()V

## java/awt/event/FocusAdapter
- method <init> ()V

## java/awt/event/FocusEvent
- method <init> ()V
- method <init> (Ljava/awt/Component;I)V
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;

## java/awt/event/FocusListener
- method <init> ()V

## java/awt/event/HierarchyListener
- method <init> ()V

## java/awt/event/ItemEvent
- method <init> ()V
- method getItem ()Ljava/lang/Object;
- method getItem ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;
- method getStateChange ()I
- method getStateChange ()I

## java/awt/event/ItemListener
- method <init> ()V

## java/awt/event/KeyAdapter
- method <init> ()V

## java/awt/event/KeyEvent
- method <init> ()V
- method consume ()V
- method consume ()V
- method getKeyChar ()C
- method getKeyChar ()C
- method getKeyCode ()I
- method getKeyCode ()I
- method getKeyText (I)Ljava/lang/String;
- method getKeyText (I)Ljava/lang/String;

## java/awt/event/KeyListener
- method <init> ()V

## java/awt/event/MouseAdapter
- method <init> ()V

## java/awt/event/MouseEvent
- method <init> ()V
- method consume ()V
- method consume ()V
- method getButton ()I
- method getButton ()I
- method getClickCount ()I
- method getClickCount ()I
- method getComponent ()Ljava/awt/Component;
- method getComponent ()Ljava/awt/Component;
- method getID ()I
- method getID ()I
- method getModifiers ()I
- method getModifiers ()I
- method getPoint ()Ljava/awt/Point;
- method getPoint ()Ljava/awt/Point;
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;
- method getWhen ()J
- method getWhen ()J
- method getX ()I
- method getX ()I
- method getY ()I
- method getY ()I
- method isControlDown ()Z
- method isControlDown ()Z
- method isMetaDown ()Z
- method isMetaDown ()Z
- method isShiftDown ()Z
- method isShiftDown ()Z

## java/awt/event/MouseListener
- method <init> ()V

## java/awt/event/MouseMotionListener
- method <init> ()V

## java/awt/event/MouseWheelListener
- method <init> ()V

## java/awt/event/WindowAdapter
- method <init> ()V

## java/awt/event/WindowEvent
- method <init> ()V
- method <init> (Ljava/awt/Window;I)V
- method getNewState ()I
- method getNewState ()I
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;

## java/awt/event/WindowListener
- method <init> ()V

## java/awt/event/WindowStateListener
- method <init> ()V

## java/awt/font/FontRenderContext
- method <init> ()V
- method <init> (Ljava/awt/geom/AffineTransform;ZZ)V

## java/awt/font/LineBreakMeasurer
- method <init> ()V
- method <init> (Ljava/text/AttributedCharacterIterator;Ljava/awt/font/FontRenderContext;)V
- method getPosition ()I
- method getPosition ()I
- method nextLayout (FIZ)Ljava/awt/font/TextLayout;
- method nextLayout (FIZ)Ljava/awt/font/TextLayout;
- method setPosition (I)V
- method setPosition (I)V

## java/awt/font/TextAttribute
- field FONT Ljava/awt/font/TextAttribute;
- field FONT Ljava/awt/font/TextAttribute;
- method <init> ()V

## java/awt/font/TextLayout
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/awt/Font;Ljava/awt/font/FontRenderContext;)V
- method draw (Ljava/awt/Graphics2D;FF)V
- method draw (Ljava/awt/Graphics2D;FF)V
- method getAscent ()F
- method getAscent ()F
- method getBounds ()Ljava/awt/geom/Rectangle2D;
- method getBounds ()Ljava/awt/geom/Rectangle2D;
- method getDescent ()F
- method getDescent ()F
- method getLeading ()F
- method getLeading ()F
- method getOutline (Ljava/awt/geom/AffineTransform;)Ljava/awt/Shape;
- method getOutline (Ljava/awt/geom/AffineTransform;)Ljava/awt/Shape;

## java/awt/geom/AffineTransform
- method <init> ()V
- method <init> ([D)V
- method concatenate (Ljava/awt/geom/AffineTransform;)V
- method concatenate (Ljava/awt/geom/AffineTransform;)V
- method createTransformedShape (Ljava/awt/Shape;)Ljava/awt/Shape;
- method createTransformedShape (Ljava/awt/Shape;)Ljava/awt/Shape;
- method getTranslateInstance (DD)Ljava/awt/geom/AffineTransform;
- method getTranslateInstance (DD)Ljava/awt/geom/AffineTransform;
- method rotate (D)V
- method rotate (D)V
- method rotate (DDD)V
- method rotate (DDD)V
- method scale (DD)V
- method scale (DD)V
- method transform (Ljava/awt/geom/Point2D;Ljava/awt/geom/Point2D;)Ljava/awt/geom/Point2D;
- method transform (Ljava/awt/geom/Point2D;Ljava/awt/geom/Point2D;)Ljava/awt/geom/Point2D;
- method transform ([FI[FII)V
- method transform ([FI[FII)V
- method translate (DD)V
- method translate (DD)V

## java/awt/geom/Arc2D
- method <init> ()V

## java/awt/geom/Arc2D$Double
- method <init> ()V
- method setArc (DDDDDDI)V
- method setArc (DDDDDDI)V

## java/awt/geom/GeneralPath
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/awt/Shape;)V
- method closePath ()V
- method closePath ()V
- method curveTo (FFFFFF)V
- method curveTo (FFFFFF)V
- method lineTo (FF)V
- method lineTo (FF)V
- method moveTo (FF)V
- method moveTo (FF)V
- method quadTo (FFFF)V
- method quadTo (FFFF)V

## java/awt/geom/Point2D
- method <init> ()V
- method getX ()D
- method getX ()D
- method getY ()D
- method getY ()D

## java/awt/geom/Point2D$Double
- method <init> ()V
- method <init> (DD)V

## java/awt/geom/Rectangle2D
- method <init> ()V
- method getHeight ()D
- method getHeight ()D
- method getMaxX ()D
- method getMaxX ()D
- method getWidth ()D
- method getWidth ()D
- method getX ()D
- method getX ()D
- method getY ()D
- method getY ()D
- method intersects (DDDD)Z
- method intersects (DDDD)Z
- method setFrame (DDDD)V
- method setFrame (DDDD)V

## java/awt/image/BufferStrategy
- method <init> ()V
- method getDrawGraphics ()Ljava/awt/Graphics;
- method getDrawGraphics ()Ljava/awt/Graphics;
- method show ()V
- method show ()V

## java/awt/image/BufferedImage
- method <init> ()V
- method <init> (III)V
- method <init> (Ljava/awt/image/ColorModel;Ljava/awt/image/WritableRaster;ZLjava/util/Hashtable;)V
- method createGraphics ()Ljava/awt/Graphics2D;
- method createGraphics ()Ljava/awt/Graphics2D;
- method getColorModel ()Ljava/awt/image/ColorModel;
- method getColorModel ()Ljava/awt/image/ColorModel;
- method getGraphics ()Ljava/awt/Graphics;
- method getGraphics ()Ljava/awt/Graphics;
- method getHeight ()I
- method getHeight ()I
- method getRGB (II)I
- method getRGB (II)I
- method getRaster ()Ljava/awt/image/WritableRaster;
- method getRaster ()Ljava/awt/image/WritableRaster;
- method getType ()I
- method getType ()I
- method getWidth ()I
- method getWidth ()I

## java/awt/image/ColorModel
- method <init> ()V
- method createCompatibleWritableRaster (II)Ljava/awt/image/WritableRaster;
- method createCompatibleWritableRaster (II)Ljava/awt/image/WritableRaster;
- method getPixelSize ()I
- method getPixelSize ()I
- method hasAlpha ()Z
- method hasAlpha ()Z

## java/awt/image/DataBufferInt
- method <init> ()V
- method getData ()[I
- method getData ()[I

## java/awt/image/DirectColorModel
- method <init> ()V
- method <init> (IIII)V
- method <init> (IIIII)V
- method getBlueMask ()I
- method getBlueMask ()I
- method getGreenMask ()I
- method getGreenMask ()I
- method getRedMask ()I
- method getRedMask ()I

## java/awt/image/ImageConsumer
- method <init> ()V
- method imageComplete (I)V
- method imageComplete (I)V
- method setColorModel (Ljava/awt/image/ColorModel;)V
- method setColorModel (Ljava/awt/image/ColorModel;)V
- method setDimensions (II)V
- method setDimensions (II)V
- method setHints (I)V
- method setHints (I)V
- method setPixels (IIIILjava/awt/image/ColorModel;[III)V
- method setPixels (IIIILjava/awt/image/ColorModel;[III)V

## java/awt/image/ImageObserver
- method <init> ()V

## java/awt/image/ImageProducer
- method <init> ()V

## java/awt/image/IndexColorModel
- method <init> ()V
- method <init> (II[B[B[B)V
- method <init> (II[B[B[BI)V
- method getBlues ([B)V
- method getBlues ([B)V
- method getGreens ([B)V
- method getGreens ([B)V
- method getMapSize ()I
- method getMapSize ()I
- method getReds ([B)V
- method getReds ([B)V
- method getTransparentPixel ()I
- method getTransparentPixel ()I

## java/awt/image/PixelGrabber
- method <init> ()V
- method <init> (Ljava/awt/Image;IIIIZ)V
- method getHeight ()I
- method getHeight ()I
- method getPixels ()Ljava/lang/Object;
- method getPixels ()Ljava/lang/Object;
- method getWidth ()I
- method getWidth ()I
- method grabPixels ()Z
- method grabPixels ()Z

## java/awt/image/Raster
- method <init> ()V
- method getDataBuffer ()Ljava/awt/image/DataBuffer;
- method getDataBuffer ()Ljava/awt/image/DataBuffer;
- method getPixel (II[I)[I
- method getPixel (II[I)[I

## java/awt/image/VolatileImage
- method <init> ()V
- method getGraphics ()Ljava/awt/Graphics;
- method getGraphics ()Ljava/awt/Graphics;
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I
- method validate (Ljava/awt/GraphicsConfiguration;)I
- method validate (Ljava/awt/GraphicsConfiguration;)I

## java/awt/image/WritableRaster
- method <init> ()V
- method setPixel (II[I)V
- method setPixel (II[I)V
- method setPixels (IIII[I)V
- method setPixels (IIII[I)V

## java/awt/print/PageFormat
- method <init> ()V
- method getHeight ()D
- method getHeight ()D
- method getImageableHeight ()D
- method getImageableHeight ()D
- method getImageableWidth ()D
- method getImageableWidth ()D
- method getImageableX ()D
- method getImageableX ()D
- method getImageableY ()D
- method getImageableY ()D
- method getOrientation ()I
- method getOrientation ()I
- method getPaper ()Ljava/awt/print/Paper;
- method getPaper ()Ljava/awt/print/Paper;
- method getWidth ()D
- method getWidth ()D
- method setOrientation (I)V
- method setOrientation (I)V
- method setPaper (Ljava/awt/print/Paper;)V
- method setPaper (Ljava/awt/print/Paper;)V

## java/awt/print/Paper
- method <init> ()V
- method getHeight ()D
- method getHeight ()D
- method getImageableHeight ()D
- method getImageableHeight ()D
- method getImageableWidth ()D
- method getImageableWidth ()D
- method getImageableX ()D
- method getImageableX ()D
- method getImageableY ()D
- method getImageableY ()D
- method getWidth ()D
- method getWidth ()D
- method setImageableArea (DDDD)V
- method setImageableArea (DDDD)V
- method setSize (DD)V
- method setSize (DD)V

## java/awt/print/Printable
- method <init> ()V

## java/awt/print/PrinterException
- method <init> ()V

## java/awt/print/PrinterJob
- method <init> ()V
- method getCopies ()I
- method getCopies ()I
- method getPrinterJob ()Ljava/awt/print/PrinterJob;
- method getPrinterJob ()Ljava/awt/print/PrinterJob;
- method pageDialog (Ljava/awt/print/PageFormat;)Ljava/awt/print/PageFormat;
- method pageDialog (Ljava/awt/print/PageFormat;)Ljava/awt/print/PageFormat;
- method print ()V
- method print ()V
- method printDialog ()Z
- method printDialog ()Z
- method setJobName (Ljava/lang/String;)V
- method setJobName (Ljava/lang/String;)V
- method setPrintable (Ljava/awt/print/Printable;Ljava/awt/print/PageFormat;)V
- method setPrintable (Ljava/awt/print/Printable;Ljava/awt/print/PageFormat;)V

## java/beans/PropertyChangeEvent
- method <init> ()V
- method getNewValue ()Ljava/lang/Object;
- method getNewValue ()Ljava/lang/Object;
- method getPropertyName ()Ljava/lang/String;
- method getPropertyName ()Ljava/lang/String;

## java/beans/PropertyChangeListener
- method <init> ()V

## java/beans/PropertyVetoException
- method <init> ()V

## java/io/BufferedOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;)V
- method <init> (Ljava/io/OutputStream;I)V
- method close ()V
- method close ()V
- method flush ()V
- method flush ()V
- method write ([BII)V
- method write ([BII)V

## java/io/CharArrayWriter
- method <init> ()V
- method toCharArray ()[C
- method toCharArray ()[C

## java/io/Closeable
- method <init> ()V
- method close ()V
- method close ()V

## java/io/FileFilter
- method <init> ()V
- method accept (Ljava/io/File;)Z
- method accept (Ljava/io/File;)Z

## java/io/FileReader
- method <init> ()V
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/lang/String;)V

## java/io/FileWriter
- method <init> ()V
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/lang/String;)V
- method close ()V
- method close ()V
- method write (Ljava/lang/String;)V
- method write (Ljava/lang/String;)V

## java/io/FilenameFilter
- method <init> ()V
- method accept (Ljava/io/File;Ljava/lang/String;)Z
- method accept (Ljava/io/File;Ljava/lang/String;)Z

## java/io/FilterReader
- method <init> ()V
- method <init> (Ljava/io/Reader;)V

## java/io/FilterWriter
- method <init> ()V
- method <init> (Ljava/io/Writer;)V

## java/io/Flushable
- method <init> ()V

## java/io/ObjectInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V
- method close ()V
- method close ()V
- method defaultReadObject ()V
- method defaultReadObject ()V
- method readObject ()Ljava/lang/Object;
- method readObject ()Ljava/lang/Object;
- method resolveClass (Ljava/io/ObjectStreamClass;)Ljava/lang/Class;
- method resolveClass (Ljava/io/ObjectStreamClass;)Ljava/lang/Class;
- method resolveProxyClass ([Ljava/lang/String;)Ljava/lang/Class;
- method resolveProxyClass ([Ljava/lang/String;)Ljava/lang/Class;

## java/io/ObjectOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;)V
- method close ()V
- method close ()V
- method defaultWriteObject ()V
- method defaultWriteObject ()V
- method writeObject (Ljava/lang/Object;)V
- method writeObject (Ljava/lang/Object;)V

## java/io/ObjectStreamClass
- method <init> ()V
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;

## java/io/ObjectStreamException
- method <init> ()V

## java/io/PushbackReader
- method <init> ()V
- method <init> (Ljava/io/Reader;I)V
- method close ()V
- method close ()V
- method read ()I
- method read ()I
- method ready ()Z
- method ready ()Z
- method unread (I)V
- method unread (I)V

## java/io/SequenceInputStream
- method <init> ()V
- method <init> (Ljava/util/Enumeration;)V

## java/io/StreamCorruptedException
- method <init> ()V

## java/io/StringReader
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method read ()I
- method read ()I
- method ready ()Z
- method ready ()Z

## java/lang/Appendable
- method <init> ()V
- method append (C)Ljava/lang/Appendable;
- method append (C)Ljava/lang/Appendable;
- method append (Ljava/lang/CharSequence;II)Ljava/lang/Appendable;
- method append (Ljava/lang/CharSequence;II)Ljava/lang/Appendable;

## java/lang/AssertionError
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/lang/Object;)V

## java/lang/CharSequence
- method <init> ()V
- method charAt (I)C
- method charAt (I)C
- method length ()I
- method length ()I
- method subSequence (II)Ljava/lang/CharSequence;
- method subSequence (II)Ljava/lang/CharSequence;

## java/lang/Enum
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method name ()Ljava/lang/String;
- method name ()Ljava/lang/String;
- method ordinal ()I
- method ordinal ()I
- method valueOf (Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;
- method valueOf (Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;

## java/lang/IllegalThreadStateException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/InheritableThreadLocal
- method <init> ()V
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;
- method set (Ljava/lang/Object;)V
- method set (Ljava/lang/Object;)V

## java/lang/Iterable
- method <init> ()V
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;

## java/lang/NoSuchFieldException
- method <init> ()V

## java/lang/NoSuchMethodException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## java/lang/Package
- method <init> ()V
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;

## java/lang/Process
- method <init> ()V
- method destroy ()V
- method destroy ()V
- method exitValue ()I
- method exitValue ()I
- method getErrorStream ()Ljava/io/InputStream;
- method getErrorStream ()Ljava/io/InputStream;
- method getInputStream ()Ljava/io/InputStream;
- method getInputStream ()Ljava/io/InputStream;
- method getOutputStream ()Ljava/io/OutputStream;
- method getOutputStream ()Ljava/io/OutputStream;
- method waitFor ()I
- method waitFor ()I

## java/lang/ProcessBuilder
- method <init> ()V
- method <init> (Ljava/util/List;)V
- method directory (Ljava/io/File;)Ljava/lang/ProcessBuilder;
- method directory (Ljava/io/File;)Ljava/lang/ProcessBuilder;
- method redirectErrorStream (Z)Ljava/lang/ProcessBuilder;
- method redirectErrorStream (Z)Ljava/lang/ProcessBuilder;
- method start ()Ljava/lang/Process;
- method start ()Ljava/lang/Process;

## java/lang/StackOverflowError
- method <init> ()V

## java/lang/StackTraceElement
- method <init> ()V
- method getClassName ()Ljava/lang/String;
- method getClassName ()Ljava/lang/String;

## java/lang/StringBuilder
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/lang/CharSequence;)V
- method <init> (Ljava/lang/String;)V
- method append (C)Ljava/lang/StringBuilder;
- method append (C)Ljava/lang/StringBuilder;
- method append (D)Ljava/lang/StringBuilder;
- method append (D)Ljava/lang/StringBuilder;
- method append (F)Ljava/lang/StringBuilder;
- method append (F)Ljava/lang/StringBuilder;
- method append (I)Ljava/lang/StringBuilder;
- method append (I)Ljava/lang/StringBuilder;
- method append (J)Ljava/lang/StringBuilder;
- method append (J)Ljava/lang/StringBuilder;
- method append (Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
- method append (Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
- method append (Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;
- method append (Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;
- method append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
- method append (Ljava/lang/Object;)Ljava/lang/StringBuilder;
- method append (Ljava/lang/String;)Ljava/lang/StringBuilder;
- method append (Ljava/lang/String;)Ljava/lang/StringBuilder;
- method append (Z)Ljava/lang/StringBuilder;
- method append (Z)Ljava/lang/StringBuilder;
- method append ([CII)Ljava/lang/StringBuilder;
- method append ([CII)Ljava/lang/StringBuilder;
- method charAt (I)C
- method charAt (I)C
- method delete (II)Ljava/lang/StringBuilder;
- method delete (II)Ljava/lang/StringBuilder;
- method deleteCharAt (I)Ljava/lang/StringBuilder;
- method deleteCharAt (I)Ljava/lang/StringBuilder;
- method getChars (II[CI)V
- method getChars (II[CI)V
- method insert (IC)Ljava/lang/StringBuilder;
- method insert (IC)Ljava/lang/StringBuilder;
- method length ()I
- method length ()I
- method replace (IILjava/lang/String;)Ljava/lang/StringBuilder;
- method replace (IILjava/lang/String;)Ljava/lang/StringBuilder;
- method reverse ()Ljava/lang/StringBuilder;
- method reverse ()Ljava/lang/StringBuilder;
- method setLength (I)V
- method setLength (I)V
- method substring (I)Ljava/lang/String;
- method substring (I)Ljava/lang/String;

## java/lang/Thread$UncaughtExceptionHandler
- method <init> ()V

## java/lang/ThreadLocal
- method <init> ()V
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;
- method remove ()V
- method remove ()V
- method set (Ljava/lang/Object;)V
- method set (Ljava/lang/Object;)V

## java/lang/Void
- field TYPE Ljava/lang/Class;
- field TYPE Ljava/lang/Class;
- method <init> ()V

## java/lang/annotation/Annotation
- method <init> ()V
- method annotationType ()Ljava/lang/Class;
- method annotationType ()Ljava/lang/Class;

## java/lang/management/ManagementFactory
- method <init> ()V
- method getOperatingSystemMXBean ()Ljava/lang/management/OperatingSystemMXBean;
- method getOperatingSystemMXBean ()Ljava/lang/management/OperatingSystemMXBean;

## java/lang/ref/PhantomReference
- method <init> ()V
- method <init> (Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V

## java/lang/ref/Reference
- method <init> ()V
- method clear ()V
- method clear ()V
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;

## java/lang/ref/ReferenceQueue
- method <init> ()V
- method poll ()Ljava/lang/ref/Reference;
- method poll ()Ljava/lang/ref/Reference;
- method remove ()Ljava/lang/ref/Reference;
- method remove ()Ljava/lang/ref/Reference;

## java/lang/ref/WeakReference
- method <init> ()V
- method <init> (Ljava/lang/Object;)V
- method <init> (Ljava/lang/Object;Ljava/lang/ref/ReferenceQueue;)V
- method clear ()V
- method clear ()V
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;

## java/lang/reflect/AccessibleObject
- method <init> ()V
- method isAccessible ()Z
- method isAccessible ()Z
- method setAccessible (Z)V
- method setAccessible (Z)V
- method setAccessible ([Ljava/lang/reflect/AccessibleObject;Z)V
- method setAccessible ([Ljava/lang/reflect/AccessibleObject;Z)V

## java/lang/reflect/Constructor
- method <init> ()V
- method getDeclaringClass ()Ljava/lang/Class;
- method getDeclaringClass ()Ljava/lang/Class;
- method getParameterTypes ()[Ljava/lang/Class;
- method getParameterTypes ()[Ljava/lang/Class;
- method isAccessible ()Z
- method isAccessible ()Z
- method newInstance ([Ljava/lang/Object;)Ljava/lang/Object;
- method newInstance ([Ljava/lang/Object;)Ljava/lang/Object;
- method setAccessible (Z)V
- method setAccessible (Z)V

## java/lang/reflect/Field
- method <init> ()V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method getAnnotation (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
- method getAnnotation (Ljava/lang/Class;)Ljava/lang/annotation/Annotation;
- method getAnnotations ()[Ljava/lang/annotation/Annotation;
- method getAnnotations ()[Ljava/lang/annotation/Annotation;
- method getBoolean (Ljava/lang/Object;)Z
- method getBoolean (Ljava/lang/Object;)Z
- method getByte (Ljava/lang/Object;)B
- method getByte (Ljava/lang/Object;)B
- method getChar (Ljava/lang/Object;)C
- method getChar (Ljava/lang/Object;)C
- method getDeclaringClass ()Ljava/lang/Class;
- method getDeclaringClass ()Ljava/lang/Class;
- method getDouble (Ljava/lang/Object;)D
- method getDouble (Ljava/lang/Object;)D
- method getFloat (Ljava/lang/Object;)F
- method getFloat (Ljava/lang/Object;)F
- method getGenericType ()Ljava/lang/reflect/Type;
- method getGenericType ()Ljava/lang/reflect/Type;
- method getInt (Ljava/lang/Object;)I
- method getInt (Ljava/lang/Object;)I
- method getLong (Ljava/lang/Object;)J
- method getLong (Ljava/lang/Object;)J
- method getModifiers ()I
- method getModifiers ()I
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getShort (Ljava/lang/Object;)S
- method getShort (Ljava/lang/Object;)S
- method getType ()Ljava/lang/Class;
- method getType ()Ljava/lang/Class;
- method isAccessible ()Z
- method isAccessible ()Z
- method isSynthetic ()Z
- method isSynthetic ()Z
- method set (Ljava/lang/Object;Ljava/lang/Object;)V
- method set (Ljava/lang/Object;Ljava/lang/Object;)V
- method setAccessible (Z)V
- method setAccessible (Z)V
- method setBoolean (Ljava/lang/Object;Z)V
- method setBoolean (Ljava/lang/Object;Z)V
- method setByte (Ljava/lang/Object;B)V
- method setByte (Ljava/lang/Object;B)V
- method setChar (Ljava/lang/Object;C)V
- method setChar (Ljava/lang/Object;C)V
- method setDouble (Ljava/lang/Object;D)V
- method setDouble (Ljava/lang/Object;D)V
- method setFloat (Ljava/lang/Object;F)V
- method setFloat (Ljava/lang/Object;F)V
- method setInt (Ljava/lang/Object;I)V
- method setInt (Ljava/lang/Object;I)V
- method setLong (Ljava/lang/Object;J)V
- method setLong (Ljava/lang/Object;J)V
- method setShort (Ljava/lang/Object;S)V
- method setShort (Ljava/lang/Object;S)V

## java/lang/reflect/GenericArrayType
- method <init> ()V
- method getGenericComponentType ()Ljava/lang/reflect/Type;
- method getGenericComponentType ()Ljava/lang/reflect/Type;

## java/lang/reflect/InvocationHandler
- method <init> ()V

## java/lang/reflect/InvocationTargetException
- method <init> ()V
- method getTargetException ()Ljava/lang/Throwable;
- method getTargetException ()Ljava/lang/Throwable;

## java/lang/reflect/Member
- method <init> ()V
- method getDeclaringClass ()Ljava/lang/Class;
- method getDeclaringClass ()Ljava/lang/Class;
- method getModifiers ()I
- method getModifiers ()I
- method isSynthetic ()Z
- method isSynthetic ()Z

## java/lang/reflect/Method
- method <init> ()V
- method getDeclaringClass ()Ljava/lang/Class;
- method getDeclaringClass ()Ljava/lang/Class;
- method getModifiers ()I
- method getModifiers ()I
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getParameterTypes ()[Ljava/lang/Class;
- method getParameterTypes ()[Ljava/lang/Class;
- method getReturnType ()Ljava/lang/Class;
- method getReturnType ()Ljava/lang/Class;
- method invoke (Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;
- method invoke (Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;
- method setAccessible (Z)V
- method setAccessible (Z)V

## java/lang/reflect/Modifier
- method <init> ()V
- method isFinal (I)Z
- method isFinal (I)Z
- method isPublic (I)Z
- method isPublic (I)Z
- method isStatic (I)Z
- method isStatic (I)Z
- method isTransient (I)Z
- method isTransient (I)Z

## java/lang/reflect/ParameterizedType
- method <init> ()V
- method getActualTypeArguments ()[Ljava/lang/reflect/Type;
- method getActualTypeArguments ()[Ljava/lang/reflect/Type;
- method getOwnerType ()Ljava/lang/reflect/Type;
- method getOwnerType ()Ljava/lang/reflect/Type;
- method getRawType ()Ljava/lang/reflect/Type;
- method getRawType ()Ljava/lang/reflect/Type;

## java/lang/reflect/Proxy
- method <init> ()V
- method getProxyClass (Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;
- method getProxyClass (Ljava/lang/ClassLoader;[Ljava/lang/Class;)Ljava/lang/Class;
- method newProxyInstance (Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;
- method newProxyInstance (Ljava/lang/ClassLoader;[Ljava/lang/Class;Ljava/lang/reflect/InvocationHandler;)Ljava/lang/Object;

## java/lang/reflect/Type
- method <init> ()V

## java/lang/reflect/TypeVariable
- method <init> ()V
- method getBounds ()[Ljava/lang/reflect/Type;
- method getBounds ()[Ljava/lang/reflect/Type;
- method getGenericDeclaration ()Ljava/lang/reflect/GenericDeclaration;
- method getGenericDeclaration ()Ljava/lang/reflect/GenericDeclaration;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;

## java/lang/reflect/WildcardType
- method <init> ()V
- method getLowerBounds ()[Ljava/lang/reflect/Type;
- method getLowerBounds ()[Ljava/lang/reflect/Type;
- method getUpperBounds ()[Ljava/lang/reflect/Type;
- method getUpperBounds ()[Ljava/lang/reflect/Type;

## java/math/BigDecimal
- method <init> ()V
- method <init> (D)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/math/BigInteger;)V
- method <init> (Ljava/math/BigInteger;I)V
- method divide (Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
- method divide (Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
- method intValue ()I
- method intValue ()I
- method longValue ()J
- method longValue ()J
- method movePointRight (I)Ljava/math/BigDecimal;
- method movePointRight (I)Ljava/math/BigDecimal;
- method setScale (II)Ljava/math/BigDecimal;
- method setScale (II)Ljava/math/BigDecimal;
- method toBigIntegerExact ()Ljava/math/BigInteger;
- method toBigIntegerExact ()Ljava/math/BigInteger;
- method toPlainString ()Ljava/lang/String;
- method toPlainString ()Ljava/lang/String;

## java/math/BigInteger
- field ONE Ljava/math/BigInteger;
- field ONE Ljava/math/BigInteger;
- field ZERO Ljava/math/BigInteger;
- field ZERO Ljava/math/BigInteger;
- method <init> ()V
- method <init> (I[B)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;I)V
- method <init> ([B)V
- method add (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method add (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method and (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method and (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method bitLength ()I
- method bitLength ()I
- method compareTo (Ljava/math/BigInteger;)I
- method compareTo (Ljava/math/BigInteger;)I
- method divide (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method divide (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method intValue ()I
- method intValue ()I
- method longValue ()J
- method longValue ()J
- method mod (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method mod (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method modInverse (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method modInverse (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method multiply (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method multiply (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method negate ()Ljava/math/BigInteger;
- method negate ()Ljava/math/BigInteger;
- method shiftLeft (I)Ljava/math/BigInteger;
- method shiftLeft (I)Ljava/math/BigInteger;
- method shiftRight (I)Ljava/math/BigInteger;
- method shiftRight (I)Ljava/math/BigInteger;
- method subtract (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method subtract (Ljava/math/BigInteger;)Ljava/math/BigInteger;
- method testBit (I)Z
- method testBit (I)Z
- method toByteArray ()[B
- method toByteArray ()[B
- method toString (I)Ljava/lang/String;
- method toString (I)Ljava/lang/String;
- method valueOf (J)Ljava/math/BigInteger;
- method valueOf (J)Ljava/math/BigInteger;

## java/net/Authenticator
- method <init> ()V
- method setDefault (Ljava/net/Authenticator;)V
- method setDefault (Ljava/net/Authenticator;)V

## java/net/ConnectException
- method <init> ()V

## java/net/DatagramPacket
- method <init> ()V
- method <init> ([BI)V
- method <init> ([BILjava/net/InetAddress;I)V
- method getAddress ()Ljava/net/InetAddress;
- method getAddress ()Ljava/net/InetAddress;
- method getData ()[B
- method getData ()[B

## java/net/DatagramSocket
- method <init> ()V
- method <init> (I)V
- method close ()V
- method close ()V
- method receive (Ljava/net/DatagramPacket;)V
- method receive (Ljava/net/DatagramPacket;)V
- method send (Ljava/net/DatagramPacket;)V
- method send (Ljava/net/DatagramPacket;)V

## java/net/InetAddress
- method <init> ()V
- method getAddress ()[B
- method getAddress ()[B
- method getAllByName (Ljava/lang/String;)[Ljava/net/InetAddress;
- method getAllByName (Ljava/lang/String;)[Ljava/net/InetAddress;
- method getByAddress ([B)Ljava/net/InetAddress;
- method getByAddress ([B)Ljava/net/InetAddress;
- method getByName (Ljava/lang/String;)Ljava/net/InetAddress;
- method getByName (Ljava/lang/String;)Ljava/net/InetAddress;
- method getHostAddress ()Ljava/lang/String;
- method getHostAddress ()Ljava/lang/String;
- method getHostName ()Ljava/lang/String;
- method getHostName ()Ljava/lang/String;
- method getLocalHost ()Ljava/net/InetAddress;
- method getLocalHost ()Ljava/net/InetAddress;

## java/net/InetSocketAddress
- method <init> ()V
- method <init> (Ljava/net/InetAddress;I)V
- method getAddress ()Ljava/net/InetAddress;
- method getAddress ()Ljava/net/InetAddress;
- method getPort ()I
- method getPort ()I

## java/net/NetworkInterface
- method <init> ()V
- method getHardwareAddress ()[B
- method getHardwareAddress ()[B
- method getNetworkInterfaces ()Ljava/util/Enumeration;
- method getNetworkInterfaces ()Ljava/util/Enumeration;

## java/net/PasswordAuthentication
- method <init> ()V
- method <init> (Ljava/lang/String;[C)V

## java/net/Proxy
- field NO_PROXY Ljava/net/Proxy;
- field NO_PROXY Ljava/net/Proxy;
- method <init> ()V

## java/net/Proxy$Type
- field DIRECT Ljava/net/Proxy$Type;
- field DIRECT Ljava/net/Proxy$Type;
- method <init> ()V
- method name ()Ljava/lang/String;
- method name ()Ljava/lang/String;
- method values ()[Ljava/net/Proxy$Type;
- method values ()[Ljava/net/Proxy$Type;

## java/net/ServerSocket
- method <init> ()V
- method close ()V
- method close ()V

## java/net/Socket
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method close ()V
- method close ()V
- method connect (Ljava/net/SocketAddress;I)V
- method connect (Ljava/net/SocketAddress;I)V
- method getInetAddress ()Ljava/net/InetAddress;
- method getInetAddress ()Ljava/net/InetAddress;
- method getInputStream ()Ljava/io/InputStream;
- method getInputStream ()Ljava/io/InputStream;
- method getKeepAlive ()Z
- method getKeepAlive ()Z
- method getLocalAddress ()Ljava/net/InetAddress;
- method getLocalAddress ()Ljava/net/InetAddress;
- method getLocalPort ()I
- method getLocalPort ()I
- method getOutputStream ()Ljava/io/OutputStream;
- method getOutputStream ()Ljava/io/OutputStream;
- method getPort ()I
- method getPort ()I
- method getReceiveBufferSize ()I
- method getReceiveBufferSize ()I
- method getSendBufferSize ()I
- method getSendBufferSize ()I
- method getSoLinger ()I
- method getSoLinger ()I
- method getTcpNoDelay ()Z
- method getTcpNoDelay ()Z
- method setKeepAlive (Z)V
- method setKeepAlive (Z)V
- method setReceiveBufferSize (I)V
- method setReceiveBufferSize (I)V
- method setSendBufferSize (I)V
- method setSendBufferSize (I)V
- method setSoLinger (ZI)V
- method setSoLinger (ZI)V
- method setSoTimeout (I)V
- method setSoTimeout (I)V
- method setTcpNoDelay (Z)V
- method setTcpNoDelay (Z)V

## java/net/SocketException
- method <init> ()V

## java/net/SocketTimeoutException
- method <init> ()V

## java/net/URI
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getScheme ()Ljava/lang/String;
- method getScheme ()Ljava/lang/String;
- method toASCIIString ()Ljava/lang/String;
- method toASCIIString ()Ljava/lang/String;
- method toURL ()Ljava/net/URL;
- method toURL ()Ljava/net/URL;

## java/net/URISyntaxException
- method <init> ()V

## java/net/URLDecoder
- method <init> ()V
- method decode (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method decode (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;

## java/net/UnknownHostException
- method <init> ()V

## java/nio/ByteOrder
- field BIG_ENDIAN Ljava/nio/ByteOrder;
- field BIG_ENDIAN Ljava/nio/ByteOrder;
- field LITTLE_ENDIAN Ljava/nio/ByteOrder;
- field LITTLE_ENDIAN Ljava/nio/ByteOrder;
- method <init> ()V
- method nativeOrder ()Ljava/nio/ByteOrder;
- method nativeOrder ()Ljava/nio/ByteOrder;

## java/nio/CharBuffer
- method <init> ()V
- method allocate (I)Ljava/nio/CharBuffer;
- method allocate (I)Ljava/nio/CharBuffer;
- method array ()[C
- method array ()[C
- method compact ()Ljava/nio/CharBuffer;
- method compact ()Ljava/nio/CharBuffer;
- method flip ()Ljava/nio/Buffer;
- method flip ()Ljava/nio/Buffer;
- method get ()C
- method get ()C
- method hasRemaining ()Z
- method hasRemaining ()Z
- method length ()I
- method length ()I
- method position ()I
- method position ()I
- method position (I)Ljava/nio/Buffer;
- method position (I)Ljava/nio/Buffer;
- method remaining ()I
- method remaining ()I
- method rewind ()Ljava/nio/Buffer;
- method rewind ()Ljava/nio/Buffer;
- method wrap (Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;
- method wrap (Ljava/lang/CharSequence;)Ljava/nio/CharBuffer;

## java/nio/DoubleBuffer
- method <init> ()V
- method get (I)D
- method get (I)D
- method isDirect ()Z
- method isDirect ()Z
- method order ()Ljava/nio/ByteOrder;
- method order ()Ljava/nio/ByteOrder;
- method put (D)Ljava/nio/DoubleBuffer;
- method put (D)Ljava/nio/DoubleBuffer;
- method put (Ljava/nio/DoubleBuffer;)Ljava/nio/DoubleBuffer;
- method put (Ljava/nio/DoubleBuffer;)Ljava/nio/DoubleBuffer;

## java/nio/FloatBuffer
- method <init> ()V
- method get ()F
- method get ()F
- method get (I)F
- method get (I)F
- method get ([FII)Ljava/nio/FloatBuffer;
- method get ([FII)Ljava/nio/FloatBuffer;
- method isDirect ()Z
- method isDirect ()Z
- method order ()Ljava/nio/ByteOrder;
- method order ()Ljava/nio/ByteOrder;
- method put (F)Ljava/nio/FloatBuffer;
- method put (F)Ljava/nio/FloatBuffer;
- method put (IF)Ljava/nio/FloatBuffer;
- method put (IF)Ljava/nio/FloatBuffer;
- method put (Ljava/nio/FloatBuffer;)Ljava/nio/FloatBuffer;
- method put (Ljava/nio/FloatBuffer;)Ljava/nio/FloatBuffer;
- method put ([F)Ljava/nio/FloatBuffer;
- method put ([F)Ljava/nio/FloatBuffer;
- method put ([FII)Ljava/nio/FloatBuffer;
- method put ([FII)Ljava/nio/FloatBuffer;

## java/nio/IntBuffer
- method <init> ()V
- method flip ()Ljava/nio/Buffer;
- method flip ()Ljava/nio/Buffer;
- method get ()I
- method get ()I
- method get (I)I
- method get (I)I
- method get ([III)Ljava/nio/IntBuffer;
- method get ([III)Ljava/nio/IntBuffer;
- method isDirect ()Z
- method isDirect ()Z
- method order ()Ljava/nio/ByteOrder;
- method order ()Ljava/nio/ByteOrder;
- method put (I)Ljava/nio/IntBuffer;
- method put (I)Ljava/nio/IntBuffer;
- method put (Ljava/nio/IntBuffer;)Ljava/nio/IntBuffer;
- method put (Ljava/nio/IntBuffer;)Ljava/nio/IntBuffer;
- method put ([I)Ljava/nio/IntBuffer;
- method put ([I)Ljava/nio/IntBuffer;
- method put ([III)Ljava/nio/IntBuffer;
- method put ([III)Ljava/nio/IntBuffer;

## java/nio/LongBuffer
- method <init> ()V
- method isDirect ()Z
- method isDirect ()Z
- method order ()Ljava/nio/ByteOrder;
- method order ()Ljava/nio/ByteOrder;
- method put (Ljava/nio/LongBuffer;)Ljava/nio/LongBuffer;
- method put (Ljava/nio/LongBuffer;)Ljava/nio/LongBuffer;

## java/nio/MappedByteBuffer
- method <init> ()V
- method force ()Ljava/nio/MappedByteBuffer;
- method force ()Ljava/nio/MappedByteBuffer;
- method get ([B)Ljava/nio/ByteBuffer;
- method get ([B)Ljava/nio/ByteBuffer;
- method position ()I
- method position ()I
- method position (I)Ljava/nio/Buffer;
- method position (I)Ljava/nio/Buffer;
- method put ([B)Ljava/nio/ByteBuffer;
- method put ([B)Ljava/nio/ByteBuffer;

## java/nio/ShortBuffer
- method <init> ()V
- method isDirect ()Z
- method isDirect ()Z
- method order ()Ljava/nio/ByteOrder;
- method order ()Ljava/nio/ByteOrder;
- method put (IS)Ljava/nio/ShortBuffer;
- method put (IS)Ljava/nio/ShortBuffer;
- method put (Ljava/nio/ShortBuffer;)Ljava/nio/ShortBuffer;
- method put (Ljava/nio/ShortBuffer;)Ljava/nio/ShortBuffer;
- method put (S)Ljava/nio/ShortBuffer;
- method put (S)Ljava/nio/ShortBuffer;
- method put ([S)Ljava/nio/ShortBuffer;
- method put ([S)Ljava/nio/ShortBuffer;

## java/nio/channels/FileChannel
- method <init> ()V
- method map (Ljava/nio/channels/FileChannel$MapMode;JJ)Ljava/nio/MappedByteBuffer;
- method map (Ljava/nio/channels/FileChannel$MapMode;JJ)Ljava/nio/MappedByteBuffer;
- method position ()J
- method position ()J
- method position (J)Ljava/nio/channels/FileChannel;
- method position (J)Ljava/nio/channels/FileChannel;
- method read (Ljava/nio/ByteBuffer;J)I
- method read (Ljava/nio/ByteBuffer;J)I
- method size ()J
- method size ()J
- method transferFrom (Ljava/nio/channels/ReadableByteChannel;JJ)J
- method transferFrom (Ljava/nio/channels/ReadableByteChannel;JJ)J
- method tryLock ()Ljava/nio/channels/FileLock;
- method tryLock ()Ljava/nio/channels/FileLock;
- method write (Ljava/nio/ByteBuffer;)I
- method write (Ljava/nio/ByteBuffer;)I
- method write (Ljava/nio/ByteBuffer;J)I
- method write (Ljava/nio/ByteBuffer;J)I

## java/nio/channels/FileChannel$MapMode
- field READ_WRITE Ljava/nio/channels/FileChannel$MapMode;
- field READ_WRITE Ljava/nio/channels/FileChannel$MapMode;
- method <init> ()V

## java/nio/channels/OverlappingFileLockException
- method <init> ()V

## java/nio/channels/Selector
- method <init> ()V
- method close ()V
- method close ()V

## java/nio/charset/CharacterCodingException
- method <init> ()V

## java/nio/charset/Charset
- method <init> ()V
- method availableCharsets ()Ljava/util/SortedMap;
- method availableCharsets ()Ljava/util/SortedMap;
- method decode (Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;
- method decode (Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;
- method defaultCharset ()Ljava/nio/charset/Charset;
- method defaultCharset ()Ljava/nio/charset/Charset;
- method forName (Ljava/lang/String;)Ljava/nio/charset/Charset;
- method forName (Ljava/lang/String;)Ljava/nio/charset/Charset;
- method isSupported (Ljava/lang/String;)Z
- method isSupported (Ljava/lang/String;)Z
- method newDecoder ()Ljava/nio/charset/CharsetDecoder;
- method newDecoder ()Ljava/nio/charset/CharsetDecoder;
- method newEncoder ()Ljava/nio/charset/CharsetEncoder;
- method newEncoder ()Ljava/nio/charset/CharsetEncoder;

## java/nio/charset/CharsetDecoder
- method <init> ()V
- method decode (Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;
- method decode (Ljava/nio/ByteBuffer;)Ljava/nio/CharBuffer;
- method decode (Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;Z)Ljava/nio/charset/CoderResult;
- method decode (Ljava/nio/ByteBuffer;Ljava/nio/CharBuffer;Z)Ljava/nio/charset/CoderResult;
- method onMalformedInput (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;
- method onMalformedInput (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;
- method onUnmappableCharacter (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;
- method onUnmappableCharacter (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetDecoder;
- method replaceWith (Ljava/lang/String;)Ljava/nio/charset/CharsetDecoder;
- method replaceWith (Ljava/lang/String;)Ljava/nio/charset/CharsetDecoder;

## java/nio/charset/CharsetEncoder
- method <init> ()V
- method encode (Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;
- method encode (Ljava/nio/CharBuffer;)Ljava/nio/ByteBuffer;
- method encode (Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;
- method encode (Ljava/nio/CharBuffer;Ljava/nio/ByteBuffer;Z)Ljava/nio/charset/CoderResult;
- method maxBytesPerChar ()F
- method maxBytesPerChar ()F
- method onMalformedInput (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;
- method onMalformedInput (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;
- method onUnmappableCharacter (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;
- method onUnmappableCharacter (Ljava/nio/charset/CodingErrorAction;)Ljava/nio/charset/CharsetEncoder;

## java/nio/charset/CoderResult
- method <init> ()V
- method isError ()Z
- method isError ()Z
- method isOverflow ()Z
- method isOverflow ()Z
- method isUnderflow ()Z
- method isUnderflow ()Z
- method throwException ()V
- method throwException ()V

## java/nio/charset/CodingErrorAction
- field REPLACE Ljava/nio/charset/CodingErrorAction;
- field REPLACE Ljava/nio/charset/CodingErrorAction;
- method <init> ()V

## java/nio/charset/IllegalCharsetNameException
- method <init> ()V

## java/nio/file/CopyOption
- method <init> ()V

## java/nio/file/Files
- method <init> ()V
- method copy (Ljava/nio/file/Path;Ljava/nio/file/Path;[Ljava/nio/file/CopyOption;)Ljava/nio/file/Path;
- method copy (Ljava/nio/file/Path;Ljava/nio/file/Path;[Ljava/nio/file/CopyOption;)Ljava/nio/file/Path;
- method deleteIfExists (Ljava/nio/file/Path;)Z
- method deleteIfExists (Ljava/nio/file/Path;)Z

## java/nio/file/StandardCopyOption
- field REPLACE_EXISTING Ljava/nio/file/StandardCopyOption;
- field REPLACE_EXISTING Ljava/nio/file/StandardCopyOption;
- method <init> ()V

## java/rmi/Remote
- method <init> ()V

## java/rmi/RemoteException
- method <init> ()V

## java/security/AccessController
- method <init> ()V
- method doPrivileged (Ljava/security/PrivilegedAction;)Ljava/lang/Object;
- method doPrivileged (Ljava/security/PrivilegedAction;)Ljava/lang/Object;
- method doPrivileged (Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;
- method doPrivileged (Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;

## java/security/CodeSource
- method <init> ()V
- method getCertificates ()[Ljava/security/cert/Certificate;
- method getCertificates ()[Ljava/security/cert/Certificate;
- method getLocation ()Ljava/net/URL;
- method getLocation ()Ljava/net/URL;

## java/security/DigestException
- method <init> ()V

## java/security/DigestInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;Ljava/security/MessageDigest;)V
- method getMessageDigest ()Ljava/security/MessageDigest;
- method getMessageDigest ()Ljava/security/MessageDigest;
- method on (Z)V
- method on (Z)V
- method read ([B)I
- method read ([B)I

## java/security/InvalidAlgorithmParameterException
- method <init> ()V

## java/security/InvalidKeyException
- method <init> ()V

## java/security/Key
- method <init> ()V

## java/security/KeyFactory
- method <init> ()V
- method generatePublic (Ljava/security/spec/KeySpec;)Ljava/security/PublicKey;
- method generatePublic (Ljava/security/spec/KeySpec;)Ljava/security/PublicKey;
- method getInstance (Ljava/lang/String;)Ljava/security/KeyFactory;
- method getInstance (Ljava/lang/String;)Ljava/security/KeyFactory;

## java/security/PrivilegedAction
- method <init> ()V

## java/security/PrivilegedActionException
- method <init> ()V
- method getException ()Ljava/lang/Exception;
- method getException ()Ljava/lang/Exception;

## java/security/PrivilegedExceptionAction
- method <init> ()V

## java/security/ProtectionDomain
- method <init> ()V
- method getCodeSource ()Ljava/security/CodeSource;
- method getCodeSource ()Ljava/security/CodeSource;

## java/security/PublicKey
- method <init> ()V

## java/security/SecureRandom
- method <init> ()V
- method nextBytes ([B)V
- method nextBytes ([B)V

## java/security/SignatureException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/Throwable;)V

## java/security/spec/InvalidKeySpecException
- method <init> ()V

## java/security/spec/X509EncodedKeySpec
- method <init> ()V
- method <init> ([B)V

## java/sql/Connection
- method <init> ()V
- method commit ()V
- method commit ()V
- method createStatement ()Ljava/sql/Statement;
- method createStatement ()Ljava/sql/Statement;
- method prepareStatement (Ljava/lang/String;)Ljava/sql/PreparedStatement;
- method prepareStatement (Ljava/lang/String;)Ljava/sql/PreparedStatement;
- method rollback ()V
- method rollback ()V
- method setAutoCommit (Z)V
- method setAutoCommit (Z)V

## java/sql/Date
- method <init> ()V
- method <init> (J)V
- method getTime ()J
- method getTime ()J

## java/sql/DriverManager
- method <init> ()V
- method getConnection (Ljava/lang/String;)Ljava/sql/Connection;
- method getConnection (Ljava/lang/String;)Ljava/sql/Connection;

## java/sql/PreparedStatement
- method <init> ()V
- method close ()V
- method close ()V
- method executeQuery ()Ljava/sql/ResultSet;
- method executeQuery ()Ljava/sql/ResultSet;
- method executeUpdate ()I
- method executeUpdate ()I
- method setBoolean (IZ)V
- method setBoolean (IZ)V
- method setBytes (I[B)V
- method setBytes (I[B)V
- method setInt (II)V
- method setInt (II)V
- method setNull (II)V
- method setNull (II)V
- method setString (ILjava/lang/String;)V
- method setString (ILjava/lang/String;)V

## java/sql/ResultSet
- method <init> ()V
- method close ()V
- method close ()V
- method getBoolean (I)Z
- method getBoolean (I)Z
- method getBytes (I)[B
- method getBytes (I)[B
- method getInt (I)I
- method getInt (I)I
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;
- method next ()Z
- method next ()Z

## java/sql/SQLException
- method <init> ()V
- method getErrorCode ()I
- method getErrorCode ()I

## java/sql/Statement
- method <init> ()V
- method close ()V
- method close ()V
- method executeQuery (Ljava/lang/String;)Ljava/sql/ResultSet;
- method executeQuery (Ljava/lang/String;)Ljava/sql/ResultSet;
- method executeUpdate (Ljava/lang/String;)I
- method executeUpdate (Ljava/lang/String;)I

## java/sql/Time
- method <init> ()V
- method <init> (J)V

## java/sql/Timestamp
- method <init> ()V
- method <init> (J)V

## java/sql/Types
- field BLOB I
- field BLOB I
- method <init> ()V

## java/text/AttributedCharacterIterator
- method <init> ()V
- method first ()C
- method first ()C
- method getBeginIndex ()I
- method getBeginIndex ()I
- method getEndIndex ()I
- method getEndIndex ()I
- method getIndex ()I
- method getIndex ()I
- method next ()C
- method next ()C

## java/text/AttributedString
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/util/Map;)V
- method getIterator ()Ljava/text/AttributedCharacterIterator;
- method getIterator ()Ljava/text/AttributedCharacterIterator;

## java/text/BreakIterator
- method <init> ()V
- method first ()I
- method first ()I
- method getLineInstance (Ljava/util/Locale;)Ljava/text/BreakIterator;
- method getLineInstance (Ljava/util/Locale;)Ljava/text/BreakIterator;
- method next ()I
- method next ()I
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## java/text/DateFormat
- method <init> ()V
- method format (Ljava/util/Date;)Ljava/lang/String;
- method format (Ljava/util/Date;)Ljava/lang/String;
- method getDateInstance (I)Ljava/text/DateFormat;
- method getDateInstance (I)Ljava/text/DateFormat;
- method getDateInstance (ILjava/util/Locale;)Ljava/text/DateFormat;
- method getDateInstance (ILjava/util/Locale;)Ljava/text/DateFormat;
- method getDateTimeInstance ()Ljava/text/DateFormat;
- method getDateTimeInstance ()Ljava/text/DateFormat;
- method getDateTimeInstance (II)Ljava/text/DateFormat;
- method getDateTimeInstance (II)Ljava/text/DateFormat;
- method getDateTimeInstance (IILjava/util/Locale;)Ljava/text/DateFormat;
- method getDateTimeInstance (IILjava/util/Locale;)Ljava/text/DateFormat;
- method getTimeInstance (ILjava/util/Locale;)Ljava/text/DateFormat;
- method getTimeInstance (ILjava/util/Locale;)Ljava/text/DateFormat;
- method parse (Ljava/lang/String;)Ljava/util/Date;
- method parse (Ljava/lang/String;)Ljava/util/Date;
- method parse (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;
- method parse (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;
- method setTimeZone (Ljava/util/TimeZone;)V
- method setTimeZone (Ljava/util/TimeZone;)V

## java/text/DateFormatSymbols
- method <init> ()V
- method <init> (Ljava/util/Locale;)V
- method getAmPmStrings ()[Ljava/lang/String;
- method getAmPmStrings ()[Ljava/lang/String;
- method getEras ()[Ljava/lang/String;
- method getEras ()[Ljava/lang/String;
- method getMonths ()[Ljava/lang/String;
- method getMonths ()[Ljava/lang/String;
- method getShortMonths ()[Ljava/lang/String;
- method getShortMonths ()[Ljava/lang/String;
- method getShortWeekdays ()[Ljava/lang/String;
- method getShortWeekdays ()[Ljava/lang/String;
- method getWeekdays ()[Ljava/lang/String;
- method getWeekdays ()[Ljava/lang/String;

## java/text/DecimalFormat
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method format (D)Ljava/lang/String;
- method format (D)Ljava/lang/String;

## java/text/Format
- method <init> ()V
- method format (Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;
- method format (Ljava/lang/Object;Ljava/lang/StringBuffer;Ljava/text/FieldPosition;)Ljava/lang/StringBuffer;
- method parseObject (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;
- method parseObject (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/lang/Object;

## java/text/MessageFormat
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method applyPattern (Ljava/lang/String;)V
- method applyPattern (Ljava/lang/String;)V
- method format (Ljava/lang/Object;)Ljava/lang/String;
- method format (Ljava/lang/Object;)Ljava/lang/String;
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
- method setFormats ([Ljava/text/Format;)V
- method setFormats ([Ljava/text/Format;)V
- method toPattern ()Ljava/lang/String;
- method toPattern ()Ljava/lang/String;

## java/text/NumberFormat
- method <init> ()V
- method format (D)Ljava/lang/String;
- method format (D)Ljava/lang/String;
- method getInstance ()Ljava/text/NumberFormat;
- method getInstance ()Ljava/text/NumberFormat;
- method getNumberInstance ()Ljava/text/NumberFormat;
- method getNumberInstance ()Ljava/text/NumberFormat;
- method parse (Ljava/lang/String;)Ljava/lang/Number;
- method parse (Ljava/lang/String;)Ljava/lang/Number;
- method setMaximumFractionDigits (I)V
- method setMaximumFractionDigits (I)V

## java/text/ParseException
- method <init> ()V
- method <init> (Ljava/lang/String;I)V

## java/text/ParsePosition
- method <init> ()V
- method <init> (I)V
- method getIndex ()I
- method getIndex ()I
- method setErrorIndex (I)V
- method setErrorIndex (I)V
- method setIndex (I)V
- method setIndex (I)V

## java/text/SimpleDateFormat
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/util/Locale;)V
- method applyPattern (Ljava/lang/String;)V
- method applyPattern (Ljava/lang/String;)V
- method format (Ljava/util/Date;)Ljava/lang/String;
- method format (Ljava/util/Date;)Ljava/lang/String;
- method parse (Ljava/lang/String;)Ljava/util/Date;
- method parse (Ljava/lang/String;)Ljava/util/Date;
- method parse (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;
- method parse (Ljava/lang/String;Ljava/text/ParsePosition;)Ljava/util/Date;
- method setLenient (Z)V
- method setLenient (Z)V
- method toLocalizedPattern ()Ljava/lang/String;
- method toLocalizedPattern ()Ljava/lang/String;
- method toPattern ()Ljava/lang/String;
- method toPattern ()Ljava/lang/String;

## java/util/AbstractMap
- method <init> ()V

## java/util/AbstractMap$SimpleEntry
- method <init> ()V
- method <init> (Ljava/lang/Object;Ljava/lang/Object;)V

## java/util/AbstractSet
- method <init> ()V

## java/util/ArrayList
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/util/Collection;)V
- method add (ILjava/lang/Object;)V
- method add (ILjava/lang/Object;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method get (I)Ljava/lang/Object;
- method get (I)Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method remove (I)Ljava/lang/Object;
- method remove (I)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method set (ILjava/lang/Object;)Ljava/lang/Object;
- method set (ILjava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method size ()I
- method subList (II)Ljava/util/List;
- method subList (II)Ljava/util/List;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/Arrays
- method <init> ()V
- method asList ([Ljava/lang/Object;)Ljava/util/List;
- method asList ([Ljava/lang/Object;)Ljava/util/List;
- method binarySearch ([CC)I
- method binarySearch ([CC)I
- method binarySearch ([Ljava/lang/Object;Ljava/lang/Object;)I
- method binarySearch ([Ljava/lang/Object;Ljava/lang/Object;)I
- method copyOf ([BI)[B
- method copyOf ([BI)[B
- method copyOf ([FI)[F
- method copyOf ([FI)[F
- method copyOf ([II)[I
- method copyOf ([II)[I
- method copyOf ([Ljava/lang/Object;I)[Ljava/lang/Object;
- method copyOf ([Ljava/lang/Object;I)[Ljava/lang/Object;
- method copyOfRange ([BII)[B
- method copyOfRange ([BII)[B
- method equals ([B[B)Z
- method equals ([B[B)Z
- method equals ([C[C)Z
- method equals ([C[C)Z
- method equals ([D[D)Z
- method equals ([D[D)Z
- method equals ([F[F)Z
- method equals ([F[F)Z
- method equals ([I[I)Z
- method equals ([I[I)Z
- method equals ([J[J)Z
- method equals ([J[J)Z
- method equals ([Ljava/lang/Object;[Ljava/lang/Object;)Z
- method equals ([Ljava/lang/Object;[Ljava/lang/Object;)Z
- method equals ([S[S)Z
- method equals ([S[S)Z
- method equals ([Z[Z)Z
- method equals ([Z[Z)Z
- method fill ([BB)V
- method fill ([BB)V
- method fill ([DD)V
- method fill ([DD)V
- method fill ([FF)V
- method fill ([FF)V
- method fill ([II)V
- method fill ([II)V
- method fill ([IIII)V
- method fill ([IIII)V
- method fill ([JJ)V
- method fill ([JJ)V
- method fill ([Ljava/lang/Object;IILjava/lang/Object;)V
- method fill ([Ljava/lang/Object;IILjava/lang/Object;)V
- method fill ([Ljava/lang/Object;Ljava/lang/Object;)V
- method fill ([Ljava/lang/Object;Ljava/lang/Object;)V
- method fill ([SS)V
- method fill ([SS)V
- method fill ([ZZ)V
- method fill ([ZZ)V
- method hashCode ([B)I
- method hashCode ([B)I
- method hashCode ([C)I
- method hashCode ([C)I
- method hashCode ([D)I
- method hashCode ([D)I
- method hashCode ([F)I
- method hashCode ([F)I
- method hashCode ([I)I
- method hashCode ([I)I
- method hashCode ([J)I
- method hashCode ([J)I
- method hashCode ([Ljava/lang/Object;)I
- method hashCode ([Ljava/lang/Object;)I
- method hashCode ([S)I
- method hashCode ([S)I
- method hashCode ([Z)I
- method hashCode ([Z)I
- method sort ([C)V
- method sort ([C)V
- method sort ([I)V
- method sort ([I)V
- method sort ([JII)V
- method sort ([JII)V
- method sort ([Ljava/lang/Object;)V
- method sort ([Ljava/lang/Object;)V
- method sort ([Ljava/lang/Object;Ljava/util/Comparator;)V
- method sort ([Ljava/lang/Object;Ljava/util/Comparator;)V
- method toString ([B)Ljava/lang/String;
- method toString ([B)Ljava/lang/String;
- method toString ([I)Ljava/lang/String;
- method toString ([I)Ljava/lang/String;

## java/util/BitSet
- method <init> ()V
- method <init> (I)V
- method clear ()V
- method clear ()V
- method get (I)Z
- method get (I)Z
- method isEmpty ()Z
- method isEmpty ()Z
- method length ()I
- method length ()I
- method set (I)V
- method set (I)V
- method set (II)V
- method set (II)V
- method set (IZ)V
- method set (IZ)V

## java/util/Collection
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method size ()I
- method size ()I
- method toArray ()[Ljava/lang/Object;
- method toArray ()[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/Collections
- method <init> ()V
- method addAll (Ljava/util/Collection;[Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;[Ljava/lang/Object;)Z
- method copy (Ljava/util/List;Ljava/util/List;)V
- method copy (Ljava/util/List;Ljava/util/List;)V
- method disjoint (Ljava/util/Collection;Ljava/util/Collection;)Z
- method disjoint (Ljava/util/Collection;Ljava/util/Collection;)Z
- method emptyList ()Ljava/util/List;
- method emptyList ()Ljava/util/List;
- method emptyMap ()Ljava/util/Map;
- method emptyMap ()Ljava/util/Map;
- method enumeration (Ljava/util/Collection;)Ljava/util/Enumeration;
- method enumeration (Ljava/util/Collection;)Ljava/util/Enumeration;
- method list (Ljava/util/Enumeration;)Ljava/util/ArrayList;
- method list (Ljava/util/Enumeration;)Ljava/util/ArrayList;
- method max (Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
- method max (Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;
- method replaceAll (Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z
- method replaceAll (Ljava/util/List;Ljava/lang/Object;Ljava/lang/Object;)Z
- method reverse (Ljava/util/List;)V
- method reverse (Ljava/util/List;)V
- method reverseOrder (Ljava/util/Comparator;)Ljava/util/Comparator;
- method reverseOrder (Ljava/util/Comparator;)Ljava/util/Comparator;
- method shuffle (Ljava/util/List;)V
- method shuffle (Ljava/util/List;)V
- method singletonList (Ljava/lang/Object;)Ljava/util/List;
- method singletonList (Ljava/lang/Object;)Ljava/util/List;
- method sort (Ljava/util/List;)V
- method sort (Ljava/util/List;)V
- method sort (Ljava/util/List;Ljava/util/Comparator;)V
- method sort (Ljava/util/List;Ljava/util/Comparator;)V
- method synchronizedList (Ljava/util/List;)Ljava/util/List;
- method synchronizedList (Ljava/util/List;)Ljava/util/List;
- method synchronizedMap (Ljava/util/Map;)Ljava/util/Map;
- method synchronizedMap (Ljava/util/Map;)Ljava/util/Map;
- method synchronizedSet (Ljava/util/Set;)Ljava/util/Set;
- method synchronizedSet (Ljava/util/Set;)Ljava/util/Set;
- method unmodifiableCollection (Ljava/util/Collection;)Ljava/util/Collection;
- method unmodifiableCollection (Ljava/util/Collection;)Ljava/util/Collection;
- method unmodifiableList (Ljava/util/List;)Ljava/util/List;
- method unmodifiableList (Ljava/util/List;)Ljava/util/List;
- method unmodifiableMap (Ljava/util/Map;)Ljava/util/Map;
- method unmodifiableMap (Ljava/util/Map;)Ljava/util/Map;
- method unmodifiableSet (Ljava/util/Set;)Ljava/util/Set;
- method unmodifiableSet (Ljava/util/Set;)Ljava/util/Set;
- method unmodifiableSortedSet (Ljava/util/SortedSet;)Ljava/util/SortedSet;
- method unmodifiableSortedSet (Ljava/util/SortedSet;)Ljava/util/SortedSet;

## java/util/Comparator
- method <init> ()V
- method compare (Ljava/lang/Object;Ljava/lang/Object;)I
- method compare (Ljava/lang/Object;Ljava/lang/Object;)I

## java/util/ConcurrentModificationException
- method <init> ()V

## java/util/Deque
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method addFirst (Ljava/lang/Object;)V
- method addFirst (Ljava/lang/Object;)V
- method addLast (Ljava/lang/Object;)V
- method addLast (Ljava/lang/Object;)V
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method containsAll (Ljava/util/Collection;)Z
- method containsAll (Ljava/util/Collection;)Z
- method getFirst ()Ljava/lang/Object;
- method getFirst ()Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method peekFirst ()Ljava/lang/Object;
- method peekFirst ()Ljava/lang/Object;
- method peekLast ()Ljava/lang/Object;
- method peekLast ()Ljava/lang/Object;
- method pollFirst ()Ljava/lang/Object;
- method pollFirst ()Ljava/lang/Object;
- method pollLast ()Ljava/lang/Object;
- method pollLast ()Ljava/lang/Object;
- method remove ()Ljava/lang/Object;
- method remove ()Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method removeAll (Ljava/util/Collection;)Z
- method removeAll (Ljava/util/Collection;)Z
- method removeFirst ()Ljava/lang/Object;
- method removeFirst ()Ljava/lang/Object;
- method removeLast ()Ljava/lang/Object;
- method removeLast ()Ljava/lang/Object;
- method retainAll (Ljava/util/Collection;)Z
- method retainAll (Ljava/util/Collection;)Z
- method size ()I
- method size ()I
- method toArray ()[Ljava/lang/Object;
- method toArray ()[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/EnumMap
- method <init> ()V
- method <init> (Ljava/lang/Class;)V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;

## java/util/EnumSet
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method allOf (Ljava/lang/Class;)Ljava/util/EnumSet;
- method allOf (Ljava/lang/Class;)Ljava/util/EnumSet;
- method clear ()V
- method clear ()V
- method complementOf (Ljava/util/EnumSet;)Ljava/util/EnumSet;
- method complementOf (Ljava/util/EnumSet;)Ljava/util/EnumSet;
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method copyOf (Ljava/util/Collection;)Ljava/util/EnumSet;
- method copyOf (Ljava/util/Collection;)Ljava/util/EnumSet;
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method noneOf (Ljava/lang/Class;)Ljava/util/EnumSet;
- method noneOf (Ljava/lang/Class;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method of (Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;Ljava/lang/Enum;)Ljava/util/EnumSet;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z

## java/util/EventListener
- method <init> ()V

## java/util/EventObject
- method <init> ()V
- method <init> (Ljava/lang/Object;)V
- method getSource ()Ljava/lang/Object;
- method getSource ()Ljava/lang/Object;

## java/util/Formatter
- method <init> ()V
- method <init> (Ljava/lang/Appendable;)V
- method <init> (Ljava/lang/Appendable;Ljava/util/Locale;)V
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
- method format (Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
- method format (Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
- method format (Ljava/util/Locale;Ljava/lang/String;[Ljava/lang/Object;)Ljava/util/Formatter;
- method locale ()Ljava/util/Locale;
- method locale ()Ljava/util/Locale;

## java/util/HashMap
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/util/Map;)V
- method clear ()V
- method clear ()V
- method containsKey (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method entrySet ()Ljava/util/Set;
- method entrySet ()Ljava/util/Set;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method size ()I

## java/util/HashSet
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/util/Collection;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method removeAll (Ljava/util/Collection;)Z
- method removeAll (Ljava/util/Collection;)Z
- method retainAll (Ljava/util/Collection;)Z
- method retainAll (Ljava/util/Collection;)Z
- method size ()I
- method size ()I

## java/util/IdentityHashMap
- method <init> ()V

## java/util/Iterator
- method <init> ()V
- method hasNext ()Z
- method hasNext ()Z
- method next ()Ljava/lang/Object;
- method next ()Ljava/lang/Object;
- method remove ()V
- method remove ()V

## java/util/LinkedHashMap
- method <init> ()V
- method <init> (Ljava/util/Map;)V
- method clear ()V
- method clear ()V
- method containsKey (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method entrySet ()Ljava/util/Set;
- method entrySet ()Ljava/util/Set;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method values ()Ljava/util/Collection;
- method values ()Ljava/util/Collection;

## java/util/LinkedHashSet
- method <init> ()V

## java/util/LinkedList
- method <init> ()V
- method <init> (Ljava/util/Collection;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method descendingIterator ()Ljava/util/Iterator;
- method descendingIterator ()Ljava/util/Iterator;
- method getFirst ()Ljava/lang/Object;
- method getFirst ()Ljava/lang/Object;
- method getLast ()Ljava/lang/Object;
- method getLast ()Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method listIterator ()Ljava/util/ListIterator;
- method listIterator ()Ljava/util/ListIterator;
- method pollFirst ()Ljava/lang/Object;
- method pollFirst ()Ljava/lang/Object;
- method pollLast ()Ljava/lang/Object;
- method pollLast ()Ljava/lang/Object;
- method pop ()Ljava/lang/Object;
- method pop ()Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method removeFirst ()Ljava/lang/Object;
- method removeFirst ()Ljava/lang/Object;
- method size ()I
- method size ()I

## java/util/List
- method <init> ()V
- method add (ILjava/lang/Object;)V
- method add (ILjava/lang/Object;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (ILjava/util/Collection;)Z
- method addAll (ILjava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method containsAll (Ljava/util/Collection;)Z
- method containsAll (Ljava/util/Collection;)Z
- method get (I)Ljava/lang/Object;
- method get (I)Ljava/lang/Object;
- method indexOf (Ljava/lang/Object;)I
- method indexOf (Ljava/lang/Object;)I
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method listIterator ()Ljava/util/ListIterator;
- method listIterator ()Ljava/util/ListIterator;
- method remove (I)Ljava/lang/Object;
- method remove (I)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method removeAll (Ljava/util/Collection;)Z
- method removeAll (Ljava/util/Collection;)Z
- method set (ILjava/lang/Object;)Ljava/lang/Object;
- method set (ILjava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method size ()I
- method toArray ()[Ljava/lang/Object;
- method toArray ()[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/ListIterator
- method <init> ()V
- method hasNext ()Z
- method hasNext ()Z
- method next ()Ljava/lang/Object;
- method next ()Ljava/lang/Object;
- method set (Ljava/lang/Object;)V
- method set (Ljava/lang/Object;)V

## java/util/Locale
- field ENGLISH Ljava/util/Locale;
- field ENGLISH Ljava/util/Locale;
- field US Ljava/util/Locale;
- field US Ljava/util/Locale;
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method getAvailableLocales ()[Ljava/util/Locale;
- method getAvailableLocales ()[Ljava/util/Locale;
- method getCountry ()Ljava/lang/String;
- method getCountry ()Ljava/lang/String;
- method getDefault ()Ljava/util/Locale;
- method getDefault ()Ljava/util/Locale;
- method getDisplayCountry (Ljava/util/Locale;)Ljava/lang/String;
- method getDisplayCountry (Ljava/util/Locale;)Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;
- method getLanguage ()Ljava/lang/String;
- method getVariant ()Ljava/lang/String;
- method getVariant ()Ljava/lang/String;
- method setDefault (Ljava/util/Locale;)V
- method setDefault (Ljava/util/Locale;)V

## java/util/Map
- method <init> ()V
- method clear ()V
- method clear ()V
- method containsKey (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method containsValue (Ljava/lang/Object;)Z
- method containsValue (Ljava/lang/Object;)Z
- method entrySet ()Ljava/util/Set;
- method entrySet ()Ljava/util/Set;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method keySet ()Ljava/util/Set;
- method keySet ()Ljava/util/Set;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method putAll (Ljava/util/Map;)V
- method putAll (Ljava/util/Map;)V
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method size ()I
- method values ()Ljava/util/Collection;
- method values ()Ljava/util/Collection;

## java/util/Map$Entry
- method <init> ()V
- method getKey ()Ljava/lang/Object;
- method getKey ()Ljava/lang/Object;
- method getValue ()Ljava/lang/Object;
- method getValue ()Ljava/lang/Object;
- method setValue (Ljava/lang/Object;)Ljava/lang/Object;
- method setValue (Ljava/lang/Object;)Ljava/lang/Object;

## java/util/MissingResourceException
- method <init> ()V

## java/util/NavigableSet
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method clear ()V
- method clear ()V
- method descendingIterator ()Ljava/util/Iterator;
- method descendingIterator ()Ljava/util/Iterator;
- method first ()Ljava/lang/Object;
- method first ()Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method last ()Ljava/lang/Object;
- method last ()Ljava/lang/Object;
- method size ()I
- method size ()I

## java/util/PropertyResourceBundle
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V

## java/util/Queue
- method <init> ()V

## java/util/ResourceBundle
- method <init> ()V
- method getBundle (Ljava/lang/String;)Ljava/util/ResourceBundle;
- method getBundle (Ljava/lang/String;)Ljava/util/ResourceBundle;
- method getBundle (Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;
- method getBundle (Ljava/lang/String;Ljava/util/Locale;)Ljava/util/ResourceBundle;
- method getString (Ljava/lang/String;)Ljava/lang/String;
- method getString (Ljava/lang/String;)Ljava/lang/String;

## java/util/Scanner
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method close ()V
- method close ()V
- method hasNextLine ()Z
- method hasNextLine ()Z
- method nextLine ()Ljava/lang/String;
- method nextLine ()Ljava/lang/String;

## java/util/Set
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method isEmpty ()Z
- method isEmpty ()Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method removeAll (Ljava/util/Collection;)Z
- method removeAll (Ljava/util/Collection;)Z
- method size ()I
- method size ()I
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;
- method toArray ([Ljava/lang/Object;)[Ljava/lang/Object;

## java/util/SortedMap
- method <init> ()V

## java/util/SortedSet
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;

## java/util/StringTokenizer
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method countTokens ()I
- method countTokens ()I
- method hasMoreElements ()Z
- method hasMoreElements ()Z
- method hasMoreTokens ()Z
- method hasMoreTokens ()Z
- method nextToken ()Ljava/lang/String;
- method nextToken ()Ljava/lang/String;

## java/util/TooManyListenersException
- method <init> ()V

## java/util/TreeMap
- method <init> ()V
- method clear ()V
- method clear ()V
- method containsKey (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method firstEntry ()Ljava/util/Map$Entry;
- method firstEntry ()Ljava/util/Map$Entry;
- method firstKey ()Ljava/lang/Object;
- method firstKey ()Ljava/lang/Object;
- method floorEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method floorEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method higherEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method higherEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method higherKey (Ljava/lang/Object;)Ljava/lang/Object;
- method higherKey (Ljava/lang/Object;)Ljava/lang/Object;
- method isEmpty ()Z
- method isEmpty ()Z
- method keySet ()Ljava/util/Set;
- method keySet ()Ljava/util/Set;
- method lastEntry ()Ljava/util/Map$Entry;
- method lastEntry ()Ljava/util/Map$Entry;
- method lastKey ()Ljava/lang/Object;
- method lastKey ()Ljava/lang/Object;
- method lowerEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method lowerEntry (Ljava/lang/Object;)Ljava/util/Map$Entry;
- method lowerKey (Ljava/lang/Object;)Ljava/lang/Object;
- method lowerKey (Ljava/lang/Object;)Ljava/lang/Object;
- method pollFirstEntry ()Ljava/util/Map$Entry;
- method pollFirstEntry ()Ljava/util/Map$Entry;
- method pollLastEntry ()Ljava/util/Map$Entry;
- method pollLastEntry ()Ljava/util/Map$Entry;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I
- method size ()I
- method values ()Ljava/util/Collection;
- method values ()Ljava/util/Collection;

## java/util/TreeSet
- method <init> ()V
- method <init> (Ljava/util/Collection;)V
- method <init> (Ljava/util/Comparator;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method clear ()V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method size ()I
- method size ()I
- method toArray ()[Ljava/lang/Object;
- method toArray ()[Ljava/lang/Object;

## java/util/UUID
- method <init> ()V
- method <init> (JJ)V
- method fromString (Ljava/lang/String;)Ljava/util/UUID;
- method fromString (Ljava/lang/String;)Ljava/util/UUID;
- method getLeastSignificantBits ()J
- method getLeastSignificantBits ()J
- method getMostSignificantBits ()J
- method getMostSignificantBits ()J
- method randomUUID ()Ljava/util/UUID;
- method randomUUID ()Ljava/util/UUID;

## java/util/WeakHashMap
- method <init> ()V

## java/util/concurrent/Callable
- method <init> ()V
- method call ()Ljava/lang/Object;
- method call ()Ljava/lang/Object;

## java/util/concurrent/ConcurrentHashMap
- method <init> ()V
- method <init> (I)V

## java/util/concurrent/ConcurrentMap
- method <init> ()V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method putIfAbsent (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method putIfAbsent (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;

## java/util/concurrent/CopyOnWriteArrayList
- method <init> ()V
- method <init> (Ljava/util/Collection;)V
- method <init> ([Ljava/lang/Object;)V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method addAll (Ljava/util/Collection;)Z
- method addAll (Ljava/util/Collection;)Z
- method addIfAbsent (Ljava/lang/Object;)Z
- method addIfAbsent (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method contains (Ljava/lang/Object;)Z
- method get (I)Ljava/lang/Object;
- method get (I)Ljava/lang/Object;
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;
- method listIterator ()Ljava/util/ListIterator;
- method listIterator ()Ljava/util/ListIterator;
- method remove (Ljava/lang/Object;)Z
- method remove (Ljava/lang/Object;)Z
- method size ()I
- method size ()I

## java/util/concurrent/CopyOnWriteArraySet
- method <init> ()V
- method add (Ljava/lang/Object;)Z
- method add (Ljava/lang/Object;)Z
- method iterator ()Ljava/util/Iterator;
- method iterator ()Ljava/util/Iterator;

## java/util/concurrent/DelayQueue
- method <init> ()V
- method add (Ljava/util/concurrent/Delayed;)Z
- method add (Ljava/util/concurrent/Delayed;)Z
- method poll (JLjava/util/concurrent/TimeUnit;)Ljava/util/concurrent/Delayed;
- method poll (JLjava/util/concurrent/TimeUnit;)Ljava/util/concurrent/Delayed;
- method size ()I
- method size ()I

## java/util/concurrent/Delayed
- method <init> ()V
- method getDelay (Ljava/util/concurrent/TimeUnit;)J
- method getDelay (Ljava/util/concurrent/TimeUnit;)J

## java/util/concurrent/ExecutionException
- method <init> ()V

## java/util/concurrent/ExecutorService
- method <init> ()V
- method execute (Ljava/lang/Runnable;)V
- method execute (Ljava/lang/Runnable;)V
- method invokeAll (Ljava/util/Collection;JLjava/util/concurrent/TimeUnit;)Ljava/util/List;
- method invokeAll (Ljava/util/Collection;JLjava/util/concurrent/TimeUnit;)Ljava/util/List;
- method isShutdown ()Z
- method isShutdown ()Z
- method shutdown ()V
- method shutdown ()V
- method shutdownNow ()Ljava/util/List;
- method shutdownNow ()Ljava/util/List;
- method submit (Ljava/util/concurrent/Callable;)Ljava/util/concurrent/Future;
- method submit (Ljava/util/concurrent/Callable;)Ljava/util/concurrent/Future;

## java/util/concurrent/Executors
- method <init> ()V
- method defaultThreadFactory ()Ljava/util/concurrent/ThreadFactory;
- method defaultThreadFactory ()Ljava/util/concurrent/ThreadFactory;
- method newCachedThreadPool ()Ljava/util/concurrent/ExecutorService;
- method newCachedThreadPool ()Ljava/util/concurrent/ExecutorService;
- method newCachedThreadPool (Ljava/util/concurrent/ThreadFactory;)Ljava/util/concurrent/ExecutorService;
- method newCachedThreadPool (Ljava/util/concurrent/ThreadFactory;)Ljava/util/concurrent/ExecutorService;
- method newFixedThreadPool (I)Ljava/util/concurrent/ExecutorService;
- method newFixedThreadPool (I)Ljava/util/concurrent/ExecutorService;

## java/util/concurrent/Future
- method <init> ()V
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;
- method isCancelled ()Z
- method isCancelled ()Z

## java/util/concurrent/FutureTask
- method <init> ()V
- method <init> (Ljava/util/concurrent/Callable;)V

## java/util/concurrent/ScheduledExecutorService
- method <init> ()V
- method scheduleAtFixedRate (Ljava/lang/Runnable;JJLjava/util/concurrent/TimeUnit;)Ljava/util/concurrent/ScheduledFuture;
- method scheduleAtFixedRate (Ljava/lang/Runnable;JJLjava/util/concurrent/TimeUnit;)Ljava/util/concurrent/ScheduledFuture;
- method shutdownNow ()Ljava/util/List;
- method shutdownNow ()Ljava/util/List;

## java/util/concurrent/ScheduledFuture
- method <init> ()V
- method cancel (Z)Z
- method cancel (Z)Z

## java/util/concurrent/ScheduledThreadPoolExecutor
- method <init> ()V
- method <init> (I)V
- method setContinueExistingPeriodicTasksAfterShutdownPolicy (Z)V
- method setContinueExistingPeriodicTasksAfterShutdownPolicy (Z)V
- method setExecuteExistingDelayedTasksAfterShutdownPolicy (Z)V
- method setExecuteExistingDelayedTasksAfterShutdownPolicy (Z)V

## java/util/concurrent/Semaphore
- method <init> ()V
- method <init> (I)V
- method acquire ()V
- method acquire ()V
- method release ()V
- method release ()V
- method tryAcquire ()Z
- method tryAcquire ()Z

## java/util/concurrent/ThreadFactory
- method <init> ()V
- method newThread (Ljava/lang/Runnable;)Ljava/lang/Thread;
- method newThread (Ljava/lang/Runnable;)Ljava/lang/Thread;

## java/util/concurrent/TimeUnit
- field MILLISECONDS Ljava/util/concurrent/TimeUnit;
- field MILLISECONDS Ljava/util/concurrent/TimeUnit;
- field SECONDS Ljava/util/concurrent/TimeUnit;
- field SECONDS Ljava/util/concurrent/TimeUnit;
- method <init> ()V
- method convert (JLjava/util/concurrent/TimeUnit;)J
- method convert (JLjava/util/concurrent/TimeUnit;)J

## java/util/concurrent/atomic/AtomicInteger
- method <init> ()V
- method <init> (I)V
- method addAndGet (I)I
- method addAndGet (I)I
- method decrementAndGet ()I
- method decrementAndGet ()I
- method get ()I
- method get ()I
- method getAndIncrement ()I
- method getAndIncrement ()I
- method incrementAndGet ()I
- method incrementAndGet ()I
- method set (I)V
- method set (I)V

## java/util/concurrent/atomic/AtomicLong
- method <init> ()V
- method get ()J
- method get ()J
- method incrementAndGet ()J
- method incrementAndGet ()J

## java/util/concurrent/atomic/AtomicReference
- method <init> ()V
- method compareAndSet (Ljava/lang/Object;Ljava/lang/Object;)Z
- method compareAndSet (Ljava/lang/Object;Ljava/lang/Object;)Z
- method get ()Ljava/lang/Object;
- method get ()Ljava/lang/Object;
- method set (Ljava/lang/Object;)V
- method set (Ljava/lang/Object;)V

## java/util/concurrent/locks/Lock
- method <init> ()V
- method lock ()V
- method lock ()V
- method unlock ()V
- method unlock ()V

## java/util/concurrent/locks/ReadWriteLock
- method <init> ()V
- method readLock ()Ljava/util/concurrent/locks/Lock;
- method readLock ()Ljava/util/concurrent/locks/Lock;
- method writeLock ()Ljava/util/concurrent/locks/Lock;
- method writeLock ()Ljava/util/concurrent/locks/Lock;

## java/util/concurrent/locks/ReentrantLock
- method <init> ()V
- method isLocked ()Z
- method isLocked ()Z
- method lock ()V
- method lock ()V
- method unlock ()V
- method unlock ()V

## java/util/concurrent/locks/ReentrantReadWriteLock
- method <init> ()V

## java/util/jar/Attributes$Name
- method <init> ()V

## java/util/jar/JarOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;)V
- method closeEntry ()V
- method closeEntry ()V
- method putNextEntry (Ljava/util/zip/ZipEntry;)V
- method putNextEntry (Ljava/util/zip/ZipEntry;)V
- method write ([B)V
- method write ([B)V

## java/util/jar/Pack200
- method <init> ()V
- method newUnpacker ()Ljava/util/jar/Pack200$Unpacker;
- method newUnpacker ()Ljava/util/jar/Pack200$Unpacker;

## java/util/jar/Pack200$Unpacker
- method <init> ()V
- method unpack (Ljava/io/InputStream;Ljava/util/jar/JarOutputStream;)V
- method unpack (Ljava/io/InputStream;Ljava/util/jar/JarOutputStream;)V

## java/util/logging/Formatter
- method <init> ()V

## java/util/logging/Handler
- method <init> ()V
- method setFormatter (Ljava/util/logging/Formatter;)V
- method setFormatter (Ljava/util/logging/Formatter;)V

## java/util/logging/Level
- field ALL Ljava/util/logging/Level;
- field ALL Ljava/util/logging/Level;
- field CONFIG Ljava/util/logging/Level;
- field CONFIG Ljava/util/logging/Level;
- field FINE Ljava/util/logging/Level;
- field FINE Ljava/util/logging/Level;
- field FINER Ljava/util/logging/Level;
- field FINER Ljava/util/logging/Level;
- field FINEST Ljava/util/logging/Level;
- field FINEST Ljava/util/logging/Level;
- field INFO Ljava/util/logging/Level;
- field INFO Ljava/util/logging/Level;
- field SEVERE Ljava/util/logging/Level;
- field SEVERE Ljava/util/logging/Level;
- field WARNING Ljava/util/logging/Level;
- field WARNING Ljava/util/logging/Level;
- method <init> ()V

## java/util/logging/LogRecord
- method <init> ()V
- method getMessage ()Ljava/lang/String;
- method getMessage ()Ljava/lang/String;
- method getMillis ()J
- method getMillis ()J
- method getSourceClassName ()Ljava/lang/String;
- method getSourceClassName ()Ljava/lang/String;
- method getSourceMethodName ()Ljava/lang/String;
- method getSourceMethodName ()Ljava/lang/String;
- method getThreadID ()I
- method getThreadID ()I
- method getThrown ()Ljava/lang/Throwable;
- method getThrown ()Ljava/lang/Throwable;

## java/util/logging/Logger
- method <init> ()V
- method fine (Ljava/lang/String;)V
- method fine (Ljava/lang/String;)V
- method finer (Ljava/lang/String;)V
- method finer (Ljava/lang/String;)V
- method getHandlers ()[Ljava/util/logging/Handler;
- method getHandlers ()[Ljava/util/logging/Handler;
- method getLogger (Ljava/lang/String;)Ljava/util/logging/Logger;
- method getLogger (Ljava/lang/String;)Ljava/util/logging/Logger;
- method info (Ljava/lang/String;)V
- method info (Ljava/lang/String;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Object;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Object;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Throwable;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;Ljava/lang/Throwable;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;[Ljava/lang/Object;)V
- method log (Ljava/util/logging/Level;Ljava/lang/String;[Ljava/lang/Object;)V
- method setLevel (Ljava/util/logging/Level;)V
- method setLevel (Ljava/util/logging/Level;)V
- method setParent (Ljava/util/logging/Logger;)V
- method setParent (Ljava/util/logging/Logger;)V
- method throwing (Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V
- method throwing (Ljava/lang/String;Ljava/lang/String;Ljava/lang/Throwable;)V
- method warning (Ljava/lang/String;)V
- method warning (Ljava/lang/String;)V

## java/util/prefs/Preferences
- method <init> ()V
- method get (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method get (Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
- method put (Ljava/lang/String;Ljava/lang/String;)V
- method put (Ljava/lang/String;Ljava/lang/String;)V
- method userRoot ()Ljava/util/prefs/Preferences;
- method userRoot ()Ljava/util/prefs/Preferences;

## java/util/regex/Matcher
- method <init> ()V
- method find ()Z
- method find ()Z
- method group (I)Ljava/lang/String;
- method group (I)Ljava/lang/String;
- method groupCount ()I
- method groupCount ()I
- method matches ()Z
- method matches ()Z
- method replaceAll (Ljava/lang/String;)Ljava/lang/String;
- method replaceAll (Ljava/lang/String;)Ljava/lang/String;

## java/util/regex/Pattern
- method <init> ()V
- method compile (Ljava/lang/String;)Ljava/util/regex/Pattern;
- method compile (Ljava/lang/String;)Ljava/util/regex/Pattern;
- method compile (Ljava/lang/String;I)Ljava/util/regex/Pattern;
- method compile (Ljava/lang/String;I)Ljava/util/regex/Pattern;
- method matcher (Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;
- method matcher (Ljava/lang/CharSequence;)Ljava/util/regex/Matcher;
- method matches (Ljava/lang/String;Ljava/lang/CharSequence;)Z
- method matches (Ljava/lang/String;Ljava/lang/CharSequence;)Z
- method pattern ()Ljava/lang/String;
- method pattern ()Ljava/lang/String;
- method split (Ljava/lang/CharSequence;)[Ljava/lang/String;
- method split (Ljava/lang/CharSequence;)[Ljava/lang/String;

## java/util/zip/CRC32
- method <init> ()V
- method getValue ()J
- method getValue ()J
- method reset ()V
- method reset ()V
- method update (I)V
- method update (I)V
- method update ([B)V
- method update ([B)V
- method update ([BII)V
- method update ([BII)V

## java/util/zip/CheckedInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;Ljava/util/zip/Checksum;)V
- method read ()I
- method read ()I

## java/util/zip/CheckedOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;Ljava/util/zip/Checksum;)V
- method write (I)V
- method write (I)V

## java/util/zip/Checksum
- method <init> ()V

## java/util/zip/GZIPInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V
- method close ()V
- method close ()V
- method read ([B)I
- method read ([B)I

## java/util/zip/GZIPOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;)V
- method close ()V
- method close ()V
- method write ([BII)V
- method write ([BII)V

## java/util/zip/Inflater
- method <init> ()V
- method <init> (Z)V
- method end ()V
- method end ()V
- method inflate ([B)I
- method inflate ([B)I
- method setInput ([B)V
- method setInput ([B)V

## java/util/zip/InflaterInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V
- method <init> (Ljava/io/InputStream;Ljava/util/zip/Inflater;)V

## java/util/zip/ZipInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;)V
- method close ()V
- method close ()V
- method closeEntry ()V
- method closeEntry ()V
- method getNextEntry ()Ljava/util/zip/ZipEntry;
- method getNextEntry ()Ljava/util/zip/ZipEntry;
- method read ([B)I
- method read ([B)I

## java/util/zip/ZipOutputStream
- method <init> ()V
- method <init> (Ljava/io/OutputStream;)V
- method close ()V
- method close ()V
- method closeEntry ()V
- method closeEntry ()V
- method finish ()V
- method finish ()V
- method putNextEntry (Ljava/util/zip/ZipEntry;)V
- method putNextEntry (Ljava/util/zip/ZipEntry;)V
- method write ([B)V
- method write ([B)V
- method write ([BII)V
- method write ([BII)V

## javax/accessibility/AccessibleContext
- method <init> ()V
- method setAccessibleDescription (Ljava/lang/String;)V
- method setAccessibleDescription (Ljava/lang/String;)V

## javax/crypto/BadPaddingException
- method <init> ()V

## javax/crypto/Cipher
- method <init> ()V
- method doFinal ([BII[BI)I
- method doFinal ([BII[BI)I
- method getIV ()[B
- method getIV ()[B
- method getInstance (Ljava/lang/String;)Ljavax/crypto/Cipher;
- method getInstance (Ljava/lang/String;)Ljavax/crypto/Cipher;
- method init (ILjava/security/Key;)V
- method init (ILjava/security/Key;)V
- method init (ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V
- method init (ILjava/security/Key;Ljava/security/spec/AlgorithmParameterSpec;)V
- method update ([BII[BI)I
- method update ([BII[BI)I

## javax/crypto/IllegalBlockSizeException
- method <init> ()V

## javax/crypto/NoSuchPaddingException
- method <init> ()V

## javax/crypto/ShortBufferException
- method <init> ()V

## javax/crypto/spec/IvParameterSpec
- method <init> ()V
- method <init> ([BII)V

## javax/crypto/spec/SecretKeySpec
- method <init> ()V
- method <init> ([BIILjava/lang/String;)V

## javax/imageio/ImageIO
- method <init> ()V
- method read (Ljava/io/InputStream;)Ljava/awt/image/BufferedImage;
- method read (Ljava/io/InputStream;)Ljava/awt/image/BufferedImage;
- method read (Ljava/net/URL;)Ljava/awt/image/BufferedImage;
- method read (Ljava/net/URL;)Ljava/awt/image/BufferedImage;
- method write (Ljava/awt/image/RenderedImage;Ljava/lang/String;Ljava/io/File;)Z
- method write (Ljava/awt/image/RenderedImage;Ljava/lang/String;Ljava/io/File;)Z

## javax/microedition/MIDlet
- method <init> ()V
- method getCommandLine ()[Ljava/lang/String;
- method getCommandLine ()[Ljava/lang/String;

## javax/microedition/amms/EffectModule
- method <init> ()V
- method getControls ()[Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;

## javax/microedition/amms/GlobalManager
- method <init> ()V
- method createEffectModule ()Ljavax/microedition/amms/EffectModule;
- method createEffectModule ()Ljavax/microedition/amms/EffectModule;
- method createMediaProcessor (Ljava/lang/String;)Ljavax/microedition/amms/MediaProcessor;
- method createMediaProcessor (Ljava/lang/String;)Ljavax/microedition/amms/MediaProcessor;
- method createSoundSource3D ()Ljavax/microedition/amms/SoundSource3D;
- method createSoundSource3D ()Ljavax/microedition/amms/SoundSource3D;
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;
- method getSpectator ()Ljavax/microedition/amms/Spectator;
- method getSpectator ()Ljavax/microedition/amms/Spectator;
- method getSupportedMediaProcessorInputTypes ()[Ljava/lang/String;
- method getSupportedMediaProcessorInputTypes ()[Ljava/lang/String;
- method getSupportedSoundSource3DPlayerTypes ()[Ljava/lang/String;
- method getSupportedSoundSource3DPlayerTypes ()[Ljava/lang/String;

## javax/microedition/amms/MediaProcessor
- method <init> ()V
- method addMediaProcessorListener (Ljavax/microedition/amms/MediaProcessorListener;)V
- method addMediaProcessorListener (Ljavax/microedition/amms/MediaProcessorListener;)V
- method complete ()V
- method complete ()V
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControl (Ljava/lang/String;)Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;
- method setInput (Ljava/io/InputStream;I)V
- method setInput (Ljava/io/InputStream;I)V
- method setInput (Ljava/lang/Object;)V
- method setInput (Ljava/lang/Object;)V
- method setOutput (Ljava/io/OutputStream;)V
- method setOutput (Ljava/io/OutputStream;)V
- method start ()V
- method start ()V

## javax/microedition/amms/MediaProcessorListener
- method <init> ()V

## javax/microedition/amms/Module
- method <init> ()V
- method addPlayer (Ljavax/microedition/media/Player;)V
- method addPlayer (Ljavax/microedition/media/Player;)V

## javax/microedition/amms/SoundSource3D
- method <init> ()V
- method getControls ()[Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;

## javax/microedition/amms/Spectator
- method <init> ()V
- method getControls ()[Ljavax/microedition/media/Control;
- method getControls ()[Ljavax/microedition/media/Control;

## javax/microedition/amms/control/AudioFormatControl
- method <init> ()V

## javax/microedition/amms/control/ContainerFormatControl
- method <init> ()V

## javax/microedition/amms/control/EffectControl
- method <init> ()V
- method getPreset ()Ljava/lang/String;
- method getPreset ()Ljava/lang/String;
- method getPresetNames ()[Ljava/lang/String;
- method getPresetNames ()[Ljava/lang/String;
- method isEnabled ()Z
- method isEnabled ()Z
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setEnforced (Z)V
- method setEnforced (Z)V
- method setPreset (Ljava/lang/String;)V
- method setPreset (Ljava/lang/String;)V

## javax/microedition/amms/control/EffectOrderControl
- method <init> ()V

## javax/microedition/amms/control/FormatControl
- method <init> ()V
- method getSupportedFormats ()[Ljava/lang/String;
- method getSupportedFormats ()[Ljava/lang/String;
- method getSupportedMetadataKeys ()[Ljava/lang/String;
- method getSupportedMetadataKeys ()[Ljava/lang/String;
- method setFormat (Ljava/lang/String;)V
- method setFormat (Ljava/lang/String;)V
- method setParameter (Ljava/lang/String;I)I
- method setParameter (Ljava/lang/String;I)I
- method setParameter (Ljava/lang/String;Ljava/lang/String;)V
- method setParameter (Ljava/lang/String;Ljava/lang/String;)V

## javax/microedition/amms/control/ImageFormatControl
- method <init> ()V
- method setFormat (Ljava/lang/String;)V
- method setFormat (Ljava/lang/String;)V
- method setParameter (Ljava/lang/String;I)I
- method setParameter (Ljava/lang/String;I)I
- method setParameter (Ljava/lang/String;Ljava/lang/String;)V
- method setParameter (Ljava/lang/String;Ljava/lang/String;)V

## javax/microedition/amms/control/MIDIChannelControl
- method <init> ()V

## javax/microedition/amms/control/PanControl
- method <init> ()V

## javax/microedition/amms/control/PriorityControl
- method <init> ()V

## javax/microedition/amms/control/VideoFormatControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/CommitControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/DirectivityControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/DistanceAttenuationControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/DopplerControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/LocationControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/MacroscopicControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/ObstructionControl
- method <init> ()V

## javax/microedition/amms/control/audio3d/OrientationControl
- method <init> ()V

## javax/microedition/amms/control/audioeffect/AudioVirtualizerControl
- method <init> ()V

## javax/microedition/amms/control/audioeffect/ChorusControl
- method <init> ()V

## javax/microedition/amms/control/audioeffect/EqualizerControl
- method <init> ()V
- method getBand (I)I
- method getBand (I)I
- method getBandLevel (I)I
- method getBandLevel (I)I
- method getBass ()I
- method getBass ()I
- method getCenterFreq (I)I
- method getCenterFreq (I)I
- method getMaxBandLevel ()I
- method getMaxBandLevel ()I
- method getMinBandLevel ()I
- method getMinBandLevel ()I
- method getNumberOfBands ()I
- method getNumberOfBands ()I
- method getTreble ()I
- method getTreble ()I
- method setBandLevel (II)V
- method setBandLevel (II)V
- method setBass (I)I
- method setBass (I)I
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setTreble (I)I
- method setTreble (I)I

## javax/microedition/amms/control/audioeffect/ReverbControl
- method <init> ()V
- method getReverbLevel ()I
- method getReverbLevel ()I
- method getReverbTime ()I
- method getReverbTime ()I
- method setReverbLevel (I)I
- method setReverbLevel (I)I
- method setReverbTime (I)V
- method setReverbTime (I)V

## javax/microedition/amms/control/audioeffect/ReverbSourceControl
- method <init> ()V

## javax/microedition/amms/control/camera/CameraControl
- method <init> ()V
- method enableShutterFeedback (Z)V
- method enableShutterFeedback (Z)V
- method getCameraRotation ()I
- method getCameraRotation ()I
- method getExposureMode ()Ljava/lang/String;
- method getExposureMode ()Ljava/lang/String;
- method getStillResolution ()I
- method getStillResolution ()I
- method getSupportedExposureModes ()[Ljava/lang/String;
- method getSupportedExposureModes ()[Ljava/lang/String;
- method getSupportedStillResolutions ()[I
- method getSupportedStillResolutions ()[I
- method getSupportedVideoResolutions ()[I
- method getSupportedVideoResolutions ()[I
- method isShutterFeedbackEnabled ()Z
- method isShutterFeedbackEnabled ()Z
- method setExposureMode (Ljava/lang/String;)V
- method setExposureMode (Ljava/lang/String;)V
- method setStillResolution (I)V
- method setStillResolution (I)V

## javax/microedition/amms/control/camera/ExposureControl
- method <init> ()V
- method getExposureCompensation ()I
- method getExposureCompensation ()I
- method getISO ()I
- method getISO ()I
- method getSupportedExposureCompensations ()[I
- method getSupportedExposureCompensations ()[I
- method getSupportedFStops ()[I
- method getSupportedFStops ()[I
- method getSupportedISOs ()[I
- method getSupportedISOs ()[I
- method getSupportedLightMeterings ()[Ljava/lang/String;
- method getSupportedLightMeterings ()[Ljava/lang/String;
- method setExposureCompensation (I)V
- method setExposureCompensation (I)V
- method setISO (I)V
- method setISO (I)V
- method setLightMetering (Ljava/lang/String;)V
- method setLightMetering (Ljava/lang/String;)V

## javax/microedition/amms/control/camera/FlashControl
- method <init> ()V
- method getSupportedModes ()[I
- method getSupportedModes ()[I
- method isFlashReady ()Z
- method isFlashReady ()Z
- method setMode (I)V
- method setMode (I)V

## javax/microedition/amms/control/camera/FocusControl
- method <init> ()V
- method getFocusSteps ()I
- method getFocusSteps ()I
- method getMacro ()Z
- method getMacro ()Z
- method getMinFocus ()I
- method getMinFocus ()I
- method isAutoFocusSupported ()Z
- method isAutoFocusSupported ()Z
- method isMacroSupported ()Z
- method isMacroSupported ()Z
- method isManualFocusSupported ()Z
- method isManualFocusSupported ()Z
- method setFocus (I)I
- method setFocus (I)I
- method setMacro (Z)V
- method setMacro (Z)V

## javax/microedition/amms/control/camera/SnapshotControl
- method <init> ()V
- method getFilePrefix ()Ljava/lang/String;
- method getFilePrefix ()Ljava/lang/String;
- method getFileSuffix ()Ljava/lang/String;
- method getFileSuffix ()Ljava/lang/String;
- method setDirectory (Ljava/lang/String;)V
- method setDirectory (Ljava/lang/String;)V
- method setFilePrefix (Ljava/lang/String;)V
- method setFilePrefix (Ljava/lang/String;)V
- method setFileSuffix (Ljava/lang/String;)V
- method setFileSuffix (Ljava/lang/String;)V
- method start (I)V
- method start (I)V
- method stop ()V
- method stop ()V
- method unfreeze (Z)V
- method unfreeze (Z)V

## javax/microedition/amms/control/camera/ZoomControl
- method <init> ()V
- method getDigitalZoom ()I
- method getDigitalZoom ()I
- method getDigitalZoomLevels ()I
- method getDigitalZoomLevels ()I
- method getMaxDigitalZoom ()I
- method getMaxDigitalZoom ()I
- method getMaxOpticalZoom ()I
- method getMaxOpticalZoom ()I
- method getMinFocalLength ()I
- method getMinFocalLength ()I
- method getOpticalZoom ()I
- method getOpticalZoom ()I
- method getOpticalZoomLevels ()I
- method getOpticalZoomLevels ()I
- method setDigitalZoom (I)I
- method setDigitalZoom (I)I
- method setOpticalZoom (I)I
- method setOpticalZoom (I)I

## javax/microedition/amms/control/imageeffect/ImageEffectControl
- method <init> ()V

## javax/microedition/amms/control/imageeffect/ImageTonalityControl
- method <init> ()V

## javax/microedition/amms/control/imageeffect/ImageTransformControl
- method <init> ()V
- method getSourceHeight ()I
- method getSourceHeight ()I
- method getSourceWidth ()I
- method getSourceWidth ()I
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setEnforced (Z)V
- method setEnforced (Z)V
- method setSourceRect (IIII)V
- method setSourceRect (IIII)V
- method setTargetSize (III)V
- method setTargetSize (III)V

## javax/microedition/amms/control/imageeffect/OverlayControl
- method <init> ()V

## javax/microedition/amms/control/imageeffect/WhiteBalanceControl
- method <init> ()V

## javax/microedition/amms/control/tuner/RDSControl
- method <init> ()V
- method getAutomaticSwitching ()Z
- method getAutomaticSwitching ()Z
- method getAutomaticTA ()Z
- method getAutomaticTA ()Z
- method getCT ()Ljava/util/Date;
- method getCT ()Ljava/util/Date;
- method getPS ()Ljava/lang/String;
- method getPS ()Ljava/lang/String;
- method getPTYString (Z)Ljava/lang/String;
- method getPTYString (Z)Ljava/lang/String;
- method getRT ()Ljava/lang/String;
- method getRT ()Ljava/lang/String;
- method isRDSSignal ()Z
- method isRDSSignal ()Z
- method setAutomaticSwitching (Z)V
- method setAutomaticSwitching (Z)V
- method setAutomaticTA (Z)V
- method setAutomaticTA (Z)V

## javax/microedition/amms/control/tuner/TunerControl
- method <init> ()V
- method getFrequency ()I
- method getFrequency ()I
- method getMaxFreq (Ljava/lang/String;)I
- method getMaxFreq (Ljava/lang/String;)I
- method getMinFreq (Ljava/lang/String;)I
- method getMinFreq (Ljava/lang/String;)I
- method getModulation ()Ljava/lang/String;
- method getModulation ()Ljava/lang/String;
- method getNumberOfPresets ()I
- method getNumberOfPresets ()I
- method getPresetFrequency (I)I
- method getPresetFrequency (I)I
- method getPresetModulation (I)Ljava/lang/String;
- method getPresetModulation (I)Ljava/lang/String;
- method getPresetName (I)Ljava/lang/String;
- method getPresetName (I)Ljava/lang/String;
- method getPresetStereoMode (I)I
- method getPresetStereoMode (I)I
- method getSignalStrength ()I
- method getSignalStrength ()I
- method getSquelch ()Z
- method getSquelch ()Z
- method getStereoMode ()I
- method getStereoMode ()I
- method seek (ILjava/lang/String;Z)I
- method seek (ILjava/lang/String;Z)I
- method setFrequency (ILjava/lang/String;)I
- method setFrequency (ILjava/lang/String;)I
- method setPreset (I)V
- method setPreset (I)V
- method setPreset (IILjava/lang/String;I)V
- method setPreset (IILjava/lang/String;I)V
- method setPresetName (ILjava/lang/String;)V
- method setPresetName (ILjava/lang/String;)V
- method setSquelch (Z)V
- method setSquelch (Z)V
- method setStereoMode (I)V
- method setStereoMode (I)V
- method usePreset (I)V
- method usePreset (I)V

## javax/microedition/content/ActionNameMap
- method <init> ()V
- method <init> ([Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)V

## javax/microedition/content/ContentHandler
- field ACTION_OPEN Ljava/lang/String;
- field ACTION_OPEN Ljava/lang/String;
- method <init> ()V
- method getAction (I)Ljava/lang/String;
- method getAction (I)Ljava/lang/String;
- method getActionCount ()I
- method getActionCount ()I
- method getAppName ()Ljava/lang/String;
- method getAppName ()Ljava/lang/String;
- method getAuthority ()Ljava/lang/String;
- method getAuthority ()Ljava/lang/String;
- method getID ()Ljava/lang/String;
- method getID ()Ljava/lang/String;
- method hasAction (Ljava/lang/String;)Z
- method hasAction (Ljava/lang/String;)Z

## javax/microedition/content/ContentHandlerException
- method <init> ()V
- method <init> (Ljava/lang/String;I)V
- method getErrorCode ()I
- method getErrorCode ()I

## javax/microedition/content/ContentHandlerServer
- method <init> ()V
- method finish (Ljavax/microedition/content/Invocation;I)Z
- method finish (Ljavax/microedition/content/Invocation;I)Z
- method getID ()Ljava/lang/String;
- method getID ()Ljava/lang/String;
- method getRequest (Z)Ljavax/microedition/content/Invocation;
- method getRequest (Z)Ljavax/microedition/content/Invocation;
- method setListener (Ljavax/microedition/content/RequestListener;)V
- method setListener (Ljavax/microedition/content/RequestListener;)V

## javax/microedition/content/Invocation
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ZLjava/lang/String;)V
- method findType ()Ljava/lang/String;
- method findType ()Ljava/lang/String;
- method getAction ()Ljava/lang/String;
- method getAction ()Ljava/lang/String;
- method getArgs ()[Ljava/lang/String;
- method getArgs ()[Ljava/lang/String;
- method getData ()[B
- method getData ()[B
- method getID ()Ljava/lang/String;
- method getID ()Ljava/lang/String;
- method getInvokingAppName ()Ljava/lang/String;
- method getInvokingAppName ()Ljava/lang/String;
- method getInvokingAuthority ()Ljava/lang/String;
- method getInvokingAuthority ()Ljava/lang/String;
- method getResponseRequired ()Z
- method getResponseRequired ()Z
- method getStatus ()I
- method getStatus ()I
- method getType ()Ljava/lang/String;
- method getType ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method getURL ()Ljava/lang/String;
- method setAction (Ljava/lang/String;)V
- method setAction (Ljava/lang/String;)V
- method setArgs ([Ljava/lang/String;)V
- method setArgs ([Ljava/lang/String;)V
- method setID (Ljava/lang/String;)V
- method setID (Ljava/lang/String;)V
- method setResponseRequired (Z)V
- method setResponseRequired (Z)V

## javax/microedition/content/Registry
- method <init> ()V
- method findHandler (Ljavax/microedition/content/Invocation;)[Ljavax/microedition/content/ContentHandler;
- method findHandler (Ljavax/microedition/content/Invocation;)[Ljavax/microedition/content/ContentHandler;
- method forID (Ljava/lang/String;Z)Ljavax/microedition/content/ContentHandler;
- method forID (Ljava/lang/String;Z)Ljavax/microedition/content/ContentHandler;
- method getRegistry (Ljava/lang/String;)Ljavax/microedition/content/Registry;
- method getRegistry (Ljava/lang/String;)Ljavax/microedition/content/Registry;
- method getResponse (Z)Ljavax/microedition/content/Invocation;
- method getResponse (Z)Ljavax/microedition/content/Invocation;
- method getServer (Ljava/lang/String;)Ljavax/microedition/content/ContentHandlerServer;
- method getServer (Ljava/lang/String;)Ljavax/microedition/content/ContentHandlerServer;
- method getSuffixes ()[Ljava/lang/String;
- method getSuffixes ()[Ljava/lang/String;
- method invoke (Ljavax/microedition/content/Invocation;)Z
- method invoke (Ljavax/microedition/content/Invocation;)Z
- method register (Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljavax/microedition/content/ActionNameMap;Ljava/lang/String;[Ljava/lang/String;)Ljavax/microedition/content/ContentHandlerServer;
- method register (Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljava/lang/String;[Ljavax/microedition/content/ActionNameMap;Ljava/lang/String;[Ljava/lang/String;)Ljavax/microedition/content/ContentHandlerServer;
- method setListener (Ljavax/microedition/content/ResponseListener;)V
- method setListener (Ljavax/microedition/content/ResponseListener;)V
- method unregister (Ljava/lang/String;)Z
- method unregister (Ljava/lang/String;)Z

## javax/microedition/content/RequestListener
- method <init> ()V

## javax/microedition/content/ResponseListener
- method <init> ()V

## javax/microedition/global/Formatter
- method <init> ()V
- method formatMessage (Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/String;
- method formatMessage (Ljava/lang/String;[Ljava/lang/String;)Ljava/lang/String;

## javax/microedition/global/ResourceManager
- method <init> ()V
- method getManager (Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/global/ResourceManager;
- method getManager (Ljava/lang/String;Ljava/lang/String;)Ljavax/microedition/global/ResourceManager;
- method getString (I)Ljava/lang/String;
- method getString (I)Ljava/lang/String;

## javax/microedition/global/StringComparator
- method <init> ()V
- method compare (Ljava/lang/String;Ljava/lang/String;)I
- method compare (Ljava/lang/String;Ljava/lang/String;)I

## javax/microedition/io/Con
- method <init> ()V
- method open (Ljava/lang/String;)Ljavax/microedition/io/Con;
- method open (Ljava/lang/String;)Ljavax/microedition/io/Con;
- method open (Ljava/lang/String;I)Ljavax/microedition/io/Con;
- method open (Ljava/lang/String;I)Ljavax/microedition/io/Con;

## javax/microedition/io/Sockavax/microedition/io/sockettConnection
- method <init> ()V

## javax/microedition/io/file/ConnectionClosedException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/io/file/FileCon
- method <init> ()V
- method openInputStream ()Ljava/io/InputStream;
- method openInputStream ()Ljava/io/InputStream;

## javax/microedition/io/file/IllegalModeException
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/khronos/opengles/GL10Ext
- method <init> ()V

## javax/microedition/khronos/opengles/GL11
- method <init> ()V

## javax/microedition/khronos/opengles/GL11Ext
- method <init> ()V

## javax/microedition/location/AddressInfo
- method <init> ()V
- method getField (I)Ljava/lang/String;
- method getField (I)Ljava/lang/String;
- method setField (ILjava/lang/String;)V
- method setField (ILjava/lang/String;)V

## javax/microedition/location/Landmark
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;Ljavax/microedition/location/QualifiedCoordinates;Ljavax/microedition/location/AddressInfo;)V
- method getAddressInfo ()Ljavax/microedition/location/AddressInfo;
- method getAddressInfo ()Ljavax/microedition/location/AddressInfo;
- method getDescription ()Ljava/lang/String;
- method getDescription ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getQualifiedCoordinates ()Ljavax/microedition/location/QualifiedCoordinates;
- method getQualifiedCoordinates ()Ljavax/microedition/location/QualifiedCoordinates;
- method setAddressInfo (Ljavax/microedition/location/AddressInfo;)V
- method setAddressInfo (Ljavax/microedition/location/AddressInfo;)V
- method setDescription (Ljava/lang/String;)V
- method setDescription (Ljava/lang/String;)V
- method setName (Ljava/lang/String;)V
- method setName (Ljava/lang/String;)V
- method setQualifiedCoordinates (Ljavax/microedition/location/QualifiedCoordinates;)V
- method setQualifiedCoordinates (Ljavax/microedition/location/QualifiedCoordinates;)V

## javax/microedition/location/LandmarkException
- method <init> ()V

## javax/microedition/location/LandmarkStore
- method <init> ()V
- method addCategory (Ljava/lang/String;)V
- method addCategory (Ljava/lang/String;)V
- method addLandmark (Ljavax/microedition/location/Landmark;Ljava/lang/String;)V
- method addLandmark (Ljavax/microedition/location/Landmark;Ljava/lang/String;)V
- method deleteLandmark (Ljavax/microedition/location/Landmark;)V
- method deleteLandmark (Ljavax/microedition/location/Landmark;)V
- method getCategories ()Ljava/util/Enumeration;
- method getCategories ()Ljava/util/Enumeration;
- method getInstance (Ljava/lang/String;)Ljavax/microedition/location/LandmarkStore;
- method getInstance (Ljava/lang/String;)Ljavax/microedition/location/LandmarkStore;
- method getLandmarks ()Ljava/util/Enumeration;
- method getLandmarks ()Ljava/util/Enumeration;
- method getLandmarks (Ljava/lang/String;Ljava/lang/String;)Ljava/util/Enumeration;
- method getLandmarks (Ljava/lang/String;Ljava/lang/String;)Ljava/util/Enumeration;
- method listLandmarkStores ()[Ljava/lang/String;
- method listLandmarkStores ()[Ljava/lang/String;
- method updateLandmark (Ljavax/microedition/location/Landmark;)V
- method updateLandmark (Ljavax/microedition/location/Landmark;)V

## javax/microedition/location/Orientation
- method <init> ()V
- method getCompassAzimuth ()F
- method getCompassAzimuth ()F
- method getOrientation ()Ljavax/microedition/location/Orientation;
- method getOrientation ()Ljavax/microedition/location/Orientation;
- method getPitch ()F
- method getPitch ()F
- method getRoll ()F
- method getRoll ()F
- method isOrientationMagnetic ()Z
- method isOrientationMagnetic ()Z

## javax/microedition/m2g/ExternalResourceHandler
- method <init> ()V

## javax/microedition/m2g/SVGAnimator
- method <init> ()V
- method createAnimator (Ljavax/microedition/m2g/SVGImage;)Ljavax/microedition/m2g/SVGAnimator;
- method createAnimator (Ljavax/microedition/m2g/SVGImage;)Ljavax/microedition/m2g/SVGAnimator;
- method getTargetComponent ()Ljava/lang/Object;
- method getTargetComponent ()Ljava/lang/Object;
- method getTimeIncrement ()F
- method getTimeIncrement ()F
- method invokeAndWait (Ljava/lang/Runnable;)V
- method invokeAndWait (Ljava/lang/Runnable;)V
- method invokeLater (Ljava/lang/Runnable;)V
- method invokeLater (Ljava/lang/Runnable;)V
- method pause ()V
- method pause ()V
- method play ()V
- method play ()V
- method setSVGEventListener (Ljavax/microedition/m2g/SVGEventListener;)V
- method setSVGEventListener (Ljavax/microedition/m2g/SVGEventListener;)V
- method setTimeIncrement (F)V
- method setTimeIncrement (F)V
- method stop ()V
- method stop ()V

## javax/microedition/m2g/SVGEventListener
- method <init> ()V
- method hideNotify ()V
- method hideNotify ()V
- method keyPressed (I)V
- method keyPressed (I)V
- method keyReleased (I)V
- method keyReleased (I)V
- method leyPressed (I)V
- method leyPressed (I)V
- method leyReleased (I)V
- method leyReleased (I)V
- method pointerPressed (II)V
- method pointerPressed (II)V
- method pointerReleased (II)V
- method pointerReleased (II)V
- method showNotify ()V
- method showNotify ()V
- method sizeChanged (II)V
- method sizeChanged (II)V

## javax/microedition/m2g/SVGImage
- method <init> ()V
- method activate ()V
- method activate ()V
- method createEmptyImage (Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/SVGImage;
- method createEmptyImage (Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/SVGImage;
- method createImage (Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;
- method createImage (Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;
- method focusOn (Lorg/w3c/dom/svg/SVGElement;)V
- method focusOn (Lorg/w3c/dom/svg/SVGElement;)V
- method getDocument ()Lorg/w3c/dom/Document;
- method getDocument ()Lorg/w3c/dom/Document;
- method getViewportHeight ()I
- method getViewportHeight ()I
- method getViewportWidth ()I
- method getViewportWidth ()I
- method incrementTime (F)V
- method incrementTime (F)V
- method setViewportHeight (I)V
- method setViewportHeight (I)V
- method setViewportWidth (I)V
- method setViewportWidth (I)V

## javax/microedition/m2g/ScalableGraphics
- method <init> ()V
- method bindTarget (Ljava/lang/Object;)V
- method bindTarget (Ljava/lang/Object;)V
- method createInstance ()Ljavax/microedition/m2g/ScalableGraphics;
- method createInstance ()Ljavax/microedition/m2g/ScalableGraphics;
- method releaseTarget ()V
- method releaseTarget ()V
- method render (IILjavax/microedition/m2g/ScalableImage;)V
- method render (IILjavax/microedition/m2g/ScalableImage;)V
- method setRenderingQuality (I)V
- method setRenderingQuality (I)V
- method setTransparency (F)V
- method setTransparency (F)V

## javax/microedition/m2g/ScalableImage
- method <init> ()V
- method createImage (Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;
- method createImage (Ljava/io/InputStream;Ljavax/microedition/m2g/ExternalResourceHandler;)Ljavax/microedition/m2g/ScalableImage;
- method getViewportHeight ()I
- method getViewportHeight ()I
- method getViewportWidth ()I
- method getViewportWidth ()I
- method requestCompleted (Ljava/lang/String;Ljava/io/InputStream;)V
- method requestCompleted (Ljava/lang/String;Ljava/io/InputStream;)V
- method setViewportHeight (I)V
- method setViewportHeight (I)V
- method setViewportWidth (I)V
- method setViewportWidth (I)V

## javax/microedition/media/TimeBase
- method <init> ()V
- method getTime ()J
- method getTime ()J

## javax/microedition/media/control/FramePositioningControl
- method <init> ()V
- method mapFrameToTime (I)J
- method mapFrameToTime (I)J
- method mapTimeToFrame (J)I
- method mapTimeToFrame (J)I
- method seek (I)I
- method seek (I)I
- method skip (I)I
- method skip (I)I

## javax/microedition/media/control/GUIControl
- method <init> ()V
- method initDisplayMode (ILjava/lang/Object;)Ljava/lang/Object;
- method initDisplayMode (ILjava/lang/Object;)Ljava/lang/Object;

## javax/microedition/media/control/PitchControl
- method <init> ()V
- method getMaxPitch ()I
- method getMaxPitch ()I
- method getMinPitch ()I
- method getMinPitch ()I
- method getPitch ()I
- method getPitch ()I
- method setPitch (I)I
- method setPitch (I)I

## javax/microedition/media/control/RateControl
- method <init> ()V
- method getMaxRate ()I
- method getMaxRate ()I
- method getMinRate ()I
- method getMinRate ()I
- method getRate ()I
- method getRate ()I
- method setRate (I)I
- method setRate (I)I

## javax/microedition/media/control/RecordControl
- method <init> ()V
- method commit ()V
- method commit ()V
- method getContentType ()Ljava/lang/String;
- method getContentType ()Ljava/lang/String;
- method reset ()V
- method reset ()V
- method setRecordLocation (Ljava/lang/String;)V
- method setRecordLocation (Ljava/lang/String;)V
- method setRecordSizeLimit (I)I
- method setRecordSizeLimit (I)I
- method setRecordStream (Ljava/io/OutputStream;)V
- method setRecordStream (Ljava/io/OutputStream;)V
- method startRecord ()V
- method startRecord ()V
- method stopRecord ()V
- method stopRecord ()V

## javax/microedition/media/control/TempoControl
- method <init> ()V
- method getTempo ()I
- method getTempo ()I
- method setTempo (I)I
- method setTempo (I)I

## javax/microedition/media/protocol/ContentDescriptor
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/media/protocol/DataSource
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/microedition/media/protocol/SourceStream
- method <init> ()V

## javax/microedition/pim/Event
- method <init> ()V
- method addDate (IIJ)V
- method addDate (IIJ)V
- method addInt (III)V
- method addInt (III)V
- method addString (IILjava/lang/String;)V
- method addString (IILjava/lang/String;)V
- method addToCategory (Ljava/lang/String;)V
- method addToCategory (Ljava/lang/String;)V
- method commit ()V
- method commit ()V
- method countValues (I)I
- method countValues (I)I
- method getDate (II)J
- method getDate (II)J
- method getRepeat ()Ljavax/microedition/pim/RepeatRule;
- method getRepeat ()Ljavax/microedition/pim/RepeatRule;
- method getString (II)Ljava/lang/String;
- method getString (II)Ljava/lang/String;

## javax/microedition/pim/EventList
- method <init> ()V
- method close ()V
- method close ()V
- method createEvent ()Ljavax/microedition/pim/Event;
- method createEvent ()Ljavax/microedition/pim/Event;
- method getCategories ()[Ljava/lang/String;
- method getCategories ()[Ljava/lang/String;
- method importEvent (Ljavax/microedition/pim/Event;)Ljavax/microedition/pim/Event;
- method importEvent (Ljavax/microedition/pim/Event;)Ljavax/microedition/pim/Event;
- method isCategory (Ljava/lang/String;)Z
- method isCategory (Ljava/lang/String;)Z
- method isSupportedField (I)Z
- method isSupportedField (I)Z
- method items ()Ljava/util/Enumeration;
- method items ()Ljava/util/Enumeration;
- method items (IJJZ)Ljava/util/Enumeration;
- method items (IJJZ)Ljava/util/Enumeration;
- method maxCategories ()I
- method maxCategories ()I
- method removeEvent (Ljavax/microedition/pim/Event;)V
- method removeEvent (Ljavax/microedition/pim/Event;)V

## javax/microedition/pim/FieldEmptyException
- method <init> ()V

## javax/microedition/pim/FieldFullException
- method <init> ()V

## javax/microedition/pim/PIMItem
- method <init> ()V
- method addBoolean (IIZ)V
- method addBoolean (IIZ)V
- method addDate (IIJ)V
- method addDate (IIJ)V
- method addInt (III)V
- method addInt (III)V
- method addString (IILjava/lang/String;)V
- method addString (IILjava/lang/String;)V
- method addStringArray (II[Ljava/lang/String;)V
- method addStringArray (II[Ljava/lang/String;)V
- method commit ()V
- method commit ()V
- method countValues (I)I
- method countValues (I)I
- method getAttributes (II)I
- method getAttributes (II)I
- method getBinary (II)[B
- method getBinary (II)[B
- method getBoolean (II)Z
- method getBoolean (II)Z
- method getDate (II)J
- method getDate (II)J
- method getFields ()[I
- method getFields ()[I
- method getInt (II)I
- method getInt (II)I
- method getPIMList ()Ljavax/microedition/pim/PIMList;
- method getPIMList ()Ljavax/microedition/pim/PIMList;
- method getString (II)Ljava/lang/String;
- method getString (II)Ljava/lang/String;
- method getStringArray (II)[Ljava/lang/String;
- method getStringArray (II)[Ljava/lang/String;
- method removeValue (II)V
- method removeValue (II)V
- method setBoolean (IIIZ)V
- method setBoolean (IIIZ)V
- method setDate (IIIJ)V
- method setDate (IIIJ)V
- method setInt (IIII)V
- method setInt (IIII)V
- method setString (IIILjava/lang/String;)V
- method setString (IIILjava/lang/String;)V
- method setStringArray (III[Ljava/lang/String;)V
- method setStringArray (III[Ljava/lang/String;)V

## javax/microedition/pim/RepeatRule
- method <init> ()V
- method dates (JJJ)Ljava/util/Enumeration;
- method dates (JJJ)Ljava/util/Enumeration;
- method getDate (I)J
- method getDate (I)J
- method getExceptDates ()Ljava/util/Enumeration;
- method getExceptDates ()Ljava/util/Enumeration;
- method getFields ()[I
- method getFields ()[I
- method getInt (I)I
- method getInt (I)I

## javax/microedition/pim/ToDo
- method <init> ()V
- method addDate (IIJ)V
- method addDate (IIJ)V
- method addString (IILjava/lang/String;)V
- method addString (IILjava/lang/String;)V
- method commit ()V
- method commit ()V
- method getString (II)Ljava/lang/String;
- method getString (II)Ljava/lang/String;

## javax/microedition/pim/ToDoList
- method <init> ()V
- method close ()V
- method close ()V
- method createToDo ()Ljavax/microedition/pim/ToDo;
- method createToDo ()Ljavax/microedition/pim/ToDo;
- method importToDo (Ljavax/microedition/pim/ToDo;)Ljavax/microedition/pim/ToDo;
- method importToDo (Ljavax/microedition/pim/ToDo;)Ljavax/microedition/pim/ToDo;
- method isSupportedField (I)Z
- method isSupportedField (I)Z
- method items ()Ljava/util/Enumeration;
- method items ()Ljava/util/Enumeration;
- method removeToDo (Ljavax/microedition/pim/ToDo;)V
- method removeToDo (Ljavax/microedition/pim/ToDo;)V

## javax/microedition/pki/Certificate
- method <init> ()V
- method getIssuer ()Ljava/lang/String;
- method getIssuer ()Ljava/lang/String;
- method getNotAfter ()J
- method getNotAfter ()J
- method getNotBefore ()J
- method getNotBefore ()J
- method getSerialNumber ()Ljava/lang/String;
- method getSerialNumber ()Ljava/lang/String;
- method getSigAlgName ()Ljava/lang/String;
- method getSigAlgName ()Ljava/lang/String;
- method getSubject ()Ljava/lang/String;
- method getSubject ()Ljava/lang/String;
- method getType ()Ljava/lang/String;
- method getType ()Ljava/lang/String;
- method getVersion ()Ljava/lang/String;
- method getVersion ()Ljava/lang/String;

## javax/microedition/pki/CertificateException
- method <init> ()V
- method <init> (Ljavax/microedition/pki/Certificate;B)V
- method getCertificate ()Ljavax/microedition/pki/Certificate;
- method getCertificate ()Ljavax/microedition/pki/Certificate;
- method getReason ()B
- method getReason ()B

## javax/microedition/rms/RecordStore$RecordHeader
- field id I
- field id I
- method <init> ()V

## javax/microedition/sensor/MeasurementRange
- method <init> ()V
- method getLargestValue ()D
- method getLargestValue ()D
- method getResolution ()D
- method getResolution ()D
- method getSmallestValue ()D
- method getSmallestValue ()D

## javax/microedition/sensor/Unit
- method <init> ()V

## javax/microedition/xml/rpc/ComplexType
- field elements [Ljavax/microedition/xml/rpc/Element;
- field elements [Ljavax/microedition/xml/rpc/Element;
- method <init> ()V

## javax/microedition/xml/rpc/Element
- method <init> ()V
- method <init> (Ljavax/xml/namespace/QName;Ljavax/microedition/xml/rpc/Type;)V
- method <init> (Ljavax/xml/namespace/QName;Ljavax/microedition/xml/rpc/Type;IIZ)V

## javax/microedition/xml/rpc/Operation
- method <init> ()V
- method invoke (Ljava/lang/Object;)Ljava/lang/Object;
- method invoke (Ljava/lang/Object;)Ljava/lang/Object;
- method newInstance (Ljavax/xml/namespace/QName;Ljavax/microedition/xml/rpc/Element;Ljavax/microedition/xml/rpc/Element;)Ljavax/microedition/xml/rpc/Operation;
- method newInstance (Ljavax/xml/namespace/QName;Ljavax/microedition/xml/rpc/Element;Ljavax/microedition/xml/rpc/Element;)Ljavax/microedition/xml/rpc/Operation;
- method setProperty (Ljava/lang/String;Ljava/lang/String;)V
- method setProperty (Ljava/lang/String;Ljava/lang/String;)V

## javax/microedition/xml/rpc/Type
- field BOOLEAN Ljavax/microedition/xml/rpc/Type;
- field BOOLEAN Ljavax/microedition/xml/rpc/Type;
- field INT Ljavax/microedition/xml/rpc/Type;
- field INT Ljavax/microedition/xml/rpc/Type;
- field STRING Ljavax/microedition/xml/rpc/Type;
- field STRING Ljavax/microedition/xml/rpc/Type;
- method <init> ()V

## javax/net/ssl/HostnameVerifier
- method <init> ()V

## javax/net/ssl/HttpsURLConnection
- method <init> ()V
- method setDefaultHostnameVerifier (Ljavax/net/ssl/HostnameVerifier;)V
- method setDefaultHostnameVerifier (Ljavax/net/ssl/HostnameVerifier;)V
- method setHostnameVerifier (Ljavax/net/ssl/HostnameVerifier;)V
- method setHostnameVerifier (Ljavax/net/ssl/HostnameVerifier;)V

## javax/obex/Authenticator
- method <init> ()V

## javax/obex/PasswordAuthentication
- method <init> ()V
- method <init> ([B[B)V

## javax/obex/ServerRequestHandler
- method <init> ()V

## javax/obex/SessionNotifier
- method <init> ()V
- method acceptAndOpen (Ljavax/obex/ServerRequestHandler;)Ljavax/microedition/io/Connection;
- method acceptAndOpen (Ljavax/obex/ServerRequestHandler;)Ljavax/microedition/io/Connection;
- method close ()V
- method close ()V

## javax/servlet/ServletException
- method <init> ()V

## javax/servlet/http/HttpServlet
- method <init> ()V
- method init (Ljavax/servlet/ServletConfig;)V
- method init (Ljavax/servlet/ServletConfig;)V

## javax/servlet/http/HttpServletRequest
- method <init> ()V
- method getParameter (Ljava/lang/String;)Ljava/lang/String;
- method getParameter (Ljava/lang/String;)Ljava/lang/String;

## javax/servlet/http/HttpServletResponse
- method <init> ()V
- method addDateHeader (Ljava/lang/String;J)V
- method addDateHeader (Ljava/lang/String;J)V
- method getWriter ()Ljava/io/PrintWriter;
- method getWriter ()Ljava/io/PrintWriter;
- method setContentType (Ljava/lang/String;)V
- method setContentType (Ljava/lang/String;)V
- method setHeader (Ljava/lang/String;Ljava/lang/String;)V
- method setHeader (Ljava/lang/String;Ljava/lang/String;)V

## javax/sound/midi/MetaEventListener
- method <init> ()V

## javax/sound/midi/MetaMessage
- method <init> ()V
- method getType ()I
- method getType ()I

## javax/sound/midi/MidiChannel
- method <init> ()V
- method controlChange (II)V
- method controlChange (II)V

## javax/sound/midi/MidiDevice
- method <init> ()V
- method close ()V
- method close ()V
- method open ()V
- method open ()V

## javax/sound/midi/MidiSystem
- method <init> ()V
- method getSequence (Ljava/io/InputStream;)Ljavax/sound/midi/Sequence;
- method getSequence (Ljava/io/InputStream;)Ljavax/sound/midi/Sequence;
- method getSequencer ()Ljavax/sound/midi/Sequencer;
- method getSequencer ()Ljavax/sound/midi/Sequencer;

## javax/sound/midi/Sequence
- method <init> ()V
- method getMicrosecondLength ()J
- method getMicrosecondLength ()J

## javax/sound/midi/Sequencer
- method <init> ()V
- method addMetaEventListener (Ljavax/sound/midi/MetaEventListener;)Z
- method addMetaEventListener (Ljavax/sound/midi/MetaEventListener;)Z
- method close ()V
- method close ()V
- method getMicrosecondPosition ()J
- method getMicrosecondPosition ()J
- method open ()V
- method open ()V
- method setMicrosecondPosition (J)V
- method setMicrosecondPosition (J)V
- method setSequence (Ljavax/sound/midi/Sequence;)V
- method setSequence (Ljavax/sound/midi/Sequence;)V
- method start ()V
- method start ()V
- method stop ()V
- method stop ()V

## javax/sound/midi/Synthesizer
- method <init> ()V
- method getChannels ()[Ljavax/sound/midi/MidiChannel;
- method getChannels ()[Ljavax/sound/midi/MidiChannel;

## javax/sound/sampled/AudioFormat
- method <init> ()V
- method <init> (Ljavax/sound/sampled/AudioFormat$Encoding;FIIIFZ)V
- method getChannels ()I
- method getChannels ()I
- method getEncoding ()Ljavax/sound/sampled/AudioFormat$Encoding;
- method getEncoding ()Ljavax/sound/sampled/AudioFormat$Encoding;
- method getFrameRate ()F
- method getFrameRate ()F
- method getFrameSize ()I
- method getFrameSize ()I
- method getSampleRate ()F
- method getSampleRate ()F
- method getSampleSizeInBits ()I
- method getSampleSizeInBits ()I

## javax/sound/sampled/AudioFormat$Encoding
- field ALAW Ljavax/sound/sampled/AudioFormat$Encoding;
- field ALAW Ljavax/sound/sampled/AudioFormat$Encoding;
- field PCM_SIGNED Ljavax/sound/sampled/AudioFormat$Encoding;
- field PCM_SIGNED Ljavax/sound/sampled/AudioFormat$Encoding;
- field ULAW Ljavax/sound/sampled/AudioFormat$Encoding;
- field ULAW Ljavax/sound/sampled/AudioFormat$Encoding;
- method <init> ()V

## javax/sound/sampled/AudioInputStream
- method <init> ()V
- method <init> (Ljava/io/InputStream;Ljavax/sound/sampled/AudioFormat;J)V
- method getFormat ()Ljavax/sound/sampled/AudioFormat;
- method getFormat ()Ljavax/sound/sampled/AudioFormat;
- method getFrameLength ()J
- method getFrameLength ()J

## javax/sound/sampled/AudioSystem
- method <init> ()V
- method getAudioInputStream (Ljava/io/InputStream;)Ljavax/sound/sampled/AudioInputStream;
- method getAudioInputStream (Ljava/io/InputStream;)Ljavax/sound/sampled/AudioInputStream;
- method getAudioInputStream (Ljavax/sound/sampled/AudioFormat;Ljavax/sound/sampled/AudioInputStream;)Ljavax/sound/sampled/AudioInputStream;
- method getAudioInputStream (Ljavax/sound/sampled/AudioFormat;Ljavax/sound/sampled/AudioInputStream;)Ljavax/sound/sampled/AudioInputStream;
- method getLine (Ljavax/sound/sampled/Line$Info;)Ljavax/sound/sampled/Line;
- method getLine (Ljavax/sound/sampled/Line$Info;)Ljavax/sound/sampled/Line;
- method isLineSupported (Ljavax/sound/sampled/Line$Info;)Z
- method isLineSupported (Ljavax/sound/sampled/Line$Info;)Z

## javax/sound/sampled/Clip
- method <init> ()V
- method open (Ljavax/sound/sampled/AudioInputStream;)V
- method open (Ljavax/sound/sampled/AudioInputStream;)V
- method setFramePosition (I)V
- method setFramePosition (I)V

## javax/sound/sampled/Control
- method <init> ()V

## javax/sound/sampled/Control$Type
- method <init> ()V

## javax/sound/sampled/DataLine
- method <init> ()V
- method getBufferSize ()I
- method getBufferSize ()I
- method getFormat ()Ljavax/sound/sampled/AudioFormat;
- method getFormat ()Ljavax/sound/sampled/AudioFormat;
- method getFramePosition ()I
- method getFramePosition ()I
- method start ()V
- method start ()V
- method stop ()V
- method stop ()V

## javax/sound/sampled/DataLine$Info
- method <init> ()V
- method <init> (Ljava/lang/Class;Ljavax/sound/sampled/AudioFormat;)V
- method <init> (Ljava/lang/Class;Ljavax/sound/sampled/AudioFormat;I)V

## javax/sound/sampled/FloatControl
- method <init> ()V
- method setValue (F)V
- method setValue (F)V

## javax/sound/sampled/FloatControl$Type
- field MASTER_GAIN Ljavax/sound/sampled/FloatControl$Type;
- field MASTER_GAIN Ljavax/sound/sampled/FloatControl$Type;
- method <init> ()V

## javax/sound/sampled/Line
- method <init> ()V
- method addLineListener (Ljavax/sound/sampled/LineListener;)V
- method addLineListener (Ljavax/sound/sampled/LineListener;)V
- method getControl (Ljavax/sound/sampled/Control$Type;)Ljavax/sound/sampled/Control;
- method getControl (Ljavax/sound/sampled/Control$Type;)Ljavax/sound/sampled/Control;

## javax/sound/sampled/Line$Info
- method <init> ()V

## javax/sound/sampled/LineEvent
- method <init> ()V
- method getType ()Ljavax/sound/sampled/LineEvent$Type;
- method getType ()Ljavax/sound/sampled/LineEvent$Type;

## javax/sound/sampled/LineEvent$Type
- field STOP Ljavax/sound/sampled/LineEvent$Type;
- field STOP Ljavax/sound/sampled/LineEvent$Type;
- method <init> ()V

## javax/sound/sampled/LineListener
- method <init> ()V

## javax/sound/sampled/LineUnavailableException
- method <init> ()V

## javax/sound/sampled/SourceDataLine
- method <init> ()V
- method available ()I
- method available ()I
- method close ()V
- method close ()V
- method isOpen ()Z
- method isOpen ()Z
- method open (Ljavax/sound/sampled/AudioFormat;)V
- method open (Ljavax/sound/sampled/AudioFormat;)V
- method start ()V
- method start ()V
- method stop ()V
- method stop ()V
- method write ([BII)I
- method write ([BII)I

## javax/swing/AbstractListModel
- method <init> ()V

## javax/swing/Action
- method <init> ()V
- method actionPerformed (Ljava/awt/event/ActionEvent;)V
- method actionPerformed (Ljava/awt/event/ActionEvent;)V
- method setEnabled (Z)V
- method setEnabled (Z)V

## javax/swing/ActionMap
- method <init> ()V
- method get (Ljava/lang/Object;)Ljavax/swing/Action;
- method get (Ljava/lang/Object;)Ljavax/swing/Action;
- method put (Ljava/lang/Object;Ljavax/swing/Action;)V
- method put (Ljava/lang/Object;Ljavax/swing/Action;)V

## javax/swing/BorderFactory
- method <init> ()V
- method createEmptyBorder (IIII)Ljavax/swing/border/Border;
- method createEmptyBorder (IIII)Ljavax/swing/border/Border;
- method createLineBorder (Ljava/awt/Color;)Ljavax/swing/border/Border;
- method createLineBorder (Ljava/awt/Color;)Ljavax/swing/border/Border;
- method createMatteBorder (IIIILjava/awt/Color;)Ljavax/swing/border/MatteBorder;
- method createMatteBorder (IIIILjava/awt/Color;)Ljavax/swing/border/MatteBorder;
- method createTitledBorder (Ljava/lang/String;)Ljavax/swing/border/TitledBorder;
- method createTitledBorder (Ljava/lang/String;)Ljavax/swing/border/TitledBorder;

## javax/swing/BoundedRangeModel
- method <init> ()V
- method getExtent ()I
- method getExtent ()I
- method getMaximum ()I
- method getMaximum ()I

## javax/swing/Box
- method <init> ()V
- method <init> (I)V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method createGlue ()Ljava/awt/Component;
- method createGlue ()Ljava/awt/Component;
- method createHorizontalGlue ()Ljava/awt/Component;
- method createHorizontalGlue ()Ljava/awt/Component;
- method createRigidArea (Ljava/awt/Dimension;)Ljava/awt/Component;
- method createRigidArea (Ljava/awt/Dimension;)Ljava/awt/Component;

## javax/swing/BoxLayout
- method <init> ()V
- method <init> (Ljava/awt/Container;I)V
- method getAxis ()I
- method getAxis ()I

## javax/swing/ButtonGroup
- method <init> ()V
- method add (Ljavax/swing/AbstractButton;)V
- method add (Ljavax/swing/AbstractButton;)V

## javax/swing/ButtonModel
- method <init> ()V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method isPressed ()Z
- method isPressed ()Z
- method isRollover ()Z
- method isRollover ()Z
- method isSelected ()Z
- method isSelected ()Z
- method removeItemListener (Ljava/awt/event/ItemListener;)V
- method removeItemListener (Ljava/awt/event/ItemListener;)V
- method setSelected (Z)V
- method setSelected (Z)V

## javax/swing/ComboBoxEditor
- method <init> ()V
- method getEditorComponent ()Ljava/awt/Component;
- method getEditorComponent ()Ljava/awt/Component;

## javax/swing/DefaultCellEditor
- method <init> ()V
- method <init> (Ljavax/swing/JComboBox;)V

## javax/swing/DefaultComboBoxModel
- method <init> ()V

## javax/swing/DefaultListCellRenderer
- method <init> ()V
- method getListCellRendererComponent (Ljavax/swing/JList;Ljava/lang/Object;IZZ)Ljava/awt/Component;
- method getListCellRendererComponent (Ljavax/swing/JList;Ljava/lang/Object;IZZ)Ljava/awt/Component;

## javax/swing/DefaultListModel
- method <init> ()V
- method addElement (Ljava/lang/Object;)V
- method addElement (Ljava/lang/Object;)V
- method clear ()V
- method clear ()V
- method indexOf (Ljava/lang/Object;)I
- method indexOf (Ljava/lang/Object;)I
- method isEmpty ()Z
- method isEmpty ()Z
- method removeElement (Ljava/lang/Object;)Z
- method removeElement (Ljava/lang/Object;)Z

## javax/swing/GrayFilter
- method <init> ()V
- method createDisabledImage (Ljava/awt/Image;)Ljava/awt/Image;
- method createDisabledImage (Ljava/awt/Image;)Ljava/awt/Image;

## javax/swing/GroupLayout
- method <init> ()V
- method <init> (Ljava/awt/Container;)V
- method createParallelGroup (Ljavax/swing/GroupLayout$Alignment;)Ljavax/swing/GroupLayout$ParallelGroup;
- method createParallelGroup (Ljavax/swing/GroupLayout$Alignment;)Ljavax/swing/GroupLayout$ParallelGroup;
- method createParallelGroup (Ljavax/swing/GroupLayout$Alignment;Z)Ljavax/swing/GroupLayout$ParallelGroup;
- method createParallelGroup (Ljavax/swing/GroupLayout$Alignment;Z)Ljavax/swing/GroupLayout$ParallelGroup;
- method createSequentialGroup ()Ljavax/swing/GroupLayout$SequentialGroup;
- method createSequentialGroup ()Ljavax/swing/GroupLayout$SequentialGroup;
- method linkSize (I[Ljava/awt/Component;)V
- method linkSize (I[Ljava/awt/Component;)V
- method setAutoCreateContainerGaps (Z)V
- method setAutoCreateContainerGaps (Z)V
- method setHorizontalGroup (Ljavax/swing/GroupLayout$Group;)V
- method setHorizontalGroup (Ljavax/swing/GroupLayout$Group;)V
- method setVerticalGroup (Ljavax/swing/GroupLayout$Group;)V
- method setVerticalGroup (Ljavax/swing/GroupLayout$Group;)V

## javax/swing/GroupLayout$Alignment
- field BASELINE Ljavax/swing/GroupLayout$Alignment;
- field BASELINE Ljavax/swing/GroupLayout$Alignment;
- field LEADING Ljavax/swing/GroupLayout$Alignment;
- field LEADING Ljavax/swing/GroupLayout$Alignment;
- field TRAILING Ljavax/swing/GroupLayout$Alignment;
- field TRAILING Ljavax/swing/GroupLayout$Alignment;
- method <init> ()V

## javax/swing/GroupLayout$Group
- method <init> ()V

## javax/swing/GroupLayout$ParallelGroup
- method <init> ()V
- method addComponent (Ljava/awt/Component;)Ljavax/swing/GroupLayout$ParallelGroup;
- method addComponent (Ljava/awt/Component;)Ljavax/swing/GroupLayout$ParallelGroup;
- method addComponent (Ljava/awt/Component;III)Ljavax/swing/GroupLayout$ParallelGroup;
- method addComponent (Ljava/awt/Component;III)Ljavax/swing/GroupLayout$ParallelGroup;
- method addGroup (Ljavax/swing/GroupLayout$Alignment;Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$ParallelGroup;
- method addGroup (Ljavax/swing/GroupLayout$Alignment;Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$ParallelGroup;
- method addGroup (Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$ParallelGroup;
- method addGroup (Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$ParallelGroup;

## javax/swing/GroupLayout$SequentialGroup
- method <init> ()V
- method addComponent (Ljava/awt/Component;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addComponent (Ljava/awt/Component;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addComponent (Ljava/awt/Component;III)Ljavax/swing/GroupLayout$SequentialGroup;
- method addComponent (Ljava/awt/Component;III)Ljavax/swing/GroupLayout$SequentialGroup;
- method addContainerGap ()Ljavax/swing/GroupLayout$SequentialGroup;
- method addContainerGap ()Ljavax/swing/GroupLayout$SequentialGroup;
- method addContainerGap (II)Ljavax/swing/GroupLayout$SequentialGroup;
- method addContainerGap (II)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGap (I)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGap (I)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGap (III)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGap (III)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGroup (Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addGroup (Ljavax/swing/GroupLayout$Group;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addPreferredGap (Ljavax/swing/LayoutStyle$ComponentPlacement;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addPreferredGap (Ljavax/swing/LayoutStyle$ComponentPlacement;)Ljavax/swing/GroupLayout$SequentialGroup;
- method addPreferredGap (Ljavax/swing/LayoutStyle$ComponentPlacement;II)Ljavax/swing/GroupLayout$SequentialGroup;
- method addPreferredGap (Ljavax/swing/LayoutStyle$ComponentPlacement;II)Ljavax/swing/GroupLayout$SequentialGroup;

## javax/swing/Icon
- method <init> ()V
- method getIconHeight ()I
- method getIconHeight ()I
- method getIconWidth ()I
- method getIconWidth ()I
- method paintIcon (Ljava/awt/Component;Ljava/awt/Graphics;II)V
- method paintIcon (Ljava/awt/Component;Ljava/awt/Graphics;II)V

## javax/swing/ImageIcon
- method <init> ()V
- method <init> (Ljava/awt/Image;)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/net/URL;)V
- method <init> ([B)V
- method getIconHeight ()I
- method getIconHeight ()I
- method getIconWidth ()I
- method getIconWidth ()I
- method getImage ()Ljava/awt/Image;
- method getImage ()Ljava/awt/Image;
- method paintIcon (Ljava/awt/Component;Ljava/awt/Graphics;II)V
- method paintIcon (Ljava/awt/Component;Ljava/awt/Graphics;II)V
- method setDescription (Ljava/lang/String;)V
- method setDescription (Ljava/lang/String;)V
- method setImage (Ljava/awt/Image;)V
- method setImage (Ljava/awt/Image;)V

## javax/swing/InputMap
- method <init> ()V
- method getParent ()Ljavax/swing/InputMap;
- method getParent ()Ljavax/swing/InputMap;
- method put (Ljavax/swing/KeyStroke;Ljava/lang/Object;)V
- method put (Ljavax/swing/KeyStroke;Ljava/lang/Object;)V
- method remove (Ljavax/swing/KeyStroke;)V
- method remove (Ljavax/swing/KeyStroke;)V

## javax/swing/JApplet
- method <init> ()V
- method getX ()I
- method getX ()I
- method getY ()I
- method getY ()I
- method paint (Ljava/awt/Graphics;)V
- method paint (Ljava/awt/Graphics;)V

## javax/swing/JButton
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljavax/swing/Icon;)V
- method <init> (Ljavax/swing/Action;)V
- method <init> (Ljavax/swing/Icon;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method getBackground ()Ljava/awt/Color;
- method getBackground ()Ljava/awt/Color;
- method getIcon ()Ljavax/swing/Icon;
- method getIcon ()Ljavax/swing/Icon;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method paintComponent (Ljava/awt/Graphics;)V
- method paintComponent (Ljava/awt/Graphics;)V
- method setActionCommand (Ljava/lang/String;)V
- method setActionCommand (Ljava/lang/String;)V
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setMargin (Ljava/awt/Insets;)V
- method setMargin (Ljava/awt/Insets;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMnemonic (C)V
- method setMnemonic (C)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setRequestFocusEnabled (Z)V
- method setRequestFocusEnabled (Z)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V

## javax/swing/JCheckBox
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Z)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method getModel ()Ljavax/swing/ButtonModel;
- method getModel ()Ljavax/swing/ButtonModel;
- method isSelected ()Z
- method isSelected ()Z
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorderPainted (Z)V
- method setBorderPainted (Z)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFocusPainted (Z)V
- method setFocusPainted (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setForeground (Ljava/awt/Color;)V
- method setForeground (Ljava/awt/Color;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setRequestFocusEnabled (Z)V
- method setRequestFocusEnabled (Z)V
- method setSelected (Z)V
- method setSelected (Z)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## javax/swing/JCheckBoxMenuItem
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method isSelected ()Z
- method isSelected ()Z
- method setSelected (Z)V
- method setSelected (Z)V

## javax/swing/JColorChooser
- method <init> ()V
- method showDialog (Ljava/awt/Component;Ljava/lang/String;Ljava/awt/Color;)Ljava/awt/Color;
- method showDialog (Ljava/awt/Component;Ljava/lang/String;Ljava/awt/Color;)Ljava/awt/Color;

## javax/swing/JComboBox
- method <init> ()V
- method <init> ([Ljava/lang/Object;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addItem (Ljava/lang/Object;)V
- method addItem (Ljava/lang/Object;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method addItemListener (Ljava/awt/event/ItemListener;)V
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getSelectedItem ()Ljava/lang/Object;
- method getSelectedItem ()Ljava/lang/Object;
- method removeActionListener (Ljava/awt/event/ActionListener;)V
- method removeActionListener (Ljava/awt/event/ActionListener;)V
- method removeAllItems ()V
- method removeAllItems ()V
- method removeItemAt (I)V
- method removeItemAt (I)V
- method setActionCommand (Ljava/lang/String;)V
- method setActionCommand (Ljava/lang/String;)V
- method setEditable (Z)V
- method setEditable (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setLightWeightPopupEnabled (Z)V
- method setLightWeightPopupEnabled (Z)V
- method setMaximumRowCount (I)V
- method setMaximumRowCount (I)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setRenderer (Ljavax/swing/ListCellRenderer;)V
- method setRenderer (Ljavax/swing/ListCellRenderer;)V
- method setSelectedIndex (I)V
- method setSelectedIndex (I)V
- method setSelectedItem (Ljava/lang/Object;)V
- method setSelectedItem (Ljava/lang/Object;)V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V

## javax/swing/JComponent
- method <init> ()V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method getBorder ()Ljavax/swing/border/Border;
- method getBorder ()Ljavax/swing/border/Border;
- method getFont ()Ljava/awt/Font;
- method getFont ()Ljava/awt/Font;
- method getHeight ()I
- method getHeight ()I
- method getWidth ()I
- method getWidth ()I
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setOpaque (Z)V
- method setOpaque (Z)V

## javax/swing/JDesktopPane
- field OUTLINE_DRAG_MODE I
- field OUTLINE_DRAG_MODE I
- method <init> ()V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method getAllFrames ()[Ljavax/swing/JInternalFrame;
- method getAllFrames ()[Ljavax/swing/JInternalFrame;
- method getHeight ()I
- method getHeight ()I
- method getSelectedFrame ()Ljavax/swing/JInternalFrame;
- method getSelectedFrame ()Ljavax/swing/JInternalFrame;
- method getWidth ()I
- method getWidth ()I
- method setDragMode (I)V
- method setDragMode (I)V

## javax/swing/JDialog
- method <init> ()V
- method <init> (Ljava/awt/Frame;Ljava/lang/String;Z)V
- method <init> (Ljava/awt/Frame;Z)V

## javax/swing/JEditorPane
- method <init> ()V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseMotionListener (Ljava/awt/event/MouseMotionListener;)V
- method addMouseMotionListener (Ljava/awt/event/MouseMotionListener;)V
- method getDocument ()Ljavax/swing/text/Document;
- method getDocument ()Ljavax/swing/text/Document;
- method getMouseListeners ()[Ljava/awt/event/MouseListener;
- method getMouseListeners ()[Ljava/awt/event/MouseListener;
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method isDisplayable ()Z
- method isDisplayable ()Z
- method isEnabled ()Z
- method isEnabled ()Z
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method removeMouseMotionListener (Ljava/awt/event/MouseMotionListener;)V
- method removeMouseMotionListener (Ljava/awt/event/MouseMotionListener;)V
- method setComponentPopupMenu (Ljavax/swing/JPopupMenu;)V
- method setComponentPopupMenu (Ljavax/swing/JPopupMenu;)V
- method setContentType (Ljava/lang/String;)V
- method setContentType (Ljava/lang/String;)V
- method setCursor (Ljava/awt/Cursor;)V
- method setCursor (Ljava/awt/Cursor;)V
- method setEditable (Z)V
- method setEditable (Z)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setPage (Ljava/net/URL;)V
- method setPage (Ljava/net/URL;)V
- method setSelectionStart (I)V
- method setSelectionStart (I)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method updateUI ()V
- method updateUI ()V
- method viewToModel (Ljava/awt/Point;)I
- method viewToModel (Ljava/awt/Point;)I

## javax/swing/JFileChooser
- method <init> ()V
- method <init> (Ljava/io/File;)V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljavax/swing/filechooser/FileSystemView;)V
- method <init> (Ljavax/swing/filechooser/FileSystemView;)V
- method addPropertyChangeListener (Ljava/beans/PropertyChangeListener;)V
- method addPropertyChangeListener (Ljava/beans/PropertyChangeListener;)V
- method getCurrentDirectory ()Ljava/io/File;
- method getCurrentDirectory ()Ljava/io/File;
- method getSelectedFile ()Ljava/io/File;
- method getSelectedFile ()Ljava/io/File;
- method getSelectedFiles ()[Ljava/io/File;
- method getSelectedFiles ()[Ljava/io/File;
- method resetChoosableFileFilters ()V
- method resetChoosableFileFilters ()V
- method setAcceptAllFileFilterUsed (Z)V
- method setAcceptAllFileFilterUsed (Z)V
- method setCurrentDirectory (Ljava/io/File;)V
- method setCurrentDirectory (Ljava/io/File;)V
- method setDialogTitle (Ljava/lang/String;)V
- method setDialogTitle (Ljava/lang/String;)V
- method setFileFilter (Ljavax/swing/filechooser/FileFilter;)V
- method setFileFilter (Ljavax/swing/filechooser/FileFilter;)V
- method setFileSelectionMode (I)V
- method setFileSelectionMode (I)V
- method setMultiSelectionEnabled (Z)V
- method setMultiSelectionEnabled (Z)V
- method setSelectedFile (Ljava/io/File;)V
- method setSelectedFile (Ljava/io/File;)V
- method showDialog (Ljava/awt/Component;Ljava/lang/String;)I
- method showDialog (Ljava/awt/Component;Ljava/lang/String;)I
- method showOpenDialog (Ljava/awt/Component;)I
- method showOpenDialog (Ljava/awt/Component;)I
- method showSaveDialog (Ljava/awt/Component;)I
- method showSaveDialog (Ljava/awt/Component;)I

## javax/swing/JFrame
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method addWindowListener (Ljava/awt/event/WindowListener;)V
- method addWindowListener (Ljava/awt/event/WindowListener;)V
- method dispose ()V
- method dispose ()V
- method getContentPane ()Ljava/awt/Container;
- method getContentPane ()Ljava/awt/Container;
- method getHeight ()I
- method getHeight ()I
- method getRootPane ()Ljavax/swing/JRootPane;
- method getRootPane ()Ljavax/swing/JRootPane;
- method getTitle ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method getWidth ()I
- method getWidth ()I
- method getX ()I
- method getX ()I
- method getY ()I
- method getY ()I
- method isVisible ()Z
- method isVisible ()Z
- method pack ()V
- method pack ()V
- method paint (Ljava/awt/Graphics;)V
- method paint (Ljava/awt/Graphics;)V
- method setContentPane (Ljava/awt/Container;)V
- method setContentPane (Ljava/awt/Container;)V
- method setCursor (Ljava/awt/Cursor;)V
- method setCursor (Ljava/awt/Cursor;)V
- method setDefaultCloseOperation (I)V
- method setDefaultCloseOperation (I)V
- method setIconImage (Ljava/awt/Image;)V
- method setIconImage (Ljava/awt/Image;)V
- method setIconImages (Ljava/util/List;)V
- method setIconImages (Ljava/util/List;)V
- method setJMenuBar (Ljavax/swing/JMenuBar;)V
- method setJMenuBar (Ljavax/swing/JMenuBar;)V
- method setResizable (Z)V
- method setResizable (Z)V
- method setSize (II)V
- method setSize (II)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/JInternalFrame
- method <init> ()V
- method <init> (Ljava/lang/String;ZZZZ)V
- method getTitle ()Ljava/lang/String;
- method getTitle ()Ljava/lang/String;
- method isIcon ()Z
- method isIcon ()Z
- method reshape (IIII)V
- method reshape (IIII)V
- method setClosed (Z)V
- method setClosed (Z)V
- method setIcon (Z)V
- method setIcon (Z)V
- method setMaximum (Z)V
- method setMaximum (Z)V
- method setSelected (Z)V
- method setSelected (Z)V
- method setTitle (Ljava/lang/String;)V
- method setTitle (Ljava/lang/String;)V
- method toFront ()V
- method toFront ()V

## javax/swing/JLabel
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;I)V
- method <init> (Ljava/lang/String;Ljavax/swing/Icon;I)V
- method <init> (Ljavax/swing/Icon;)V
- method getBackground ()Ljava/awt/Color;
- method getBackground ()Ljava/awt/Color;
- method getFont ()Ljava/awt/Font;
- method getFont ()Ljava/awt/Font;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method paintComponent (Ljava/awt/Graphics;)V
- method paintComponent (Ljava/awt/Graphics;)V
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setAlignmentY (F)V
- method setAlignmentY (F)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setDisabledIcon (Ljavax/swing/Icon;)V
- method setDisabledIcon (Ljavax/swing/Icon;)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setHorizontalAlignment (I)V
- method setHorizontalAlignment (I)V
- method setIcon (Ljavax/swing/Icon;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setVerticalTextPosition (I)V
- method setVerticalTextPosition (I)V

## javax/swing/JLayeredPane
- method <init> ()V

## javax/swing/JList
- method <init> ()V
- method <init> (Ljavax/swing/ListModel;)V
- method addListSelectionListener (Ljavax/swing/event/ListSelectionListener;)V
- method addListSelectionListener (Ljavax/swing/event/ListSelectionListener;)V
- method clearSelection ()V
- method clearSelection ()V
- method ensureIndexIsVisible (I)V
- method ensureIndexIsVisible (I)V
- method getBackground ()Ljava/awt/Color;
- method getBackground ()Ljava/awt/Color;
- method getFont ()Ljava/awt/Font;
- method getFont ()Ljava/awt/Font;
- method getForeground ()Ljava/awt/Color;
- method getForeground ()Ljava/awt/Color;
- method getSelectedIndex ()I
- method getSelectedIndex ()I
- method getSelectedValue ()Ljava/lang/Object;
- method getSelectedValue ()Ljava/lang/Object;
- method getSelectedValuesList ()Ljava/util/List;
- method getSelectedValuesList ()Ljava/util/List;
- method getSelectionBackground ()Ljava/awt/Color;
- method getSelectionBackground ()Ljava/awt/Color;
- method getSelectionForeground ()Ljava/awt/Color;
- method getSelectionForeground ()Ljava/awt/Color;
- method setCellRenderer (Ljavax/swing/ListCellRenderer;)V
- method setCellRenderer (Ljavax/swing/ListCellRenderer;)V
- method setFixedCellHeight (I)V
- method setFixedCellHeight (I)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setListData (Ljava/util/Vector;)V
- method setListData (Ljava/util/Vector;)V
- method setListData ([Ljava/lang/Object;)V
- method setListData ([Ljava/lang/Object;)V
- method setSelectedIndex (I)V
- method setSelectedIndex (I)V
- method setSelectedIndices ([I)V
- method setSelectedIndices ([I)V
- method setSelectedValue (Ljava/lang/Object;Z)V
- method setSelectedValue (Ljava/lang/Object;Z)V
- method setSelectionBackground (Ljava/awt/Color;)V
- method setSelectionBackground (Ljava/awt/Color;)V
- method setSelectionForeground (Ljava/awt/Color;)V
- method setSelectionForeground (Ljava/awt/Color;)V
- method setSelectionMode (I)V
- method setSelectionMode (I)V

## javax/swing/JMenu
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljavax/swing/JMenuItem;)Ljavax/swing/JMenuItem;
- method add (Ljavax/swing/JMenuItem;)Ljavax/swing/JMenuItem;
- method addSeparator ()V
- method addSeparator ()V
- method removeAll ()V
- method removeAll ()V
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setMnemonic (C)V
- method setMnemonic (C)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V

## javax/swing/JMenuBar
- method <init> ()V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljavax/swing/JMenu;)Ljavax/swing/JMenu;
- method add (Ljavax/swing/JMenu;)Ljavax/swing/JMenu;
- method getHeight ()I
- method getHeight ()I
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/JMenuItem
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Ljavax/swing/Icon;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setActionCommand (Ljava/lang/String;)V
- method setActionCommand (Ljava/lang/String;)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setIcon (Ljavax/swing/Icon;)V
- method setMnemonic (C)V
- method setMnemonic (C)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/JOptionPane
- method <init> ()V
- method getFrameForComponent (Ljava/awt/Component;)Ljava/awt/Frame;
- method getFrameForComponent (Ljava/awt/Component;)Ljava/awt/Frame;
- method showConfirmDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)I
- method showConfirmDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)I
- method showConfirmDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;II)I
- method showConfirmDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;II)I
- method showInputDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)Ljava/lang/String;
- method showInputDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)Ljava/lang/String;
- method showInputDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;ILjavax/swing/Icon;[Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method showInputDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;ILjavax/swing/Icon;[Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method showInputDialog (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;
- method showInputDialog (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;
- method showMessageDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)V
- method showMessageDialog (Ljava/awt/Component;Ljava/lang/Object;Ljava/lang/String;I)V

## javax/swing/JPanel
- method <init> ()V
- method <init> (Ljava/awt/LayoutManager;)V
- method <init> (Ljava/awt/LayoutManager;Z)V
- method <init> (Z)V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;Ljava/lang/Object;)V
- method add (Ljava/awt/Component;Ljava/lang/Object;)V
- method add (Ljava/lang/String;Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/lang/String;Ljava/awt/Component;)Ljava/awt/Component;
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addComponentListener (Ljava/awt/event/ComponentListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method getComponent (I)Ljava/awt/Component;
- method getComponent (I)Ljava/awt/Component;
- method getComponentCount ()I
- method getComponentCount ()I
- method getInsets ()Ljava/awt/Insets;
- method getInsets ()Ljava/awt/Insets;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getVisibleRect ()Ljava/awt/Rectangle;
- method getVisibleRect ()Ljava/awt/Rectangle;
- method paint (Ljava/awt/Graphics;)V
- method paint (Ljava/awt/Graphics;)V
- method paintComponent (Ljava/awt/Graphics;)V
- method paintComponent (Ljava/awt/Graphics;)V
- method remove (Ljava/awt/Component;)V
- method remove (Ljava/awt/Component;)V
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method removeMouseListener (Ljava/awt/event/MouseListener;)V
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setLayout (Ljava/awt/LayoutManager;)V
- method setLayout (Ljava/awt/LayoutManager;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setOpaque (Z)V
- method setOpaque (Z)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V

## javax/swing/JPasswordField
- method <init> ()V
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## javax/swing/JPopupMenu
- method <init> ()V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljavax/swing/Action;)Ljavax/swing/JMenuItem;
- method add (Ljavax/swing/Action;)Ljavax/swing/JMenuItem;
- method add (Ljavax/swing/JMenuItem;)Ljavax/swing/JMenuItem;
- method add (Ljavax/swing/JMenuItem;)Ljavax/swing/JMenuItem;
- method addSeparator ()V
- method addSeparator ()V
- method getComponent (I)Ljava/awt/Component;
- method getComponent (I)Ljava/awt/Component;
- method getComponentCount ()I
- method getComponentCount ()I
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getSize ()Ljava/awt/Dimension;
- method getSize ()Ljava/awt/Dimension;
- method isShowing ()Z
- method isShowing ()Z
- method remove (I)V
- method remove (I)V
- method removeAll ()V
- method removeAll ()V
- method setDefaultLightWeightPopupEnabled (Z)V
- method setDefaultLightWeightPopupEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setVisible (Z)V
- method setVisible (Z)V
- method show (Ljava/awt/Component;II)V
- method show (Ljava/awt/Component;II)V

## javax/swing/JPopupMenu$Separator
- method <init> ()V

## javax/swing/JProgressBar
- method <init> ()V
- method getValue ()I
- method getValue ()I
- method paint (Ljava/awt/Graphics;)V
- method paint (Ljava/awt/Graphics;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setMaximum (I)V
- method setMaximum (I)V
- method setMinimum (I)V
- method setMinimum (I)V
- method setStringPainted (Z)V
- method setStringPainted (Z)V
- method setValue (I)V
- method setValue (I)V
- method setVisible (Z)V
- method setVisible (Z)V
- method update (Ljava/awt/Graphics;)V
- method update (Ljava/awt/Graphics;)V

## javax/swing/JRadioButton
- method <init> ()V
- method getModel ()Ljavax/swing/ButtonModel;
- method getModel ()Ljavax/swing/ButtonModel;
- method isSelected ()Z
- method isSelected ()Z
- method setSelected (Z)V
- method setSelected (Z)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## javax/swing/JRadioButtonMenuItem
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method <init> (Ljava/lang/String;Z)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setAccelerator (Ljavax/swing/KeyStroke;)V
- method setActionCommand (Ljava/lang/String;)V
- method setActionCommand (Ljava/lang/String;)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setMnemonic (C)V
- method setMnemonic (C)V
- method setSelected (Z)V
- method setSelected (Z)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/JRootPane
- method <init> ()V
- method invalidate ()V
- method invalidate ()V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setDefaultButton (Ljavax/swing/JButton;)V
- method setDefaultButton (Ljavax/swing/JButton;)V
- method validate ()V
- method validate ()V

## javax/swing/JScrollBar
- method <init> ()V
- method addAdjustmentListener (Ljava/awt/event/AdjustmentListener;)V
- method addAdjustmentListener (Ljava/awt/event/AdjustmentListener;)V
- method getMaximum ()I
- method getMaximum ()I
- method getModel ()Ljavax/swing/BoundedRangeModel;
- method getModel ()Ljavax/swing/BoundedRangeModel;
- method setValue (I)V
- method setValue (I)V

## javax/swing/JScrollPane
- method <init> ()V
- method <init> (Ljava/awt/Component;)V
- method <init> (Ljava/awt/Component;II)V
- method addMouseWheelListener (Ljava/awt/event/MouseWheelListener;)V
- method addMouseWheelListener (Ljava/awt/event/MouseWheelListener;)V
- method getAccessibleContext ()Ljavax/accessibility/AccessibleContext;
- method getAccessibleContext ()Ljavax/accessibility/AccessibleContext;
- method getVerticalScrollBar ()Ljavax/swing/JScrollBar;
- method getVerticalScrollBar ()Ljavax/swing/JScrollBar;
- method getVerticalScrollBarPolicy ()I
- method getVerticalScrollBarPolicy ()I
- method getViewport ()Ljavax/swing/JViewport;
- method getViewport ()Ljavax/swing/JViewport;
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setAutoscrolls (Z)V
- method setAutoscrolls (Z)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setColumnHeaderView (Ljava/awt/Component;)V
- method setColumnHeaderView (Ljava/awt/Component;)V
- method setCorner (Ljava/lang/String;Ljava/awt/Component;)V
- method setCorner (Ljava/lang/String;Ljava/awt/Component;)V
- method setHorizontalScrollBarPolicy (I)V
- method setHorizontalScrollBarPolicy (I)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setOpaque (Z)V
- method setOpaque (Z)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setRowHeaderView (Ljava/awt/Component;)V
- method setRowHeaderView (Ljava/awt/Component;)V
- method setSize (Ljava/awt/Dimension;)V
- method setSize (Ljava/awt/Dimension;)V
- method setVerticalScrollBarPolicy (I)V
- method setVerticalScrollBarPolicy (I)V
- method setViewportBorder (Ljavax/swing/border/Border;)V
- method setViewportBorder (Ljavax/swing/border/Border;)V
- method setViewportView (Ljava/awt/Component;)V
- method setViewportView (Ljava/awt/Component;)V
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/JSeparator
- method <init> ()V
- method <init> (I)V

## javax/swing/JSlider
- method <init> ()V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method getValue ()I
- method getValue ()I
- method setMajorTickSpacing (I)V
- method setMajorTickSpacing (I)V
- method setMaximum (I)V
- method setMaximum (I)V
- method setMinimum (I)V
- method setMinimum (I)V
- method setMinorTickSpacing (I)V
- method setMinorTickSpacing (I)V
- method setOpaque (Z)V
- method setOpaque (Z)V
- method setPaintLabels (Z)V
- method setPaintLabels (Z)V
- method setPaintTicks (Z)V
- method setPaintTicks (Z)V
- method setSnapToTicks (Z)V
- method setSnapToTicks (Z)V
- method setValue (I)V
- method setValue (I)V

## javax/swing/JSplitPane
- method <init> ()V
- method <init> (ILjava/awt/Component;Ljava/awt/Component;)V
- method setDividerLocation (I)V
- method setDividerLocation (I)V
- method setLeftComponent (Ljava/awt/Component;)V
- method setLeftComponent (Ljava/awt/Component;)V
- method setOneTouchExpandable (Z)V
- method setOneTouchExpandable (Z)V
- method setRightComponent (Ljava/awt/Component;)V
- method setRightComponent (Ljava/awt/Component;)V

## javax/swing/JTabbedPane
- method <init> ()V
- method <init> (I)V
- method addTab (Ljava/lang/String;Ljava/awt/Component;)V
- method addTab (Ljava/lang/String;Ljava/awt/Component;)V
- method getBackground ()Ljava/awt/Color;
- method getBackground ()Ljava/awt/Color;
- method getForeground ()Ljava/awt/Color;
- method getForeground ()Ljava/awt/Color;
- method getHeight ()I
- method getHeight ()I
- method getInsets ()Ljava/awt/Insets;
- method getInsets ()Ljava/awt/Insets;
- method getWidth ()I
- method getWidth ()I
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V

## javax/swing/JTable
- method <init> ()V
- method <init> (Ljavax/swing/table/TableModel;)V
- method editCellAt (II)Z
- method editCellAt (II)Z
- method getColumnModel ()Ljavax/swing/table/TableColumnModel;
- method getColumnModel ()Ljavax/swing/table/TableColumnModel;
- method getDefaultRenderer (Ljava/lang/Class;)Ljavax/swing/table/TableCellRenderer;
- method getDefaultRenderer (Ljava/lang/Class;)Ljavax/swing/table/TableCellRenderer;
- method getModel ()Ljavax/swing/table/TableModel;
- method getModel ()Ljavax/swing/table/TableModel;
- method getRowCount ()I
- method getRowCount ()I
- method getSelectedRow ()I
- method getSelectedRow ()I
- method getTableHeader ()Ljavax/swing/table/JTableHeader;
- method getTableHeader ()Ljavax/swing/table/JTableHeader;
- method repaint ()V
- method repaint ()V
- method setPreferredScrollableViewportSize (Ljava/awt/Dimension;)V
- method setPreferredScrollableViewportSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setRowHeight (I)V
- method setRowHeight (I)V
- method setRowSelectionInterval (II)V
- method setRowSelectionInterval (II)V

## javax/swing/JTextArea
- method <init> ()V
- method <init> (II)V
- method <init> (Ljava/lang/String;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method getCaret ()Ljavax/swing/text/Caret;
- method getCaret ()Ljavax/swing/text/Caret;
- method getDocument ()Ljavax/swing/text/Document;
- method getDocument ()Ljavax/swing/text/Document;
- method getLineEndOffset (I)I
- method getLineEndOffset (I)I
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method requestFocus ()V
- method requestFocus ()V
- method setAutoscrolls (Z)V
- method setAutoscrolls (Z)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setCaretColor (Ljava/awt/Color;)V
- method setCaretColor (Ljava/awt/Color;)V
- method setColumns (I)V
- method setColumns (I)V
- method setEditable (Z)V
- method setEditable (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setForeground (Ljava/awt/Color;)V
- method setForeground (Ljava/awt/Color;)V
- method setLineWrap (Z)V
- method setLineWrap (Z)V
- method setMargin (Ljava/awt/Insets;)V
- method setMargin (Ljava/awt/Insets;)V
- method setSelectionColor (Ljava/awt/Color;)V
- method setSelectionColor (Ljava/awt/Color;)V
- method setSize (Ljava/awt/Dimension;)V
- method setSize (Ljava/awt/Dimension;)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method setWrapStyleWord (Z)V
- method setWrapStyleWord (Z)V

## javax/swing/JTextField
- method <init> ()V
- method <init> (I)V
- method <init> (Ljava/lang/String;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addActionListener (Ljava/awt/event/ActionListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method addMouseListener (Ljava/awt/event/MouseListener;)V
- method copy ()V
- method copy ()V
- method cut ()V
- method cut ()V
- method getPreferredSize ()Ljava/awt/Dimension;
- method getPreferredSize ()Ljava/awt/Dimension;
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method paste ()V
- method paste ()V
- method requestFocus ()V
- method requestFocus ()V
- method setAlignmentX (F)V
- method setAlignmentX (F)V
- method setBackground (Ljava/awt/Color;)V
- method setBackground (Ljava/awt/Color;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBorder (Ljavax/swing/border/Border;)V
- method setBounds (IIII)V
- method setBounds (IIII)V
- method setCaretPosition (I)V
- method setCaretPosition (I)V
- method setEditable (Z)V
- method setEditable (Z)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setForeground (Ljava/awt/Color;)V
- method setForeground (Ljava/awt/Color;)V
- method setHorizontalAlignment (I)V
- method setHorizontalAlignment (I)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V

## javax/swing/JTextPane
- method <init> ()V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method addKeyListener (Ljava/awt/event/KeyListener;)V
- method getDocument ()Ljavax/swing/text/Document;
- method getDocument ()Ljavax/swing/text/Document;
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method isEditable ()Z
- method isEditable ()Z
- method setCaretPosition (I)V
- method setCaretPosition (I)V
- method setEditable (Z)V
- method setEditable (Z)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setFont (Ljava/awt/Font;)V
- method setFont (Ljava/awt/Font;)V
- method setSelectionEnd (I)V
- method setSelectionEnd (I)V
- method setSelectionStart (I)V
- method setSelectionStart (I)V
- method setText (Ljava/lang/String;)V
- method setText (Ljava/lang/String;)V
- method updateUI ()V
- method updateUI ()V

## javax/swing/JToolBar
- method <init> ()V
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method add (Ljava/awt/Component;)Ljava/awt/Component;
- method addSeparator ()V
- method addSeparator ()V
- method setFloatable (Z)V
- method setFloatable (Z)V

## javax/swing/JTree
- method <init> ()V
- method addTreeSelectionListener (Ljavax/swing/event/TreeSelectionListener;)V
- method addTreeSelectionListener (Ljavax/swing/event/TreeSelectionListener;)V
- method expandRow (I)V
- method expandRow (I)V
- method getSelectionModel ()Ljavax/swing/tree/TreeSelectionModel;
- method getSelectionModel ()Ljavax/swing/tree/TreeSelectionModel;
- method setCellRenderer (Ljavax/swing/tree/TreeCellRenderer;)V
- method setCellRenderer (Ljavax/swing/tree/TreeCellRenderer;)V
- method setEnabled (Z)V
- method setEnabled (Z)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMaximumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setMinimumSize (Ljava/awt/Dimension;)V
- method setModel (Ljavax/swing/tree/TreeModel;)V
- method setModel (Ljavax/swing/tree/TreeModel;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setPreferredSize (Ljava/awt/Dimension;)V
- method setSelectionRow (I)V
- method setSelectionRow (I)V
- method setVisible (Z)V
- method setVisible (Z)V
- method updateUI ()V
- method updateUI ()V

## javax/swing/JViewport
- method <init> ()V
- method getViewRect ()Ljava/awt/Rectangle;
- method getViewRect ()Ljava/awt/Rectangle;
- method setOpaque (Z)V
- method setOpaque (Z)V

## javax/swing/KeyStroke
- method <init> ()V
- method getKeyStroke (II)Ljavax/swing/KeyStroke;
- method getKeyStroke (II)Ljavax/swing/KeyStroke;

## javax/swing/LayoutStyle
- method <init> ()V

## javax/swing/LayoutStyle$ComponentPlacement
- field RELATED Ljavax/swing/LayoutStyle$ComponentPlacement;
- field RELATED Ljavax/swing/LayoutStyle$ComponentPlacement;
- field UNRELATED Ljavax/swing/LayoutStyle$ComponentPlacement;
- field UNRELATED Ljavax/swing/LayoutStyle$ComponentPlacement;
- method <init> ()V

## javax/swing/ListCellRenderer
- method <init> ()V

## javax/swing/ListModel
- method <init> ()V
- method getElementAt (I)Ljava/lang/Object;
- method getElementAt (I)Ljava/lang/Object;
- method getSize ()I
- method getSize ()I

## javax/swing/LookAndFeel
- method <init> ()V
- method installProperty (Ljavax/swing/JComponent;Ljava/lang/String;Ljava/lang/Object;)V
- method installProperty (Ljavax/swing/JComponent;Ljava/lang/String;Ljava/lang/Object;)V

## javax/swing/Scrollable
- method <init> ()V

## javax/swing/SwingUtilities
- method <init> ()V
- method invokeAndWait (Ljava/lang/Runnable;)V
- method invokeAndWait (Ljava/lang/Runnable;)V
- method invokeLater (Ljava/lang/Runnable;)V
- method invokeLater (Ljava/lang/Runnable;)V
- method isEventDispatchThread ()Z
- method isEventDispatchThread ()Z
- method updateComponentTreeUI (Ljava/awt/Component;)V
- method updateComponentTreeUI (Ljava/awt/Component;)V

## javax/swing/UIDefaults
- method <init> ()V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method keys ()Ljava/util/Enumeration;
- method keys ()Ljava/util/Enumeration;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;

## javax/swing/UIManager
- method <init> ()V
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method getBorder (Ljava/lang/Object;)Ljavax/swing/border/Border;
- method getBorder (Ljava/lang/Object;)Ljavax/swing/border/Border;
- method getDefaults ()Ljavax/swing/UIDefaults;
- method getDefaults ()Ljavax/swing/UIDefaults;
- method getInsets (Ljava/lang/Object;)Ljava/awt/Insets;
- method getInsets (Ljava/lang/Object;)Ljava/awt/Insets;
- method getInstalledLookAndFeels ()[Ljavax/swing/UIManager$LookAndFeelInfo;
- method getInstalledLookAndFeels ()[Ljavax/swing/UIManager$LookAndFeelInfo;
- method getLookAndFeelDefaults ()Ljavax/swing/UIDefaults;
- method getLookAndFeelDefaults ()Ljavax/swing/UIDefaults;
- method getString (Ljava/lang/Object;)Ljava/lang/String;
- method getString (Ljava/lang/Object;)Ljava/lang/String;
- method getSystemLookAndFeelClassName ()Ljava/lang/String;
- method getSystemLookAndFeelClassName ()Ljava/lang/String;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method setLookAndFeel (Ljava/lang/String;)V
- method setLookAndFeel (Ljava/lang/String;)V

## javax/swing/UIManager$LookAndFeelInfo
- method <init> ()V
- method getClassName ()Ljava/lang/String;
- method getClassName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;
- method getName ()Ljava/lang/String;

## javax/swing/border/EmptyBorder
- method <init> ()V
- method <init> (IIII)V

## javax/swing/event/ChangeListener
- method <init> ()V

## javax/swing/event/DocumentEvent
- method <init> ()V

## javax/swing/event/DocumentListener
- method <init> ()V

## javax/swing/event/EventListenerList
- method <init> ()V
- method add (Ljava/lang/Class;Ljava/util/EventListener;)V
- method add (Ljava/lang/Class;Ljava/util/EventListener;)V
- method getListenerList ()[Ljava/lang/Object;
- method getListenerList ()[Ljava/lang/Object;
- method remove (Ljava/lang/Class;Ljava/util/EventListener;)V
- method remove (Ljava/lang/Class;Ljava/util/EventListener;)V

## javax/swing/event/HyperlinkEvent
- method <init> ()V
- method getEventType ()Ljavax/swing/event/HyperlinkEvent$EventType;
- method getEventType ()Ljavax/swing/event/HyperlinkEvent$EventType;
- method getURL ()Ljava/net/URL;
- method getURL ()Ljava/net/URL;

## javax/swing/event/HyperlinkEvent$EventType
- field ACTIVATED Ljavax/swing/event/HyperlinkEvent$EventType;
- field ACTIVATED Ljavax/swing/event/HyperlinkEvent$EventType;
- method <init> ()V

## javax/swing/event/HyperlinkListener
- method <init> ()V

## javax/swing/event/InternalFrameAdapter
- method <init> ()V

## javax/swing/event/ListSelectionListener
- method <init> ()V

## javax/swing/event/TreeSelectionListener
- method <init> ()V

## javax/swing/filechooser/FileFilter
- method <init> ()V

## javax/swing/filechooser/FileSystemView
- method <init> ()V
- method getDefaultDirectory ()Ljava/io/File;
- method getDefaultDirectory ()Ljava/io/File;

## javax/swing/filechooser/FileView
- method <init> ()V

## javax/swing/plaf/ColorUIResource
- method <init> ()V
- method <init> (III)V

## javax/swing/plaf/ComponentUI
- method <init> ()V

## javax/swing/plaf/FontUIResource
- method <init> ()V
- method <init> (Ljava/lang/String;II)V

## javax/swing/plaf/TabbedPaneUI
- method <init> ()V

## javax/swing/plaf/TextUI
- method <init> ()V
- method getEditorKit (Ljavax/swing/text/JTextComponent;)Ljavax/swing/text/EditorKit;
- method getEditorKit (Ljavax/swing/text/JTextComponent;)Ljavax/swing/text/EditorKit;

## javax/swing/plaf/basic/BasicHTML
- method <init> ()V
- method getHTMLBaseline (Ljavax/swing/text/View;II)I
- method getHTMLBaseline (Ljavax/swing/text/View;II)I

## javax/swing/plaf/metal/MetalComboBoxEditor
- method <init> ()V

## javax/swing/table/AbstractTableModel
- method <init> ()V

## javax/swing/table/DefaultTableCellRenderer
- method <init> ()V
- method setToolTipText (Ljava/lang/String;)V
- method setToolTipText (Ljava/lang/String;)V

## javax/swing/table/JTableHeader
- method <init> ()V
- method getDefaultRenderer ()Ljavax/swing/table/TableCellRenderer;
- method getDefaultRenderer ()Ljavax/swing/table/TableCellRenderer;

## javax/swing/table/TableCellRenderer
- method <init> ()V
- method getTableCellRendererComponent (Ljavax/swing/JTable;Ljava/lang/Object;ZZII)Ljava/awt/Component;
- method getTableCellRendererComponent (Ljavax/swing/JTable;Ljava/lang/Object;ZZII)Ljava/awt/Component;

## javax/swing/table/TableColumn
- method <init> ()V
- method getHeaderValue ()Ljava/lang/Object;
- method getHeaderValue ()Ljava/lang/Object;
- method setCellEditor (Ljavax/swing/table/TableCellEditor;)V
- method setCellEditor (Ljavax/swing/table/TableCellEditor;)V
- method setCellRenderer (Ljavax/swing/table/TableCellRenderer;)V
- method setCellRenderer (Ljavax/swing/table/TableCellRenderer;)V
- method setPreferredWidth (I)V
- method setPreferredWidth (I)V

## javax/swing/table/TableColumnModel
- method <init> ()V
- method getColumn (I)Ljavax/swing/table/TableColumn;
- method getColumn (I)Ljavax/swing/table/TableColumn;

## javax/swing/text/AbstractDocument
- method <init> ()V
- method readLock ()V
- method readLock ()V
- method readUnlock ()V
- method readUnlock ()V

## javax/swing/text/AttributeSet
- method <init> ()V
- method getAttribute (Ljava/lang/Object;)Ljava/lang/Object;
- method getAttribute (Ljava/lang/Object;)Ljava/lang/Object;
- method isDefined (Ljava/lang/Object;)Z
- method isDefined (Ljava/lang/Object;)Z

## javax/swing/text/BadLocationException
- method <init> ()V

## javax/swing/text/Caret
- method <init> ()V
- method isSelectionVisible ()Z
- method isSelectionVisible ()Z
- method isVisible ()Z
- method isVisible ()Z
- method setSelectionVisible (Z)V
- method setSelectionVisible (Z)V
- method setVisible (Z)V
- method setVisible (Z)V

## javax/swing/text/DefaultCaret
- method <init> ()V
- method setUpdatePolicy (I)V
- method setUpdatePolicy (I)V

## javax/swing/text/DefaultEditorKit
- method <init> ()V

## javax/swing/text/DefaultEditorKit$CopyAction
- method <init> ()V

## javax/swing/text/Document
- method <init> ()V
- method addDocumentListener (Ljavax/swing/event/DocumentListener;)V
- method addDocumentListener (Ljavax/swing/event/DocumentListener;)V
- method getLength ()I
- method getLength ()I
- method getProperty (Ljava/lang/Object;)Ljava/lang/Object;
- method getProperty (Ljava/lang/Object;)Ljava/lang/Object;
- method getText (II)Ljava/lang/String;
- method getText (II)Ljava/lang/String;
- method insertString (ILjava/lang/String;Ljavax/swing/text/AttributeSet;)V
- method insertString (ILjava/lang/String;Ljavax/swing/text/AttributeSet;)V
- method putProperty (Ljava/lang/Object;Ljava/lang/Object;)V
- method putProperty (Ljava/lang/Object;Ljava/lang/Object;)V

## javax/swing/text/EditorKit
- method <init> ()V
- method read (Ljava/io/Reader;Ljavax/swing/text/Document;I)V
- method read (Ljava/io/Reader;Ljavax/swing/text/Document;I)V

## javax/swing/text/Element
- method <init> ()V
- method getAttributes ()Ljavax/swing/text/AttributeSet;
- method getAttributes ()Ljavax/swing/text/AttributeSet;
- method getEndOffset ()I
- method getEndOffset ()I
- method getStartOffset ()I
- method getStartOffset ()I

## javax/swing/text/GlyphView
- method <init> ()V

## javax/swing/text/GlyphView$GlyphPainter
- method <init> ()V
- method paint (Ljavax/swing/text/GlyphView;Ljava/awt/Graphics;Ljava/awt/Shape;II)V
- method paint (Ljavax/swing/text/GlyphView;Ljava/awt/Graphics;Ljava/awt/Shape;II)V

## javax/swing/text/JTextComponent
- method <init> ()V
- method getActionMap ()Ljavax/swing/ActionMap;
- method getActionMap ()Ljavax/swing/ActionMap;
- method getCaret ()Ljavax/swing/text/Caret;
- method getCaret ()Ljavax/swing/text/Caret;
- method getCaretPosition ()I
- method getCaretPosition ()I
- method getDocument ()Ljavax/swing/text/Document;
- method getDocument ()Ljavax/swing/text/Document;
- method getHighlighter ()Ljavax/swing/text/Highlighter;
- method getHighlighter ()Ljavax/swing/text/Highlighter;
- method getSelectedText ()Ljava/lang/String;
- method getSelectedText ()Ljava/lang/String;
- method getSelectionEnd ()I
- method getSelectionEnd ()I
- method getText ()Ljava/lang/String;
- method getText ()Ljava/lang/String;
- method getTopLevelAncestor ()Ljava/awt/Container;
- method getTopLevelAncestor ()Ljava/awt/Container;
- method hasFocus ()Z
- method hasFocus ()Z
- method isEditable ()Z
- method isEditable ()Z
- method isEnabled ()Z
- method isEnabled ()Z
- method moveCaretPosition (I)V
- method moveCaretPosition (I)V
- method replaceSelection (Ljava/lang/String;)V
- method replaceSelection (Ljava/lang/String;)V
- method requestFocus ()V
- method requestFocus ()V
- method setCaretPosition (I)V
- method setCaretPosition (I)V
- method setSelectionStart (I)V
- method setSelectionStart (I)V

## javax/swing/text/LayeredHighlighter
- method <init> ()V
- method paintLayeredHighlights (Ljava/awt/Graphics;IILjava/awt/Shape;Ljavax/swing/text/JTextComponent;Ljavax/swing/text/View;)V
- method paintLayeredHighlights (Ljava/awt/Graphics;IILjava/awt/Shape;Ljavax/swing/text/JTextComponent;Ljavax/swing/text/View;)V

## javax/swing/text/PlainDocument
- method <init> ()V
- method insertString (ILjava/lang/String;Ljavax/swing/text/AttributeSet;)V
- method insertString (ILjava/lang/String;Ljavax/swing/text/AttributeSet;)V

## javax/swing/text/Position
- method <init> ()V

## javax/swing/text/Position$Bias
- field Backward Ljavax/swing/text/Position$Bias;
- field Backward Ljavax/swing/text/Position$Bias;
- field Forward Ljavax/swing/text/Position$Bias;
- field Forward Ljavax/swing/text/Position$Bias;
- method <init> ()V

## javax/swing/text/Segment
- field array [C
- field array [C
- field count I
- field count I
- field offset I
- field offset I
- method <init> ()V
- method <init> ([CII)V

## javax/swing/text/SimpleAttributeSet
- method <init> ()V

## javax/swing/text/StyleConstants
- field NameAttribute Ljava/lang/Object;
- field NameAttribute Ljava/lang/Object;
- method <init> ()V
- method setBackground (Ljavax/swing/text/MutableAttributeSet;Ljava/awt/Color;)V
- method setBackground (Ljavax/swing/text/MutableAttributeSet;Ljava/awt/Color;)V
- method setForeground (Ljavax/swing/text/MutableAttributeSet;Ljava/awt/Color;)V
- method setForeground (Ljavax/swing/text/MutableAttributeSet;Ljava/awt/Color;)V

## javax/swing/text/StyledDocument
- method <init> ()V
- method getForeground (Ljavax/swing/text/AttributeSet;)Ljava/awt/Color;
- method getForeground (Ljavax/swing/text/AttributeSet;)Ljava/awt/Color;
- method getLength ()I
- method getLength ()I
- method getText (II)Ljava/lang/String;
- method getText (II)Ljava/lang/String;
- method setCharacterAttributes (IILjavax/swing/text/AttributeSet;Z)V
- method setCharacterAttributes (IILjavax/swing/text/AttributeSet;Z)V

## javax/swing/text/TextAction
- method <init> ()V
- method <init> (Ljava/lang/String;)V

## javax/swing/text/Utilities
- method <init> ()V
- method getParagraphElement (Ljavax/swing/text/JTextComponent;I)Ljavax/swing/text/Element;
- method getParagraphElement (Ljavax/swing/text/JTextComponent;I)Ljavax/swing/text/Element;

## javax/swing/text/View
- method <init> ()V
- method <init> (Ljavax/swing/text/Element;)V
- method changedUpdate (Ljavax/swing/event/DocumentEvent;Ljava/awt/Shape;Ljavax/swing/text/ViewFactory;)V
- method changedUpdate (Ljavax/swing/event/DocumentEvent;Ljava/awt/Shape;Ljavax/swing/text/ViewFactory;)V
- method getAlignment (I)F
- method getAlignment (I)F
- method getAttributes ()Ljavax/swing/text/AttributeSet;
- method getAttributes ()Ljavax/swing/text/AttributeSet;
- method getParent ()Ljavax/swing/text/View;
- method getParent ()Ljavax/swing/text/View;
- method getPreferredSpan (I)F
- method getPreferredSpan (I)F
- method paint (Ljava/awt/Graphics;Ljava/awt/Shape;)V
- method paint (Ljava/awt/Graphics;Ljava/awt/Shape;)V
- method setParent (Ljavax/swing/text/View;)V
- method setParent (Ljavax/swing/text/View;)V
- method setSize (FF)V
- method setSize (FF)V

## javax/swing/text/ViewFactory
- method <init> ()V

## javax/swing/text/html/HTML
- method <init> ()V

## javax/swing/text/html/HTML$Attribute
- field ALIGN Ljavax/swing/text/html/HTML$Attribute;
- field ALIGN Ljavax/swing/text/html/HTML$Attribute;
- field ALT Ljavax/swing/text/html/HTML$Attribute;
- field ALT Ljavax/swing/text/html/HTML$Attribute;
- field BORDER Ljavax/swing/text/html/HTML$Attribute;
- field BORDER Ljavax/swing/text/html/HTML$Attribute;
- field HEIGHT Ljavax/swing/text/html/HTML$Attribute;
- field HEIGHT Ljavax/swing/text/html/HTML$Attribute;
- field HREF Ljavax/swing/text/html/HTML$Attribute;
- field HREF Ljavax/swing/text/html/HTML$Attribute;
- field HSPACE Ljavax/swing/text/html/HTML$Attribute;
- field HSPACE Ljavax/swing/text/html/HTML$Attribute;
- field SRC Ljavax/swing/text/html/HTML$Attribute;
- field SRC Ljavax/swing/text/html/HTML$Attribute;
- field VSPACE Ljavax/swing/text/html/HTML$Attribute;
- field VSPACE Ljavax/swing/text/html/HTML$Attribute;
- field WIDTH Ljavax/swing/text/html/HTML$Attribute;
- field WIDTH Ljavax/swing/text/html/HTML$Attribute;
- method <init> ()V

## javax/swing/text/html/HTML$Tag
- field A Ljavax/swing/text/html/HTML$Tag;
- field A Ljavax/swing/text/html/HTML$Tag;
- field CONTENT Ljavax/swing/text/html/HTML$Tag;
- field CONTENT Ljavax/swing/text/html/HTML$Tag;
- field IMG Ljavax/swing/text/html/HTML$Tag;
- field IMG Ljavax/swing/text/html/HTML$Tag;
- method <init> ()V

## javax/swing/text/html/HTMLDocument
- method <init> ()V
- method getCharacterElement (I)Ljavax/swing/text/Element;
- method getCharacterElement (I)Ljavax/swing/text/Element;
- method getStyleSheet ()Ljavax/swing/text/html/StyleSheet;
- method getStyleSheet ()Ljavax/swing/text/html/StyleSheet;

## javax/swing/text/html/HTMLEditorKit
- method <init> ()V
- method install (Ljavax/swing/JEditorPane;)V
- method install (Ljavax/swing/JEditorPane;)V

## javax/swing/text/html/HTMLEditorKit$HTMLFactory
- method <init> ()V
- method create (Ljavax/swing/text/Element;)Ljavax/swing/text/View;
- method create (Ljavax/swing/text/Element;)Ljavax/swing/text/View;

## javax/swing/text/html/HTMLEditorKit$LinkController
- method <init> ()V

## javax/swing/text/html/InlineView
- method <init> ()V
- method <init> (Ljavax/swing/text/Element;)V

## javax/swing/text/html/StyleSheet
- method <init> ()V
- method addRule (Ljava/lang/String;)V
- method addRule (Ljava/lang/String;)V
- method getViewAttributes (Ljavax/swing/text/View;)Ljavax/swing/text/AttributeSet;
- method getViewAttributes (Ljavax/swing/text/View;)Ljavax/swing/text/AttributeSet;
- method importStyleSheet (Ljava/net/URL;)V
- method importStyleSheet (Ljava/net/URL;)V

## javax/swing/tree/DefaultMutableTreeNode
- method <init> ()V
- method <init> (Ljava/lang/Object;)V
- method <init> (Ljava/lang/Object;Z)V
- method add (Ljavax/swing/tree/MutableTreeNode;)V
- method add (Ljavax/swing/tree/MutableTreeNode;)V
- method remove (I)V
- method remove (I)V

## javax/swing/tree/DefaultTreeCellRenderer
- method <init> ()V
- method getTreeCellRendererComponent (Ljavax/swing/JTree;Ljava/lang/Object;ZZZIZ)Ljava/awt/Component;
- method getTreeCellRendererComponent (Ljavax/swing/JTree;Ljava/lang/Object;ZZZIZ)Ljava/awt/Component;

## javax/swing/tree/DefaultTreeModel
- method <init> ()V
- method <init> (Ljavax/swing/tree/TreeNode;)V
- method <init> (Ljavax/swing/tree/TreeNode;Z)V

## javax/swing/tree/TreeSelectionModel
- method <init> ()V
- method getSelectionRows ()[I
- method getSelectionRows ()[I
- method setSelectionMode (I)V
- method setSelectionMode (I)V

## javax/wireless/messaging/MessageCon
- method <init> ()V
- method end (Ljavax/wireless/messaging/Message;)V
- method end (Ljavax/wireless/messaging/Message;)V
- method newMessage (Ljava/lang/String;)Ljavax/wireless/messaging/Message;
- method newMessage (Ljava/lang/String;)Ljavax/wireless/messaging/Message;

## javax/wireless/messaging/MessagePart
- method <init> ()V
- method <init> (Ljava/io/InputStream;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method <init> ([BIILjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method <init> ([BLjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V
- method getContent ()[B
- method getContent ()[B
- method getContentLocation ()Ljava/lang/String;
- method getContentLocation ()Ljava/lang/String;
- method getLength ()I
- method getLength ()I
- method getMIMEType ()Ljava/lang/String;
- method getMIMEType ()Ljava/lang/String;

## javax/wireless/messaging/MultipartMessage
- method <init> ()V
- method addAddress (Ljava/lang/String;Ljava/lang/String;)Z
- method addAddress (Ljava/lang/String;Ljava/lang/String;)Z
- method addMessagePart (Ljavax/wireless/messaging/MessagePart;)V
- method addMessagePart (Ljavax/wireless/messaging/MessagePart;)V
- method getAddresses (Ljava/lang/String;)[Ljava/lang/String;
- method getAddresses (Ljava/lang/String;)[Ljava/lang/String;
- method getMessageParts ()[Ljavax/wireless/messaging/MessagePart;
- method getMessageParts ()[Ljavax/wireless/messaging/MessagePart;
- method getSubject ()Ljava/lang/String;
- method getSubject ()Ljava/lang/String;
- method getTimestamp ()Ljava/util/Date;
- method getTimestamp ()Ljava/util/Date;
- method removeAddresses ()V
- method removeAddresses ()V
- method setAddress (Ljava/lang/String;)V
- method setAddress (Ljava/lang/String;)V
- method setHeader (Ljava/lang/String;Ljava/lang/String;)V
- method setHeader (Ljava/lang/String;Ljava/lang/String;)V
- method setStartContentId (Ljava/lang/String;)V
- method setStartContentId (Ljava/lang/String;)V
- method setSubject (Ljava/lang/String;)V
- method setSubject (Ljava/lang/String;)V

## javax/wireless/messaging/SizeExceededException
- method <init> ()V

## javax/xml/bind/DatatypeConverter
- method <init> ()V
- method parseBase64Binary (Ljava/lang/String;)[B
- method parseBase64Binary (Ljava/lang/String;)[B
- method printBase64Binary ([B)Ljava/lang/String;
- method printBase64Binary ([B)Ljava/lang/String;

## javax/xml/namespace/QName
- method <init> ()V
- method <init> (Ljava/lang/String;Ljava/lang/String;)V

## javax/xml/parsers/DocumentBuilder
- method <init> ()V
- method parse (Ljava/io/InputStream;)Lorg/w3c/dom/Document;
- method parse (Ljava/io/InputStream;)Lorg/w3c/dom/Document;

## javax/xml/parsers/DocumentBuilderFactory
- method <init> ()V
- method newDocumentBuilder ()Ljavax/xml/parsers/DocumentBuilder;
- method newDocumentBuilder ()Ljavax/xml/parsers/DocumentBuilder;
- method newInstance ()Ljavax/xml/parsers/DocumentBuilderFactory;
- method newInstance ()Ljavax/xml/parsers/DocumentBuilderFactory;

## javax/xml/parsers/FactoryConfigurationError
- method <init> ()V

## javax/xml/rpc/JAXRPCException
- method <init> ()V
- method <init> (Ljava/lang/String;)V
- method getLinkedCause ()Ljava/lang/Throwable;
- method getLinkedCause ()Ljava/lang/Throwable;

## javax/xml/rpc/Stub
- method <init> ()V

## net/rim/device/api/system/KeyListener
- method <init> ()V
- method keyChar (CZI)Z
- method keyChar (CZI)Z
- method keyDown (II)Z
- method keyDown (II)Z
- method keyRepeat (II)Z
- method keyRepeat (II)Z
- method keyStatus (II)Z
- method keyStatus (II)Z
- method keyUp (II)Z
- method keyUp (II)Z

## org/netbeans/microedition/util/CancellableTask
- method <init> ()V
- method hasFailed ()Z
- method hasFailed ()Z

## org/w3c/dom/Document
- method <init> ()V
- method getDocumentElement ()Lorg/w3c/dom/Element;
- method getDocumentElement ()Lorg/w3c/dom/Element;
- method getElementById (Ljava/lang/String;)Lorg/w3c/dom/Element;
- method getElementById (Ljava/lang/String;)Lorg/w3c/dom/Element;

## org/w3c/dom/Element
- method <init> ()V

## org/w3c/dom/svg/SVGAnimationElement
- method <init> ()V
- method beginElementAt (F)V
- method beginElementAt (F)V
- method endElementAt (F)V
- method endElementAt (F)V

## org/w3c/dom/svg/SVGElement
- method <init> ()V
- method getId ()Ljava/lang/String;
- method getId ()Ljava/lang/String;
- method getMatrixTrait (Ljava/lang/String;)Lorg/w3c/dom/svg/SVGMatrix;
- method getMatrixTrait (Ljava/lang/String;)Lorg/w3c/dom/svg/SVGMatrix;
- method setFloatTrait (Ljava/lang/String;F)V
- method setFloatTrait (Ljava/lang/String;F)V
- method setMatrixTrait (Ljava/lang/String;Lorg/w3c/dom/svg/SVGMatrix;)V
- method setMatrixTrait (Ljava/lang/String;Lorg/w3c/dom/svg/SVGMatrix;)V
- method setRGBColorTrait (Ljava/lang/String;Lorg/w3c/dom/svg/SVGRGBColor;)V
- method setRGBColorTrait (Ljava/lang/String;Lorg/w3c/dom/svg/SVGRGBColor;)V
- method setTrait (Ljava/lang/String;Ljava/lang/String;)V
- method setTrait (Ljava/lang/String;Ljava/lang/String;)V

## org/w3c/dom/svg/SVGLocatableElement
- method <init> ()V
- method getBBox ()Lorg/w3c/dom/svg/SVGRect;
- method getBBox ()Lorg/w3c/dom/svg/SVGRect;
- method getScreenBBox ()Lorg/w3c/dom/svg/SVGRect;
- method getScreenBBox ()Lorg/w3c/dom/svg/SVGRect;
- method getScreenCTM ()Lorg/w3c/dom/svg/SVGMatrix;
- method getScreenCTM ()Lorg/w3c/dom/svg/SVGMatrix;

## org/w3c/dom/svg/SVGMatrix
- method <init> ()V
- method inverse ()Lorg/w3c/dom/svg/SVGMatrix;
- method inverse ()Lorg/w3c/dom/svg/SVGMatrix;
- method mMultiply (Lorg/w3c/dom/svg/SVGMatrix;)Lorg/w3c/dom/svg/SVGMatrix;
- method mMultiply (Lorg/w3c/dom/svg/SVGMatrix;)Lorg/w3c/dom/svg/SVGMatrix;
- method mRotate (F)Lorg/w3c/dom/svg/SVGMatrix;
- method mRotate (F)Lorg/w3c/dom/svg/SVGMatrix;
- method mTranslate (FF)Lorg/w3c/dom/svg/SVGMatrix;
- method mTranslate (FF)Lorg/w3c/dom/svg/SVGMatrix;

## org/w3c/dom/svg/SVGRGBColor
- method <init> ()V

## org/w3c/dom/svg/SVGRect
- method <init> ()V
- method getHeight ()F
- method getHeight ()F
- method getWidth ()F
- method getWidth ()F
- method getX ()F
- method getX ()F
- method getY ()F
- method getY ()F

## org/w3c/dom/svg/SVGSVGElement
- method <init> ()V
- method createSVGMatrixComponents (FFFFFF)Lorg/w3c/dom/svg/SVGMatrix;
- method createSVGMatrixComponents (FFFFFF)Lorg/w3c/dom/svg/SVGMatrix;
- method createSVGRGBColor (III)Lorg/w3c/dom/svg/SVGRGBColor;
- method createSVGRGBColor (III)Lorg/w3c/dom/svg/SVGRGBColor;
- method setCurrentTime (F)V
- method setCurrentTime (F)V

