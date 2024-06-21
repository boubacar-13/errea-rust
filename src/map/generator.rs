extern crate noise;

use noise::{NoiseFn, Perlin, Seedable};

pub fn generate_map(seed: u64) -> Vec<Vec<u8>> {
    let perlin = Perlin::new().set_seed(seed as u32);
    let mut map = vec![vec![0; 10]; 10]; // Carte 10x10 pour l'exemple

    for y in 0..10 {
        for x in 0..10 {
            let value = perlin.get([x as f64 / 10.0, y as f64 / 10.0]);
            map[y][x] = (value * 10.0) as u8;
        }
    }

    map
}
