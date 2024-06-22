pub mod generator;

use generator::generate_map;

pub struct Map {
    data: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(seed: u64, size: usize) -> Self {
        let data = generate_map(seed, size);
        Map { data }
    }

    pub fn display(&self) {
        // Display the map
        for row in &self.data {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    // Fonction pour consommer une ressource sur la carte
    pub fn consume_resource(&mut self, x: usize, y: usize) -> Option<u8> {
        if x < self.data.len() && y < self.data[0].len() {
            let resource = self.data[y][x];
            if resource > 0 && resource < 4 {
                self.data[y][x] = 0;
                return Some(resource);
            }
        }
        None
    }
}
