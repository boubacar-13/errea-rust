pub mod communication;

use communication::Communication;

pub struct Station {
    resources: usize,
    samples: usize,
    chemical_samples: usize,
    comm: Communication,
}

impl Station {
    pub fn new() -> Self {
        Station {
            resources: 0,
            samples: 0,
            chemical_samples: 0,
            comm: Communication::new(),
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

    pub fn analyze_chemical_sample(&mut self, _chemical_sample: crate::map::Tile) {
        self.chemical_samples += 1;
        println!("Chemical sample analyzed at the station.");
    }

    pub fn process_data(&mut self) {
        println!("Processing data at the station...");
        self.comm.transmit_data();
    }

     pub fn resolve_conflicts(&mut self) {
        println!("Resolving conflicts at the station...");
     }

    pub fn create_robot_if_needed(&mut self) {
        if self.resources >= 10 {
            self.resources -= 10;
            println!("Creating a new robot.");
        } else {
            println!("Not enough resources to create a new robot.");
        }
    }

    pub fn report(&self) {
        println!("Resources: {}, Samples: {}, Chemical Samples: {}", self.resources, self.samples, self.chemical_samples);
    }
}
