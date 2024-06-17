use crate::map::Map;
use crate::robot::Robot;

pub struct Station {
    robots: Vec<Robot>,
}

impl Station {
    pub fn new() -> Self {
        Self { robots: Vec::new() }
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    pub fn update(&mut self, map: &mut Map) {
        for robot in &mut self.robots {
            let (x, y) = robot.get_position();
            let new_x = (x + 1) % map.get_width();
            let new_y = (y + 1) % map.get_height();
            robot.set_position(new_x, new_y);
        }
    }
}