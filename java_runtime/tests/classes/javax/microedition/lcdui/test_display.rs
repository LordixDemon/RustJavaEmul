use alloc::{boxed::Box, collections::BTreeMap, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::{
        java::lang::String,
        java::util::Vector,
        javax::microedition::{
            MIDlet,
            lcdui::{Canvas, Command, Display, Displayable, Font, Form, Graphics, Image, Item, StringItem, TextBox, TextField},
        },
    },
};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TestRunnable;

impl TestRunnable {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestRunnable",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("runCount", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        let count: i32 = jvm.get_field(&this, "runCount", "I").await?;
        jvm.put_field(&mut this, "runCount", "I", count + 1).await
    }
}

async fn test_jvm_with_runnable() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    let class = Box::new(ClassDefinitionImpl::from_class_proto(
        TestRunnable::as_proto(),
        Box::new(runtime) as Box<_>,
    ));
    jvm.register_class(class, None).await?;
    Ok(jvm)
}

async fn wait_run_count(jvm: &Jvm, runnable: &Box<dyn jvm::ClassInstance>, expected: i32) -> Result<i32> {
    for _ in 0..200 {
        let run_count: i32 = jvm.get_field(runnable, "runCount", "I").await?;
        if run_count >= expected {
            return Ok(run_count);
        }
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
    jvm.get_field(runnable, "runCount", "I").await
}

#[tokio::test]
async fn display_get_display_returns_singleton() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let midlet = ClassInstanceRef::<MIDlet>::new(None);

    let display_a: ClassInstanceRef<Display> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Display",
            "getDisplay",
            "(Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;",
            (midlet.clone(),),
        )
        .await?;
    let display_b: ClassInstanceRef<Display> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Display",
            "getDisplay",
            "(Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;",
            (midlet,),
        )
        .await?;

    assert!(
        display_a
            .instance
            .as_ref()
            .unwrap()
            .equals(display_b.instance.as_ref().unwrap().as_ref())?
    );
    Ok(())
}

#[tokio::test]
async fn display_tracks_current_displayable() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let midlet = ClassInstanceRef::<MIDlet>::new(None);
    let display: ClassInstanceRef<Display> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Display",
            "getDisplay",
            "(Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;",
            (midlet,),
        )
        .await?;
    let form = jvm
        .new_class(
            "javax/microedition/lcdui/Form",
            "(Ljava/lang/String;)V",
            (ClassInstanceRef::<String>::new(None),),
        )
        .await?;

    let _: () = jvm
        .invoke_virtual(&display, "setCurrent", "(Ljavax/microedition/lcdui/Displayable;)V", (form.clone(),))
        .await?;
    let current: ClassInstanceRef<Displayable> = jvm
        .invoke_virtual(&display, "getCurrent", "()Ljavax/microedition/lcdui/Displayable;", ())
        .await?;

    assert!(current.instance.as_ref().unwrap().equals(form.as_ref())?);
    Ok(())
}

#[tokio::test]
async fn display_call_serially_runs_runnable() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let midlet = ClassInstanceRef::<MIDlet>::new(None);
    let display: ClassInstanceRef<Display> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Display",
            "getDisplay",
            "(Ljavax/microedition/midlet/MIDlet;)Ljavax/microedition/lcdui/Display;",
            (midlet,),
        )
        .await?;
    let runnable = jvm.new_class("TestRunnable", "()V", ()).await?;

    let _: () = jvm
        .invoke_virtual(&display, "callSerially", "(Ljava/lang/Runnable;)V", (runnable.clone(),))
        .await?;
    let run_count = wait_run_count(&jvm, &runnable, 1).await?;
    assert_eq!(run_count, 1);

    let _: () = jvm
        .invoke_virtual(&display, "callSerially", "(Ljava/lang/Runnable;)V", (runnable.clone(),))
        .await?;
    let run_count = wait_run_count(&jvm, &runnable, 2).await?;
    assert_eq!(run_count, 2);

    Ok(())
}

