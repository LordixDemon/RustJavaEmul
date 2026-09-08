use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_get_composite_transform_null_is_npe() -> Result<()> {
    let jvm = test_jvm().await?;
    let camera = jvm.new_class("javax/microedition/m3g/Camera", "()V", ()).await?;
    let null: ClassInstanceRef<jvm::Array<i8>> = None.into();
    let result: Result<()> = jvm
        .invoke_virtual(&camera, "getCompositeTransform", "(Ljavax/microedition/m3g/Transform;)V", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("expected NullPointerException");
    };
    assert_eq!(exception.class_definition().name(), "java/lang/NullPointerException");
    Ok(())
}

#[tokio::test]
async fn test_find_walks_skinned_mesh_skeleton() -> Result<()> {
    let jvm = test_jvm().await?;

    let bone = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&bone, "setUserID", "(I)V", (5i32,)).await?;

    let mut mesh = jvm.new_class("javax/microedition/m3g/SkinnedMesh", "()V", ()).await?;
    jvm.put_field(&mut mesh, "skeleton", "Ljavax/microedition/m3g/Group;", bone).await?;

    let root = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&root, "addChild", "(Ljavax/microedition/m3g/Node;)V", (mesh,)).await?;

    let found: ClassInstanceRef<()> = jvm.invoke_virtual(&root, "find", "(I)Ljavax/microedition/m3g/Object3D;", (5i32,)).await?;
    assert!(!found.is_null(), "Object3D.find must visit SkinnedMesh.skeleton");

    let user_id: i32 = jvm.invoke_virtual(&found, "getUserID", "()I", ()).await?;
    assert_eq!(user_id, 5);
    Ok(())
}

#[tokio::test]
async fn test_m3g_static_constants() -> Result<()> {
    let jvm = test_jvm().await?;
    let rgba: i32 = jvm.get_static_field("javax/microedition/m3g/Image2D", "RGBA", "I").await?;
    let linear: i32 = jvm.get_static_field("javax/microedition/m3g/Texture2D", "FILTER_LINEAR", "I").await?;
    let nearest: i32 = jvm.get_static_field("javax/microedition/m3g/Texture2D", "FILTER_NEAREST", "I").await?;
    let antialias: i32 = jvm.get_static_field("javax/microedition/m3g/Graphics3D", "ANTIALIAS", "I").await?;
    let ambient: i32 = jvm.get_static_field("javax/microedition/m3g/Light", "AMBIENT", "I").await?;
    assert_eq!(rgba, 100);
    assert_eq!(linear, 209);
    assert_eq!(nearest, 210);
    assert_eq!(antialias, 2);
    assert_eq!(ambient, 128);
    Ok(())
}

#[tokio::test]
async fn test_get_references_returns_direct_children() -> Result<()> {
    let jvm = test_jvm().await?;
    let root = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let child = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&root, "addChild", "(Ljavax/microedition/m3g/Node;)V", (child,))
        .await?;

    let null_refs: ClassInstanceRef<jvm::Array<ClassInstanceRef<()>>> = None.into();
    let count: i32 = jvm
        .invoke_virtual(&root, "getReferences", "([Ljavax/microedition/m3g/Object3D;)I", (null_refs,))
        .await?;
    assert_eq!(count, 1);

    let refs = jvm.instantiate_array("Ljavax/microedition/m3g/Object3D;", 1).await?;
    let filled: i32 = jvm
        .invoke_virtual(&root, "getReferences", "([Ljavax/microedition/m3g/Object3D;)I", (refs.clone(),))
        .await?;
    assert_eq!(filled, 1);
    let stored: Vec<ClassInstanceRef<()>> = jvm.load_array(&refs, 0, 1).await?;
    assert!(!stored[0].is_null());
    Ok(())
}

