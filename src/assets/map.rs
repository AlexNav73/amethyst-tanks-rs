use amethyst::{
    assets::{self, Asset, Handle, ProcessingState, ResultExt, SimpleFormat},
    ecs::VecStorage,
};
use ron::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct MapAsset(pub Vec<String>);

pub type MapHandle = Handle<MapAsset>;

impl Asset for MapAsset {
    const NAME: &'static str = "amethyst_tanks_rs::MapAsset";

    type Data = Self;
    type HandleStorage = VecStorage<MapHandle>;
}

impl From<MapAsset> for assets::Result<ProcessingState<MapAsset>> {
    fn from(value: MapAsset) -> assets::Result<ProcessingState<MapAsset>> {
        Ok(ProcessingState::Loaded(value))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct MapFormat;

impl<A> SimpleFormat<A> for MapFormat
where
    A: Asset,
    A::Data: for<'a> Deserialize<'a> + Send + Sync + 'static,
{
    const NAME: &'static str = "MapAsset";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<A::Data, assets::Error> {
        let mut deserializer =
            Deserializer::from_bytes(&bytes).chain_err(|| "Failed deserialize map file")?;

        let val =
            A::Data::deserialize(&mut deserializer).chain_err(|| "Failed parsing map file")?;

        deserializer.end().chain_err(|| "Failed parsing map file")?;

        Ok(val)
    }
}

#[derive(Debug)]
pub struct MapSource;

impl assets::Source for MapSource {
    fn modified(&self, _path: &str) -> assets::Result<u64> {
        Ok(0)
    }

    fn load(&self, path: &str) -> assets::Result<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};

        let file = File::open(path).chain_err(|| "Failed open map file")?;
        let mut reader = BufReader::new(file);

        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .chain_err(|| "Failed read map file")?;

        Ok(bytes)
    }
}
