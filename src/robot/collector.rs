use super::RobotModule;
use crate::map::Map;
use crate::station::Station;

pub struct Collector;

impl Collector {
    pub fn new() -> Self {
        Collector
    }
}

impl RobotModule for Collector {
    fn act(&mut self, _map: &mut Map, station: &mut Station) {
        // Implement the collection behavior here
        println!("Collecting resources...");
        // Add the collection logic here
        station.transmit_data();
    }
}
