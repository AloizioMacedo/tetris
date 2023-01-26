use egui::{Color32, Pos2, Rect, Stroke};
use tetris::constants::{Movement, FPS, HEIGHT, SCALE, WIDTH};
use tetris::game::{new_game, Game};

use std::time::{Duration, Instant};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(WIDTH as f32, HEIGHT as f32)),
        ..Default::default()
    };
    eframe::run_native("Snake", options, Box::new(|_cc| Box::new(MyApp::default())))
}

struct MyApp {
    game: Game,
    time: Instant,
    current_move_command: Option<Movement>,
    is_paused: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: new_game(),
            time: Instant::now(),
            current_move_command: None,
            is_paused: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let events = ui.input().events.clone();

            for event in &events {
                self.current_move_command = match event {
                    egui::Event::Key {
                        key,
                        pressed,
                        modifiers: _,
                    } => self.get_movement(pressed, key),
                    _ => self.current_move_command,
                }
            }

            paint_rectangle(ui);

            if self.is_paused {
                ui.heading("PAUSED");
                return;
            }

            self.paint_pieces(ui);

            let time_now = Instant::now();
            let delta_t = time_now.duration_since(self.time);

            if delta_t >= Duration::from_millis((1000. * (1. / FPS)) as u64) {
                self.game.step(self.current_move_command);
                self.time = time_now;
                self.current_move_command = None;
            }
        });

        ctx.request_repaint();
    }
}

fn paint_rectangle(ui: &mut egui::Ui) {
    ui.painter().rect_stroke(
        Rect {
            min: Pos2 { x: 0., y: 0. },
            max: Pos2 {
                x: WIDTH as f32,
                y: HEIGHT as f32,
            },
        },
        0.,
        Stroke {
            width: 1.,
            color: Color32::GRAY,
        },
    );
}

impl MyApp {
    fn get_movement(&self, pressed: &bool, key: &egui::Key) -> Option<Movement> {
        if *pressed {
            match key {
                egui::Key::ArrowUp => Some(Movement::UP),
                egui::Key::ArrowDown => Some(Movement::DOWN),
                egui::Key::ArrowLeft => Some(Movement::LEFT),
                egui::Key::ArrowRight => Some(Movement::RIGHT),
                _ => self.current_move_command,
            }
        } else {
            self.current_move_command
        }
    }

    fn paint_pieces(&mut self, ui: &mut egui::Ui) {
        for piece in self.game.list_pieces() {
            for coord in piece.coords {
                ui.painter().rect_filled(
                    Rect {
                        min: Pos2 {
                            x: coord[0] as f32 - SCALE as f32 / 2.,
                            y: coord[1] as f32 - SCALE as f32 / 2.,
                        },
                        max: Pos2 {
                            x: coord[0] as f32 + SCALE as f32 / 2.,
                            y: coord[1] as f32 + SCALE as f32 / 2.,
                        },
                    },
                    SCALE as f32 / 5.,
                    piece.color,
                )
            }
        }
    }
}
