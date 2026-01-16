use serde::{Deserialize, Serialize};
use serde_json;

use super::basic::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandServer {
    // UpgradeProvince(UpgradeProvince),
    // BuyArmy(BuyArmy),
    Attack(Attack),
    ChooseCountry(ChooseCountry),
}

impl CommandServer {
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(command: &String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(command)
    }
}
