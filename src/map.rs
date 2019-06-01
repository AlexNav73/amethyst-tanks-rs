use crate::assets::map::MapAsset;
use crate::components::{Grass, Player, Wall};

pub struct Map {
    pub grid: Vec<Vec<MapCell>>,
}

impl Map {
    pub fn size(&self) -> f32 {
        self.grid.first().map(|x| x.len() as f32).unwrap_or(1.0)
    }
}

pub enum MapCell {
    Wall(Wall),
    Grass(Grass),
    Player(Player),
}

impl From<&MapAsset> for Map {
    fn from(asset: &MapAsset) -> Map {
        let grid = asset.0.iter().map(from_str).collect();
        Self { grid }
    }
}

fn from_str(row: &String) -> Vec<MapCell> {
    row.chars()
        .map(|c| match c {
            'g' => MapCell::Grass(Grass),
            'w' => MapCell::Wall(Wall),
            'p' => MapCell::Player(Player),
            _ => unreachable!(),
        })
        .collect()
}
