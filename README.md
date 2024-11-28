# Space Invaders
A terminal-based Space Invaders clone implemented in Rust.


# Features

- Classic Space Invaders gameplay
- Terminal-based rendering with color
- Simple controls
- Dynamic enemy movement
- Scoring system

# Prerequisites

Rust (latest stable version)
Cargo package manager

# Installation

### Clone the repository:

```bash
git clone https://github.com/yourusername/rust-space-invaders.git
cd rust-space-invaders
```

### Build the project:
```bash
cargo build --release
```

# Running the Game

```bash
cargo run
```

# Code Snippets
### Game Initialization

```rust
fn new() -> Self {
    let mut game = Game {
        player: GameObject { 
            x: SCREEN_WIDTH / 2, 
            y: SCREEN_HEIGHT - 2,
            alive: true 
        },
        enemies: Vec::new(),
        player_bullets: Vec::new(),
        enemy_bullets: Vec::new(),
        score: 0,
        game_over: false,
        enemy_move_counter: 0,
    };
    game.spawn_enemies();
    game
}
```

### Enemy Movement Logic
```rust
fn move_enemies(&mut self) {
    self.enemy_move_counter += 1;
    if self.enemy_move_counter < 5 {
        return;
    }
    self.enemy_move_counter = 0;

    let mut move_down = false;
    let mut direction = 1;

    for enemy in &mut self.enemies {
        if enemy.alive {
            enemy.x = (enemy.x as i32 + direction)
                .max(0)
                .min(SCREEN_WIDTH as i32 - 1) as usize;
            
            if enemy.x == 0 || enemy.x == SCREEN_WIDTH - 1 {
                move_down = true;
                direction *= -1;
            }
        }
    }
    // ... additional movement logic
}
```

### Controls

- `Left Arrow`: Move ship left
- `Right Arrow`: Move ship right
- `Space`: Shoot
- `Esc`: Exit game

### Gameplay

- Destroy all enemies before they reach the bottom of the screen
- Enemies move across the screen and occasionally shoot
- Each destroyed enemy gives you 10 points
- Game ends if an enemy bullet hits your ship or enemies reach the bottom

### Dependencies

- `crossterm`: Terminal manipulation
- `rand`: Random number generation

### Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5 Open a Pull Request




