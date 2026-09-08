use core::sync::atomic::{AtomicBool, Ordering};

use jvm::{Array, ClassInstanceRef};
use parking_lot::Mutex;

pub type InvalidateGpuImage = fn(&ClassInstanceRef<Array<i32>>);
pub type PublishGpuImageToScreen = fn(
    pixels: &ClassInstanceRef<Array<i32>>,
    screen_width: i32,
    screen_height: i32,
    dest_x: i32,
    dest_y: i32,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
    clip: (i32, i32, i32, i32),
) -> bool;

static INVALIDATE: Mutex<Option<InvalidateGpuImage>> = Mutex::new(None);
static PUBLISH: Mutex<Option<PublishGpuImageToScreen>> = Mutex::new(None);
static HAS_GPU_IMAGE_HOOKS: AtomicBool = AtomicBool::new(false);

pub fn register_gpu_image_hooks(invalidate: InvalidateGpuImage, publish: PublishGpuImageToScreen) {
    *INVALIDATE.lock() = Some(invalidate);
    *PUBLISH.lock() = Some(publish);
    HAS_GPU_IMAGE_HOOKS.store(true, Ordering::Release);
}

pub fn invalidate_gpu_image(pixels: &ClassInstanceRef<Array<i32>>) {
    if !HAS_GPU_IMAGE_HOOKS.load(Ordering::Relaxed) {
        return;
    }
    if let Some(hook) = *INVALIDATE.lock() {
        hook(pixels);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn publish_gpu_image_to_screen(
    pixels: &ClassInstanceRef<Array<i32>>,
    screen_width: i32,
    screen_height: i32,
    dest_x: i32,
    dest_y: i32,
    src_x: i32,
    src_y: i32,
    width: i32,
    height: i32,
    clip: (i32, i32, i32, i32),
) -> bool {
    if !HAS_GPU_IMAGE_HOOKS.load(Ordering::Relaxed) {
        return false;
    }
    let Some(hook) = *PUBLISH.lock() else {
        return false;
    };
    hook(pixels, screen_width, screen_height, dest_x, dest_y, src_x, src_y, width, height, clip)
}