#[tokio::test]
async fn test_index_buffer_and_second_texture_unit() -> Result<()> {
    let jvm = test_jvm().await?;
    let lengths = {
        let mut array = jvm.instantiate_array("I", 1).await?;
        jvm.store_array(&mut array, 0, vec![3i32]).await?;
        array
    };
    let tsa = jvm
        .new_class("javax/microedition/m3g/TriangleStripArray", "(I[I)V", (0i32, lengths))
        .await?;
    let count: i32 = jvm.invoke_virtual(&tsa, "getIndexCount", "()I", ()).await?;
    assert_eq!(count, 3);

    let vb = jvm.new_class("javax/microedition/m3g/VertexBuffer", "()V", ()).await?;
    let coords = jvm.new_class("javax/microedition/m3g/VertexArray", "(III)V", (1i32, 2i32, 1i32)).await?;
    let null_f: ClassInstanceRef<jvm::Array<f32>> = None.into();
    let _: () = jvm
        .invoke_virtual(
            &vb,
            "setTexCoords",
            "(ILjavax/microedition/m3g/VertexArray;F[F)V",
            (1i32, coords, 1.0f32, null_f.clone()),
        )
        .await?;
    let fetched: ClassInstanceRef<()> = jvm
        .invoke_virtual(&vb, "getTexCoords", "(I[F)Ljavax/microedition/m3g/VertexArray;", (1i32, null_f))
        .await?;
    assert!(!fetched.is_null());

    let appearance = jvm.new_class("javax/microedition/m3g/Appearance", "()V", ()).await?;
    let image = jvm.new_class("javax/microedition/m3g/Image2D", "(III)V", (100i32, 1i32, 1i32)).await?;
    let texture = jvm
        .new_class("javax/microedition/m3g/Texture2D", "(Ljavax/microedition/m3g/Image2D;)V", (image,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&appearance, "setTexture", "(ILjavax/microedition/m3g/Texture2D;)V", (1i32, texture))
        .await?;
    let unit1: ClassInstanceRef<()> = jvm
        .invoke_virtual(&appearance, "getTexture", "(I)Ljavax/microedition/m3g/Texture2D;", (1i32,))
        .await?;
    assert!(!unit1.is_null());
    Ok(())
}

#[tokio::test]
async fn test_skinned_mesh_add_transform() -> Result<()> {
    let jvm = test_jvm().await?;
    let positions = jvm.new_class("javax/microedition/m3g/VertexArray", "(III)V", (3i32, 3i32, 1i32)).await?;
    let vb = jvm.new_class("javax/microedition/m3g/VertexBuffer", "()V", ()).await?;
    let null_f: ClassInstanceRef<jvm::Array<f32>> = None.into();
    let _: () = jvm
        .invoke_virtual(
            &vb,
            "setPositions",
            "(Ljavax/microedition/m3g/VertexArray;F[F)V",
            (positions, 1.0f32, null_f.clone()),
        )
        .await?;
    let lengths = {
        let mut array = jvm.instantiate_array("I", 1).await?;
        jvm.store_array(&mut array, 0, vec![3i32]).await?;
        array
    };
    let ib = jvm
        .new_class("javax/microedition/m3g/TriangleStripArray", "(I[I)V", (0i32, lengths))
        .await?;
    let appearance = jvm.new_class("javax/microedition/m3g/Appearance", "()V", ()).await?;
    let skeleton = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let bone = jvm.new_class("javax/microedition/m3g/Group", "()V", ()).await?;
    let _: () = jvm
        .invoke_virtual(&skeleton, "addChild", "(Ljavax/microedition/m3g/Node;)V", (bone.clone(),))
        .await?;
    let mesh = jvm
        .new_class(
            "javax/microedition/m3g/SkinnedMesh",
            "(Ljavax/microedition/m3g/VertexBuffer;Ljavax/microedition/m3g/IndexBuffer;Ljavax/microedition/m3g/Appearance;Ljavax/microedition/m3g/Group;)V",
            (vb, ib, appearance, skeleton),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &mesh,
            "addTransform",
            "(Ljavax/microedition/m3g/Node;III)V",
            (bone.clone(), 1i32, 0i32, 3i32),
        )
        .await?;
    let null_i: ClassInstanceRef<jvm::Array<i32>> = None.into();
    let count: i32 = jvm
        .invoke_virtual(&mesh, "getBoneVertices", "(Ljavax/microedition/m3g/Node;[I[F)I", (bone, null_i, null_f))
        .await?;
    assert_eq!(count, 3);
    Ok(())
}

#[tokio::test]
async fn test_graphics3d_singleton_and_spec_properties() -> Result<()> {
    let jvm = test_jvm().await?;
    let first: ClassInstanceRef<()> = jvm
        .invoke_static(
            "javax/microedition/m3g/Graphics3D",
            "getInstance",
            "()Ljavax/microedition/m3g/Graphics3D;",
            (),
        )
        .await?;
    let second: ClassInstanceRef<()> = jvm
        .invoke_static(
            "javax/microedition/m3g/Graphics3D",
            "getInstance",
            "()Ljavax/microedition/m3g/Graphics3D;",
            (),
        )
        .await?;
    let first_id: i32 = jvm
        .invoke_static("java/lang/System", "identityHashCode", "(Ljava/lang/Object;)I", (first,))
        .await?;
    let second_id: i32 = jvm
        .invoke_static("java/lang/System", "identityHashCode", "(Ljava/lang/Object;)I", (second,))
        .await?;
    assert_eq!(first_id, second_id);

    let table: ClassInstanceRef<()> = jvm
        .invoke_static("javax/microedition/m3g/Graphics3D", "getProperties", "()Ljava/util/Hashtable;", ())
        .await?;
    let antialias_key = JavaLangString::from_rust_string(&jvm, "supportAntialiasing").await?;
    let antialias: ClassInstanceRef<()> = jvm
        .invoke_virtual(&table, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (antialias_key,))
        .await?;
    assert!(!antialias.is_null());
    let antialias: bool = jvm.invoke_virtual(&antialias, "booleanValue", "()Z", ()).await?;
    assert!(!antialias);

    let units_key = JavaLangString::from_rust_string(&jvm, "numTextureUnits").await?;
    let units: ClassInstanceRef<()> = jvm
        .invoke_virtual(&table, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (units_key,))
        .await?;
    let units: i32 = jvm.invoke_virtual(&units, "intValue", "()I", ()).await?;
    assert_eq!(units, 2);

    let version: ClassInstanceRef<()> = jvm
        .invoke_static(
            "java/lang/System",
            "getProperty",
            "(Ljava/lang/String;)Ljava/lang/String;",
            (JavaLangString::from_rust_string(&jvm, "microedition.m3g.version").await?,),
        )
        .await?;
    assert!(!version.is_null());
    assert_eq!(JavaLangString::to_rust_string(&jvm, &version).await?, "1.1");
    Ok(())
}
