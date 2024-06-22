pub mod generator;

pub use generator::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    ScientificInterest,
}

impl Tile {
    pub fn is_collectible(&self) -> bool {
        matches!(self, Tile::Energy | Tile::Mineral | Tile::ScientificInterest)
    }
}

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

    pub fn display(&self) {
        for row in &self.tiles {
            for &tile in row {
                let symbol = match tile {
                    generator::Tile::Empty => ' ',
                    generator::Tile::Obstacle => '#',
                    generator::Tile::Energy => 'E',
                    generator::Tile::Mineral => 'M',
                    generator::Tile::ScientificInterest => 'S',
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
}
