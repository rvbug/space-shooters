use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType},
    style::{Color, SetForegroundColor},
};
use rand::Rng;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::thread;
use std::io;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 20;
const PLAYER_CHAR: char = '^';
const ENEMY_CHAR: char = 'W';
const BULLET_CHAR: char = '|';

#[derive(Clone, PartialEq)]
struct GameObject {
    x: usize,
    y: usize,
    alive: bool,
}

struct Game {
    player: GameObject,
    enemies: Vec<GameObject>,
    player_bullets: Vec<GameObject>,
    enemy_bullets: Vec<GameObject>,
    score: usize,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            player: GameObject { x: SCREEN_WIDTH / 2, y: SCREEN_HEIGHT - 2, alive: true },
            enemies: Vec::new(),
            player_bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            score: 0,
            game_over: false,
        };
        game.spawn_enemies();
        game
    }

    fn spawn_enemies(&mut self) {
        let mut rng = rand::thread_rng();
        for row in 0..4 {
            for col in 0..8 {
                self.enemies.push(GameObject {
                    x: col * 5 + 5,
                    y: row * 3 + 2,
                    alive: true,
                });
            }
        }
    }

    fn move_player(&mut self, direction: i32) {
        let new_x = self.player.x as i32 + direction;
        if new_x > 0 && new_x < SCREEN_WIDTH as i32 - 1 {
            self.player.x = new_x as usize;
        }
    }

    fn shoot_bullet(&mut self) {
        self.player_bullets.push(GameObject {
            x: self.player.x,
            y: self.player.y - 1,
            alive: true,
        });
    }

    fn move_bullets(&mut self) {
        // Move player bullets up
        for bullet in &mut self.player_bullets {
            if bullet.y > 0 && bullet.alive {
                bullet.y -= 1;
            } else {
                bullet.alive = false;
            }
        }

        // Move enemy bullets down
        for bullet in &mut self.enemy_bullets {
            if bullet.y < SCREEN_HEIGHT - 1 && bullet.alive {
                bullet.y += 1;
            } else {
                bullet.alive = false;
            }
        }

        // Check for collisions
        self.check_collisions();
    }

    fn enemy_shoot(&mut self) {
        let mut rng = rand::thread_rng();
        for enemy in &self.enemies {
            if enemy.alive && rng.gen_bool(0.02) {
                self.enemy_bullets.push(GameObject {
                    x: enemy.x,
                    y: enemy.y + 1,
                    alive: true,
                });
            }
        }
    }

    fn move_enemies(&mut self) {
        let mut move_down = false;
        let mut direction = 1;

        for enemy in &mut self.enemies {
            if enemy.alive {
                enemy.x = (enemy.x as i32 + direction).max(0).min(SCREEN_WIDTH as i32 - 1) as usize;
                
                // Change direction and move down when hitting screen edges
                if enemy.x == 0 || enemy.x == SCREEN_WIDTH - 1 {
                    move_down = true;
                    direction *= -1;
                }
            }
        }

        if move_down {
            for enemy in &mut self.enemies {
                if enemy.alive {
                    enemy.y += 1;
                    
                    // Game over if enemies reach bottom
                    if enemy.y >= SCREEN_HEIGHT - 2 {
                        self.game_over = true;
                    }
                }
            }
        }
    }

    fn check_collisions(&mut self) {
        // Player bullets hitting enemies
        for bullet in &mut self.player_bullets {
            if !bullet.alive { continue; }
            
            for enemy in &mut self.enemies {
                if enemy.alive && bullet.x == enemy.x && bullet.y == enemy.y {
                    bullet.alive = false;
                    enemy.alive = false;
                    self.score += 10;
                    break;
                }
            }
        }

        // Enemy bullets hitting player
        for bullet in &mut self.enemy_bullets {
            if !bullet.alive { continue; }
            
            if bullet.x == self.player.x && bullet.y == self.player.y {
                bullet.alive = false;
                self.player.alive = false;
                self.game_over = true;
                break;
            }
        }

        // Clean up dead objects
        self.player_bullets.retain(|b| b.alive);
        self.enemy_bullets.retain(|b| b.alive);
        self.enemies.retain(|e| e.alive);
    }

    fn render(&self) -> String {
        let mut screen = vec![vec![' '; SCREEN_WIDTH]; SCREEN_HEIGHT];

        // Draw player
        if self.player.alive {
            screen[self.player.y][self.player.x] = PLAYER_CHAR;
        }

        // Draw enemies
        for enemy in &self.enemies {
            if enemy.alive {
                screen[enemy.y][enemy.x] = ENEMY_CHAR;
            }
        }

        // Draw player bullets
        for bullet in &self.player_bullets {
            if bullet.alive {
                screen[bullet.y][bullet.x] = BULLET_CHAR;
            }
        }

        // Draw enemy bullets
        for bullet in &self.enemy_bullets {
            if bullet.alive {
                screen[bullet.y][bullet.x] = BULLET_CHAR;
            }
        }

        // Convert screen to string
        let mut output = String::new();
        for row in &screen {
            output.push_str(&row.iter().collect::<String>());
            output.push('\n');
        }
        
        // Add score
        output.push_str(&format!("Score: {}", self.score));
        
        output
    }
}

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All))?;

    let mut game = Game::new();
    let mut last_frame = Instant::now();
    let frame_duration = Duration::from_millis(100);

    while !game.game_over {
        // Handle input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => game.move_player(-1),
                    KeyCode::Right => game.move_player(1),
                    KeyCode::Char(' ') => game.shoot_bullet(),
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Game logic
        if last_frame.elapsed() >= frame_duration {
            game.move_bullets();
            game.move_enemies();
            game.enemy_shoot();
            last_frame = Instant::now();
        }

        // Render
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::CurrentLine)
        )?;
        
        print!("{}", game.render());
        stdout.flush()?;

        // Check game end conditions
        if game.enemies.is_empty() {
            println!("\nCongratulations! You won!");
            break;
        }

        // Slight pause to control game speed
        thread::sleep(Duration::from_millis(50));
    }

    // Clean up terminal
    terminal::disable_raw_mode()?;
    execute!(stdout, SetForegroundColor(Color::Reset))?;

    if game.game_over {
        println!("\nGame Over! Final Score: {}", game.score);
    }

    Ok(())
}
