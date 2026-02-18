use std::time::Duration; // Importation du type Duration pour gérer les durées.

use sdl2::rect::Rect; // Importation du type Rect pour définir des rectangles à dessiner.
use sdl2::pixels::Color; // Importation du type Color pour définir les couleurs.
use sdl2::render::Canvas; // Importation du type Canvas pour dessiner sur la fenêtre.
use sdl2::ttf::Sdl2TtfContext; // Importation du contexte TTF pour gérer les polices de caractères.
use sdl2::video::Window; // Importation du type Window pour gérer la fenêtre SDL.
use sdl2::EventPump; // Importation de EventPump pour gérer la file d'événements SDL.
use sdl2::event::Event; // Importation du type Event pour gérer les événements.
use sdl2::keyboard::Keycode; // Importation du type Keycode pour gérer les touches du clavier.

use crate::start_game; // Importation de la fonction `start_game` du module local.

/// Enumération des options du menu.
pub enum MenuOption {
    Confinement, // Option "Confinement"
    Journey, // Option "Journey normal"
    Beaucoup, // Option "Sa fait beaucoup la, non?"
    Quit, // Option "Quitter"
}

/// Affiche le menu principal et gère les interactions utilisateur.
pub fn show_menu(
    canvas: &mut Canvas<Window>, // Canvas pour dessiner sur la fenêtre.
    event_pump: &mut EventPump, // EventPump pour gérer les événements.
    ttf_context: &Sdl2TtfContext // Contexte pour les polices TTF.
) -> Option<MenuOption> { // Retourne une option du menu ou None si l'utilisateur quitte.

    // Chargement de la police depuis les assets avec une taille de 50 pixels.
    let font = ttf_context
     .load_font(std::path::Path::new("assets/expressway.otf"), 50)
     .expect("Could not load font"); // Erreur si la police ne peut être chargée.

    // Définition des items du menu.
    let menu_items = ["Confinement", "Journée normal", "Cauchemar", "Quit"];
    let mut selected_index = 0; // Index de l'option actuellement sélectionnée.

    loop {
        // Efface l'écran en blanc.
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        // Dessine chaque option du menu.
        for (i, item) in menu_items.iter().enumerate() {
            // La couleur dépend si l'option est sélectionnée ou non.
            let color = if i == selected_index {
                Color::RED // Rouge si sélectionnée.
            } else {
                Color::BLACK // Noir sinon.
            };

            // Crée une surface texte pour l'item.
            let surface = font.render(item)
                .blended(color)
                .map_err(|e| e.to_string())
                .unwrap();
            let texture_creator = canvas.texture_creator(); // Créeur de texture.
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            // Définit la position et la taille du texte.
            let target = Rect::new(20, 20 + (i as i32) * 100, surface.width(), surface.height());
            canvas.copy(&texture, None, Some(target)).unwrap(); // Copie la texture sur le canvas.
        }

        canvas.present(); // Présente le canvas mis à jour à l'écran.

        // Gère les événements utilisateur.
        for event in event_pump.poll_iter() {
            match event {
                // Si l'utilisateur appuie sur la flèche haut.
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if selected_index > 0 { // On remonte dans le menu.
                        selected_index -= 1;
                    }
                }
                // Si l'utilisateur appuie sur la flèche bas.
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if selected_index < menu_items.len() - 1 { // On descend dans le menu.
                        selected_index += 1;
                    }
                }
                // Si l'utilisateur appuie sur Entrée.
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match selected_index {
                    0 => return Some(MenuOption::Confinement), // Confinement
                    1 => return Some(MenuOption::Journey), // Journey
                    2 => return Some(MenuOption::Beaucoup), // Beaucoup
                    3 => return Some(MenuOption::Quit), // Quitter
                    _ => {}
                },
                Event::Quit { .. } => return None, // Quitte l'application si l'utilisateur ferme la fenêtre.
                _ => {}
            }
        }
    }
}

