use std::vec;

use macroquad::prelude::*;

#[derive(PartialEq)]
struct Bird {
    pos: Vec2,
    dir: f32,
}
const VELOCITY: f32 = 1.0;
const LOCAL_BIRD_RADIUS: f32 = 100.0;
const TURN_RATE: f32 = 0.05;

fn draw_bird(bird: &Bird) {
    draw_rectangle(bird.pos.x, bird.pos.y, 30.0, 30.0, GREEN);
}

#[macroquad::main("Boids")]
async fn main() {
    let b1 = Bird {
        pos: Vec2 {
            x: screen_width() / 2.0,
            y: screen_height() / 2.0 + 50.0,
        },
        dir: 79.0f32.to_radians(),
    };
    let b2 = Bird {
        pos: Vec2 {
            x: screen_width() / 2.0 + 50.0,
            y: screen_height() / 2.0 - 50.0,
        },
        dir: 0.0f32.to_radians(),
    };
    let b3 = Bird {
        pos: Vec2 {
            x: screen_width() / 2.0 - 50.0,
            y: screen_height() / 2.0,
        },
        dir: 160.0f32.to_radians(),
    };
    let mut birds = vec![b1, b2, b3];

    let center_of_mass = |local_birds: &[&Bird]| -> Vec2 {
        if local_birds.is_empty() {
            return Vec2::ZERO;
        }

        let sum = local_birds.iter().fold(Vec2::ZERO, |acc, bird| Vec2 {
            x: acc.x + bird.pos.x,
            y: acc.y + bird.pos.y,
        });

        Vec2 {
            x: sum.x / local_birds.len() as f32,
            y: sum.y / local_birds.len() as f32,
        }
    };

    loop {
        clear_background(BLACK);
        let updates: Vec<_> = birds
            .iter()
            .map(|bird| {
                let dy = VELOCITY * f32::sin(bird.dir);
                let dx = VELOCITY * f32::cos(bird.dir);
                let new_pos = Vec2 {
                    x: bird.pos.x + dx,
                    y: bird.pos.y + dy,
                };

                let mut local_birds = vec![];
                for i_bird in &birds {
                    if bird == i_bird {
                        continue;
                    }
                    let dx = i_bird.pos.x - bird.pos.x;
                    let dy = i_bird.pos.y - bird.pos.y;
                    let distance = f32::sqrt(dx * dx + dy * dy);

                    if distance * distance < LOCAL_BIRD_RADIUS * LOCAL_BIRD_RADIUS {
                        local_birds.push(i_bird);
                    }
                }

                let center = center_of_mass(&local_birds);

                // bird -> center
                let to_center = Vec2 {
                    x: center.x - bird.pos.x,
                    y: center.y - bird.pos.y,
                };

                // angle -> center
                let angle_to_center = f32::atan2(to_center.y, to_center.x);

                // current heading vs center heading normalize
                let mut angle_diff = angle_to_center - bird.dir;
                while angle_diff > std::f32::consts::PI {
                    angle_diff -= 2.0 * std::f32::consts::PI;
                }
                while angle_diff < -std::f32::consts::PI {
                    angle_diff += 2.0 * std::f32::consts::PI;
                }

                let new_dir = angle_diff * TURN_RATE;

                (new_pos, new_dir)
            })
            .collect();

        for (bird, (new_pos, new_dir)) in birds.iter_mut().zip(updates) {
            bird.pos = new_pos;
            bird.dir = new_dir;
        }

        for bird in &birds {
            draw_bird(bird);
        }
        next_frame().await
    }
}
