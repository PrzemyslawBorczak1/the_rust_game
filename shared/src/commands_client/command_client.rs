use serde::{Deserialize, Serialize};

use super::basic::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum CommandClient {
    ChangeProvince(Vec<ChangeProvince>),
    ChangeCountry(Vec<ChangeCountry>),
    Init(Init),
}

impl CommandClient {
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(command: &String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(command)
    }

    pub fn serialize_vec(vec_changes: &Vec<Self>) -> Result<String, serde_json::Error> {
        serde_json::to_string(vec_changes)
    }
}
