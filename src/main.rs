#![allow(warnings)]
use bevy::prelude::*;
use wash_cycle::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
