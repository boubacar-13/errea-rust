use super::RobotModule;
use crate::map::Map;
use crate::station::Station;

pub struct Explorer;

impl Explorer {
    pub fn new() -> Self {
        Explorer
    }
}

impl RobotModule for Explorer {
    fn act(&mut self, _map: &mut Map, station: &mut Station) {
        // Implémentez le comportement d'exploration ici
        println!("Exploring the map for scientific interests...");
        // Ajoutez la logique d'exploration ici
        station.transmit_data();
    }
}
