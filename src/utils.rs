#[allow(dead_code)]
pub trait SysUtils {
    // Default implementation to convert bytes to gigabytes
    fn to_gb(&self, bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }
}
