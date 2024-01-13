use std::net;

use anyhow::{bail, Result};
use mecha_network_ctl::{
    errors::{WirelessNetworkError, WirelessNetworkErrorCodes},
    wireless_network::WirelessNetworkControl,
};

use tracing::{debug, error, info};
use wifi_ctrl::sta::ScanResult;

pub struct NetworkService {}

impl NetworkService {
    pub async fn get_network_list() -> Result<Vec<ScanResult>> {
        let task = "get_network_List";
        let network_path = "/var/run/wpa_supplicant/wlp2s0".to_string();
        println!("inside get_network_list");
        let network = WirelessNetworkControl { path: network_path };

        let network_list = match network.scan_wireless_network().await {
            Ok(v) => v,
            Err(e) => {
                bail!(WirelessNetworkError::new(
                    WirelessNetworkErrorCodes::NoWirelessNetworkFound,
                    format!("error while getting network list {}", e),
                ));
            }
        };

        Ok(network_list)
    }

    // pub async fn set_network_status(status: bool) -> Result<()> {
    //     let task = "set_network_status";

    //     let network_path = "/sys/class/net/wlan0".to_string();

    //     let network = Network {
    //         path: format!("{}/uevent", network_path),
    //     };

    //     match network.set_network_status(status) {
    //         Ok(v) => v,
    //         Err(e) => {
    //             bail!(NetworkError::new(
    //                 NetworkErrorCodes::SetNetworkStatusError,
    //                 format!("error while setting network status {}", e),
    //             ));
    //         }
    //     };

    //     Ok(())
    // }
}
