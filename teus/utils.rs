#[allow(dead_code)]
pub trait SysUtils {
    // Default implementation to convert bytes to gigabytes
    fn to_gb(&self, bytes: u64) -> f64 {
        bytes as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

#[cfg(test)]
mod tests {
    use super::SysUtils;

    struct TestStruct;
    impl SysUtils for TestStruct {}

    #[test]
    fn test_to_gb_conversion() {
        let test_struct = TestStruct;
        
        // Test zero bytes
        assert_eq!(test_struct.to_gb(0), 0.0);
        
        // Test 1 GB (1024^3 bytes)
        let one_gb_bytes = 1024_u64.pow(3);
        assert_eq!(test_struct.to_gb(one_gb_bytes), 1.0);
        
        // Test 2 GB
        let two_gb_bytes = 2 * 1024_u64.pow(3);
        assert_eq!(test_struct.to_gb(two_gb_bytes), 2.0);
        
        // Test fractional GB (512 MB = 0.5 GB)
        let half_gb_bytes = 512 * 1024 * 1024;
        assert_eq!(test_struct.to_gb(half_gb_bytes), 0.5);
        
        // Test precision with a known value
        let bytes = 1536 * 1024 * 1024; // 1.5 GB
        let result = test_struct.to_gb(bytes);
        assert!((result - 1.5).abs() < f64::EPSILON);
    }
}
