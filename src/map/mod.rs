pub mod generator;

use generator::generate_map;

pub struct Map {
    // Définissez les structures de la carte ici
    data: Vec<Vec<u8>>,
}

impl Map {
    pub fn new(seed: u64) -> Self {
        let data = generate_map(seed);
        Map { data }
    }

    pub fn display(&self) {
        // Implémentez l'affichage de la carte ici
        for row in &self.data {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}
