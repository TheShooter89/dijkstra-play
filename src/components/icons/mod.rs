//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.
mod base;
pub use base::*;

mod icons_path_d;
pub use icons_path_d::Icon;

mod ua_flag;
pub use ua_flag::UkraineFlag;

mod dijkstra_play_logo;
pub use dijkstra_play_logo::DijkstraLogo;
