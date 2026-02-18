// Modules importés pour organiser les fonctionnalités du projet
mod event;
mod layout;
mod menu;
mod traffic;
mod texture;

use sdl2::image::LoadTexture;  // Pour charger des textures avec SDL2
use sdl2::pixels::Color;      // Pour les couleurs dans SDL2
use std::time::{Duration, Instant}; // Pour gérer les durées et les instants
use event::*;                 // Importation des fonctionnalités de gestion des événements
use layout::*;                // Importation des fonctionnalités de mise en page
use menu::*;                  // Importation des fonctionnalités de menu
use traffic::*;               // Importation des fonctionnalités de gestion du trafic
use texture::*;               // Importation des fonctionnalités de gestion des textures

// Constantes pour gérer les FPS et la durée de chaque frame
const FPS: u32 = 120;
const FRAME_DURATION: Duration = Duration::from_millis(6000 / FPS as u64); // Calcul de la durée de chaque frame en millisecondes

fn main() {
    // Initialisation du contexte SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap(); // Sous-système vidéo pour créer des fenêtres

    // Création de la fenêtre SDL
    let window = video_subsystem
        .window("smart-road", 800, 800) // Titre et dimensions de la fenêtre
        .position_centered()            // Centrage de la fenêtre à l'écran
        .build()                        // Construction de la fenêtre
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap(); // Création du canvas pour dessiner sur la fenêtre

    canvas.set_draw_color(Color::WHITE); // Définir la couleur de dessin du canvas
    canvas.clear(); // Effacer le canvas avec la couleur définie
    canvas.present(); // Présenter le contenu du canvas à l'écran
    let mut event_pump = sdl_context.event_pump().unwrap(); // Création de l'event pump pour gérer les événements

    // Initialisation du contexte SDL2_ttf pour la gestion des polices de caractères
    let ttf_context = sdl2::ttf::init().unwrap();

    // Affiche le menu principal et récupère l'option sélectionnée
    let menu_option = show_menu(&mut canvas, &mut event_pump, &ttf_context);
    // Traite l'option sélectionnée dans le menu
    handle_menu_option(canvas, event_pump, &ttf_context, menu_option);
}

fn start_game(
    mut canvas: sdl2::render::Canvas<sdl2::video::Window>, // Canvas pour dessiner sur la fenêtre
    mut event_pump: sdl2::EventPump, // Gestionnaire d'événements SDL
    ttf_context: &sdl2::ttf::Sdl2TtfContext, // Contexte de la gestion des polices
    font_size: u16, // Taille de la police
    mut spawn_cooldown: Duration, // Durée avant le prochain spawn d'éléments
) {
    // Charge la police avec la taille demandée
    let font = ttf_context
        .load_font(std::path::Path::new("assets/expressway.otf"), font_size) // Chargement de la police depuis le fichier
        .expect("Could not load font"); // Gestion des erreurs si la police ne peut pas être chargée

    let (width, height) = canvas.output_size().unwrap(); // Obtient la taille du canvas
    let mut smart_road = SmartRoad::new(); // Création d'une nouvelle instance de SmartRoad
    let texture_creator = canvas.texture_creator(); // Création du créateur de textures

    // Charge les textures nécessaires
    let textures = Textures::load(&texture_creator); // Charge les textures via le créateur de textures
    let textures_map = textures.as_hash_map(); // Convertit les textures en un HashMap pour un accès facile

    // Charge la texture de la ville
    let city_texture = texture_creator.load_texture("assets/city.png").unwrap(); // Charge la texture depuis le fichier

    create_layout(&mut canvas, &city_texture); // Crée le layout initial avec la texture de la ville

    let mut last_spawn_time = Instant::now(); // Initialise le temps du dernier spawn
    let mut is_stats = false; // Indique si les statistiques doivent être affichées
    let mut is_stop = false; // Indique si le jeu doit être arrêté
    let mut stats = Stats::new(); // Création d'une nouvelle instance de statistiques

    'running: loop { // Boucle principale du jeu
        let frame_start = Instant::now(); // Démarre le chronomètre pour cette frame

        // Gère les événements du jeu
        handle_events(
            &mut event_pump,
            &mut smart_road,
            &mut is_stats,
            &mut is_stop,
            &mut stats,
            width,
            height,
            &mut last_spawn_time,
            &mut spawn_cooldown,
        );

        if is_stop {
            break 'running; // Quitte la boucle si le jeu doit être arrêté
        }

        if is_stats {
            stats_layout(&mut canvas, stats, &font, &city_texture); // Affiche les statistiques
        } else {
            update_layout(&mut canvas, &city_texture); // Met à jour le layout
            smart_road.regulate(&mut canvas, textures_map.clone()); // Régule le trafic
        }
        canvas.present(); // Présente le contenu du canvas à l'écran

        let frame_end = Instant::now(); // Démarre le chronomètre pour la fin de la frame
        let frame_duration = frame_end - frame_start; // Calcule la durée de la frame

        // Attends si la durée de la frame est plus courte que la durée de frame souhaitée
        if frame_duration < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - frame_duration); // Attente pour maintenir le FPS
        }
    }

    // Affiche à nouveau le menu et récupère l'option sélectionnée après la fin du jeu
    let menu_option = show_menu(&mut canvas, &mut event_pump, &ttf_context);

    // Traite l'option sélectionnée dans le menu après la fin du jeu
    handle_menu_option(canvas, event_pump, ttf_context, menu_option);
}
