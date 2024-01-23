#[cfg(not(target_arch = "wasm32"))]
mod desktop;

#[cfg(target_arch = "wasm32")]
mod web;

pub mod game_view {
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::desktop::game_view::GameView;

    #[cfg(target_arch = "wasm32")]
    pub use super::web::game_view::GameView;
}

pub mod game_view_trait;




