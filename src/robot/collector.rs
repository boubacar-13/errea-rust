use super::RobotModule;
use crate::map::Map;
use crate::station::Station;

pub struct Collector;

impl Collector {
    pub fn new() -> Self {
        Collector
    }
}

#[warn(unused_variables)]
impl RobotModule for Collector {
    fn act(&mut self, _map: &mut Map, _station: &mut Station) {
        // Implémentez le comportement de collecte ici
        println!("Collecting resources...");
        // Ajoutez la logique de collecte ici
    }
}
