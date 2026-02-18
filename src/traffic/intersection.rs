use super::Vehicle; // Importation du type Vehicle du module parent.
use sdl2::render::{Texture, WindowCanvas}; // Importation des types Texture et WindowCanvas de la bibliothèque SDL2.
use std::collections::{HashMap, VecDeque}; // Importation des types HashMap et VecDeque de la bibliothèque standard.

// Définition de la structure Intersection, représentant un carrefour où les véhicules interagissent.
pub struct Intersection {
    pub vehicles: VecDeque<InstructedVehicle>, // Liste des véhicules instruits en attente ou en mouvement.
    pub moves: Moves, // Représente les états de mouvement du carrefour.
    pub waiting_room: Vec<Vehicle>, // Liste des véhicules en attente d'instruction.
    pub min_time: u32, // Temps minimum que les véhicules ont mis pour traverser l'intersection.
    pub max_time: u32, // Temps maximum que les véhicules ont mis pour traverser l'intersection.
    pub min_velocity: u32, // Vitesse minimale des véhicules qui ont traversé l'intersection.
    pub max_velocity: u32, // Vitesse maximale des véhicules qui ont traversé l'intersection.
    pub nbr_passed_vehicles: u32, // Nombre total de véhicules qui ont traversé l'intersection.
    pub avg_velocity: f32, // Vitesse moyenne des véhicules ayant traversé l'intersection.
}

impl Intersection {
    // Création d'une nouvelle instance de Intersection avec des valeurs par défaut.
    pub fn new() -> Self {
        Intersection {
            vehicles: VecDeque::new(),
            moves: Moves::new(),
            waiting_room: vec![],
            min_time: u32::MAX,
            max_time: u32::MIN,
            min_velocity: u32::MAX,
            max_velocity: u32::MIN,
            nbr_passed_vehicles: u32::MIN,
            avg_velocity: 0.0,
        }
    }

    // Déplace les véhicules de la salle d'attente vers la liste des véhicules en mouvement.
    pub fn waiting(&mut self) {
        let list = self.waiting_room.clone(); // Clone la liste des véhicules en attente.
        self.waiting_room = vec![]; // Vide la salle d'attente.
        for v in list {
            self.add_vehicle(v); // Ajoute chaque véhicule à la liste des véhicules en mouvement.
        }
    }

    // Ajoute un véhicule à la liste des véhicules en mouvement, en l'instruisant d'abord.
    pub fn add_vehicle(&mut self, mut v: Vehicle) {
        let instrs = self.instruct_vehicle(&v); // Obtient les instructions pour le véhicule.
        v.time += 1; // Incrémente le temps du véhicule.
        if instrs.len() == 0 {
            self.waiting_room.push(v); // Ajoute le véhicule à la salle d'attente si aucune instruction n'est trouvée.
            return;
        }
        self.vehicles.push_back(InstructedVehicle::new(v, instrs)); // Ajoute le véhicule instruit à la liste des véhicules.
    }

    // Génère les instructions pour un véhicule basé sur les états de mouvement actuels du carrefour.
    pub fn instruct_vehicle(&mut self, v: &Vehicle) -> VecDeque<Instruction> {
        let mut algo = Algorithm::new(); // Crée une nouvelle instance de l'algorithme.
        let mut res = algo.algorithm(&self.moves, v, VecDeque::new()); // Exécute l'algorithme pour obtenir les instructions.
        if res.len() == 0 && self.moves.states.len() > 0 {
            return VecDeque::new(); // Retourne une file vide si aucune instruction n'est trouvée.
        }
        let mut sim_v = v.clone(); // Clone le véhicule pour la simulation.
        let mut ix = 0; // Index pour parcourir les instructions.
        while !sim_v.is_out() { // Continue jusqu'à ce que le véhicule soit hors de l'intersection.
            let x = sim_v.position.x / 2; // Coordonnée x du véhicule.
            let y = sim_v.position.y / 2; // Coordonnée y du véhicule.
            if ix >= self.moves.states.len() { // Ajoute un nouvel état si nécessaire.
                self.moves.add_state();
            }
            let (mut xs, mut ys) = (vec![x / 20], vec![y / 20]); // Coordonnées divisées par 20.
            if x % 20 != 0 {
                xs.push((x / 20) + 1); // Ajoute la coordonnée x suivante si nécessaire.
            }
            if y % 20 != 0 {
                ys.push((y / 20) + 1); // Ajoute la coordonnée y suivante si nécessaire.
            }
            for a in xs {
                for b in &ys {
                    self.moves.states[ix].occupy(a as usize, *b as usize); // Marque les cases comme occupées.
                }
            }
            if ix >= res.len() { // Vérifie si des instructions supplémentaires sont nécessaires.
                if sim_v.speed != super::Speed::High {
                    res.push_back(Instruction::Accelerate); // Accélère si la vitesse n'est pas déjà élevée.
                    sim_v.accelerate(); // Accélère le véhicule.
                } else {
                    res.push_back(Instruction::Still); // Maintient la vitesse si elle est déjà élevée.
                }
                sim_v.drive(); // Avance le véhicule.
                ix += 1; // Passe à l'instruction suivante.
                continue;
            }
            match res[ix] {
                Instruction::Accelerate => sim_v.accelerate(), // Accélère si l'instruction le demande.
                Instruction::Deaccelerate => sim_v.decelerate(), // Décélère si l'instruction le demande.
                Instruction::Still => {} // Maintient la vitesse actuelle si l'instruction le demande.
            }
            sim_v.drive(); // Avance le véhicule.
            ix += 1; // Passe à l'instruction suivante.
        }
        res // Retourne les instructions générées.
    }

