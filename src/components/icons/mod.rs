//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod ua_flag;
pub use ua_flag::UkraineFlag;

mod dijkstra_play_logo;
pub use dijkstra_play_logo::DijkstraLogo;

mod github_logo;
pub use github_logo::GithubLogo;

mod crates_io_logo;
pub use crates_io_logo::CratesIoLogo;

mod instagram_logo;
pub use instagram_logo::InstagramLogo;

mod linkedin_logo;
pub use linkedin_logo::LinkedinLogo;

mod circle_play;
pub use circle_play::CirclePlay;

mod filepage;
pub use filepage::FilePage;
