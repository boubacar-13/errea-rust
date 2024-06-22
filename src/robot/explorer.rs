use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct Explorer;

impl RobotModule for Explorer {

fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station) {
        // Logique de mouvement simple
        if robot.x < map.width - 1 {
            robot.move_to(robot.x + 1, robot.y, map);
        } else if robot.y < map.height - 1 {
            robot.move_to(0, robot.y + 1, map);
        }

        // Collecte de l'échantillon
        if let Some(_sample) = map.collect_sample(robot.x, robot.y) {
            station.receive_sample();
        }
}
}