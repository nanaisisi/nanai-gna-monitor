use nanai_gna_dll_load::{GnaDevice, GnaLibrary};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GnaHardwareStatus {
    pub is_detected: bool,
    pub is_loaded: bool,
    pub device_name: String,
    pub description: String,
}

pub fn detect_gna_status() -> GnaHardwareStatus {
    // 1. GnaLibrary によるロード試行
    let library_res = GnaLibrary::load_default();
    let is_loaded = library_res.is_ok();

    // 2. GnaDevice によるオープン試行
    let is_detected = if let Ok(ref lib) = library_res {
        GnaDevice::open_first(lib).is_ok()
    } else {
        false
    };

    let description = if is_detected && is_loaded {
        "Intel GNA 3.0 NPU: Active & Ready".to_string()
    } else if is_detected {
        "Intel GNA Device Detected (DLL not active)".to_string()
    } else if is_loaded {
        "GNA Runtime Library Loaded (Emulated/Standby)".to_string()
    } else {
        "GNA Hardware / Driver Standby (No active device)".to_string()
    };

    GnaHardwareStatus {
        is_detected,
        is_loaded,
        device_name: "Intel Gaussian & Neural Accelerator (GNA 3.0)".to_string(),
        description,
    }
}
