use serde::{Deserialize, Serialize};

use crate::resources::GameWorld;

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeProvince {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeCountry {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Init {
    pub world: GameWorld,
}
