use noise::{NoiseFn, Perlin};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    ScientificInterest,
    Robot,
     Station,
}

impl Tile {
    pub fn is_collectible(&self) -> bool {
        matches!(self, Tile::Energy | Tile::Mineral | Tile::ScientificInterest)
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let perlin = Perlin::new(seed);

        let mut tiles = vec![vec![Tile::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                let value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);
                if value > 0.5 {
                    tiles[y][x] = Tile::Obstacle;
                } else if value > 0.25 {
                    tiles[y][x] = Tile::Energy;
                } else if value > 0.0 {
                    tiles[y][x] = Tile::Mineral;
                } else {
                    tiles[y][x] = Tile::ScientificInterest;
                }
            }
        }

        // Ensure the borders are obstacles
        for x in 0..width {
            tiles[0][x] = Tile::Obstacle;
            tiles[height - 1][x] = Tile::Obstacle;
        }
        for y in 0..height {
            tiles[y][0] = Tile::Obstacle;
            tiles[y][width - 1] = Tile::Obstacle;
        }

        Map {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[y][x])
        } else {
            None
        }
    }
}
