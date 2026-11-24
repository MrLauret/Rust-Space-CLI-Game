pub mod models;
pub mod utils;
pub mod services;

use services::game_state::GameState;

fn main() {
    let mut game = GameState::new();
    
    game.start();
}
