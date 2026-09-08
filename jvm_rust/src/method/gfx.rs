use alloc::boxed::Box;
use core::slice;

use jvm::{ClassInstance, JavaValue};

use crate::array_class_instance::ArrayClassInstanceImpl;
use crate::class_instance::ClassInstanceImpl;

const TARGET_IMAGE: &str = "targetImage";
const TARGET_DESC: &str = "Ljavax/microedition/lcdui/Image;";

struct GraphicsCache {
    color_slot: Option<usize>,
    color: i32,
    translate_x: i32,
    translate_y: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    width: i32,
    height: i32,
    pixels: Box<dyn ClassInstance>,
    pixel_ptr: *mut i32,
    pixel_len: usize,
}

unsafe impl Send for GraphicsCache {}
unsafe impl Sync for GraphicsCache {}

impl GraphicsCache {
    fn clip(&self) -> Clip {
        Clip {
            translate_x: self.translate_x,
            translate_y: self.translate_y,
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            width: self.width,
            height: self.height,
            color: self.color,
        }
    }

    fn pixels_mut(&mut self) -> &mut [i32] {
        unsafe { slice::from_raw_parts_mut(self.pixel_ptr, self.pixel_len) }
    }

    fn set_clip_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let x = x.max(0);
        let y = y.max(0);
        let x2 = (x + width).min(self.width);
        let y2 = (y + height).min(self.height);
        self.x = x;
        self.y = y;
        self.w = (x2 - x).max(0);
        self.h = (y2 - y).max(0);
    }
}

#[derive(Clone, Copy)]
struct Clip {
    translate_x: i32,
    translate_y: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    width: i32,
    height: i32,
    color: i32,
}

pub(super) struct GfxSession {
    clip: Clip,
    pixels: *mut i32,
    pixel_len: usize,
}

impl GfxSession {
    pub(super) fn warm(graphics: &ClassInstanceImpl) -> Option<Self> {
        if let Some(session) = Self::begin(graphics) {
            return Some(session);
        }
        let _ = with_offscreen(graphics, |_, _| {});
        Self::begin(graphics)
    }

    fn begin(graphics: &ClassInstanceImpl) -> Option<Self> {
        let cache = graphics.native_mut::<GraphicsCache>()?;
        if cache.pixel_ptr.is_null() || cache.pixel_len == 0 {
            return None;
        }
        Some(Self {
            clip: cache.clip(),
            pixels: cache.pixel_ptr,
            pixel_len: cache.pixel_len,
        })
    }

    fn pixels(&mut self) -> &mut [i32] {
        unsafe { slice::from_raw_parts_mut(self.pixels, self.pixel_len) }
    }

    pub(super) fn set_color(&mut self, rgb: i32) {
        self.clip.color = (0xff00_0000u32 as i32) | (rgb & 0x00ff_ffff);
    }

    pub(super) fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let clip = self.clip;
        fill_rect_clip(clip, self.pixels(), x, y, width, height);
    }

    pub(super) fn draw_line(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) {
        let clip = self.clip;
        x1 += clip.translate_x;
        y1 += clip.translate_y;
        x2 += clip.translate_x;
        y2 += clip.translate_y;
        plot_line(self.pixels(), clip, x1, y1, x2, y2);
    }

    pub(super) fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        let clip = self.clip;
        fill_triangle_spans(
            self.pixels(),
            clip,
            x1 + clip.translate_x,
            y1 + clip.translate_y,
            x2 + clip.translate_x,
            y2 + clip.translate_y,
            x3 + clip.translate_x,
            y3 + clip.translate_y,
        );
    }

    #[allow(dead_code)]
    pub(super) fn draw_rgb(
        &mut self,
        src: &[i32],
        offset: i32,
        scan_length: i32,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        process_alpha: bool,
    ) {
        if width <= 0 || height <= 0 || scan_length <= 0 || offset < 0 {
            return;
        }
        let src_offset = offset as usize;
        if src_offset >= src.len() {
            return;
        }
        let src = &src[src_offset..];
        let clip = self.clip;
        let x = x + clip.translate_x;
        let y = y + clip.translate_y;
        let (draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y) = clipped_rect(x, y, width, height, clip.x, clip.y, clip.w, clip.h);
        blit_i32(
            self.pixels(),
            clip.width,
            src,
            scan_length,
            draw_x,
            draw_y,
            draw_w,
            draw_h,
            src_off_x,
            src_off_y,
            process_alpha,
        );
    }

    pub(super) fn finish(self, graphics: &ClassInstanceImpl) {
        if let Some(cache) = graphics.native_mut::<GraphicsCache>() {
            cache.color = self.clip.color;
        }
    }
}

