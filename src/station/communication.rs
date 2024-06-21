use std::collections::HashMap;

pub struct Communication;

impl Communication {
    pub fn new() -> Self {
        Communication
    }

    pub fn transmit_data(&self) {
        // Implémentez la transmission des données ici
        println!("Transmitting data to the station...");
        // Ajoutez la logique de transmission de données ici
    }

    pub fn receive_data(&self) -> HashMap<String, String> {
        // Simuler la réception de données
        let mut data = HashMap::new();
        data.insert("sample_key".to_string(), "sample_value".to_string());
        data
    }
}
