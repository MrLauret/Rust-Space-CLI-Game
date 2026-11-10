pub mod models;
pub mod utils;
pub mod services;

use services::motor_mision::Motor;

fn main() {
    let mut motor = Motor::new();
    
    motor.start();
}
