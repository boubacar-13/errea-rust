# errea-rust

Description
Le projet "Robot Explorer" est une simulation en Rust où des robots opèrent sur une carte générée aléatoirement à la recherche d'échantillons. Chaque robot a un rôle spécifique (explorateur, collecteur, analyseur chimique) et peut interagir avec divers types de tuiles sur la carte, y compris des obstacles, des ressources énergétiques, des minéraux et des zones d'intérêt scientifique.

Fonctionnalités
Génération de Carte: Utilisation du bruit de Perlin pour générer une carte aléatoire avec divers types de tuiles.

Robots Interchangeables: Les robots sont modélisés avec des comportements interchangeables grâce à des modules dynamiques.

Interaction avec la Station: Les échantillons collectés sont envoyés à une station spatiale où ils sont analysés, stockés et potentiellement transmis à la Terre.

Installation
Clonez ce dépôt sur votre machine :
git clone https://github.com/boubacar-13/errea-rust.git

Accédez au répertoire du projet :
cd robot-explorer

Compilez et exécutez le projet avec Cargo :
cargo run

Utilisation
Une fois le projet lancé, vous verrez une fenêtre graphique affichant la carte générée et les déplacements des robots. Les robots se déplacent automatiquement selon leurs comportements définis et collectent des échantillons lorsqu'ils sont sur une tuile appropriée.

Lancer les tests unitaires :
cargo test
