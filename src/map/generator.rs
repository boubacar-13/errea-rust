extern crate noise;

use noise::{Fbm, Perlin, Seedable};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};

pub fn generate_map(seed: u64, size: usize) -> Vec<Vec<u8>> {
    let fbm = Fbm::<Perlin>::new(1).set_seed(seed as u32);
    let mut map = vec![vec![0; size]; size];

    let noise_map = PlaneMapBuilder::new(&fbm)
        .set_size(size, size)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();

    for y in 0..size {
        for x in 0..size {
            let value = noise_map.get_value(x, y);
            map[y][x] = ((value + 1.0) * 128.0) as u8; // Normalisation of values between 0 and 255
        }
    }

    // Adding obstacles on map borders
    for i in 0..size {
        map[0][i] = 255; // Upper border
        map[size - 1][i] = 255; // Lower border
        map[i][0] = 255; // Left border
        map[i][size - 1] = 255; // Right border
    }

    // Adding resources (1: energy, 2: minerais, 3: scientifics places of interest)
    for i in 0..(size * size / 10) { 
        let x = (seed as usize + i * 17) % size;
        let y = (seed as usize + i * 23) % size;
        map[y][x] = (i % 3 + 1) as u8; // Spread resources on map
    }

    // Save the generated map as an image
    use std::path::Path;
    
    let path = Path::new("map.png");
    noise_map.write_to_file(&path);
        map
}
