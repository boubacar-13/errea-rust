mod map;
mod robot;
mod station;

use map::Map;
use robot::{Robot, RobotType};
use robot::explorer::Explorer;
use robot::collector::Collector;
use robot::chemical_analyzer::ChemicalAnalyzer;
use station::Station;

fn main() {
    let mut map = Map::new(100, 100, 42);
    let mut station = Station::new();

    let mut explorer_robot = Robot::new(RobotType::Explorer, Box::new(Explorer));
    let mut collector_robot = Robot::new(RobotType::Collector, Box::new(Collector));
    let mut chemical_robot = Robot::new(RobotType::ChemicalAnalyzer, Box::new(ChemicalAnalyzer));

    explorer_robot.execute(&mut map, &mut station);
    collector_robot.execute(&mut map, &mut station);
    chemical_robot.execute(&mut map, &mut station);
}
