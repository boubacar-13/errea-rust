use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct Explorer;

impl Explorer {
    pub fn new() -> Self {
        Explorer
    }
}

impl RobotModule for Explorer {
    fn act(&mut self, robot: &mut Robot, _map: &mut Map, station: &mut Station) {
        // Implement the exploration behavior here
        println!("Exploring the map for scientific interests...");
        // Add the exploration logic here
        if let Some(sample) = map.collect_sample(robot.x, robot.y) {
            robot.has_sample = true;
        }
        if robot.has_sample {
            station.receive_sample();
            robot.has_sample = false;
        }
    }
}
