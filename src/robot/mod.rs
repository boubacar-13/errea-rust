pub mod explorer;
pub mod collector;

use explorer::Explorer;
use collector::Collector;
use crate::map::Map;
use crate::station::Station;

pub enum RobotType {
    Explorer,
    Collector,
}

#[allow(dead_code)] // Ajoutez cette ligne pour désactiver l'avertissement pour le champ kind
pub struct Robot {
    kind: RobotType,
    module: Box<dyn RobotModule>,
}

impl Robot {
    pub fn new(kind: RobotType) -> Self {
        let module: Box<dyn RobotModule> = match kind {
            RobotType::Explorer => Box::new(Explorer::new()),
            RobotType::Collector => Box::new(Collector::new()),
        };
        Robot { kind, module }
    }

    pub fn act(&mut self, map: &mut Map, station: &mut Station) {
        self.module.act(map, station);
    }
}

pub trait RobotModule {
    fn act(&mut self, map: &mut Map, station: &mut Station);
}