#[tokio::test]
async fn command_constants_and_accessors_match_midp() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let label = JavaLangString::from_rust_string(&jvm, "Back").await?;
    let command = jvm
        .new_class("javax/microedition/lcdui/Command", "(Ljava/lang/String;II)V", (label.clone(), 2, 7))
        .await?;

    let back: i32 = jvm.get_static_field("javax/microedition/lcdui/Command", "BACK", "I").await?;
    let exit: i32 = jvm.get_static_field("javax/microedition/lcdui/Command", "EXIT", "I").await?;
    let command_type: i32 = jvm.invoke_virtual(&command, "getCommandType", "()I", ()).await?;
    let priority: i32 = jvm.invoke_virtual(&command, "getPriority", "()I", ()).await?;
    let returned_label: ClassInstanceRef<String> = jvm.invoke_virtual(&command, "getLabel", "()Ljava/lang/String;", ()).await?;
    let returned_long_label: ClassInstanceRef<String> = jvm.invoke_virtual(&command, "getLongLabel", "()Ljava/lang/String;", ()).await?;

    assert_eq!(back, 2);
    assert_eq!(exit, 7);
    assert_eq!(command_type, 2);
    assert_eq!(priority, 7);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &returned_label).await?, "Back");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &returned_long_label).await?, "Back");

    Ok(())
}

#[tokio::test]
async fn displayable_stores_commands_without_duplicates() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let form = jvm
        .new_class(
            "javax/microedition/lcdui/Form",
            "(Ljava/lang/String;)V",
            (ClassInstanceRef::<String>::new(None),),
        )
        .await?;
    let label = JavaLangString::from_rust_string(&jvm, "OK").await?;
    let command: ClassInstanceRef<Command> = jvm
        .new_class("javax/microedition/lcdui/Command", "(Ljava/lang/String;II)V", (label, 4, 1))
        .await?
        .into();

    let _: () = jvm
        .invoke_virtual(&form, "addCommand", "(Ljavax/microedition/lcdui/Command;)V", (command.clone(),))
        .await?;
    let _: () = jvm
        .invoke_virtual(&form, "addCommand", "(Ljavax/microedition/lcdui/Command;)V", (command.clone(),))
        .await?;
    let commands: ClassInstanceRef<Vector> = jvm.get_field(&form, "commands", "Ljava/util/Vector;").await?;
    let size: i32 = jvm.invoke_virtual(&commands, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let _: () = jvm
        .invoke_virtual(&form, "removeCommand", "(Ljavax/microedition/lcdui/Command;)V", (command,))
        .await?;
    let size: i32 = jvm.invoke_virtual(&commands, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn form_tracks_title_and_item_count() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let title = JavaLangString::from_rust_string(&jvm, "Options").await?;
    let form: ClassInstanceRef<Form> = jvm
        .new_class("javax/microedition/lcdui/Form", "(Ljava/lang/String;)V", (title,))
        .await?
        .into();
    let row = JavaLangString::from_rust_string(&jvm, "Sound").await?;

    let first_index: i32 = jvm.invoke_virtual(&form, "append", "(Ljava/lang/String;)I", (row.clone(),)).await?;
    let second_index: i32 = jvm.invoke_virtual(&form, "append", "(Ljava/lang/String;)I", (row,)).await?;
    let size: i32 = jvm.invoke_virtual(&form, "size", "()I", ()).await?;

    let new_title = JavaLangString::from_rust_string(&jvm, "Settings").await?;
    let _: () = jvm.invoke_virtual(&form, "setTitle", "(Ljava/lang/String;)V", (new_title,)).await?;
    let returned_title: ClassInstanceRef<String> = jvm.invoke_virtual(&form, "getTitle", "()Ljava/lang/String;", ()).await?;
    let _: () = jvm.invoke_virtual(&form, "deleteAll", "()V", ()).await?;
    let cleared_size: i32 = jvm.invoke_virtual(&form, "size", "()I", ()).await?;

    assert_eq!(first_index, 0);
    assert_eq!(second_index, 1);
    assert_eq!(size, 2);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &returned_title).await?, "Settings");
    assert_eq!(cleared_size, 0);

    Ok(())
}

