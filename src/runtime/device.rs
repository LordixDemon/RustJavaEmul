use std::io::Write;

use java_runtime::Runtime;

use super::RuntimeImpl;

#[async_trait::async_trait]
impl<T> Runtime for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    fn platform_request(&self, url: &str) -> bool {
        tracing::info!("MIDlet.platformRequest({url})");
        true
    }

    fn set_lights(&self, num: i32, level: i32) {
        tracing::debug!("DeviceControl.setLights({num}, {level})");
    }

    fn start_vibra(&self, freq: i32, duration_ms: i64) {
        tracing::debug!("DeviceControl.startVibra({freq}, {duration_ms})");
    }

    fn stop_vibra(&self) {
        tracing::debug!("DeviceControl.stopVibra()");
    }

    fn play_tone(&self, note: i32, duration_ms: i32, volume: i32) {
        tracing::debug!("playTone({note}, {duration_ms}, {volume})");
    }

    fn device_profile(&self) -> java_runtime::DeviceProfile {
        RuntimeImpl::device_profile(self)
    }
}
