//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

mod navbar;
pub use navbar::Navbar;

mod ua_flag;
pub use ua_flag::UkraineFlag;

mod github_logo;
pub use github_logo::GithubLogo;

mod crates_io_logo;
pub use crates_io_logo::CratesIoLogo;
