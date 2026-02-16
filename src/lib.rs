//! A sticky n-body code written entirely in Rust.
//! 
//! Generally for use in [ram pressure stripping](https://en.wikipedia.org/wiki/Ram_pressure) implementations. 
//! 
//! The simulation has a number of components, but we apply the following prescription:
//! 
//! 1. A (number of) static gravitational potentials to simulate dark matter and unperturbed stellar disk.
//! 2. A set of particles, meant to serve as individual (spherical) gas clouds. Using this implementation we
//! can apply the Gunn and Gott condition to simulate ram pressure acceleration.
//! 3. A ram pressure shadow, allowing for clouds to deplete the ram pressure wind's effectiveness downstream.

pub mod config;
pub mod snapshot;

pub mod generators;
pub mod static_potential;
pub mod clouds;

pub mod core;
