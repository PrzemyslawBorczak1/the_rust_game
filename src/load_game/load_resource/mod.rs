pub mod id_map;
pub use id_map::*;

pub mod textures;
pub use textures::*;

pub mod vec_country;
pub use vec_country::*;

pub mod vec_province;
pub use vec_province::*;

pub mod resources;
pub use resources::*;

use bevy::prelude::error;
use bevy_ecs::system::In;
fn loading_error(In(result): In<anyhow::Result<()>>) {
    match result {
        Ok(_) => {}
        Err(e) => {
            error!("Loading error occurred: {:#}", e);
        }
    }
}
