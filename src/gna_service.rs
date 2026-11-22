use nanai_gna_dll_load::{
    GnaDevice, GnaLibrary, GnaLoadTestConfig, GnaLoadTester, inspect_system_gna_state,
    publish_self_gna_allocated_bytes,
};
use std::sync::{
    Arc, LazyLock, Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GnaHardwareStatus {
    is_detected: bool,
    is_loaded: bool,
    device_name: String,
    description: String,
    completed_inferences: usize,
    avg_latency_ms: f64,
    throughput_ips: f64,
    hw_usage_percentage: f64,
    active_process_count: usize,
    has_external_processes: bool,
    is_load_mode: bool,
    last_test_status: String,
}

impl GnaHardwareStatus {
    pub fn is_detected(&self) -> bool {
        self.is_detected
    }

    pub fn is_loaded(&self) -> bool {
        self.is_loaded
    }

    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn completed_inferences(&self) -> usize {
        self.completed_inferences
    }

    pub fn avg_latency_ms(&self) -> f64 {
        self.avg_latency_ms
    }

    pub fn throughput_ips(&self) -> f64 {
        self.throughput_ips
    }

    pub fn hw_usage_percentage(&self) -> f64 {
        self.hw_usage_percentage
    }

    pub fn active_process_count(&self) -> usize {
        self.active_process_count
    }

    pub fn has_external_processes(&self) -> bool {
        self.has_external_processes
    }

    pub fn is_load_mode(&self) -> bool {
        self.is_load_mode
    }

    pub fn last_test_status(&self) -> &str {
        &self.last_test_status
    }
}

struct GnaSessionState {
    library: Option<GnaLibrary>,
    device: Option<GnaDevice>,
    completed_inferences: usize,
    avg_latency_ms: f64,
    throughput_ips: f64,
    hw_usage_percentage: f64,
    last_test_status: String,
}

static LOAD_MODE_ACTIVE: AtomicBool = AtomicBool::new(false);

static GNA_SESSION: LazyLock<Arc<Mutex<GnaSessionState>>> = LazyLock::new(|| {
    let mut lib_opt = None;
    let mut dev_opt = None;
    let mut status_msg = "Not tested yet".to_string();

    if let Ok(lib) = GnaLibrary::load_default() {
        if let Ok(dev) = GnaDevice::open_first(&lib) {
            dev_opt = Some(dev);
        }
        lib_opt = Some(lib);
    } else {
        status_msg = "GNA Library not found".to_string();
    }

    Arc::new(Mutex::new(GnaSessionState {
        library: lib_opt,
        device: dev_opt,
        completed_inferences: 0,
        avg_latency_ms: 0.0,
        throughput_ips: 0.0,
        hw_usage_percentage: 0.0,
        last_test_status: status_msg,
    }))
});

pub fn is_load_mode_active() -> bool {
    LOAD_MODE_ACTIVE.load(Ordering::Relaxed)
}

pub fn get_gna_status() -> GnaHardwareStatus {
    let sys_util = inspect_system_gna_state(None).ok();
    let active_process_count = sys_util
        .as_ref()
        .map(|u| u.active_gna_processes().len())
        .unwrap_or(0);
    let has_external_processes = sys_util
        .as_ref()
        .map(|u| u.has_external_processes())
        .unwrap_or(false);

    let state = GNA_SESSION.lock().unwrap();
    let is_detected = state.device.is_some();
    let is_loaded = state.library.is_some();
    let is_load_mode = is_load_mode_active();

    let description = if is_detected && is_loaded {
        if is_load_mode {
            "Intel GNA 3.0 NPU: Load Mode Active".to_string()
        } else {
            "Intel GNA 3.0 NPU: Active & Session Held".to_string()
        }
    } else if is_detected {
        "Intel GNA Device Detected (Session Open)".to_string()
    } else if is_loaded {
        "GNA Runtime Library Loaded (Device not found)".to_string()
    } else {
        "GNA Hardware / Driver not found".to_string()
    };

    let device_name = if is_detected {
        "Intel Gaussian & Neural Accelerator 3.0 (Device ID: 0x474E)".to_string()
    } else {
        "Intel GNA Device (not found)".to_string()
    };

    GnaHardwareStatus {
        is_detected,
        is_loaded,
        device_name,
        description,
        completed_inferences: state.completed_inferences,
        avg_latency_ms: state.avg_latency_ms,
        throughput_ips: state.throughput_ips,
        hw_usage_percentage: state.hw_usage_percentage,
        active_process_count,
        has_external_processes,
        is_load_mode,
        last_test_status: state.last_test_status.clone(),
    }
}

fn execute_gna_workload(
    device: &GnaDevice,
    config: GnaLoadTestConfig,
) -> Result<(usize, f64, f64, f64), String> {
    let report = GnaLoadTester::run_default_workload(device, config)
        .map_err(|e| format!("Load test failed: {e}"))?;
    let _ = publish_self_gna_allocated_bytes();

    let completed = report.completed_inferences();
    let ips = report.inferences_per_second();
    let avg_latency = report.latency_avg().as_secs_f64() * 1000.0;
    let hw_usage = report
        .hw_duty_cycle_percentage()
        .unwrap_or_else(|| report.usage_monitor().cumulative_hw_usage_percentage());

    Ok((completed, avg_latency, ips, hw_usage))
}

pub fn run_gna_benchmark() {
    let mut state = GNA_SESSION.lock().unwrap();
    if let Some(dev) = &state.device {
        let config = GnaLoadTestConfig::new()
            .with_iterations(200)
            .with_concurrency(1)
            .with_track_hw_usage(true);

        match execute_gna_workload(dev, config) {
            Ok((completed, avg_latency, ips, hw_usage)) => {
                state.completed_inferences += completed;
                state.avg_latency_ms = avg_latency;
                state.throughput_ips = ips;
                state.hw_usage_percentage = hw_usage;
                state.last_test_status = format!(
                    "{completed} inferences ({ips:.1} IPS, avg {avg_latency:.3}ms, HW {hw_usage:.1}%)"
                );
            }
            Err(err) => {
                state.last_test_status = err;
            }
        }
    } else {
        state.last_test_status = "Cannot run: GNA device not found".to_string();
    }
}

pub fn toggle_gna_load_mode() {
    let was_active = LOAD_MODE_ACTIVE.fetch_xor(true, Ordering::SeqCst);
    if was_active {
        let mut state = GNA_SESSION.lock().unwrap();
        state.last_test_status = "Load mode stopped".to_string();
        return;
    }

    std::thread::spawn(|| {
        let Ok(lib) = GnaLibrary::load_default() else {
            LOAD_MODE_ACTIVE.store(false, Ordering::SeqCst);
            let mut state = GNA_SESSION.lock().unwrap();
            state.last_test_status = "Load mode failed: GNA Library not found".to_string();
            return;
        };
        let Ok(dev) = GnaDevice::open_first(&lib) else {
            LOAD_MODE_ACTIVE.store(false, Ordering::SeqCst);
            let mut state = GNA_SESSION.lock().unwrap();
            state.last_test_status = "Load mode failed: GNA Device not found".to_string();
            return;
        };

        while LOAD_MODE_ACTIVE.load(Ordering::Relaxed) {
            let config = GnaLoadTestConfig::new()
                .with_duration(Duration::from_millis(500))
                .with_optional_iterations(None)
                .with_concurrency(4)
                .with_track_hw_usage(true);

            match execute_gna_workload(&dev, config) {
                Ok((completed, avg_latency, ips, hw_usage)) => {
                    let mut state = GNA_SESSION.lock().unwrap();
                    state.completed_inferences += completed;
                    state.avg_latency_ms = avg_latency;
                    state.throughput_ips = ips;
                    state.hw_usage_percentage = hw_usage;
                    state.last_test_status = format!(
                        "Load Mode: {completed} reqs/500ms ({ips:.1} IPS, {avg_latency:.3}ms, HW {hw_usage:.1}%)"
                    );
                }
                Err(err) => {
                    let mut state = GNA_SESSION.lock().unwrap();
                    state.last_test_status = format!("Load Mode error: {err}");
                    LOAD_MODE_ACTIVE.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    });
}