#[tokio::test]
async fn form_stores_string_image_and_item_instances() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let form: ClassInstanceRef<Form> = jvm
        .new_class(
            "javax/microedition/lcdui/Form",
            "(Ljava/lang/String;)V",
            (ClassInstanceRef::<String>::new(None),),
        )
        .await?
        .into();
    let text = JavaLangString::from_rust_string(&jvm, "Player").await?;

    let text_index: i32 = jvm.invoke_virtual(&form, "append", "(Ljava/lang/String;)I", (text,)).await?;
    let text_item: ClassInstanceRef<Item> = jvm
        .invoke_virtual(&form, "get", "(I)Ljavax/microedition/lcdui/Item;", (text_index,))
        .await?;
    let returned_text: ClassInstanceRef<String> = jvm.invoke_virtual(&text_item, "getText", "()Ljava/lang/String;", ()).await?;

    let image: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(II)Ljavax/microedition/lcdui/Image;",
            (2, 2),
        )
        .await?;
    let image_index: i32 = jvm
        .invoke_virtual(&form, "append", "(Ljavax/microedition/lcdui/Image;)I", (image.clone(),))
        .await?;
    let image_item: ClassInstanceRef<Item> = jvm
        .invoke_virtual(&form, "get", "(I)Ljavax/microedition/lcdui/Item;", (image_index,))
        .await?;
    let returned_image: ClassInstanceRef<Image> = jvm
        .invoke_virtual(&image_item, "getImage", "()Ljavax/microedition/lcdui/Image;", ())
        .await?;

    let custom_item: ClassInstanceRef<StringItem> = jvm
        .new_class(
            "javax/microedition/lcdui/StringItem",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            (
                ClassInstanceRef::<String>::new(None),
                JavaLangString::from_rust_string(&jvm, "Custom").await?,
            ),
        )
        .await?
        .into();
    let custom_index: i32 = jvm
        .invoke_virtual(
            &form,
            "append",
            "(Ljavax/microedition/lcdui/Item;)I",
            (ClassInstanceRef::<Item>::new(custom_item.instance),),
        )
        .await?;
    let size: i32 = jvm.invoke_virtual(&form, "size", "()I", ()).await?;

    assert_eq!(text_index, 0);
    assert_eq!(image_index, 1);
    assert_eq!(custom_index, 2);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &returned_text).await?, "Player");
    assert!(!returned_image.is_null());
    assert_eq!(size, 3);

    Ok(())
}

#[tokio::test]
async fn textbox_supports_titles_limits_and_editing() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let title = JavaLangString::from_rust_string(&jvm, "Name").await?;
    let text = JavaLangString::from_rust_string(&jvm, "abc").await?;
    let textbox: ClassInstanceRef<TextBox> = jvm
        .new_class(
            "javax/microedition/lcdui/TextBox",
            "(Ljava/lang/String;Ljava/lang/String;II)V",
            (title, text, 10, 0),
        )
        .await?
        .into();

    let _: () = jvm
        .invoke_virtual(
            &textbox,
            "insert",
            "(Ljava/lang/String;I)V",
            (JavaLangString::from_rust_string(&jvm, "ZZ").await?, 1),
        )
        .await?;
    let edited: ClassInstanceRef<String> = jvm.invoke_virtual(&textbox, "getString", "()Ljava/lang/String;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &edited).await?, "aZZbc");

    let _: () = jvm.invoke_virtual(&textbox, "delete", "(II)V", (1, 2)).await?;
    let size: i32 = jvm.invoke_virtual(&textbox, "size", "()I", ()).await?;
    let max_size: i32 = jvm.invoke_virtual(&textbox, "setMaxSize", "(I)I", (2,)).await?;
    let truncated: ClassInstanceRef<String> = jvm.invoke_virtual(&textbox, "getString", "()Ljava/lang/String;", ()).await?;

    let chars = jvm.instantiate_array("C", 2).await?;
    let copied: i32 = jvm.invoke_virtual(&textbox, "getChars", "([C)I", (chars.clone(),)).await?;
    let copied_chars: Vec<JavaChar> = jvm.load_array(&chars, 0, 2).await?;

    let _: () = jvm.invoke_virtual(&textbox, "setConstraints", "(I)V", (2,)).await?;
    let constraints: i32 = jvm.invoke_virtual(&textbox, "getConstraints", "()I", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &textbox,
            "setTitle",
            "(Ljava/lang/String;)V",
            (JavaLangString::from_rust_string(&jvm, "Driver").await?,),
        )
        .await?;
    let returned_title: ClassInstanceRef<String> = jvm.invoke_virtual(&textbox, "getTitle", "()Ljava/lang/String;", ()).await?;

    assert_eq!(size, 3);
    assert_eq!(max_size, 2);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &truncated).await?, "ab");
    assert_eq!(copied, 2);
    assert_eq!(copied_chars, vec![b'a' as JavaChar, b'b' as JavaChar]);
    assert_eq!(constraints, 2);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &returned_title).await?, "Driver");

    Ok(())
}

