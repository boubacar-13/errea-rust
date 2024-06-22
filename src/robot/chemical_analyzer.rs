use crate::map::Map;
use crate::station::Station;
use crate::robot::{Robot, RobotModule};

pub struct ChemicalAnalyzer;

impl ChemicalAnalyzer {
    pub fn new() -> Self {
        ChemicalAnalyzer
    }
}

impl RobotModule for ChemicalAnalyzer {
    fn act(&mut self, robot: &mut Robot, _map: &mut Map, station: &mut Station) {
        // Implémentez le comportement d'analyse chimique ici
        println!("Analyzing chemicals in the map...");
        // Logique d'analyse chimique
        if let Some(chemical_sample) = map.collect_chemical_sample(robot.x, robot.y) {
            robot.has_sample = true;
        }
        if robot.has_sample {
            station.receive_chemical_sample();
            robot.has_sample = false;
        }
    }
}
