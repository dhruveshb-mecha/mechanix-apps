use anyhow::{bail, Result};
use mecha_display_ctl::{DisplayControl, DisplayError, DisplayErrorCodes};
use tracing::{debug, error, info};

pub struct DisplayService {}

impl DisplayService {
    pub async fn get_display_brightness() -> Result<u8> {
        let task = "get_display_brightness";
        let display_path = "/sys/class/backlight/backlight".to_string();
        let display = DisplayControl {
            path: format!("{}/brightness", display_path),
        };
        let display_brightness = match display.get_display_brightness() {
            Ok(v) => v,
            Err(e) => {
                bail!(DisplayError::new(
                    DisplayErrorCodes::InvalidBrightnessPathError,
                    format!("error while getting display info {}", e),
                ));
            }
        };

        info!(task, "display brightness is {:?}", display_brightness);

        debug!(task, "display brightness is {:?}", display_brightness);

        Ok(display_brightness)
    }

    pub  fn set_display_brightness(brightness: u8) -> Result<()> {
        let task = "set_display_brightness";

        let display_path = "/sys/class/backlight/backlight".to_string();

        let display = DisplayControl {
            path: format!("{}/brightness", display_path),
        };

        match display.set_display_brightness(brightness) {
            Ok(v) => v,
            Err(e) => {
                bail!(DisplayError::new(
                    DisplayErrorCodes::InvalidBrightnessValueError,
                    format!("error while setting display brightness {}", e),
                ));
            }
        };

        Ok(())
    }
}
