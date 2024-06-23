// Fichier : robot/tests.rs

use super::*;
use crate::station::Station;

#[test]
fn test_robot_move_to_valid() {
    let mut map = Map::new(10, 10, 42);
    let mut robot = Robot::new(RobotType::Explorer, Box::new(Explorer));

    robot.move_to(1, 1, &map);

    assert_eq!(robot.x, 1);
    assert_eq!(robot.y, 1);
}

#[test]
fn test_robot_move_to_invalid_obstacle() {
    let mut map = Map::new(10, 10, 42);
    let mut robot = Robot::new(RobotType::Explorer, Box::new(Explorer));

    // Place un obstacle à la position (1, 1)
    map.tiles[1][1] = Tile::Obstacle;

    // Essaye de déplacer le robot sur l'obstacle
    robot.move_to(1, 1, &map);

    // Vérifie que le robot n'a pas bougé
    assert_eq!(robot.x, 0);
    assert_eq!(robot.y, 0);
}

#[test]
fn test_robot_execute_explorer() {
    let mut map = Map::new(10, 10, 42);
    let mut station = Station::new();
    let mut robot = Robot::new(RobotType::Explorer, Box::new(Explorer));

    // Déplace le robot à droite
    robot.execute(&mut map, &mut station);

    assert_eq!(robot.x, 1);
    assert_eq!(robot.y, 0);

    // Collecte un échantillon à la nouvelle position
    if let Some(_sample) = map.collect_sample(robot.x, robot.y) {
        station.receive_sample();
    }

    // Vérifie que la station a reçu un échantillon
    assert_eq!(station.samples, 1);
}
