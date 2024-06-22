pub mod explorer;
pub mod collector;
pub mod chemical_analyzer;

use crate::map::Map;
use crate::station::Station;

pub trait RobotModule {
    fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station);
}

pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub kind: RobotType,
    pub module: Box<dyn RobotModule>,
}

impl Robot {
    pub fn new(kind: RobotType, module: Box<dyn RobotModule>) -> Self {
        Robot {
            x: 0,
            y: 0,
            kind,
            module,
        }
    }

    pub fn execute(&mut self, map: &mut Map, station: &mut Station) {
        let x = self.x;
        let y = self.y;
        let kind = &self.kind;
        let mut module = self.module.as_mut();
        module.act(self, map, station);
    }
}

pub enum RobotType {
    Explorer,
    Collector,
    ChemicalAnalyzer,
}
