use serde::Serialize;
use clap::Args;

#[derive(Args)]
pub struct GetBatteryInfoArguments {
    #[arg(default_value_t = 1)]
    pub poll_interval: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BatteryInfo {
    time_to_full: Option<String>,
    time_to_empty: String,
    current_battery_health: f32,
}

pub fn get_battery_info(args: GetBatteryInfoArguments) -> Result<(), String> {
    !unimplemented!();
}