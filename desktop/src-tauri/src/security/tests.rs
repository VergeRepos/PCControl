#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pairing_code() {
        let code = crate::security::generate_pairing_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_numeric()));
    }

    #[test]
    fn test_verify_pairing_code() {
        let code = "123456";
        assert!(crate::security::verify_pairing_code(code, Some(code)));
        assert!(!crate::security::verify_pairing_code("000000", Some(code)));
        assert!(!crate::security::verify_pairing_code(code, None));
    }

    #[test]
    fn test_device_permissions_default() {
        let perms = crate::database::DevicePermissions::default();
        assert!(perms.system_monitoring);
        assert!(!perms.power_controls);
        assert!(!perms.process_control);
        assert!(!perms.remote_input);
    }

    #[test]
    fn test_check_permission() {
        use crate::database::{Device, DevicePermissions};

        let mut device = Device {
            id: "test".to_string(),
            name: "Test".to_string(),
            device_type: "mobile".to_string(),
            paired_at: 0,
            last_seen: None,
            secret: "secret".to_string(),
            permissions: DevicePermissions::default(),
        };

        // Should have system_monitoring by default
        assert!(crate::security::check_permission(&device, "system_monitoring").is_ok());

        // Should not have power_controls by default
        assert!(crate::security::check_permission(&device, "power_controls").is_err());

        // Enable power_controls
        device.permissions.power_controls = true;
        assert!(crate::security::check_permission(&device, "power_controls").is_ok());
    }
}
