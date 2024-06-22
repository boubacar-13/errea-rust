use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct Collector;

impl Collector {
    pub fn new() -> Self {
        Collector
    }
}

impl RobotModule for Collector {
    fn act(&mut self, robot: &mut Robot, _map: &mut Map, station: &mut Station) {
        // Implement the collection behavior here
        println!("Collecting resources from the map...");
        // Add the collection logic here
        if let Some(resource) = map.collect_resource(robot.x, robot.y) {
            robot.has_sample = true;
        }
        if robot.has_sample {
            station.receive_resource();
            robot.has_sample = false;
        }
    }
}
