use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpgradeProvince {}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyArmy {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub from_province: u32,
    pub to_province: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChooseCountry(pub u32);