pub(super) fn set_color_fast(graphics: &ClassInstanceImpl, rgb: i32) -> bool {
    let rgb = rgb & 0x00ff_ffff;
    let color = (0xff00_0000u32 as i32) | rgb;
    if let Some(cache) = graphics.native_mut::<GraphicsCache>() {
        cache.color = color;
        return true;
    }
    set_color(graphics, rgb)
}

pub(super) fn fill_rect_fast(graphics: &ClassInstanceImpl, x: i32, y: i32, width: i32, height: i32) -> bool {
    if width <= 0 || height <= 0 {
        return true;
    }
    if let Some(cache) = graphics.native_mut::<GraphicsCache>() {
        let clip = cache.clip();
        let pixels = cache.pixels_mut();
        fill_rect_clip(clip, pixels, x, y, width, height);
        return true;
    }
    fill_rect(graphics, x, y, width, height)
}

pub(super) fn draw_line_fast(graphics: &ClassInstanceImpl, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) -> bool {
    if let Some(cache) = graphics.native_mut::<GraphicsCache>() {
        let clip = cache.clip();
        let pixels = cache.pixels_mut();
        x1 += clip.translate_x;
        y1 += clip.translate_y;
        x2 += clip.translate_x;
        y2 += clip.translate_y;
        plot_line(pixels, clip, x1, y1, x2, y2);
        return true;
    }
    draw_line(graphics, x1, y1, x2, y2)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fill_triangle_fast(
    graphics: &ClassInstanceImpl,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
) -> bool {
    if let Some(cache) = graphics.native_mut::<GraphicsCache>() {
        let clip = cache.clip();
        let pixels = cache.pixels_mut();
        fill_triangle_spans(
            pixels,
            clip,
            x1 + clip.translate_x,
            y1 + clip.translate_y,
            x2 + clip.translate_x,
            y2 + clip.translate_y,
            x3 + clip.translate_x,
            y3 + clip.translate_y,
        );
        return true;
    }
    fill_triangle(graphics, x1, y1, x2, y2, x3, y3)
}

pub(super) fn set_color(graphics: &dyn ClassInstance, rgb: i32) -> bool {
    let rgb = rgb & 0x00ff_ffff;
    let color = (0xff00_0000u32 as i32) | rgb;
    let color_slot = if let Some(cache) = cache_mut(graphics) {
        cache.color = color;
        Some(cache.color_slot)
    } else {
        None
    };
    if let Some(color_slot) = color_slot {
        if let Some(color_slot) = color_slot
            && let Some(instance) = graphics.as_any().downcast_ref::<ClassInstanceImpl>()
        {
            instance.store_i32_slot(color_slot, rgb);
            return true;
        }
        return put_i32(graphics, "color", rgb);
    }
    put_i32(graphics, "color", rgb)
}

pub(super) fn set_clip(graphics: &dyn ClassInstance, x: i32, y: i32, width: i32, height: i32) -> bool {
    let (tx, ty) = translate_of(graphics);
    let clip_x = x + tx;
    let clip_y = y + ty;
    let clip_w = width.max(0);
    let clip_h = height.max(0);
    if !put_i32(graphics, "clipX", clip_x)
        || !put_i32(graphics, "clipY", clip_y)
        || !put_i32(graphics, "clipW", clip_w)
        || !put_i32(graphics, "clipH", clip_h)
    {
        return false;
    }
    if let Some(cache) = cache_mut(graphics) {
        cache.set_clip_rect(clip_x, clip_y, clip_w, clip_h);
    }
    true
}

pub(super) fn clip_rect(graphics: &dyn ClassInstance, x: i32, y: i32, width: i32, height: i32) -> bool {
    let (tx, ty) = translate_of(graphics);
    let clip_x = named_i32(graphics, "clipX");
    let clip_y = named_i32(graphics, "clipY");
    let clip_w = named_i32(graphics, "clipW");
    let clip_h = named_i32(graphics, "clipH");
    let x = x + tx;
    let y = y + ty;
    let x0 = clip_x.max(x);
    let y0 = clip_y.max(y);
    let x1 = (clip_x + clip_w).min(x + width.max(0));
    let y1 = (clip_y + clip_h).min(y + height.max(0));
    let clip_w = (x1 - x0).max(0);
    let clip_h = (y1 - y0).max(0);
    if !put_i32(graphics, "clipX", x0)
        || !put_i32(graphics, "clipY", y0)
        || !put_i32(graphics, "clipW", clip_w)
        || !put_i32(graphics, "clipH", clip_h)
    {
        return false;
    }
    if let Some(cache) = cache_mut(graphics) {
        cache.set_clip_rect(x0, y0, clip_w, clip_h);
    }
    true
}

pub(super) fn translate(graphics: &dyn ClassInstance, x: i32, y: i32) -> bool {
    let (tx, ty) = translate_of(graphics);
    if !put_i32(graphics, "translateX", tx + x) || !put_i32(graphics, "translateY", ty + y) {
        return false;
    }
    if let Some(cache) = cache_mut(graphics) {
        cache.translate_x = tx + x;
        cache.translate_y = ty + y;
    }
    true
}

fn translate_of(graphics: &dyn ClassInstance) -> (i32, i32) {
    if let Some(cache) = cache_mut(graphics) {
        return (cache.translate_x, cache.translate_y);
    }
    (named_i32(graphics, "translateX"), named_i32(graphics, "translateY"))
}

fn put_i32(instance: &dyn ClassInstance, name: &str, value: i32) -> bool {
    instance
        .as_any()
        .downcast_ref::<ClassInstanceImpl>()
        .is_some_and(|obj| obj.store_named_i32(name, value))
}

pub(super) fn fill_rect(graphics: &dyn ClassInstance, x: i32, y: i32, width: i32, height: i32) -> bool {
    if width <= 0 || height <= 0 {
        return true;
    }
    paint(graphics, |clip, pixels| {
        fill_rect_clip(clip, pixels, x, y, width, height);
    })
}

fn fill_rect_clip(clip: Clip, pixels: &mut [i32], x: i32, y: i32, width: i32, height: i32) {
    let x = x + clip.translate_x;
    let y = y + clip.translate_y;
    let (draw_x, draw_y, draw_w, draw_h, _, _) = clipped_rect(x, y, width, height, clip.x, clip.y, clip.w, clip.h);
    if draw_w <= 0 || draw_h <= 0 {
        return;
    }
    let stride = clip.width;
    let draw_w = draw_w as usize;
    for row in 0..draw_h {
        let start = ((draw_y + row) * stride + draw_x) as usize;
        if let Some(line) = pixels.get_mut(start..start + draw_w) {
            line.fill(clip.color);
        }
    }
}

pub(super) fn draw_line(graphics: &dyn ClassInstance, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) -> bool {
    paint(graphics, |clip, pixels| {
        x1 += clip.translate_x;
        y1 += clip.translate_y;
        x2 += clip.translate_x;
        y2 += clip.translate_y;
        plot_line(pixels, clip, x1, y1, x2, y2);
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn fill_triangle(
    graphics: &dyn ClassInstance,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
) -> bool {
    paint(graphics, |clip, pixels| {
        fill_triangle_spans(
            pixels,
            clip,
            x1 + clip.translate_x,
            y1 + clip.translate_y,
            x2 + clip.translate_x,
            y2 + clip.translate_y,
            x3 + clip.translate_x,
            y3 + clip.translate_y,
        );
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_rgb(
    graphics: &dyn ClassInstance,
    rgb: &dyn ClassInstance,
    offset: i32,
    scan_length: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    process_alpha: bool,
) -> bool {
    if width <= 0 || height <= 0 || scan_length <= 0 || offset < 0 {
        return true;
    }
    let Some(src) = rgb.as_array_instance().and_then(|array| array.i32_slice()) else {
        return false;
    };
    let src_offset = offset as usize;
    if src_offset >= src.len() {
        return true;
    }
    let src = &src[src_offset..];
    paint(graphics, |clip, dest| {
        let x = x + clip.translate_x;
        let y = y + clip.translate_y;
        let (draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y) = clipped_rect(x, y, width, height, clip.x, clip.y, clip.w, clip.h);
        blit_i32(dest, clip.width, src, scan_length, draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y, process_alpha);
    })
}

pub(super) fn draw_image(
    graphics: &dyn ClassInstance,
    image: &dyn ClassInstance,
    x: i32,
    y: i32,
    anchor: i32,
) -> bool {
    let width = named_i32(image, "width");
    let height = named_i32(image, "height");
    let Some(src_obj) = image_argb(image) else {
        return false;
    };
    let Some(src) = src_obj.as_array_instance().and_then(|array| array.i32_slice()) else {
        return false;
    };
    let (x, y) = anchor_xy(x, y, width, height, anchor);
    paint(graphics, |clip, dest| {
        let x = x + clip.translate_x;
        let y = y + clip.translate_y;
        let (draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y) = clipped_rect(x, y, width, height, clip.x, clip.y, clip.w, clip.h);
        blit_i32(dest, clip.width, src, width, draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y, true);
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_region(
    graphics: &dyn ClassInstance,
    image: &dyn ClassInstance,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
    transform: i32,
    dest_x: i32,
    dest_y: i32,
    anchor: i32,
) -> bool {
    if width <= 0 || height <= 0 || src_x < 0 || src_y < 0 {
        return true;
    }
    let image_width = named_i32(image, "width");
    let image_height = named_i32(image, "height");
    if src_x + width > image_width || src_y + height > image_height {
        return false;
    }
    let Some(src_obj) = image_argb(image) else {
        return false;
    };
    let Some(src) = src_obj.as_array_instance().and_then(|array| array.i32_slice()) else {
        return false;
    };
    let (draw_width, draw_height) = transformed_size(width, height, transform);
    let (dest_x, dest_y) = anchor_xy(dest_x, dest_y, draw_width, draw_height, anchor);
    if transform == 0 {
        return paint(graphics, |clip, dest| {
            let x = dest_x + clip.translate_x;
            let y = dest_y + clip.translate_y;
            let (draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y) = clipped_rect(x, y, width, height, clip.x, clip.y, clip.w, clip.h);
            blit_i32(
                dest,
                clip.width,
                src,
                image_width,
                draw_x,
                draw_y,
                draw_w,
                draw_h,
                src_x + src_off_x,
                src_y + src_off_y,
                true,
            );
        });
    }
    paint(graphics, |clip, dest| {
        let x = dest_x + clip.translate_x;
        let y = dest_y + clip.translate_y;
        let (draw_x, draw_y, draw_w, draw_h, src_off_x, src_off_y) = clipped_rect(x, y, draw_width, draw_height, clip.x, clip.y, clip.w, clip.h);
        for row in 0..draw_h {
            for col in 0..draw_w {
                let dx = src_off_x + col;
                let dy = src_off_y + row;
                let (sx, sy) = inverse_transform_point(dx, dy, width, height, transform);
                let src_index = ((src_y + sy) * image_width + src_x + sx) as usize;
                let dest_index = ((draw_y + row) * clip.width + draw_x + col) as usize;
                if let (Some(&src_pixel), Some(dst)) = (src.get(src_index), dest.get_mut(dest_index as usize)) {
                    *dst = compose(*dst, src_pixel);
                }
            }
        }
    })
}

fn paint(graphics: &dyn ClassInstance, f: impl FnOnce(Clip, &mut [i32])) -> bool {
    if let Some(cache) = cache_mut(graphics) {
        let clip = cache.clip();
        let pixels = cache.pixels_mut();
        f(clip, pixels);
        return true;
    }
    with_offscreen(graphics, f)
}

fn cache_mut(graphics: &dyn ClassInstance) -> Option<&mut GraphicsCache> {
    graphics.as_any().downcast_ref::<ClassInstanceImpl>()?.native_mut()
}

fn with_offscreen(graphics: &dyn ClassInstance, f: impl FnOnce(Clip, &mut [i32])) -> bool {
    let mut callback = Some(f);
    let mut ok = false;
    let mut need_load = false;
    graphics.with_native_scratch(&mut |slot| {
        if slot.as_ref().and_then(|value| value.downcast_ref::<GraphicsCache>()).is_none() {
            need_load = true;
            return;
        }
        paint_from_slot(slot, &mut callback, &mut ok);
    });
    if ok {
        return true;
    }
    if !need_load {
        return false;
    }
    let Some(cache) = cache_from_graphics(graphics) else {
        return false;
    };
    let mut pending = Some(cache);
    graphics.with_native_scratch(&mut |slot| {
        if let Some(cache) = pending.take() {
            *slot = Some(Box::new(cache));
        }
        paint_from_slot(slot, &mut callback, &mut ok);
    });
    ok
}

fn paint_from_slot(
    slot: &mut Option<Box<dyn core::any::Any + Send + Sync>>,
    callback: &mut Option<impl FnOnce(Clip, &mut [i32])>,
    ok: &mut bool,
) {
    let Some(cache) = slot.as_mut().and_then(|value| value.downcast_mut::<GraphicsCache>()) else {
        return;
    };
    if cache.pixel_ptr.is_null() {
        if let Some((ptr, len)) = cache
            .pixels
            .as_any()
            .downcast_ref::<ArrayClassInstanceImpl>()
            .and_then(ArrayClassInstanceImpl::i32_ptr_mut)
        {
            cache.pixel_ptr = ptr;
            cache.pixel_len = len;
        } else {
            return;
        }
    }
    let clip = cache.clip();
    let pixels = cache.pixels_mut();
    if let Some(callback) = callback.take() {
        callback(clip, pixels);
        *ok = true;
    }
}

fn cache_from_graphics(graphics: &dyn ClassInstance) -> Option<GraphicsCache> {
    let clip = clip_from_graphics(graphics)?;
    let pixels = pixels_object(graphics)?;
    let (pixel_ptr, pixel_len) = pixels
        .as_any()
        .downcast_ref::<ArrayClassInstanceImpl>()
        .and_then(ArrayClassInstanceImpl::i32_ptr_mut)?;
    Some(GraphicsCache {
        color_slot: color_slot(graphics),
        color: clip.color,
        translate_x: clip.translate_x,
        translate_y: clip.translate_y,
        x: clip.x,
        y: clip.y,
        w: clip.w,
        h: clip.h,
        width: clip.width,
        height: clip.height,
        pixels,
        pixel_ptr,
        pixel_len,
    })
}

fn clip_from_graphics(graphics: &dyn ClassInstance) -> Option<Clip> {
    let image = match graphics.get_named_field(TARGET_IMAGE, TARGET_DESC) {
        Some(JavaValue::Object(Some(image))) => image,
        _ => return None,
    };
    let width = named_i32(image.as_ref(), "width");
    let height = named_i32(image.as_ref(), "height");
    if width <= 0 || height <= 0 {
        return None;
    }
    let clip_x = named_i32(graphics, "clipX").max(0);
    let clip_y = named_i32(graphics, "clipY").max(0);
    let clip_w = named_i32(graphics, "clipW");
    let clip_h = named_i32(graphics, "clipH");
    let clip_x2 = (clip_x + clip_w).min(width);
    let clip_y2 = (clip_y + clip_h).min(height);
    Some(Clip {
        translate_x: named_i32(graphics, "translateX"),
        translate_y: named_i32(graphics, "translateY"),
        x: clip_x,
        y: clip_y,
        w: (clip_x2 - clip_x).max(0),
        h: (clip_y2 - clip_y).max(0),
        width,
        height,
        color: (0xff00_0000u32 as i32) | (named_i32(graphics, "color") & 0x00ff_ffff),
    })
}

fn pixels_object(graphics: &dyn ClassInstance) -> Option<Box<dyn ClassInstance>> {
    let image = match graphics.get_named_field(TARGET_IMAGE, TARGET_DESC) {
        Some(JavaValue::Object(Some(image))) => image,
        _ => return None,
    };
    match image.get_named_field("argb", "[I") {
        Some(JavaValue::Object(Some(pixels))) => Some(pixels),
        _ => None,
    }
}

fn image_argb(image: &dyn ClassInstance) -> Option<Box<dyn ClassInstance>> {
    match image.get_named_field("argb", "[I") {
        Some(JavaValue::Object(Some(pixels))) => Some(pixels),
        _ => None,
    }
}

fn color_slot(instance: &dyn ClassInstance) -> Option<usize> {
    let instance = instance.as_any().downcast_ref::<crate::class_instance::ClassInstanceImpl>()?;
    let class = instance.class_definition();
    let class = class.as_any().downcast_ref::<crate::class_definition::ClassDefinitionImpl>()?;
    Some(class.field_impl("color", "I", false)?.slot())
}

fn named_i32(instance: &dyn ClassInstance, name: &str) -> i32 {
    match instance.get_named_field(name, "I") {
        Some(JavaValue::Int(value)) => value,
        Some(JavaValue::Boolean(value)) => i32::from(value),
        Some(JavaValue::Byte(value)) => i32::from(value),
        Some(JavaValue::Short(value)) => i32::from(value),
        Some(JavaValue::Char(value)) => i32::from(value),
        _ => 0,
    }
}

fn clipped_rect(x: i32, y: i32, width: i32, height: i32, clip_x: i32, clip_y: i32, clip_w: i32, clip_h: i32) -> (i32, i32, i32, i32, i32, i32) {
    let draw_x = x.max(clip_x);
    let draw_y = y.max(clip_y);
    let end_x = (x + width).min(clip_x + clip_w);
    let end_y = (y + height).min(clip_y + clip_h);
    (draw_x, draw_y, end_x - draw_x, end_y - draw_y, draw_x - x, draw_y - y)
}

fn anchor_xy(mut x: i32, mut y: i32, width: i32, height: i32, anchor: i32) -> (i32, i32) {
    if anchor & 1 != 0 {
        x -= width / 2;
    } else if anchor & 8 != 0 {
        x -= width;
    }
    if anchor & 2 != 0 {
        y -= height / 2;
    } else if anchor & 32 != 0 {
        y -= height;
    }
    (x, y)
}

fn blit_i32(
    dest: &mut [i32],
    dest_width: i32,
    src: &[i32],
    src_width: i32,
    draw_x: i32,
    draw_y: i32,
    draw_w: i32,
    draw_h: i32,
    src_x: i32,
    src_y: i32,
    process_alpha: bool,
) {
    if draw_w <= 0 || draw_h <= 0 || dest_width <= 0 || src_width <= 0 {
        return;
    }
    let draw_w = draw_w as usize;
    for row in 0..draw_h {
        let src_start = ((src_y + row) * src_width + src_x) as usize;
        let dest_start = ((draw_y + row) * dest_width + draw_x) as usize;
        let Some(src_row) = src.get(src_start..src_start + draw_w) else {
            continue;
        };
        let Some(dest_row) = dest.get_mut(dest_start..dest_start + draw_w) else {
            continue;
        };
        if !process_alpha {
            for (dst, src_pixel) in dest_row.iter_mut().zip(src_row) {
                *dst = (0xff00_0000u32 as i32) | (src_pixel & 0x00ff_ffff);
            }
        } else if src_row.iter().all(|pixel| ((*pixel as u32) >> 24) == 0xff) {
            dest_row.copy_from_slice(src_row);
        } else {
            for (dst, src_pixel) in dest_row.iter_mut().zip(src_row) {
                *dst = compose(*dst, *src_pixel);
            }
        }
    }
}

fn compose(dst: i32, src: i32) -> i32 {
    let alpha = ((src as u32) >> 24) & 0xff;
    if alpha == 0xff {
        return src;
    }
    if alpha == 0 {
        return dst;
    }
    let inv = 255 - alpha;
    let src = src as u32;
    let dst = dst as u32;
    let r = (((src >> 16) & 0xff) * alpha + ((dst >> 16) & 0xff) * inv) / 255;
    let g = (((src >> 8) & 0xff) * alpha + ((dst >> 8) & 0xff) * inv) / 255;
    let b = ((src & 0xff) * alpha + (dst & 0xff) * inv) / 255;
    (0xff00_0000 | (r << 16) | (g << 8) | b) as i32
}

fn plot_line(pixels: &mut [i32], clip: Clip, mut x1: i32, mut y1: i32, x2: i32, y2: i32) {
    if y1 == y2 {
        fill_horizontal(pixels, clip, x1.min(x2), y1, (x2 - x1).abs() + 1);
        return;
    }
    if x1 == x2 {
        let y = y1.min(y2);
        for row in 0..(y2 - y1).abs() + 1 {
            put_clipped(pixels, clip, x1, y + row);
        }
        return;
    }
    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        put_clipped(pixels, clip, x1, y1);
        if x1 == x2 && y1 == y2 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn put_clipped(pixels: &mut [i32], clip: Clip, x: i32, y: i32) {
    if x >= clip.x && y >= clip.y && x < clip.x + clip.w && y < clip.y + clip.h && x >= 0 && y >= 0 && x < clip.width && y < clip.height {
        let index = (y * clip.width + x) as usize;
        if let Some(slot) = pixels.get_mut(index) {
            *slot = clip.color;
        }
    }
}

fn fill_horizontal(pixels: &mut [i32], clip: Clip, x: i32, y: i32, width: i32) {
    if y < clip.y || y >= clip.y + clip.h || y < 0 || y >= clip.height || width <= 0 {
        return;
    }
    let start = x.max(clip.x).max(0);
    let end = (x + width).min(clip.x + clip.w).min(clip.width);
    if end <= start {
        return;
    }
    let offset = (y * clip.width + start) as usize;
    let count = (end - start) as usize;
    if let Some(line) = pixels.get_mut(offset..offset + count) {
        line.fill(clip.color);
    }
}

fn fill_triangle_spans(pixels: &mut [i32], clip: Clip, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    let mut verts = [(x1, y1), (x2, y2), (x3, y3)];
    verts.sort_by_key(|vertex| vertex.1);
    let (x_top, y_top) = verts[0];
    let (x_mid, y_mid) = verts[1];
    let (x_bot, y_bot) = verts[2];
    if y_top == y_bot {
        let min_x = x_top.min(x_mid).min(x_bot);
        let max_x = x_top.max(x_mid).max(x_bot);
        fill_horizontal(pixels, clip, min_x, y_top, max_x - min_x + 1);
        return;
    }
    for y in y_top..=y_bot {
        let x_long = interpolate_x(y_top, x_top, y_bot, x_bot, y);
        let x_short = if y < y_mid || y_mid == y_bot {
            interpolate_x(y_top, x_top, y_mid, x_mid, y)
        } else {
            interpolate_x(y_mid, x_mid, y_bot, x_bot, y)
        };
        let start = x_long.min(x_short);
        let end = x_long.max(x_short);
        fill_horizontal(pixels, clip, start, y, end - start + 1);
    }
}

fn transformed_size(width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        4..=7 => (height, width),
        _ => (width, height),
    }
}

fn inverse_transform_point(tx: i32, ty: i32, width: i32, height: i32, transform: i32) -> (i32, i32) {
    match transform {
        1 => (tx, height - 1 - ty),
        2 => (width - 1 - tx, ty),
        3 => (width - 1 - tx, height - 1 - ty),
        4 => (ty, tx),
        5 => (ty, height - 1 - tx),
        6 => (width - 1 - ty, tx),
        7 => (width - 1 - ty, height - 1 - tx),
        _ => (tx, ty),
    }
}

fn interpolate_x(y0: i32, x0: i32, y1: i32, x1: i32, y: i32) -> i32 {
    if y1 == y0 {
        return x0;
    }
    x0 + ((x1 - x0) as i64 * (y - y0) as i64 / (y1 - y0) as i64) as i32
}