    // Régule le mouvement des véhicules et met à jour les statistiques.
    pub fn regulate(&mut self, canvas: &mut WindowCanvas, texture: HashMap<String, &Texture>) {
        let mut list = vec![]; // Liste des indices des véhicules qui ont terminé leur instruction.
        let mut total = 0.0; // Total des vitesses des véhicules pour calculer la vitesse moyenne.
        for ix in 0..self.vehicles.len() {
            let v = self.vehicles[ix].vehicle.get_speed(); // Obtient la vitesse du véhicule.
            if v > self.max_velocity {
                self.max_velocity = v; // Met à jour la vitesse maximale.
            }
            if v < self.min_velocity {
                self.min_velocity = v; // Met à jour la vitesse minimale.
            }
            total += v as f32; // Ajoute la vitesse au total.
            self.vehicles[ix].vehicle.time += 1; // Incrémente le temps du véhicule.
            self.vehicles[ix].follow_instruction(canvas, texture.clone()); // Suit les instructions du véhicule.
            if self.vehicles[ix].is_empty_instructions() { // Vérifie si toutes les instructions ont été exécutées.
                self.nbr_passed_vehicles += 1; // Incrémente le nombre de véhicules passés.
                list.push(ix); // Ajoute l'indice du véhicule à la liste des véhicules terminés.
                let t = self.vehicles[ix].vehicle.time; // Obtient le temps du véhicule.
                if t > self.max_time {
                    self.max_time = t; // Met à jour le temps maximum.
                }
                if t < self.min_time {
                    self.min_time = t; // Met à jour le temps minimum.
                }
            }
        }
        if self.vehicles.len() > 0 { // Met à jour la vitesse moyenne.
            if self.average_velocity() == 0.0 {
                self.avg_velocity = total / self.vehicles.len() as f32;
            } else {
                self.avg_velocity = ((total / self.vehicles.len() as f32) + self.avg_velocity) / 2.0;
            }
        }
        list.reverse(); // Inverse la liste pour supprimer les éléments du début de la liste des véhicules.
        for jx in list {
            self.vehicles.remove(jx); // Supprime les véhicules terminés de la liste des véhicules en mouvement.
        }
    }

    // Supprime les véhicules qui sont sortis des limites de l'intersection.
    pub fn remove_out_of_bounds_vehicles(&mut self) {
        self.vehicles.retain(|vehicle| !vehicle.vehicle.is_out()); // Conserve uniquement les véhicules encore dans les limites.
    }

    // Retourne la vitesse moyenne des véhicules ayant traversé l'intersection.
    pub fn average_velocity(&self) -> f32 {
        self.avg_velocity
    }
}

// Structure représentant un véhicule avec des instructions.
#[derive(Clone, Debug)]
pub struct InstructedVehicle {
    pub vehicle: Vehicle, // Le véhicule associé.
    pub instructions: VecDeque<Instruction>, // Liste des instructions à suivre.
}

impl InstructedVehicle {
    // Création d'un nouveau InstructedVehicle.
    pub fn new(v: Vehicle, instrs: VecDeque<Instruction>) -> Self {
        InstructedVehicle {
            vehicle: v,
            instructions: instrs,
        }
    }

