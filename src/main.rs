use std::collections::VecDeque;

use raylib::consts::KeyboardKey::*;
use raylib::core::texture::Image;
use raylib::prelude::*;

const TILE_SIZE: f32 = 16.0;
const MAP_SIZE: f32 = 50.0;

const SCREEN_WIDTH: f32 = MAP_SIZE * TILE_SIZE;
const SCREEN_HEIGHT: f32 = MAP_SIZE * TILE_SIZE;

const GAME_SPEED: u32 = 5;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

struct Snake {
    body: VecDeque<Vector2>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    pub fn new() -> Snake {
        let mut instance = Snake {
            body: VecDeque::new(),
            direction: Direction::RIGHT,
            next_direction: Direction::RIGHT,
        };
        instance.body.push_back(Vector2::new(4.0, MAP_SIZE * 0.5));
        instance.body.push_back(Vector2::new(3.0, MAP_SIZE * 0.5));
        instance.body.push_back(Vector2::new(2.0, MAP_SIZE * 0.5));

        instance
    }

    pub fn update(&mut self) {
        if self.direction != self.next_direction {
            match self.direction {
                Direction::RIGHT if self.next_direction != Direction::LEFT => {
                    self.direction = self.next_direction
                }
                Direction::DOWN if self.next_direction != Direction::UP => {
                    self.direction = self.next_direction
                }
                Direction::LEFT if self.next_direction != Direction::RIGHT => {
                    self.direction = self.next_direction
                }
                Direction::UP if self.next_direction != Direction::DOWN => {
                    self.direction = self.next_direction
                }
                _ => {}
            }
        }

        let mut new_head = self.body.front().unwrap().clone();

        match self.direction {
            Direction::RIGHT => new_head.x += 1.0,
            Direction::DOWN => new_head.y += 1.0,
            Direction::LEFT => new_head.x -= 1.0,
            Direction::UP => new_head.y -= 1.0,
        }

        if new_head.x >= MAP_SIZE {
            new_head.x = 0.0;
        }
        if new_head.x < 0.0 {
            new_head.x = MAP_SIZE - 1.0;
        }
        if new_head.y >= MAP_SIZE {
            new_head.y = 0.0;
        }
        if new_head.y < 0.0 {
            new_head.y = MAP_SIZE - 1.0;
        }

        self.body.push_front(new_head);
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        let tail = *self.body.back().unwrap();
        self.body.push_back(tail);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for (i, tile) in self.body.iter().enumerate() {
            let color = if i == 0 { Color::DARKGREEN } else { Color::GREEN };
            d.draw_rectangle_rec(
                Rectangle {
                    x: tile.x * TILE_SIZE,
                    y: tile.y * TILE_SIZE,
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                },
                color,
            );
        }
    }
}

struct Food {
    position: Vector2,
}

impl Food {
    pub fn new() -> Food {
        Food {
            position: Vector2::new(
                get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
                get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
            ),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rec(
            Rectangle {
                x: self.position.x * TILE_SIZE,
                y: self.position.y * TILE_SIZE,
                width: TILE_SIZE,
                height: TILE_SIZE,
            },
            Color::RED,
        )
    }

    pub fn respawn(&mut self) {
        self.position = Vector2::new(
            get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
            get_random_value::<i32>(0, (MAP_SIZE as i32) - 1) as f32,
        )
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Snake Game")
        .vsync()
        .build();

    let icon_image = Image::load_image("icon/icon.png").expect("Failed to load icon image");
    rl.set_window_icon(&icon_image);

    rl.set_target_fps(60);

    let mut snake = Snake::new();
    let mut food = Food::new();
    let mut frame_count = 0;

    while !rl.window_should_close() {
        // Update
        if rl.is_key_pressed(KEY_RIGHT) && snake.direction != Direction::LEFT {
            snake.next_direction = Direction::RIGHT;
        }
        if rl.is_key_pressed(KEY_DOWN) && snake.direction != Direction::UP {
            snake.next_direction = Direction::DOWN;
        }
        if rl.is_key_pressed(KEY_LEFT) && snake.direction != Direction::RIGHT {
            snake.next_direction = Direction::LEFT;
        }
        if rl.is_key_pressed(KEY_UP) && snake.direction != Direction::DOWN {
            snake.next_direction = Direction::UP;
        }

        if frame_count % GAME_SPEED == 0 {
            snake.update();
        }

        if snake.body.iter().skip(1).any(|&tile| tile == snake.body[0]) {
            return;
        }

        if snake.body[0] == food.position {
            food.respawn();
            snake.grow();
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        snake.draw(&mut d);
        food.draw(&mut d);

        frame_count += 1;
    }
}