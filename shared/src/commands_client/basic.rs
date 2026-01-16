use serde::{Deserialize, Serialize};

use crate::resources::{GameWorld, Province};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeProvince {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeCountry {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Init {
    pub world: GameWorld,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProvince {
    pub id: u32,
    pub province: Province,
}