#[tokio::test]
async fn textfield_supports_item_text_editing() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let label = JavaLangString::from_rust_string(&jvm, "Name").await?;
    let text = JavaLangString::from_rust_string(&jvm, "car").await?;
    let field: ClassInstanceRef<TextField> = jvm
        .new_class(
            "javax/microedition/lcdui/TextField",
            "(Ljava/lang/String;Ljava/lang/String;II)V",
            (label, text, 8, 0),
        )
        .await?
        .into();

    let numeric: i32 = jvm.get_static_field("javax/microedition/lcdui/TextField", "NUMERIC", "I").await?;
    let _: () = jvm.invoke_virtual(&field, "setConstraints", "(I)V", (numeric,)).await?;
    let _: () = jvm
        .invoke_virtual(
            &field,
            "insert",
            "(Ljava/lang/String;I)V",
            (JavaLangString::from_rust_string(&jvm, "bon").await?, 0),
        )
        .await?;
    let _: () = jvm.invoke_virtual(&field, "delete", "(II)V", (0, 3)).await?;
    let caret: i32 = jvm.invoke_virtual(&field, "getCaretPosition", "()I", ()).await?;
    let value: ClassInstanceRef<String> = jvm.invoke_virtual(&field, "getString", "()Ljava/lang/String;", ()).await?;
    let constraints: i32 = jvm.invoke_virtual(&field, "getConstraints", "()I", ()).await?;

    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "car");
    assert_eq!(caret, 3);
    assert_eq!(constraints, numeric);

    Ok(())
}

#[tokio::test]
async fn canvas_exposes_midp_key_capabilities() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let canvas: ClassInstanceRef<Canvas> = jvm.new_class("javax/microedition/lcdui/Canvas", "()V", ()).await?.into();

    let up: i32 = jvm.get_static_field("javax/microedition/lcdui/Canvas", "UP", "I").await?;
    let game_action: i32 = jvm.invoke_virtual(&canvas, "getGameAction", "(I)I", (b'2' as i32,)).await?;
    let key_code: i32 = jvm.invoke_virtual(&canvas, "getKeyCode", "(I)I", (up,)).await?;
    let key_name: ClassInstanceRef<String> = jvm.invoke_virtual(&canvas, "getKeyName", "(I)Ljava/lang/String;", (-5,)).await?;
    let double_buffered: bool = jvm.invoke_virtual(&canvas, "isDoubleBuffered", "()Z", ()).await?;
    let pointer_events: bool = jvm.invoke_virtual(&canvas, "hasPointerEvents", "()Z", ()).await?;

    assert_eq!(up, 1);
    assert_eq!(game_action, 1);
    assert_eq!(key_code, -1);
    assert_eq!(JavaLangString::to_rust_string(&jvm, &key_name).await?, "FIRE");
    assert!(double_buffered);
    assert!(pointer_events);

    Ok(())
}

