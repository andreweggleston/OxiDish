/// Defines the arguments required to start the server application using [`clap`].
///
/// [`clap`]: https://github.com/clap-rs/clap/
pub mod config;

/// Contains the setup code for the API build with Axum.
///
/// The Realworld API routes exist in child modules of this.
pub mod http;

/// Contains structs that map to database tables.
pub mod models;