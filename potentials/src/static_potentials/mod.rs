//!Functionality for computing accelerations under static gravitational potential.
//! We adopt a similar paradigm to [Galpy](https://www.galpy.org/) using a cylindrical
//! coordinate system (generally).
//! All potentials adopt GALAXY units [kpc, Myr, Msun] as their units 
//! 

use lin_alg::f64::{Vec3, } ;

use std::f64::consts::PI;
use serde_json::{Map, Value};


// ----------------------------------------------
// Constants in galactic units ------------------
// ----------------------------------------------
static H0_GAL: f64 = 6.9e-5;                    // Myr^-1
static G_GALAXY: f64 = 4.498502151469554E-12;   // kpc^3 Msun^-1 Myr^-2
static RHO_CRIT: f64 = 3.0 * H0_GAL / (8.0 * PI * G_GALAXY);                   // Msun / kpc ^3


/// Primary trait for all potentials.
pub trait Potential{
    fn name(&self) -> String;

    fn print_name(&self)  {
        println!("Potential: {}", self.name());
    }

    fn printout(&self) {
        Self::print_name(&self);
    }
    
    fn acceleration(&self, q: &Vec3, t: f64) -> Vec3;

    fn pot(&self, q: &Vec3, t: f64) -> f64;

    fn gradient(&self, _q: &Vec3, _t: f64) -> Vec3 {
        Vec3::new_zero()
    }
    

}

pub trait LoadablePotential: Potential + Sized {
    fn default() -> Self;

    fn load(_params: &Map<String, Value>) -> Self 
        where Self: Sized
        {
            LoadablePotential::default()
        }
}


pub mod nfw;
pub mod burkert;
pub mod miyamoto_nagai;


pub use nfw::NFWPotential;
pub use burkert::BurkertPotential;
pub use miyamoto_nagai::MiyamotoNagaiPotential;


#[derive(Debug)]
pub enum PotentialType {
    NFWPotential,
    BurkertPotential,
    MiyamotoNagaiPotential,
}

