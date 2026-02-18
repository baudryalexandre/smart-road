use std::collections::HashMap;
use sdl2::render::{Texture, WindowCanvas};
use super::{Intersection, Vehicle};

// Structure représentant une route intelligente avec un carrefour.
pub struct SmartRoad {
    pub intersection: Intersection, // Instance de Intersection pour gérer les véhicules et les mouvements.
    total_cars: u32, // Nombre total de véhicules ajoutés à la route.
    average_velocity: f32, // Vitesse moyenne des véhicules.
}

impl SmartRoad {
    // Crée une nouvelle instance de SmartRoad.
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(), // Initialise l'intersection.
            total_cars: 0, // Initialise le compteur de véhicules.
            average_velocity: 0.0, // Initialise la vitesse moyenne.
        }
    }

    // Ajoute un véhicule à l'intersection et met à jour le compteur de véhicules.
    pub fn add_vehicle(&mut self, v: Vehicle) {
        self.intersection.add_vehicle(v); // Ajoute le véhicule à l'intersection.
        self.total_cars += 1; // Incrémente le compteur de véhicules.
    }

    // Régule les véhicules dans l'intersection, met à jour les statistiques et rend les véhicules sur le canvas.
    pub fn regulate(&mut self, canvas: &mut WindowCanvas, texture: HashMap<String, &Texture>) {
        self.intersection.waiting(); // Réactive les véhicules en attente.
        self.intersection.remove_out_of_bounds_vehicles(); // Supprime les véhicules qui sont hors des limites de l'écran.
        
        // Met à jour la vitesse moyenne en fonction de la vitesse moyenne calculée pour l'intersection.
        if self.intersection.average_velocity() != 0.0 {
            self.average_velocity =
                (self.intersection.average_velocity() as f32 + self.average_velocity) / 2.0;
        }

        // Régule les véhicules, rend les véhicules sur le canvas et supprime l'état de mouvement actuel.
        self.intersection.regulate(canvas, texture);
        self.intersection.moves.drop_state(); // Supprime le premier état de mouvement.
    }

    // Retourne les statistiques actuelles de la route intelligente.
    pub fn stats(&self) -> Stats {
        // Calcule la vitesse minimale en multipliant par 10 si elle n'est pas la valeur maximale possible.
        let mut min_v = 0;
        if self.intersection.min_velocity != u32::MAX {
            min_v = self.intersection.min_velocity * 10;
        }
        
        // Calcule le temps minimal en le divisant par 10 si il n'est pas la valeur maximale possible.
        let mut min_t = 0.0;
        if self.intersection.min_time != u32::MAX {
            min_t = self.intersection.min_time as f32 / 10.0;
        }
        
        // Crée une instance de Stats avec les valeurs calculées.
        Stats {
            total_cars: self.total_cars - self.intersection.waiting_room.len() as u32, // Nombre total de véhicules en circulation.
            nbr_passed: self.intersection.nbr_passed_vehicles, // Nombre de véhicules ayant quitté l'intersection.
            max_velocity: self.intersection.max_velocity * 10, // Vitesse maximale des véhicules.
            min_velocity: min_v, // Vitesse minimale des véhicules.
            average_velocity: self.average_velocity * 10.0, // Vitesse moyenne des véhicules.
            max_time: self.intersection.max_time as f32 / 10.0, // Temps maximal pour les véhicules à traverser l'intersection.
            min_time: min_t, // Temps minimal pour les véhicules à traverser l'intersection.
        }
    }
}

// Structure représentant les statistiques d'une route intelligente.
#[derive(Clone, Debug, Copy)]
pub struct Stats {
    pub total_cars: u32, // Nombre total de véhicules ajoutés à la route.
    pub nbr_passed: u32, // Nombre de véhicules ayant quitté l'intersection.
    pub max_velocity: u32, // Vitesse maximale des véhicules.
    pub min_velocity: u32, // Vitesse minimale des véhicules.
    pub average_velocity: f32, // Vitesse moyenne des véhicules.
    pub max_time: f32, // Temps maximal pour traverser l'intersection.
    pub min_time: f32, // Temps minimal pour traverser l'intersection.
}

impl Stats {
    // Crée une nouvelle instance de Stats avec des valeurs initiales par défaut.
    pub fn new() -> Self {
        Stats {
            total_cars: 0, // Initialise le nombre total de véhicules à 0.
            nbr_passed: 0, // Initialise le nombre de véhicules passés à 0.
            max_velocity: 0, // Initialise la vitesse maximale à 0.
            min_velocity: 0, // Initialise la vitesse minimale à 0.
            average_velocity: 0.0, // Initialise la vitesse moyenne à 0.0.
            max_time: 0.0, // Initialise le temps maximal à 0.0.
            min_time: 0.0, // Initialise le temps minimal à 0.0.
        }
    }
}
