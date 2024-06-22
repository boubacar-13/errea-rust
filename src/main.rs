mod robot;
mod map;
mod station;

use map::Map;
use robot::{Robot, RobotType};
use station::Station;

fn main() {
    // Initialisation de la seed pour la génération de la carte
    let seed = 42; // Vous pouvez rendre cela dynamique
    let size = 100; // Taille de la carte, configurable
    let mut map = Map::new(seed, size);

    // Création de la station
    let mut station = Station::new();

    // Création des robots
    let mut robots = vec![
        Robot::new(RobotType::Explorer),
        Robot::new(RobotType::Collector),
        // Ajoutez plus de robots selon les besoins
    ];

    // Simulation principale
    for _ in 0..100 {
        for robot in &mut robots {
            robot.act(&mut map, &mut station);
        }
        station.process_data();
    }

    // Affichage des résultats
    map.display();
    station.report();
}