#[tokio::test]
async fn font_exposes_midp_constants_and_accessors() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;

    let bold: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "STYLE_BOLD", "I").await?;
    let italic: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "STYLE_ITALIC", "I").await?;
    let large: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "SIZE_LARGE", "I").await?;
    let font: ClassInstanceRef<Font> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Font",
            "getFont",
            "(III)Ljavax/microedition/lcdui/Font;",
            (0, bold | italic, large),
        )
        .await?;

    let style: i32 = jvm.invoke_virtual(&font, "getStyle", "()I", ()).await?;
    let size: i32 = jvm.invoke_virtual(&font, "getSize", "()I", ()).await?;
    let is_bold: bool = jvm.invoke_virtual(&font, "isBold", "()Z", ()).await?;
    let is_italic: bool = jvm.invoke_virtual(&font, "isItalic", "()Z", ()).await?;
    let is_plain: bool = jvm.invoke_virtual(&font, "isPlain", "()Z", ()).await?;

    assert_eq!(bold, 1);
    assert_eq!(italic, 2);
    assert_eq!(large, 16);
    assert_eq!(style, 3);
    assert_eq!(size, 16);
    assert!(is_bold);
    assert!(is_italic);
    assert!(!is_plain);

    Ok(())
}

#[tokio::test]
async fn font_metrics_are_ordered_and_expose_baseline() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;

    let small: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "SIZE_SMALL", "I").await?;
    let medium: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "SIZE_MEDIUM", "I").await?;
    let large: i32 = jvm.get_static_field("javax/microedition/lcdui/Font", "SIZE_LARGE", "I").await?;
    let small_font: ClassInstanceRef<Font> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Font",
            "getFont",
            "(III)Ljavax/microedition/lcdui/Font;",
            (0, 0, small),
        )
        .await?;
    let medium_font: ClassInstanceRef<Font> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Font",
            "getFont",
            "(III)Ljavax/microedition/lcdui/Font;",
            (0, 0, medium),
        )
        .await?;
    let large_font: ClassInstanceRef<Font> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Font",
            "getFont",
            "(III)Ljavax/microedition/lcdui/Font;",
            (0, 0, large),
        )
        .await?;

    let small_height: i32 = jvm.invoke_virtual(&small_font, "getHeight", "()I", ()).await?;
    let medium_height: i32 = jvm.invoke_virtual(&medium_font, "getHeight", "()I", ()).await?;
    let large_height: i32 = jvm.invoke_virtual(&large_font, "getHeight", "()I", ()).await?;
    let small_baseline: i32 = jvm.invoke_virtual(&small_font, "getBaselinePosition", "()I", ()).await?;
    let medium_baseline: i32 = jvm.invoke_virtual(&medium_font, "getBaselinePosition", "()I", ()).await?;
    let large_baseline: i32 = jvm.invoke_virtual(&large_font, "getBaselinePosition", "()I", ()).await?;

    assert_eq!((small_height, medium_height, large_height), (8, 12, 16));
    assert!(small_height < medium_height);
    assert!(medium_height < large_height);
    assert!((1..small_height).contains(&small_baseline));
    assert!((1..medium_height).contains(&medium_baseline));
    assert!((1..large_height).contains(&large_baseline));

    Ok(())
}

#[tokio::test]
async fn graphics_draw_string_uses_font_metrics_for_center_anchor() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let image: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(II)Ljavax/microedition/lcdui/Image;",
            (48, 24),
        )
        .await?;
    let graphics: ClassInstanceRef<Graphics> = jvm
        .invoke_virtual(&image, "getGraphics", "()Ljavax/microedition/lcdui/Graphics;", ())
        .await?;
    let hcenter: i32 = jvm.get_static_field("javax/microedition/lcdui/Graphics", "HCENTER", "I").await?;
    let vcenter: i32 = jvm.get_static_field("javax/microedition/lcdui/Graphics", "VCENTER", "I").await?;
    let text = JavaLangString::from_rust_string(&jvm, "OK").await?;

    let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0x00ff_ffff,)).await?;
    let _: () = jvm
        .invoke_virtual(&graphics, "drawString", "(Ljava/lang/String;III)V", (text, 24, 12, hcenter | vcenter))
        .await?;

    let pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(&image, "argb", "[I").await?;
    let pixels: Vec<i32> = jvm.load_array(&pixels, 0, 48 * 24).await?;
    let mut min_x = 48;
    let mut min_y = 24;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut count = 0;
    for (index, pixel) in pixels.iter().enumerate() {
        if *pixel == 0 {
            continue;
        }
        let x = index % 48;
        let y = index / 48;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
        count += 1;
    }

    assert!(count > 0);
    assert!((min_x + max_x + 1).abs_diff(48) <= 1);
    assert!((min_y + max_y + 1).abs_diff(24) <= 1);

    Ok(())
}

