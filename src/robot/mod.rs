pub mod explorer;
pub mod collector;
pub mod chemical_analyzer;

use explorer::Explorer;
use collector::Collector;
use chemical_analyzer::ChemicalAnalyzer;
use crate::map::Map;
use crate::station::Station;

pub trait RobotModule {
    fn act(&mut self, map: &mut Map, station: &mut Station);
}

pub enum RobotType {
    Explorer,
    Collector,
    ChemicalAnalyzer,
}

#[warn(dead_code)]
pub struct Robot {
    kind: RobotType,
    module: Box<dyn RobotModule>,
}

impl Robot {
    pub fn new(kind: RobotType) -> Self {
        let module: Box<dyn RobotModule> = match kind {
            RobotType::Explorer => Box::new(Explorer::new()),
            RobotType::Collector => Box::new(Collector::new()),
            RobotType::ChemicalAnalyzer => Box::new(ChemicalAnalyzer::new()),
        };

        Robot { kind, module }
    }

    pub fn act(&mut self, map: &mut Map, station: &mut Station) {
        self.module.act(map, station);
    }
}
