use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct Collector;

impl RobotModule for Collector {
    fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station) {
        if let Some(_resource) = map.collect_resource(robot.x, robot.y) {
            station.receive_resource();
        }
    }
}
