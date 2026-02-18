# 🚗 Smart Road

**Smart Road** est une simulation de carrefour intelligent développée en Rust avec la bibliothèque graphique SDL2. Le programme simule des véhicules circulant depuis les quatre points cardinaux à travers une intersection, en évitant les collisions grâce à un algorithme de planification de trajectoire.

---

(/assets/smart-road.gif)

## 📋 Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021)
- SDL2 avec les extensions **image** et **ttf**

### Installation de SDL2 (Linux / Debian)

```bash
sudo apt install libsdl2-dev libsdl2-image-dev libsdl2-ttf-dev
```

### Installation de SDL2 (macOS)

```bash
brew install sdl2 sdl2_image sdl2_ttf
```

---

## 🚀 Lancement

```bash
cargo run
```

---

## 🗂️ Structure du projet

```
src/
├── main.rs               # Point d'entrée, boucle de jeu principale
├── menu/
│   └── menu.rs           # Menu principal et écran de crédits
├── event/
│   └── event_handler.rs  # Gestion des entrées clavier
├── layout/
│   └── layout.rs         # Rendu de la carte et de l'interface
├── texture/
│   └── texture.rs        # Chargement et gestion des textures
└── traffic/
    ├── mod.rs            # Exports du module traffic
    ├── smart_road.rs     # Structure principale SmartRoad + statistiques
    ├── intersection.rs   # Logique de l'intersection et algorithme anti-collision
    └── vehicle.rs        # Structure Vehicle, directions, vitesses, déplacements
assets/
├── city.png              # Texture de fond (vue de la ville)
├── car.png               # Sprite voiture rouge
├── blue_car.png          # Sprite voiture bleue
├── yellow_car.png        # Sprite voiture jaune
└── expressway.otf        # Police de caractères
```

---

## 🎮 Contrôles

| Touche       | Action                                          |
|--------------|-------------------------------------------------|
| `↑`          | Faire apparaître un véhicule venant du **Sud** (direction Nord) |
| `↓`          | Faire apparaître un véhicule venant du **Nord** (direction Sud) |
| `→`          | Faire apparaître un véhicule venant de l'**Ouest** (direction Est) |
| `←`          | Faire apparaître un véhicule venant de l'**Est** (direction Ouest) |
| `R`          | Faire apparaître un véhicule dans une **direction aléatoire** |
| `Échap`      | Afficher les statistiques de la session         |
| `Échap` ×2  | Quitter vers le menu principal                  |

---

## 🧠 Fonctionnement

### Modes de jeu

Le menu principal propose trois modes, qui varient le délai entre chaque apparition de véhicule :

| Mode                       | Cooldown de spawn |
|----------------------------|-------------------|
| **Confinement**            | 5 000 ms          |
| **Journey normal**         | 250 ms            |
| **Sa fait beaucoup là, non ?** | 100 ms        |

### Les véhicules

Chaque véhicule possède une **direction d'entrée** (Nord, Sud, Est, Ouest) et une **intention de virage** (gauche, droite, tout droit), déterminée aléatoirement à la création :

| Virage       | Couleur  | Vitesse max |
|--------------|----------|-------------|
| Tout droit   | 🔴 Rouge  | Haute       |
| Gauche       | 🟡 Jaune  | Normale     |
| Droite       | 🔵 Bleu   | Basse       |

### Algorithme anti-collision

L'intersection utilise une grille d'états temporels (`Moves`) composée de matrices 20×20 (`State`). Avant d'entrer dans l'intersection, chaque véhicule planifie l'intégralité de sa trajectoire en simulant ses mouvements case par case. L'algorithme (`Algorithm`) cherche la séquence d'instructions optimale parmi trois actions possibles :

- **Accélérer**
- **Maintenir la vitesse**
- **Décélérer**

Si une case est déjà réservée par un autre véhicule dans le même intervalle de temps, le véhicule est placé en **salle d'attente** jusqu'à ce qu'un créneau libre soit disponible, garantissant l'absence de collision.

---

## 📊 Statistiques

En fin de session (touche `Échap`), le programme affiche un récapitulatif :

- Nombre total de véhicules ayant circulé
- Nombre de véhicules ayant traversé l'intersection
- Vitesse maximale, minimale et moyenne
- Temps de traversée maximal, minimal

---

## 🏗️ Architecture technique

- **Langage** : Rust (2021)
- **Rendu graphique** : SDL2 (`sdl2 = "0.37.0"`)
- **Génération aléatoire** : `rand = "0.8.5"`
- **FPS cible** : 120 images par seconde

---

## 👤 Auteur

**Baudry Alexandre**
