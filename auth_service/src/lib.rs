#![allow(dead_code, unused_variables)]

mod database;

pub mod auth_utils;

pub mod util;

pub fn authenticate(cred: auth_utils::models::Credentials) {
    if let database::Status::Connected = database::connect_to_database() {
        auth_utils::login(cred);
    }
}

// src/util.rs (more preferred)
// src/util/mod.rs
