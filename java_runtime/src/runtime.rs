mod io;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::time::Duration;

use dyn_clone::{DynClone, clone_trait_object};

use jvm::{ClassDefinition, ClassInstance, Jvm, Result as JvmResult};

use crate::DeviceProfile;

pub use io::{File, FileDescriptorId, FileSize, FileStat, FileType, IOError, IOResult};

#[async_trait::async_trait]
pub trait SpawnCallback: Sync + Send {
    async fn call(&self) -> JvmResult<()>;

    fn current_java_thread(&self) -> Option<Box<dyn ClassInstance>> {
        None
    }
}

#[async_trait::async_trait]
pub trait RuntimeClock: Sync + Send {
    async fn sleep(&self, duration: Duration);
    async fn r#yield(&self);
    fn spawn(&self, jvm: &Jvm, callback: Box<dyn SpawnCallback>);

    fn now(&self) -> u64; // unix time in millis
    fn current_task_id(&self) -> u64;
    fn current_java_thread(&self) -> Option<Box<dyn ClassInstance>> {
        None
    }
    fn set_current_java_thread(&self, _thread: Option<Box<dyn ClassInstance>>) {}
}

#[async_trait::async_trait]
pub trait RuntimeFs: Sync + Send {
    fn stdin(&self) -> IOResult<FileDescriptorId>;
    fn stdout(&self) -> IOResult<FileDescriptorId>;
    fn stderr(&self) -> IOResult<FileDescriptorId>;

    async fn open(&self, path: &str, write: bool) -> IOResult<FileDescriptorId>;
    fn get_file(&self, fd: FileDescriptorId) -> IOResult<Box<dyn File>>;
    fn close_file(&self, fd: FileDescriptorId);
    async fn unlink(&self, path: &str) -> IOResult<()>;
    async fn metadata(&self, path: &str) -> IOResult<FileStat>;

    fn list_directory(&self, _path: &str) -> Vec<String> {
        Vec::new()
    }
}

#[async_trait::async_trait]
pub trait RuntimeScreen: Sync + Send {
    fn decode_image(&self, _data: &[u8]) -> Option<DecodedImage> {
        None
    }

    fn screen_width(&self) -> i32 {
        240
    }

    fn screen_height(&self) -> i32 {
        320
    }

    fn screen_draw_pixels(&self, _x: i32, _y: i32, _width: i32, _height: i32, _pixels: &[i32], _process_alpha: bool) {}

    #[allow(clippy::too_many_arguments)]
    fn screen_draw_pixels_strided(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
        process_alpha: bool,
    ) {
        let mut clipped = Vec::with_capacity((width * height) as usize);
        for row in 0..height {
            let start = ((source_y + row) * source_width + source_x) as usize;
            clipped.extend_from_slice(&pixels[start..start + width as usize]);
        }

        self.screen_draw_pixels(x, y, width, height, &clipped, process_alpha);
    }

    #[allow(clippy::too_many_arguments)]
    fn screen_replace_pixels_strided(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        pixels: &[i32],
        source_width: i32,
        source_x: i32,
        source_y: i32,
    ) {
        self.screen_draw_pixels_strided(x, y, width, height, pixels, source_width, source_x, source_y, false);
    }

    fn screen_fill_rect(&self, _x: i32, _y: i32, _width: i32, _height: i32, _argb: i32) {}

    fn screen_present(&self) {}

    fn set_current_displayable(&self, _displayable: Option<Box<dyn ClassInstance>>) {}

    fn is_current_displayable(&self, _displayable: &dyn ClassInstance) -> bool {
        true
    }

    fn game_key_states(&self) -> i32 {
        0
    }

    fn target_frame_rate(&self) -> u32 {
        30
    }
}

#[async_trait::async_trait]
pub trait RuntimeStore: Sync + Send {
    fn rms_open_record_store(&self, _name: &str, _create_if_necessary: bool) -> bool {
        true
    }

    fn rms_num_records(&self, _name: &str) -> i32 {
        0
    }

    fn rms_next_record_id(&self, name: &str) -> i32 {
        self.rms_num_records(name) + 1
    }

    fn rms_add_record(&self, _name: &str, _data: &[i8]) -> i32 {
        0
    }

    fn rms_delete_record(&self, _name: &str, _record_id: i32) {}

    fn rms_get_record(&self, _name: &str, _record_id: i32) -> Option<Vec<i8>> {
        None
    }

    fn rms_set_record(&self, _name: &str, _record_id: i32, _data: &[i8]) {}

    fn rms_list_record_stores(&self) -> Vec<String> {
        Vec::new()
    }

    fn rms_get_size(&self, name: &str) -> i32 {
        self.rms_num_records(name) * 64
    }

    fn rms_get_size_available(&self, _name: &str) -> i32 {
        64 * 1024
    }
}

#[async_trait::async_trait]
pub trait RuntimeNet: Sync + Send {
    async fn http_request(
        &self,
        _method: &str,
        _url: &str,
        _headers: &[(String, String)],
        _body: &[u8],
    ) -> IOResult<(i32, Vec<(String, String)>, Vec<u8>)> {
        Err(IOError::Unsupported)
    }
}

#[async_trait::async_trait]
pub trait RuntimeClassDefine: Sync + Send {
    async fn find_rustjar_class(&self, jvm: &Jvm, classpath: &str, class: &str) -> JvmResult<Option<Box<dyn ClassDefinition>>>;
    async fn define_class(&self, jvm: &Jvm, data: &[u8]) -> JvmResult<Box<dyn ClassDefinition>>;
    async fn define_array_class(&self, jvm: &Jvm, element_type_name: &str) -> JvmResult<Box<dyn ClassDefinition>>;

    fn allows_runtime_class(&self, _name: &str) -> bool {
        true
    }
}

#[async_trait::async_trait]
pub trait Runtime: RuntimeClock + RuntimeFs + RuntimeScreen + RuntimeStore + RuntimeNet + RuntimeClassDefine + DynClone {
    fn platform_request(&self, _url: &str) -> bool {
        false
    }

    fn set_lights(&self, _num: i32, _level: i32) {}

    fn start_vibra(&self, _freq: i32, _duration_ms: i64) {}

    fn stop_vibra(&self) {}

    fn play_tone(&self, _note: i32, _duration_ms: i32, _volume: i32) {}

    fn device_profile(&self) -> DeviceProfile {
        DeviceProfile::Generic
    }
}

clone_trait_object!(Runtime);

#[derive(Clone)]
pub struct DecodedImage {
    pub width: i32,
    pub height: i32,
    pub argb: Vec<i32>,
}
