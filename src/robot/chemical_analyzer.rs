use crate::map::Map;
use crate::station::Station;
use crate::robot::RobotModule;

pub struct ChemicalAnalyzer;

impl ChemicalAnalyzer {
    pub fn new() -> Self {
        ChemicalAnalyzer
    }
}

impl RobotModule for ChemicalAnalyzer {
    fn act(&mut self, map: &mut Map, station: &mut Station) {
        // Implémentez le comportement d'analyse chimique ici
        println!("Analyzing the chemical composition of samples...");
        // Ajoutez la logique d'analyse chimique ici
        station.transmit_data();
    }
}