    // Suit la première instruction et rend le véhicule sur le canvas.
    pub fn follow_instruction(&mut self, canvas: &mut WindowCanvas, texture: HashMap<String, &Texture>) {
        match self.instructions[0] {
            Instruction::Still => {} // Ne fait rien si l'instruction est de rester en place.
            Instruction::Deaccelerate => self.vehicle.decelerate(), // Décélère le véhicule si l'instruction le demande.
            Instruction::Accelerate => self.vehicle.accelerate(), // Accélère le véhicule si l'instruction le demande.
        }
        self.vehicle.drive(); // Avance le véhicule.
        self.vehicle.render(canvas, texture); // Rend le véhicule sur le canvas.
        self.instructions.pop_front(); // Retire l'instruction exécutée de la liste.
    }

    // Vérifie si la liste des instructions est vide.
    pub fn is_empty_instructions(&self) -> bool {
        self.instructions.len() == 0
    }
}

// Enumération représentant les types d'instructions que les véhicules peuvent recevoir.
#[derive(Clone, Debug)]
pub enum Instruction {
    Deaccelerate, // Décélération
    Still, // Rester en place
    Accelerate, // Accélération
}

// Structure représentant les états de mouvement du carrefour.
#[derive(Clone, Debug)]
pub struct Moves {
    states: VecDeque<State>, // Liste des états de mouvement.
}

impl Moves {
    // Création d'un nouvel ensemble d'états de mouvement.
    pub fn new() -> Self {
        Moves {
            states: VecDeque::new(),
        }
    }

    // Ajoute un nouvel état à la liste des états de mouvement.
    pub fn add_state(&mut self) {
        self.states.push_back(State::new())
    }

    // Supprime le premier état de la liste des états de mouvement.
    pub fn drop_state(&mut self) {
        self.states.pop_front();
    }
}

// Structure représentant un état de mouvement du carrefour.
#[derive(Clone, Debug)]
pub struct State {
    board: Vec<Vec<bool>>, // Matrice représentant les cases occupées du carrefour.
}

impl State {
    // Création d'un nouvel état avec une matrice vide.
    pub fn new() -> Self {
        let line = vec![false; 20]; // Crée une ligne de cases non occupées.
        State {
            board: vec![line; 20], // Crée la matrice de 20x20 cases non occupées.
        }
    }

    // Vérifie si une case est occupée.
    pub fn is_occupied(&mut self, x: usize, y: usize) -> bool {
        if x >= 20 || y >= 20 {
            return false; // Retourne false si les coordonnées sont en dehors des limites.
        }
        self.board[x][y] // Retourne l'état de la case spécifiée.
    }

    // Marque une case comme occupée.
    pub fn occupy(&mut self, x: usize, y: usize) {
        if x >= 20 || y >= 20 {
            return; // Ne fait rien si les coordonnées sont en dehors des limites.
        }
        self.board[x][y] = true; // Marque la case comme occupée.
    }
}

// Structure représentant un algorithme pour générer des instructions.
pub struct Algorithm {
    visited: HashMap<String, VecDeque<Instruction>>, // Liste des chemins déjà visités avec leurs instructions.
}

impl Algorithm {
    // Création d'un nouvel algorithme.
    pub fn new() -> Self {
        Algorithm {
            visited: HashMap::new(), // Initialise la liste des chemins visités.
        }
    }

