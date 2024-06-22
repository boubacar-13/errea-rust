pub mod explorer;
pub mod collector;
pub mod chemical_analyzer;

use explorer::Explorer;
use collector::Collector;
use chemical_analyzer::ChemicalAnalyzer;

pub enum RobotType {
    Explorer,
    Collector,
    ChemicalAnalyzer,
}
#[warn(dead_code)]
pub struct Robot {
    kind: RobotType,
    module: Box<dyn RobotModule>,
    x: usize,
    y: usize,
    has_sample: bool,
}

impl Robot {
    pub fn new(kind: RobotType, x: usize, y: usize) -> Self {
        let module: Box<dyn RobotModule> = match kind {
            RobotType::Explorer => Box::new(Explorer::new()),
            RobotType::Collector => Box::new(Collector::new()),
            RobotType::ChemicalAnalyzer => Box::new(ChemicalAnalyzer::new()),
        };
        Robot { kind, module, x, y, has_sample: false }
    }

    pub fn act(&mut self, map: &mut Map, station: &mut Station) {
        self.module.act(self, map, station);
    }
}

pub trait RobotModule {
    fn act(&mut self, robot: &mut Robot, map: &mut Map, station: &mut Station);
}
