use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
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
