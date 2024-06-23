// Fichier : station/tests.rs

use super::*;

#[test]
fn test_station_receive_resource() {
    let mut station = Station::new();

    station.receive_resource();
    assert_eq!(station.resources, 1);
}

#[test]
fn test_station_analyze_chemical_sample() {
    let mut station = Station::new();

    station.analyze_chemical_sample(Tile::Energy);
    assert_eq!(station.chemical_samples, 1);
}

#[test]
fn test_station_create_robot_if_needed() {
    let mut station = Station::new();

    station.resources = 5;
    station.create_robot_if_needed();
    assert_eq!(station.resources, 5); // Vérifie qu'aucun robot n'est créé avec moins de 10 ressources

    station.resources = 10;
    station.create_robot_if_needed();
    assert_eq!(station.resources, 0); // Vérifie que les ressources sont correctement déduites
}