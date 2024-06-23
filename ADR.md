# Architecture Decision Record (ADR) pour le Projet Robot Explorer

## Contexte

Le projet "Robot Explorer" est une simulation en Rust où des robots naviguent et collectent des échantillons sur une carte générée aléatoirement. Chaque robot a un type spécifique (explorateur, collecteur, analyseur chimique) et interagit avec une carte constituée de tuiles variées telles que des obstacles, des ressources énergétiques, des minéraux et des zones d'intérêt scientifique. La simulation se déroule dans une station spatiale où les échantillons collectés sont analysés et traités.

## Décision d'Architecture

# Structure du Projet

Le projet est structuré en plusieurs modules principaux :

map: Contient la logique de génération de carte (generator.rs) et la définition des types de tuiles (Tile).
robot: Inclut les différents types de robots (explorer.rs, collector.rs, chemical_analyzer.rs) et une interface générique (RobotModule) pour définir les comportements des robots.
station: Gère la station spatiale où les échantillons sont reçus, analysés et traités. Comprend également la communication avec la Terre (communication.rs).
main.rs: Point d'entrée de l'application, où la fenêtre graphique est configurée et où la simulation est gérée.
Robot et Modules
Les robots sont définis comme des structures avec des coordonnées (x et y), un type spécifique (RobotType) et un module trait (RobotModule) qui définit leurs actions. Les modules sont dynamiquement interchangeables via des boîtes (Box<dyn RobotModule>), permettant une flexibilité dans les comportements des robots.

## Carte et Génération Aléatoire

La carte est générée aléatoirement à l'aide du bruit de Perlin pour créer des tuiles variées (Empty, Obstacle, Energy, Mineral, ScientificInterest). Les bordures de la carte sont automatiquement définies comme des obstacles pour encadrer l'espace exploré.

## Interaction avec la Station

La station spatiale reçoit les échantillons collectés par les robots et les traite en fonction du type d'échantillon. Elle peut analyser des échantillons chimiques, recevoir des ressources et transmettre des données vers la Terre.

## Justification

Modularité et Extensibilité: L'utilisation de modules pour les robots permet d'ajouter facilement de nouveaux comportements de robot sans modifier la logique principale du robot ou de la station.

Flexibilité dans la Génération de Carte: L'utilisation du bruit de Perlin pour la génération de carte permet de créer des environnements variés et réalistes.

Séparation des Responsabilités: Chaque module (carte, robots, station) a des responsabilités clairement définies, ce qui facilite la maintenance et l'évolutivité du projet.

## Conséquences

Complexité de la Génération de Carte: La génération de carte basée sur le bruit de Perlin peut être complexe à ajuster pour obtenir des résultats optimaux en termes de jouabilité et de réalisme.

Gestion Dynamique des Modules: L'utilisation de Box<dyn RobotModule> ajoute une surcharge légère en termes de performance due à l'allocation dynamique de mémoire, bien que cela soit généralement négligeable pour ce type d'application.
