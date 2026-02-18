use sdl2::event::Event; // Importation du module pour gérer les événements SDL2
use sdl2::keyboard::Keycode; // Importation des codes des touches du clavier SDL2
use std::time::{Duration, Instant}; // Importation des modules pour gérer le temps
use rand::Rng; // Importation du module pour générer des nombres aléatoires

use crate::traffic; // Importation du module local traffic
use traffic::*; // Importation de tous les éléments du module traffic

/// Fonction pour gérer les événements du programme.
/// Cette fonction gère les entrées de l'utilisateur et met à jour l'état du programme en conséquence.
pub fn handle_events(
    event_pump: &mut sdl2::EventPump, // Permet de récupérer les événements de SDL2
    smart_road: &mut SmartRoad, // Référence mutable vers l'objet SmartRoad pour pouvoir le modifier
    is_stats: &mut bool, // Référence mutable pour indiquer si les statistiques sont affichées
    is_stop: &mut bool, // Référence mutable pour indiquer si le programme doit s'arrêter
    stats: &mut Stats, // Référence mutable vers l'objet Stats pour pouvoir le mettre à jour
    width: u32, // Largeur de l'écran ou de la route
    height: u32, // Hauteur de l'écran ou de la route
    last_spawn_time: &mut Instant, // Référence mutable vers l'instant du dernier spawn d'un véhicule
    spawn_cooldown: &mut Duration, // Référence mutable vers la durée minimum entre deux spawns de véhicules
) {
    // Boucle sur tous les événements dans la file d'attente d'événements SDL2
    for event in event_pump.poll_iter() {
        match event {
            // Si l'utilisateur ferme la fenêtre ou appuie sur Échap
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                if *is_stats { // Si les statistiques sont affichées, on arrête le programme
                    *is_stop = true;
                } else { // Sinon, on affiche les statistiques
                    *is_stats = true;
                    *stats = smart_road.stats(); // On met à jour les statistiques
                }
            }
            // Si l'utilisateur appuie sur la touche "Up" (flèche vers le haut)
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                if last_spawn_time.elapsed() >= *spawn_cooldown { // Vérifie si le cooldown de spawn est terminé
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rand::thread_rng().gen(), // Génère un ID aléatoire pour le véhicule
                        Direction::North, // Direction du véhicule : Nord
                    ));
                    *last_spawn_time = Instant::now(); // Met à jour le temps du dernier spawn
                }
            }
            // Si l'utilisateur appuie sur la touche "Down" (flèche vers le bas)
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                if last_spawn_time.elapsed() >= *spawn_cooldown { // Vérifie si le cooldown de spawn est terminé
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rand::thread_rng().gen(), // Génère un ID aléatoire pour le véhicule
                        Direction::South, // Direction du véhicule : Sud
                    ));
                    *last_spawn_time = Instant::now(); // Met à jour le temps du dernier spawn
                }
            }
            // Si l'utilisateur appuie sur la touche "Right" (flèche vers la droite)
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                if last_spawn_time.elapsed() >= *spawn_cooldown { // Vérifie si le cooldown de spawn est terminé
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rand::thread_rng().gen(), // Génère un ID aléatoire pour le véhicule
                        Direction::East, // Direction du véhicule : Est
                    ));
                    *last_spawn_time = Instant::now(); // Met à jour le temps du dernier spawn
                }
            }
            // Si l'utilisateur appuie sur la touche "Left" (flèche vers la gauche)
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                if last_spawn_time.elapsed() >= *spawn_cooldown { // Vérifie si le cooldown de spawn est terminé
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rand::thread_rng().gen(), // Génère un ID aléatoire pour le véhicule
                        Direction::West, // Direction du véhicule : Ouest
                    ));
                    *last_spawn_time = Instant::now(); // Met à jour le temps du dernier spawn
                }
            }
            // Si l'utilisateur appuie sur la touche "R"
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                if last_spawn_time.elapsed() >= *spawn_cooldown { // Vérifie si le cooldown de spawn est terminé
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rand::thread_rng().gen(), // Génère un ID aléatoire pour le véhicule
                        rand::thread_rng().gen(), // Génère une direction aléatoire pour le véhicule
                    ));
                    *last_spawn_time = Instant::now(); // Met à jour le temps du dernier spawn
                }
            }
            _ => {} // Ignore tous les autres événements
        }
    }
}
