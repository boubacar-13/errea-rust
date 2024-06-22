pub mod generator;

pub use generator::{Map, Tile};

impl Map {
    pub fn collect_sample(&mut self, x: usize, y: usize) -> Option<Tile> {
        if let Some(tile) = self.get_tile(x, y) {
            if tile.is_collectible() {
                self.tiles[y][x] = Tile::Empty;
                return Some(tile);
            }
        }
        None
    }

    pub fn collect_resource(&mut self, x: usize, y: usize) -> Option<Tile> {
        self.collect_sample(x, y)
    }

    pub fn collect_chemical_sample(&mut self, x: usize, y: usize) -> Option<Tile> {
        self.collect_sample(x, y)
    }
}
