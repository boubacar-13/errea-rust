mod map;
mod robot;
mod station;

use map::Map;
use robot::{Robot, RobotType};
use station::Station;

fn main() {
    let mut map = Map::new(100, 100); // Crée une carte 100x100
    let mut station = Station::new();

    // Ajout de robots à la station
    station.add_robot(Robot::new(RobotType::Explorer));
    station.add_robot(Robot::new(RobotType::Miner));
    station.add_robot(Robot::new(RobotType::Scientist));

    // Début de la simulation
    for _ in 0..100 {
        station.update(&mut map);
    }

    println!("Simulation terminée.");
}