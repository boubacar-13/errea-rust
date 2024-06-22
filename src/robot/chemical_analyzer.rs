use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct ChemicalAnalyzer;

impl RobotModule for ChemicalAnalyzer {
    fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station) {
        if let Some(chemical_sample) = map.collect_chemical_sample(robot.x, robot.y) {
            station.analyze_chemical_sample(chemical_sample);
        }
    }
}
