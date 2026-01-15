pub mod id_map;
pub use id_map::*;

pub mod vec_country;
pub use vec_country::*;

pub mod vec_province;
pub use vec_province::*;

pub mod resources;
pub use resources::*;

use bevy::{ecs::system::In, prelude::error};

fn loading_error(In(result): In<anyhow::Result<()>>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Loading error occurred: {:#}", e);
        }
    }
}
