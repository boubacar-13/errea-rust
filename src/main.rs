mod map;
mod robot;
mod station;

use map::Map;
use robot::{Robot, RobotType};
use station::Station;

fn main() {
        // Initialize the seed for the map generation
    let seed = 42; // Can be dynamically generated
    let size = 100; // Size of the map
    let mut map = Map::new(size, size, seed); // Exemples de dimensions et de seed
     
    // Create station
     let mut station = Station::new();

     // Create robots
     let mut robots = vec![
        Robot::new(RobotType::Explorer, 0, 0),
        Robot::new(RobotType::Collector, 1, 1),
        Robot::new(RobotType::ChemicalAnalyzer, 2, 2),
    ];

    for _ in 0..100 { // Simulation de 100 étapes
        for robot in &mut robots {
            robot.act(&mut map, &mut station);
        }
        station.process_data();
        if station.create_robot() {
            robots.push(Robot::new(RobotType::Explorer, 0, 0)); // Ajout d'un new robot
        }
        station.report();
    }
}
