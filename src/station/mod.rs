pub mod communication;

use communication::Communication;
use std::collections::HashMap;

#[warn(dead_code)]
pub struct Station {
    comm: Communication,
    scientific_data: HashMap<String, String>,
    resources: HashMap<String, u32>,
}

impl Station {
    pub fn new() -> Self {
        Station {
            comm: Communication::new(),
            scientific_data: HashMap::new(),
            resources: HashMap::new(),
        }
    }
    pub fn receive_sample(&mut self) {
        self.samples += 1;
        println!("Sample received at the station.");
    }

    pub fn receive_resource(&mut self) {
        self.resources += 1;
        println!("Resource received at the station.");
    }

    pub fn process_data(&mut self) {
        // Traitez les données reçues des robots
        println!("Processing data from robots...");
        // Utiliser le champ comm
        let data = self.comm.receive_data();
        for (key, value) in data {
            self.scientific_data.insert(key.clone(), value.clone());
        }
    }

     pub fn create_robot(&self) -> bool {
          if self.resources > 10 {
               println!("Creating a new robot.");
               true
          } else {
               println!("Not enough resources to create a new robot.");
               false
          }
    }

    pub fn report(&self) {
        // Générer le rapport des découvertes
        println!("Reporting findings to Earth...");
        for (key, value) in &self.scientific_data {
            println!("{}: {}", key, value);
        }
    }

    // Ajoutez cette méthode pour permettre la transmission de données
    pub fn transmit_data(&self) {
        self.comm.transmit_data();
    }
}
