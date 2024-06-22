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
        // Implement the exploration behavior here
        println!("Exploring the map for scientific interests...");
        // Add the exploration logic here
        station.transmit_data();
    }
}
