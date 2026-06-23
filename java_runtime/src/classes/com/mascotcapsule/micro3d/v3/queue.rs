use alloc::{boxed::Box, vec, vec::Vec};
use core::hash::{Hash, Hasher};

use jvm::{ClassInstance, ClassInstanceRef};
use parking_lot::Mutex;

use super::{Graphics3D, raster::PreparedFigure};

static GRAPHICS_3D_RENDER_QUEUES: Mutex<Vec<(u64, Vec<PreparedFigure>)>> = Mutex::new(Vec::new());

const FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
const FNV_PRIME_64: u64 = 0x100000001b3;

#[derive(Default)]
struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = if self.0 == 0 { FNV_OFFSET_BASIS_64 } else { self.0 };
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME_64);
        }
        self.0 = hash;
    }
}

pub(super) fn graphics_3d_key(this: &ClassInstanceRef<Graphics3D>) -> u64 {
    if this.is_null() {
        return 0;
    }

    let rust_this: Box<dyn ClassInstance> = this.clone().into();
    let mut hasher = IdentityHasher::default();
    rust_this.hash(&mut hasher);
    hasher.finish()
}

pub(super) fn push_graphics_3d_queue(key: u64, prepared: PreparedFigure) {
    let mut queues = GRAPHICS_3D_RENDER_QUEUES.lock();
    if let Some((_, queue)) = queues.iter_mut().find(|(entry_key, _)| *entry_key == key) {
        queue.push(prepared);
        return;
    }
    queues.push((key, vec![prepared]));
}

pub(super) fn take_graphics_3d_queue(key: u64) -> Vec<PreparedFigure> {
    let mut queues = GRAPHICS_3D_RENDER_QUEUES.lock();
    if let Some(index) = queues.iter().position(|(entry_key, _)| *entry_key == key) {
        let (_, queue) = queues.swap_remove(index);
        return queue;
    }
    Vec::new()
}
