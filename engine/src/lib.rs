//! Biomes game engine — Rust core compiled to WASM.
//!
//! Domain types live in [`biome`], [`grid`] and [`rng`]. The WASM-facing
//! API (grid creation, snapshots, cell lookup) lives in [`wasm_api`].

pub mod biome;
pub mod grid;
pub mod rng;

#[cfg(target_arch = "wasm32")]
mod wasm_api;
