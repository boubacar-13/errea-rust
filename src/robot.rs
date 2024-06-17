pub struct Robot {
    robot_type: RobotType,
    position: (usize, usize),
}

pub enum RobotType {
    Explorer,
    Miner,
    Scientist,
}

impl Robot {
    pub fn new(robot_type: RobotType) -> Self {
        Self {
            robot_type,
            position: (0, 0), // Position initiale
        }
    }
    // Getter for position
    pub fn get_position(&self) -> (usize, usize) {
        self.position
    }

    // Setter for position
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.position = (x, y);
    }
    // Ajoutez des méthodes pour gérer les robots ici
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.position = (x, y);
    }
}