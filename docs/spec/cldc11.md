# CLDC 1.1 public API (machine-readable target)

This dump lists public members required for `api_complete` of packages referenced by the JAR corpus. Runtime registration is dumped separately in `runtime_api.md`.

## java.lang.Boolean
- method <init> (Z)V
- method <init> (Ljava/lang/String;)V
- method booleanValue ()Z
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method toString ()Ljava/lang/String;
- method toString (Z)Ljava/lang/String;
- method valueOf (Z)Ljava/lang/Boolean;
- method valueOf (Ljava/lang/String;)Ljava/lang/Boolean;
- field TRUE Ljava/lang/Boolean;
- field FALSE Ljava/lang/Boolean;

## java.lang.Byte
- method <init> (B)V
- method byteValue ()B
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method floatValue ()F
- method hashCode ()I
- method intValue ()I
- method longValue ()J
- method parseByte (Ljava/lang/String;)B
- method shortValue ()S
- method toString ()Ljava/lang/String;
- method toString (B)Ljava/lang/String;
- method valueOf (B)Ljava/lang/Byte;
- field MIN_VALUE B
- field MAX_VALUE B

## java.lang.Character
- method <init> (C)V
- method charValue ()C
- method digit (CI)I
- method equals (Ljava/lang/Object;)Z
- method hashCode ()I
- method isDigit (C)Z
- method isLowerCase (C)Z
- method isUpperCase (C)Z
- method toLowerCase (C)C
- method toString ()Ljava/lang/String;
- method toUpperCase (C)C
- field MIN_VALUE C
- field MAX_VALUE C
- field MIN_RADIX I
- field MAX_RADIX I

## java.lang.Double
- method <init> (D)V
- method doubleToLongBits (D)J
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method floatValue ()F
- method hashCode ()I
- method intValue ()I
- method isInfinite (D)Z
- method isNaN (D)Z
- method longBitsToDouble (J)D
- method longValue ()J
- method parseDouble (Ljava/lang/String;)D
- method toString ()Ljava/lang/String;
- method toString (D)Ljava/lang/String;
- method valueOf (D)Ljava/lang/Double;
- field NaN D
- field POSITIVE_INFINITY D
- field NEGATIVE_INFINITY D
- field MIN_VALUE D
- field MAX_VALUE D

## java.lang.Float
- method <init> (F)V
- method <init> (Ljava/lang/String;)V
- method floatToIntBits (F)I
- method floatValue ()F
- method intBitsToFloat (I)F
- method isInfinite (F)Z
- method isInfinite ()Z
- method isNaN (F)Z
- method isNaN ()Z
- method parseFloat (Ljava/lang/String;)F
- method toString ()Ljava/lang/String;
- method toString (F)Ljava/lang/String;
- method valueOf (F)Ljava/lang/Float;
- field NaN F
- field POSITIVE_INFINITY F
- field NEGATIVE_INFINITY F
- field MIN_VALUE F
- field MAX_VALUE F

## java.lang.Integer
- method <init> (I)V
- method <init> (Ljava/lang/String;)V
- method byteValue ()B
- method doubleValue ()D
- method equals (Ljava/lang/Object;)Z
- method floatValue ()F
- method hashCode ()I
- method intValue ()I
- method longValue ()J
- method parseInt (Ljava/lang/String;)I
- method parseInt (Ljava/lang/String;I)I
- method shortValue ()S
- method toBinaryString (I)Ljava/lang/String;
- method toHexString (I)Ljava/lang/String;
- method toOctalString (I)Ljava/lang/String;
- method toString ()Ljava/lang/String;
- method toString (I)Ljava/lang/String;
- method valueOf (Ljava/lang/String;)Ljava/lang/Integer;
- method valueOf (Ljava/lang/String;I)Ljava/lang/Integer;
- field MIN_VALUE I
- field MAX_VALUE I

## java.lang.Long
- method <init> (J)V
- method <init> (Ljava/lang/String;)V
- method parseLong (Ljava/lang/String;)J
- method parseLong (Ljava/lang/String;I)J
- method toString ()Ljava/lang/String;
- method toString (J)Ljava/lang/String;
- method toString (JI)Ljava/lang/String;
- method valueOf (J)Ljava/lang/Long;
- field MIN_VALUE J
- field MAX_VALUE J

## java.lang.Math
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
- method ceil (D)D
- method floor (D)D
- method toRadians (D)D
- method toDegrees (D)D
- field E D
- field PI D

## java.lang.System
- method arraycopy (Ljava/lang/Object;ILjava/lang/Object;II)V
- method currentTimeMillis ()J
- method exit (I)V
- method gc ()V
- method getProperty (Ljava/lang/String;)Ljava/lang/String;
- method identityHashCode (Ljava/lang/Object;)I
- field out Ljava/io/PrintStream;
- field err Ljava/io/PrintStream;

## java.util.Hashtable
- method <init> ()V
- method <init> (I)V
- method clear ()V
- method contains (Ljava/lang/Object;)Z
- method containsKey (Ljava/lang/Object;)Z
- method elements ()Ljava/util/Enumeration;
- method get (Ljava/lang/Object;)Ljava/lang/Object;
- method isEmpty ()Z
- method keys ()Ljava/util/Enumeration;
- method put (Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
- method remove (Ljava/lang/Object;)Ljava/lang/Object;
- method size ()I

## java.util.Vector
- method <init> ()V
- method <init> (I)V
- method <init> (II)V
- method addElement (Ljava/lang/Object;)V
- method capacity ()I
- method contains (Ljava/lang/Object;)Z
- method copyInto ([Ljava/lang/Object;)V
- method elementAt (I)Ljava/lang/Object;
- method elements ()Ljava/util/Enumeration;
- method ensureCapacity (I)V
- method firstElement ()Ljava/lang/Object;
- method indexOf (Ljava/lang/Object;)I
- method indexOf (Ljava/lang/Object;I)I
- method insertElementAt (Ljava/lang/Object;I)V
- method isEmpty ()Z
- method lastElement ()Ljava/lang/Object;
- method lastIndexOf (Ljava/lang/Object;)I
- method lastIndexOf (Ljava/lang/Object;I)I
- method removeAllElements ()V
- method removeElement (Ljava/lang/Object;)Z
- method removeElementAt (I)V
- method setElementAt (Ljava/lang/Object;I)V
- method setSize (I)V
- method size ()I
- method toString ()Ljava/lang/String;
- method trimToSize ()V
