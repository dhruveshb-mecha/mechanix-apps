use crate::{
    modules::battery::{
        self,
        errors::{BatteryServiceError, BatteryServiceErrorCodes},
    },
  
};

use anyhow::{bail, Result};
use mecha_battery_ctl::{Battery, PowerSupplyInfo};
use tracing::{debug, error, info};

pub struct BatteryService {}

impl BatteryService {
    pub async fn get_battery_status() -> Result<u8> {
        let task = "get_battery_status";
        let battery_path = "/sys/class/power_supply/bq27441-0".to_string();
        let battery = Battery {
            path: format!("{}/uevent", battery_path),
            currnet_now: format!("{}/current_now", battery_path),
        };
        let battery_info = match battery.info() {
            Ok(v) => v,
            Err(e) => {
                bail!(BatteryServiceError::new(
                    BatteryServiceErrorCodes::GetBatteryInfoError,
                    format!("error while getting battery info {}", e),
                    true
                ));
            }
        };

        info!(task, "battery info is {:?}", battery_info);

        debug!(task, "battery info is {:?}", battery_info);

        let battery_capacity = battery_info.capacity;

        Ok(battery_capacity)
        
    }
}
