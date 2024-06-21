extern crate noise;

use noise::{Fbm, Perlin, Seedable};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

pub fn generate_map(seed: u64) -> Vec<Vec<u8>> {
    let fbm = Fbm::<Perlin>::new(1).set_seed(seed as u32);
    let size = 100; // Taille de la carte 100x100
    let mut map = vec![vec![0; size]; size];

    let noise_map = PlaneMapBuilder::new(&fbm)
        .set_size(size, size)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();

    for y in 0..size {
        for x in 0..size {
            let value = noise_map.get_value(x, y);
            map[y][x] = ((value + 1.0) * 128.0) as u8; // Normalisation des valeurs entre 0 et 255
        }
    }

    // Enregistrement de la carte générée en tant qu'image (optionnel)
    use std::path::Path;
    
    let path = Path::new("map.png");
    noise_map.write_to_file(&path);

    map
}
