
use barnes_hut;
use lin_alg::f64::Vec3;
use rayon::prelude::*;
use crate::{clouds::{BodyArray, Body},
            config::{SimSetup},
};
use std::error::Error;


pub fn run_simulation(bodyarray: &BodyArray, config: &SimSetup) 
    -> Result<(), Box<dyn Error>> 
{
    
    run_timesteps(bodyarray);

    
    Ok(())
}


pub fn run_timesteps(bodies: &BodyArray) {
    let config = barnes_hut::BhConfig::default();

    let bounding_box = barnes_hut::Cube::new(Vec3::new_zero(),  40.0);
    
    let tree = barnes_hut::Tree::new(&bodies.bodies, &bounding_box, &config);
    
    

}