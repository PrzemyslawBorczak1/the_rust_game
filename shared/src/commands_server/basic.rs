use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Attack {
    pub from_province: u32,
    pub to_province: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChooseCountry(pub u32);

#[derive(Serialize, Deserialize, Debug)]
// province id
pub struct BuyBank(pub u32);
#[derive(Serialize, Deserialize, Debug)]
// province id
pub struct BuyArmy(pub u32);

#[derive(Serialize, Deserialize, Debug)]
// country id
pub struct MakePeace(pub u32);
#[derive(Serialize, Deserialize, Debug)]
// province id
pub struct StartWar(pub u32);
