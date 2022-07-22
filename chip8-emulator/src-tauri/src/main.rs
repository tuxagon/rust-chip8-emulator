#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chip8_core::*;
use std::sync::Mutex;

struct Chip8State {
    emulator: Mutex<Emulator>,
}

impl Chip8State {
    fn new() -> Chip8State {
        Self {
            emulator: Mutex::new(Emulator::new()),
        }
    }

    fn tick(&self) {
        self.emulator.lock().unwrap().tick();
    }

    fn tick_timers(&self) {
        self.emulator.lock().unwrap().tick_timers();
    }

    fn reset(&self) {
        self.emulator.lock().unwrap().reset();
    }

    pub fn load_game(&self, rom: Vec<u8>) {
        self.emulator.lock().unwrap().load(&rom);
    }

    pub fn draw_screen(&self) -> Vec<bool> {
        self.emulator.lock().unwrap().get_display().to_vec()
    }

    pub fn keypress(&self, key: &str, pressed: bool) {
        if let Some(k) = Self::key2btn(key) {
            self.emulator.lock().unwrap().keypress(k, pressed);
        }
    }

    fn key2btn(key: &str) -> Option<usize> {
        match key {
            "1" => Some(0x1),
            "2" => Some(0x2),
            "3" => Some(0x3),
            "4" => Some(0xC),
            "q" => Some(0x4),
            "w" => Some(0x5),
            "e" => Some(0x6),
            "r" => Some(0xD),
            "a" => Some(0x7),
            "s" => Some(0x8),
            "d" => Some(0x9),
            "f" => Some(0xE),
            "z" => Some(0xA),
            "x" => Some(0x0),
            "c" => Some(0xB),
            "v" => Some(0xF),
            _ => None,
        }
    }
}

#[tauri::command]
fn reset(state: tauri::State<Chip8State>) -> String {
    state.reset();

    format!("reset invoked")
}

#[tauri::command]
fn load_game(data: Vec<u8>, state: tauri::State<Chip8State>) -> String {
    println!("{:?}", data);
    state.load_game(data);

    format!("load_game invoked")
}

#[tauri::command]
fn tick(state: tauri::State<Chip8State>) -> String {
    state.tick();

    format!("tick invoked")
}

#[tauri::command]
fn tick_timers(state: tauri::State<Chip8State>) -> String {
    state.tick_timers();

    format!("tick_timers invoked")
}

#[tauri::command]
fn draw_screen(state: tauri::State<Chip8State>) -> Vec<bool> {
    state.draw_screen()
}

#[tauri::command]
fn keypress(key: &str, pressed: bool, state: tauri::State<Chip8State>) -> String {
    state.keypress(key, pressed);
    format!("keypress invoked")
}

fn main() {
    tauri::Builder::default()
        .manage(Chip8State::new())
        .invoke_handler(tauri::generate_handler![
            draw_screen,
            keypress,
            load_game,
            reset,
            tick,
            tick_timers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
