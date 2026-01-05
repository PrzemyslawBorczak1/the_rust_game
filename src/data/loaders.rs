use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
pub use id_map::*;
use thiserror::Error;
pub use vec_country::*;
pub use vec_province::*;

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

pub mod vec_province {
    use super::super::VecProvince;
    use super::*;

    #[derive(Default)]
    pub struct VecProvinceLoader;

    // todo error

    impl AssetLoader for VecProvinceLoader {
        type Asset = VecProvince;
        type Settings = ();
        type Error = LoaderError;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            _settings: &(),
            _load_context: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let provinces = serde_json::from_slice(&bytes)?;
            Ok(VecProvince(provinces))
        }

        fn extensions(&self) -> &[&str] {
            &["json"]
        }
    }
}

pub mod vec_country {
    use super::super::VecCountry;
    use super::*;

    #[derive(Default)]
    pub struct VecCountryLoader;

    impl AssetLoader for VecCountryLoader {
        type Asset = VecCountry;
        type Settings = ();
        type Error = LoaderError;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            _settings: &(),
            _load_context: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let countries = serde_json::from_slice(&bytes)?;
            Ok(VecCountry(countries))
        }

        fn extensions(&self) -> &[&str] {
            &["json"]
        }
    }
}

pub mod id_map {
    use super::super::IdMap;
    use super::*;

    #[derive(Debug, Error)]
    pub enum IdMapLoaderError {
        #[error(transparent)]
        Io(#[from] std::io::Error),

        #[error(transparent)]
        Json(#[from] serde_json::Error),

        #[error(transparent)]
        Image(#[from] image::ImageError),

        #[error("val {value}; idx {max}   width: {width} height: {height}")]
        InvalidProvinceId {
            value: u32,
            max: u32,
            width: u32,
            height: u32,
        },
    }

    #[derive(Default)]
    pub struct IdMapLoader;

    impl AssetLoader for IdMapLoader {
        type Asset = IdMap;
        type Settings = ();
        type Error = IdMapLoaderError;

        async fn load(
            &self,
            reader: &mut dyn Reader,
            _settings: &(),
            _load_context: &mut LoadContext<'_>,
        ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let img = image::load_from_memory(&bytes)?;
            let rgba = img.to_rgba8();

            let width = rgba.width();
            let height = rgba.height();

            let pixels: Vec<u32> = rgba
                .pixels()
                .map(|px| {
                    let g = px[1] as u32;
                    let b = px[2] as u32;
                    (g << 8) | b
                })
                .collect();

            return Ok(IdMap {
                width,
                height,
                map: pixels,
            });
        }

        fn extensions(&self) -> &[&str] {
            &["json"]
        }
    }
}
