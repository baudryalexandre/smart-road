use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;

// Représente un véhicule dans le jeu.
#[derive(Clone, Debug, Copy)]
pub struct Vehicle {
    pub position: Position,       // Position actuelle du véhicule.
    pub turn: Turning,           // Direction dans laquelle le véhicule tourne.
    pub direction: Direction,    // Direction de déplacement du véhicule.
    pub speed: Speed,            // Vitesse actuelle du véhicule.
    pub environment: Environment, // Environnement dans lequel le véhicule se déplace.
    pub pivot: Pivot,            // Point de pivot pour les virages.
    pub color: Color,            // Couleur du véhicule pour le rendre distinct.
    pub time: u32,               // Temps de vie ou temps depuis que le véhicule a été créé.
}

impl Vehicle {
    // Crée un nouveau véhicule avec des caractéristiques basées sur la direction du virage.
    pub fn new(w: u32, h: u32, turn: Turning, direction: Direction) -> Self {
        let color: Color;
        let speed: Speed;
        match turn {
            Turning::Right => {
                color = Color::BLUE;    // Couleur bleue pour les virages à droite.
                speed = Speed::Low;     // Vitesse basse pour les virages à droite.
            },
            Turning::Straight => {
                color = Color::RED;     // Couleur rouge pour aller tout droit.
                speed = Speed::High;    // Vitesse élevée pour aller tout droit.
            },
            Turning::Left => {
                color = Color::YELLOW;  // Couleur jaune pour les virages à gauche.
                speed = Speed::Normal;  // Vitesse normale pour les virages à gauche.
            },
        }
        Vehicle {
            position: Position::new(w, h, turn, direction), // Initialise la position du véhicule.
            turn,                                         // Initialise la direction du virage.
            direction,                                    // Initialise la direction du véhicule.
            speed,                                        // Initialise la vitesse du véhicule.
            environment: Environment::new(w as i32, h as i32), // Initialise l'environnement du véhicule.
            pivot: Pivot::new(Environment::new(w as i32, h as i32), direction, turn), // Initialise le point de pivot.
            color,                                        // Initialise la couleur du véhicule.
            time: 1,                                      // Initialise le temps à 1.
        }
    }

    // Augmente la vitesse du véhicule en fonction de sa couleur.
    pub fn accelerate(&mut self) {
        if self.color == Color::YELLOW {
            match self.speed {
                Speed::No => self.speed = Speed::Low,      // Augmente de No à Low.
                Speed::Low => self.speed = Speed::Normal,  // Augmente de Low à Normal.
                Speed::Normal => {}, // Pas de changement si déjà en Normal.
                Speed::High => {},   // Pas de changement si déjà en High.
            }
        } else if self.color == Color::BLUE {
            match self.speed {
                Speed::No => self.speed = Speed::Low, // Augmente de No à Low.
                Speed::Low => {},   // Pas de changement si déjà en Low.
                Speed::Normal => {}, // Pas de changement si déjà en Normal.
                Speed::High => {},   // Pas de changement si déjà en High.
            }
        } else {
            match self.speed {
                Speed::No => self.speed = Speed::Low,      // Augmente de No à Low.
                Speed::Low => self.speed = Speed::Normal,  // Augmente de Low à Normal.
                Speed::Normal => self.speed = Speed::High, // Augmente de Normal à High.
                Speed::High => {},   // Pas de changement si déjà en High.
            }
        }
    }

    // Diminue la vitesse du véhicule.
    pub fn decelerate(&mut self) {
        match self.speed {
            Speed::No => {}, // Pas de changement si déjà en No.
            Speed::Low => self.speed = Speed::No,    // Diminue de Low à No.
            Speed::Normal => self.speed = Speed::Low, // Diminue de Normal à Low.
            Speed::High => self.speed = Speed::Normal, // Diminue de High à Normal.
        }
    }

