use anyhow::{bail, Result};
use mecha_device_oem_info::DistroInfo;

use tracing::{debug, error, info};

pub struct DeviceOEMInfo {}

impl DeviceOEMInfo {
    pub fn get_device_oem_info_service() -> Result<DistroInfo> {
        let distro_info = DistroInfo::get_distro_info()?;
        info!("Distro info: {:?}", distro_info);
        Ok(distro_info)
    }
}
