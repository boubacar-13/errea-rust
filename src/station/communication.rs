pub struct Communication;

impl Communication {
    pub fn new() -> Self {
        Communication
    }

    pub fn transmit_data(&self) {
        println!("Transmitting data to Earth...");
    }
}
