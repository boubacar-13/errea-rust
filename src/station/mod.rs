pub mod communication;

use communication::Communication;

pub struct Station {
    comm: Communication,
}

impl Station {
    pub fn new() -> Self {
        Station {
            comm: Communication::new(),
        }
    }

    pub fn process_data(&mut self) {
        // Traitez les données reçues des robots
        println!("Processing data from robots...");
        // Utiliser le champ comm
        self.comm.transmit_data();
    }

    pub fn report(&self) {
        // Générer le rapport des découvertes
        println!("Reporting findings to Earth...");
    }
}
