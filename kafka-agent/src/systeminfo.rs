use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

lazy_static! {
    static ref SYSTEM_INFO: SystemInformation = SystemInformation::new();
    static ref N_A: &'static str = "N/A";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInformation {
    host_name: String,
    distribution_id: String,
    name: String,
    kernel_version: String,
    os_version: String,
}

impl SystemInformation {
    fn new() -> Self {
        let mut sys = System::new_all();

        sys.refresh_all();

        SystemInformation {
            host_name: sys.host_name().unwrap_or(N_A.to_owned()),
            distribution_id: sys.distribution_id(),
            name: sys.name().unwrap_or(N_A.to_owned()),
            kernel_version: sys.kernel_version().unwrap_or(N_A.to_owned()),
            os_version: sys.os_version().unwrap_or(N_A.to_owned()),
        }
    }

    pub fn get_host_name(&self) -> String {
        self.host_name.clone()
    }
}

pub fn get_system_info() -> SystemInformation {
    SYSTEM_INFO.clone()
}
