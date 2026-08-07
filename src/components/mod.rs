//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::{Hero, HeroContent};

mod app;
pub use app::App;

mod echo;
pub use echo::Echo;

mod navbar;
pub use navbar::Navbar;

mod footer;
pub use footer::{Footer, FooterAside, FooterContactsNav};

pub mod home;

pub mod controls;

pub mod icons;

pub mod editor;