    // Déplace le véhicule selon sa direction et applique les changements de direction en cas de virage.
    pub fn drive(&mut self) {
        match self.direction {
            Direction::North => self.position.y -= self.speed as i32, // Déplace vers le Nord.
            Direction::South => self.position.y += self.speed as i32, // Déplace vers le Sud.
            Direction::East => self.position.x += self.speed as i32,  // Déplace vers l'Est.
            Direction::West => self.position.x -= self.speed as i32,  // Déplace vers l'Ouest.
        }
        match self.turn {
            Turning::Straight => {} // Pas de changement pour aller tout droit.
            Turning::Left => {
                if self.is_at_pivot() {
                    self.turn = Turning::Straight; // Change la direction en allant tout droit.
                    self.position = self.pivot.position; // Réinitialise la position au pivot.
                    // Met à jour la direction selon le virage à gauche.
                    match self.direction {
                        Direction::North => self.direction = Direction::West,
                        Direction::South => self.direction = Direction::East,
                        Direction::East => self.direction = Direction::North,
                        Direction::West => self.direction = Direction::South,
                    }
                }
            }
            Turning::Right => {
                if self.is_at_pivot() {
                    self.turn = Turning::Straight; // Change la direction en allant tout droit.
                    self.position = self.pivot.position; // Réinitialise la position au pivot.
                    // Met à jour la direction selon le virage à droite.
                    match self.direction {
                        Direction::North => self.direction = Direction::East,
                        Direction::South => self.direction = Direction::West,
                        Direction::East => self.direction = Direction::South,
                        Direction::West => self.direction = Direction::North,
                    }
                }
            }
        }
    }

    // Vérifie si le véhicule a atteint le point de pivot pour le virage.
    pub fn is_at_pivot(self) -> bool {
        match self.pivot.over {
            true => {
                self.position.x >= self.pivot.position.x && self.position.y >= self.pivot.position.y
            }
            false => {
                self.position.x <= self.pivot.position.x && self.position.y <= self.pivot.position.y
            }
        }
    }

    // Vérifie si le véhicule est hors des limites de l'environnement.
    pub fn is_out(self) -> bool {
        self.position.x > self.environment.width
            || self.position.x < -40
            || self.position.y > self.environment.height
            || self.position.y < -40
    }

    // Renders (affiche) le véhicule sur le canvas en utilisant les textures.
    pub fn render(&mut self, canvas: &mut WindowCanvas, texture: HashMap<String, &Texture>) {
        // Crée un rectangle pour la position et la taille du véhicule à afficher.
        let rect = Rect::new(self.position.x, self.position.y, 40, 40);

        // Sélectionne le sprite du véhicule en fonction de sa direction.
        let sprite = match self.direction {
            Direction::South => Rect::new(3, 5, 60, 60), // Sprite par défaut pour le Sud.
            Direction::North => Rect::new(3, 205, 60, 60),
            Direction::East => Rect::new(3, 135, 60, 60),
            Direction::West => Rect::new(3, 65, 60, 60),
        };

        // Affiche le sprite sur le canvas selon la couleur du véhicule.
        if self.color == Color::RED {
            if let Some(texture) = texture.get("car_texture_red") {
                canvas.copy(texture, sprite, rect).unwrap(); // Affiche la texture rouge.
            } else {
                println!("Texture non trouvée"); // Message d'erreur si la texture n'est pas trouvée.
            }
        } else if self.color == Color::YELLOW {
            if let Some(texture) = texture.get("car_texture_yellow") {
                canvas.copy(texture, sprite, rect).unwrap(); // Affiche la texture jaune.
            } else {
                println!("Texture non trouvée"); // Message d'erreur si la texture n'est pas trouvée.
            }
        } else {
            if let Some(texture) = texture.get("car_texture_blue") {
                canvas.copy(texture, sprite, rect).unwrap(); // Affiche la texture bleue.
            } else {
                println!("Texture non trouvée"); // Message d'erreur si la texture n'est pas trouvée.
            }
        }
    }

    // Retourne la vitesse du véhicule sous forme de valeur entière.
    pub fn get_speed(self) -> u32 {
        match self.speed {
            Speed::High => 7,   // Vitesse élevée.
            Speed::Normal => 5, // Vitesse normale.
            Speed::Low => 3,    // Vitesse basse.
            Speed::No => 0,     // Pas de vitesse.
        }
    }
}

// Représente un point de pivot pour les virages.
#[derive(Clone, Debug, Copy)]
pub struct Pivot {
    position: Position, // Position du point de pivot.
    over: bool,        // Indique si le pivot est au-dessus ou en dessous de la position du véhicule.
}