    // Exécute l'algorithme pour générer des instructions pour un véhicule.
    pub fn algorithm(
        &mut self,
        moves: &Moves,
        v: &Vehicle,
        instr: VecDeque<Instruction>,
    ) -> VecDeque<Instruction> {
        if v.is_out() || moves.states.len() == 0 {
            return instr; // Retourne les instructions si le véhicule est hors de l'intersection ou s'il n'y a pas d'états.
        }
        let mut algo = (0, 0, 0); // Compteurs pour les types d'instructions.
        for s in &instr {
            match *s {
                Instruction::Accelerate => algo.0 += 1, // Compte les instructions d'accélération.
                Instruction::Still => algo.1 += 1, // Compte les instructions de maintien.
                Instruction::Deaccelerate => algo.2 += 1, // Compte les instructions de décélération.
            }
        }
        let key = format!("{}:{}:{}", algo.0, algo.1, algo.2); // Crée une clé pour le cache.
        if self.visited.contains_key(&key) { // Vérifie si le chemin a déjà été visité.
            let mut res = VecDeque::new();
            match self.visited.get(&key) {
                Some(v) => res = v.clone(), // Récupère les instructions du cache.
                None => {}
            }
            return res; // Retourne les instructions trouvées dans le cache.
        }
        let x = v.position.x / 2; // Coordonnée x du véhicule.
        let y = v.position.y / 2; // Coordonnée y du véhicule.
        let (mut xs, mut ys) = (vec![x / 20], vec![y / 20]); // Coordonnées divisées par 20.
        let mut sim_moves = moves.clone(); // Clone l'état des mouvements pour simulation.
        if x % 20 != 0 {
            xs.push((x / 20) + 1); // Ajoute la coordonnée x suivante si nécessaire.
        }
        if y % 20 != 0 {
            ys.push((y / 20) + 1); // Ajoute la coordonnée y suivante si nécessaire.
        }
        let (mut a1, mut b1) = (0, 0); // Détermine les déplacements en fonction de la direction du véhicule.
        match v.direction {
            super::Direction::North => b1 -= 1,
            super::Direction::South => b1 += 1,
            super::Direction::East => a1 += 1,
            super::Direction::West => a1 -= 1,
        }
        for a in xs {
            for b in &ys {
                let mut ok = sim_moves.states[0].is_occupied(a as usize, *b as usize); // Vérifie si la case est occupée.
                if ok {
                    return VecDeque::new(); // Retourne une file vide si la case est occupée.
                }
                ok = sim_moves.states[0].is_occupied((a + a1) as usize, (*b + b1) as usize); // Vérifie la case suivante.
                if ok {
                    return VecDeque::new(); // Retourne une file vide si la case suivante est occupée.
                }
            }
        }
        let mut sim_v1 = v.clone(); // Clone le véhicule pour simulation.
        let mut m1 = moves.clone(); // Clone les mouvements pour simulation.
        let mut instr1 = instr.clone(); // Clone les instructions pour simulation.
        let mut res: VecDeque<Instruction>;
        if v.speed != super::Speed::High {
            sim_v1.accelerate(); // Accélère le véhicule.
            sim_v1.drive(); // Avance le véhicule.
            m1.drop_state(); // Supprime l'état actuel.
            instr1.push_back(Instruction::Accelerate); // Ajoute l'instruction d'accélération.
            res = self.algorithm(&m1, &sim_v1, instr1); // Exécute l'algorithme avec les nouvelles instructions.
            if res.len() > 0 {
                self.visited.insert(key, res.clone()); // Enregistre les instructions dans le cache.
                return res; // Retourne les instructions générées.
            }
        }
        sim_v1 = v.clone(); // Réinitialise le véhicule pour une autre simulation.
        m1 = moves.clone(); // Réinitialise les mouvements pour une autre simulation.
        instr1 = instr.clone(); // Réinitialise les instructions pour une autre simulation.
        sim_v1.drive(); // Avance le véhicule sans accélérer.
        m1.drop_state(); // Supprime l'état actuel.
        instr1.push_back(Instruction::Still); // Ajoute l'instruction de maintien.
        res = self.algorithm(&m1, &sim_v1, instr1); // Exécute l'algorithme avec les nouvelles instructions.
        if res.len() > 0 {
            self.visited.insert(key, res.clone()); // Enregistre les instructions dans le cache.
            return res; // Retourne les instructions générées.
        }
        if v.speed != super::Speed::No {
            sim_v1 = v.clone(); // Réinitialise le véhicule pour une autre simulation.
            m1 = moves.clone(); // Réinitialise les mouvements pour une autre simulation.
            instr1 = instr.clone(); // Réinitialise les instructions pour une autre simulation.
            sim_v1.decelerate(); // Décélère le véhicule.
            sim_v1.drive(); // Avance le véhicule.
            m1.drop_state(); // Supprime l'état actuel.
            instr1.push_back(Instruction::Deaccelerate); // Ajoute l'instruction de décélération.
            res = self.algorithm(&m1, &sim_v1, instr1); // Exécute l'algorithme avec les nouvelles instructions.
            if res.len() > 0 {
                self.visited.insert(key, res.clone()); // Enregistre les instructions dans le cache.
                return res; // Retourne les instructions générées.
            }
        }
        self.visited.insert(key, res.clone()); // Enregistre les instructions dans le cache même si aucune instruction n'a été générée.
        res // Retourne les instructions générées.
    }
}
