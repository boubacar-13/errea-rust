use super::*;

#[test]
fn test_map_creation() {
    let map_width = 10;
    let map_height = 10;
    let seed = 42;

    let map = Map::new(map_width, map_height, seed);

    assert_eq!(map.width, map_width);
    assert_eq!(map.height, map_height);

    // Vérifie que toutes les cases de la carte sont initialisées
    for y in 0..map.height {
        for x in 0..map.width {
            assert!(map.tiles[y][x] == Tile::Empty
                    || map.tiles[y][x] == Tile::Obstacle
                    || map.tiles[y][x] == Tile::Energy
                    || map.tiles[y][x] == Tile::Mineral
                    || map.tiles[y][x] == Tile::ScientificInterest);
        }
    }
}

#[test]
fn test_map_collect_sample() {
    let mut map = Map::new(10, 10, 42);

    // Place un échantillon collectible à une position spécifique
    map.tiles[5][5] = Tile::Energy;

    // Vérifie que la collecte d'échantillon fonctionne correctement
    if let Some(collected_tile) = map.collect_sample(5, 5) {
        assert_eq!(collected_tile, Tile::Energy);
    } else {
        panic!("Failed to collect sample at (5, 5)");
    }

    // Vérifie que la case est maintenant vide après collecte
    assert_eq!(map.tiles[5][5], Tile::Empty);
}

#[test]
fn test_map_get_tile() {
    let map = Map::new(10, 10, 42);

    // Teste l'accès à une case valide
    assert_eq!(map.get_tile(0, 0), Some(Tile::Obstacle));
    assert_eq!(map.get_tile(5, 5), Some(Tile::Empty));

    // Teste l'accès à une case invalide
    assert_eq!(map.get_tile(11, 11), None);
}