impl Pivot {
    // Crée un nouveau point de pivot en fonction de l'environnement, de la direction et du virage.
    pub fn new(env: Environment, dir: Direction, turn: Turning) -> Self {
        let mut pos = env.center; // Initialise la position au centre de l'environnement.
        let mut over = true;      // Par défaut, le pivot est au-dessus.
        match turn {
            Turning::Straight => {}
            Turning::Right => match dir {
                Direction::North => {
                    pos = Position {
                        x: (env.center.x + 80),  // Position pour un virage à droite vers le Nord.
                        y: (env.center.y + 80),
                    };
                    over = false;
                }
                Direction::South => {
                    pos = Position {
                        x: (env.center.x - 120), // Position pour un virage à droite vers le Sud.
                        y: (env.center.y - 120),
                    };
                    over = true;
                }
                Direction::West => {
                    pos = Position {
                        x: (env.center.x + 80),  // Position pour un virage à droite vers l'Ouest.
                        y: (env.center.y - 120),
                    };
                    over = false;
                }
                Direction::East => {
                    pos = Position {
                        x: (env.center.x - 120), // Position pour un virage à droite vers l'Est.
                        y: (env.center.y + 80),
                    };
                    over = true;
                }
            },
            Turning::Left => match dir {
                Direction::North => {
                    pos = Position {
                        x: (env.center.x),  // Position pour un virage à gauche vers le Nord.
                        y: (env.center.y - 40),
                    };
                    over = false;
                }
                Direction::South => {
                    pos = Position {
                        x: (env.center.x - 40), // Position pour un virage à gauche vers le Sud.
                        y: (env.center.y),
                    };
                    over = true;
                }
                Direction::West => {
                    pos = Position {
                        x: (env.center.x - 40), // Position pour un virage à gauche vers l'Ouest.
                        y: (env.center.y - 40),
                    };
                    over = false;
                }
                Direction::East => {
                    pos = Position {
                        x: (env.center.x),  // Position pour un virage à gauche vers l'Est.
                        y: (env.center.y),
                    };
                    over = true;
                }
            },
        }
        Pivot {
            position: pos, // Position calculée du pivot.
            over,         // État de position au-dessus ou en dessous.
        }
    }
}

// Enumération des directions dans lesquelles un véhicule peut se déplacer.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Turning {
    Left,
    Right,
    Straight,
}

impl Distribution<Turning> for Standard {
    // Permet de générer aléatoirement une direction de virage.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turning {
        match rng.gen_range(0..3) {
            0 => Turning::Left,
            1 => Turning::Right,
            _ => Turning::Straight,
        }
    }
}

// Enumération des directions cardinales.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Distribution<Direction> for Standard {
    // Permet de générer aléatoirement une direction de déplacement.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }
}

// Enumération des vitesses possibles pour un véhicule.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Speed {
    No = 0,      // Pas de vitesse.
    Low = 12,    // Vitesse basse.
    Normal = 20, // Vitesse normale.
    High = 30,   // Vitesse élevée.
}

// Représente la position d'un véhicule ou d'un point dans l'environnement.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32, // Coordonnée x de la position.
    pub y: i32, // Coordonnée y de la position.
}

impl Position {
    // Crée une nouvelle position en fonction des dimensions de l'environnement, du virage et de la direction.
    pub fn new(w: u32, h: u32, turn: Turning, dir: Direction) -> Self {
        let mut n = 0;
        match turn {
            Turning::Left => {}
            Turning::Right => n += 80, // Ajuste la position pour les virages à droite.
            Turning::Straight => n += 40, // Ajuste la position pour aller tout droit.
        }
        match dir {
            Direction::North => Position {
                x: (w as i32 / 2 + n), // Position pour le Nord.
                y: (h as i32),
            },
            Direction::West => Position {
                x: (w as i32),
                y: (h as i32 / 2 - 40 - n), // Position pour l'Ouest.
            },
            Direction::South => Position {
                x: (w as i32 / 2 - 40 - n), // Position pour le Sud.
                y: (-40),
            },
            Direction::East => Position {
                x: (-40),
                y: (w as i32 / 2 + n), // Position pour l'Est.
            },
        }
    }
}

// Représente l'environnement dans lequel les véhicules se déplacent.
#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub width: i32,    // Largeur de l'environnement.
    pub height: i32,   // Hauteur de l'environnement.
    pub center: Position, // Position centrale de l'environnement.
}

impl Environment {
    // Crée un nouvel environnement avec des dimensions spécifiques.
    pub fn new(width: i32, height: i32) -> Environment {
        Environment {
            width,
            height,
            center: Position {
                x: width / 2, // Position centrale en x.
                y: height / 2, // Position centrale en y.
            },
        }
    }
}
