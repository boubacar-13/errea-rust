mod robot;
mod map;
mod station;

use map::Map;
use robot::{Robot, RobotType};
use station::Station;

fn main() {
    // Initialize the seed for the map generation
    let seed = 42; // Can be dynamically generated
    let size = 100; // Size of the map
    let mut map = Map::new(seed, size);

    // Create station
    let mut station = Station::new();

    // Create robots
    let mut robots = vec![
        Robot::new(RobotType::Explorer),
        Robot::new(RobotType::Collector),
        Robot::new(RobotType::ChemicalAnalyzer),
    ];

    // Main simulation
    for _ in 0..100 {
        for robot in &mut robots {
            robot.act(&mut map, &mut station);
        }
        station.process_data();
    }

    // Display results
    map.display();
    station.report();
}