/// Gère l'option sélectionnée dans le menu.
pub fn handle_menu_option(
    mut canvas: sdl2::render::Canvas<sdl2::video::Window>, // Canvas pour dessiner sur la fenêtre.
    mut event_pump: sdl2::EventPump, // EventPump pour gérer les événements.
    ttf_context: &sdl2::ttf::Sdl2TtfContext, // Contexte pour les polices TTF.
    option: Option<MenuOption>, // Option du menu sélectionnée par l'utilisateur.
) {
    let spawn_cooldown: Duration; // Durée du cooldown entre les apparitions de véhicules.
    match option {
        Some(MenuOption::Confinement) => {
            spawn_cooldown = Duration::from_millis(5000); // Cooldown de 5 secondes.
            start_game(canvas, event_pump, &ttf_context, 150, spawn_cooldown); // Démarre le jeu avec ces paramètres.
        }
        Some(MenuOption::Journey) => {
            spawn_cooldown = Duration::from_millis(250); // Cooldown de 250 ms.
            start_game(canvas, event_pump, &ttf_context, 150, spawn_cooldown); // Démarre le jeu avec ces paramètres.
        }
        Some(MenuOption::Beaucoup) => {
            spawn_cooldown = Duration::from_millis(100); // Cooldown de 100 ms.
            start_game(canvas, event_pump, &ttf_context, 150, spawn_cooldown); // Démarre le jeu avec ces paramètres.
        }
        _ => {
            show_credits(&mut canvas, &mut event_pump, &ttf_context); // Affiche les crédits si aucune option n'est sélectionnée.
        }
    }
}

/// Affiche les crédits du jeu.
fn show_credits(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, // Canvas pour dessiner sur la fenêtre.
    event_pump: &mut sdl2::EventPump, // EventPump pour gérer les événements.
    ttf_context: &sdl2::ttf::Sdl2TtfContext, // Contexte pour les polices TTF.
) {
    let texture_creator = canvas.texture_creator(); // Créeur de texture.

    // Chargement des polices avec différentes tailles.
    let font = ttf_context.load_font("assets/expressway.otf", 64).unwrap();
    let middle_font= ttf_context.load_font("assets/expressway.otf", 48).unwrap();
    let small_font = ttf_context.load_font("assets/expressway.otf", 32).unwrap();
    
    let mut text_color = Color::RGB(0, 0, 0); // Couleur noire pour le texte.
    
    // Crée la surface et texture pour le titre "Game Credits".
    let credits_surface = font.render("Game Credits")
        .blended(text_color)
        .unwrap();
    let credits_texture = texture_creator.create_texture_from_surface(&credits_surface).unwrap();
    
    // Position du texte "Game Credits".
    let credits_rect = Rect::new(100, 100, credits_surface.width(), credits_surface.height());

    // Crée la surface et texture pour le premier auteur.
    let author1_surface = middle_font.render("Baudry Alexandre")
        .blended(text_color)
        .unwrap();
    let author1_texture = texture_creator.create_texture_from_surface(&author1_surface).unwrap();
    
    // Position du texte "Baudry Alexandre".
    let author1_rect = Rect::new(100, 200, author1_surface.width(), author1_surface.height());
    
    // Change la couleur du texte en rouge pour le message de retour.
    text_color = Color::RED;

    // Crée la surface et texture pour le message "Au revoir".
    let return_surface = small_font.render("Au revoir")
        .blended(text_color)
        .unwrap();
    let return_texture = texture_creator.create_texture_from_surface(&return_surface).unwrap();
    
    // Position du texte "Au revoir".
    let return_rect = Rect::new(100, 600, return_surface.width(), return_surface.height());
    
    // Boucle pour afficher les crédits jusqu'à ce que l'utilisateur appuie sur Entrée.
    loop {
        canvas.set_draw_color(Color::WHITE); // Efface l'écran en blanc.
        canvas.clear();

        // Dessine chaque élément des crédits sur le canvas.
        canvas.copy(&credits_texture, None, credits_rect).unwrap();
        canvas.copy(&author1_texture, None, author1_rect).unwrap();
        canvas.copy(&return_texture, None, return_rect).unwrap();

        canvas.present(); // Présente le canvas mis à jour à l'écran.

        // Gère les événements utilisateur.
        for event in event_pump.poll_iter() {
            match event {
                // Si l'utilisateur appuie sur Entrée, sort de la boucle et retourne au menu.
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match 0 {
                    0 => return,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
