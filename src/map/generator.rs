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
            map[y][x] = ((value + 1.0) * 128.0) as u8; // Normalisation des valeurs entre 0 et 255
        }
    }

    // Ajouter des obstacles sur les bordures de la carte
    for i in 0..size {
        map[0][i] = 255; // Bordure supérieure
        map[size - 1][i] = 255; // Bordure inférieure
        map[i][0] = 255; // Bordure gauche
        map[i][size - 1] = 255; // Bordure droite
    }

    // Ajouter des ressources (1: énergie, 2: minerais, 3: lieux d'intérêt scientifique)
    for i in 0..(size * size / 10) { // Exemples d'ajout de ressources, ajustez au besoin
        let x = (seed as usize + i * 17) % size;
        let y = (seed as usize + i * 23) % size;
        map[y][x] = (i % 3 + 1) as u8; // Répartir les ressources de manière simple
    }

    // Enregistrement de la carte générée en tant qu'image (optionnel)
    use std::path::Path;
    
    let path = Path::new("map.png");
    noise_map.write_to_file(&path);
        map
}