#[tokio::test]
async fn graphics_tracks_state_and_applies_translation() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let image: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(II)Ljavax/microedition/lcdui/Image;",
            (20, 20),
        )
        .await?;
    let graphics: ClassInstanceRef<Graphics> = jvm
        .invoke_virtual(&image, "getGraphics", "()Ljavax/microedition/lcdui/Graphics;", ())
        .await?;

    let _: () = jvm.invoke_virtual(&graphics, "setColor", "(I)V", (0x00ff0000,)).await?;
    let red: i32 = jvm.invoke_virtual(&graphics, "getRedComponent", "()I", ()).await?;
    let green: i32 = jvm.invoke_virtual(&graphics, "getGreenComponent", "()I", ()).await?;
    let _: () = jvm.invoke_virtual(&graphics, "translate", "(II)V", (3, 4)).await?;
    let _: () = jvm.invoke_virtual(&graphics, "setClip", "(IIII)V", (0, 0, 20, 20)).await?;
    let clip_x: i32 = jvm.invoke_virtual(&graphics, "getClipX", "()I", ()).await?;
    let translate_x: i32 = jvm.invoke_virtual(&graphics, "getTranslateX", "()I", ()).await?;
    let _: () = jvm.invoke_virtual(&graphics, "fillRect", "(IIII)V", (0, 0, 2, 2)).await?;

    let pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(&image, "argb", "[I").await?;
    let pixel: Vec<i32> = jvm.load_array(&pixels, 4 * 20 + 3, 1).await?;

    assert_eq!(red, 255);
    assert_eq!(green, 0);
    assert_eq!(clip_x, 0);
    assert_eq!(translate_x, 3);
    assert_eq!(pixel[0], 0xffff0000u32 as i32);

    Ok(())
}

#[tokio::test]
async fn image_copy_and_region_transform_preserve_pixels() -> Result<()> {
    let jvm = test_jvm_with_runnable().await?;
    let mut rgb = jvm.instantiate_array("I", 4).await?;
    let source_pixels = vec![0xff000001u32 as i32, 0xff000002u32 as i32, 0xff000003u32 as i32, 0xff000004u32 as i32];
    jvm.store_array(&mut rgb, 0, source_pixels.clone()).await?;

    let image: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createRGBImage",
            "([IIIZ)Ljavax/microedition/lcdui/Image;",
            (rgb, 2, 2, true),
        )
        .await?;
    let copy: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(Ljavax/microedition/lcdui/Image;)Ljavax/microedition/lcdui/Image;",
            (image.clone(),),
        )
        .await?;
    let rotated: ClassInstanceRef<Image> = jvm
        .invoke_static(
            "javax/microedition/lcdui/Image",
            "createImage",
            "(Ljavax/microedition/lcdui/Image;IIIII)Ljavax/microedition/lcdui/Image;",
            (image.clone(), 0, 0, 2, 2, 5),
        )
        .await?;

    let mutable: bool = jvm.invoke_virtual(&image, "isMutable", "()Z", ()).await?;
    let copy_pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(&copy, "argb", "[I").await?;
    let rotated_pixels: ClassInstanceRef<Array<i32>> = jvm.get_field(&rotated, "argb", "[I").await?;
    let copy_pixels: Vec<i32> = jvm.load_array(&copy_pixels, 0, 4).await?;
    let rotated_pixels: Vec<i32> = jvm.load_array(&rotated_pixels, 0, 4).await?;

    assert!(!mutable);
    assert_eq!(copy_pixels, source_pixels);
    assert_eq!(
        rotated_pixels,
        vec![0xff000003u32 as i32, 0xff000001u32 as i32, 0xff000004u32 as i32, 0xff000002u32 as i32]
    );

    Ok(())
}
