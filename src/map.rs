#[allow(dead_code)]
pub struct Map {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Cell {
    Empty,
    Obstacle,
    Resource(ResourceType),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ResourceType {
    Energy,
    Minerals,
    ScientificInterest,
}

#[allow(dead_code)]
impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::Empty; width]; height];
        Self { width, height, grid }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    // Ajoutez des méthodes pour gérer la carte ici
}