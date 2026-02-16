
use lin_alg::f64::{Vec3, } ;

use std::fmt::Error;
use std::f64::consts::PI;
use serde_json::{Map, Value};

static G_GALAXY: f64 = 4.498502151469554E-12;   // kpc^3 Msun^-1 Myr^-2


///
/// All potentials adopt GALAXY units [kpc, Myr, Msun] as their units 
/// 


pub mod utils {
    use super::*;

    pub fn r200(m200: &f64, h0: &f64) -> f64 {
        ((G_GALAXY * m200) / (100.0 * h0 * h0)).cbrt()
    }


}


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

}

pub trait LoadablePotential: Potential + Sized {
    fn default() -> Self;

    fn load(&self, params: &Map<String, Value>) -> Self 
        where Self: Sized
        {
            LoadablePotential::default()
        }
}

#[derive(Debug)]
pub enum PotentialType {
    NFWPotential,
    BurkertPotential,
}


//////////////////////////////////////////////////////////////////////////////////////////
/////////////////////// Below are custom potentials. /////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////


pub struct NFWPotential {
    /// A (spherical) NFW profile
    pub rho_0: f64,
    pub r_s: f64,

}


impl NFWPotential {
    pub fn from_m200c (mass: f64, c: f64) -> Self {
        let r200 = utils::r200(&mass, &70.0);
        
        println!("mass {} c {}", mass, c);
        dbg!("r200 {}", r200);

        let r_s = r200 / c;

        let rho_0 = mass * c.powi(3) / (4.0 * PI * r200.powi(3) * ((1.0 + c).ln() - c / (1.0 + c)));

        NFWPotential { rho_0, r_s }

    }
}


impl Potential for NFWPotential {

    fn name(&self) -> String {"NFW Potential".to_string()}

    fn acceleration(&self, q: &Vec3, _t: f64) -> Vec3 {
        let r_hat: Vec3 = q.to_normalized();

        let magnitude = 500.0;

        Vec3 { x: magnitude * r_hat.x, y: magnitude * r_hat.y, z: magnitude * r_hat.z }      
    }

    fn printout(&self) {
        println!("{} with central density of {:2e} Msun and scale radius of {}", 
            self.name(), self.rho_0, self.r_s);
    }

}

impl LoadablePotential for NFWPotential {

    fn default() -> Self {
        NFWPotential { rho_0: 1.0e-26, r_s: 8.0 }
    }

}


pub struct BurkertPotential{
    pub mass: f64,
    pub r0: f64,
}


impl Potential for BurkertPotential {
    fn name(&self) -> String {"BurkertPotential".to_string()}

    fn acceleration(&self, q: &Vec3, t: f64) -> Vec3 {
        let r_hat: Vec3 = q.to_normalized();
        
        let magnitude = 500.0;

        Vec3 { x: magnitude * r_hat.x, y: magnitude * r_hat.y, z: magnitude * r_hat.z }
    }

}

impl LoadablePotential for BurkertPotential {

    fn default() -> Self {
        BurkertPotential { mass: 1.0e12, r0: 8.0 }
    }

}

