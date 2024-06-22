use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct Explorer;

impl RobotModule for Explorer {
    fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station) {
        if let Some(_sample) = map.collect_sample(robot.x, robot.y) {
            station.receive_sample();
        }
    }
}
