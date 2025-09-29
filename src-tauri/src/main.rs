#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use std::sync::Mutex;
use tauri::State;

// État partagé : garde le nombre secret
struct GameState {
    secret: Mutex<u32>,
}

// Commande appelée depuis le frontend
#[tauri::command]
fn check_guess(state: State<GameState>, guess: u32) -> String {
    let secret = *state.secret.lock().expect("mutex poisoned");
    if guess < secret {
        "Plus grand !".to_string()
    } else if guess > secret {
        "Plus petit !".to_string()
    } else {
        "Bravo, c'était le bon nombre !".to_string()
    }
}

fn main() {
    tauri::Builder::default()
        .manage(GameState {
            secret: Mutex::new(rand::rng().random_range(1..=100)),
        })
        .invoke_handler(tauri::generate_handler![check_guess])
        .run(tauri::generate_context!())
        .expect("Erreur lors de l'exécution de l'application Tauri");
}
