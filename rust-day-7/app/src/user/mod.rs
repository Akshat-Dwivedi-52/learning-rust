pub mod profile;
use crate::auth::login;
use utils::{generate_id, log};

pub fn create_user(name: &str) {
    log("Creating User....");

    let user_id = generate_id();
    println!("Assigned ID: {}", user_id);

    profile::show_profile(name);
    login(name);
}