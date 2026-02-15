use crate::clouds::Body;

use lin_alg::f64::Vec3;
use serde_json::{Map, Value};

use std::fmt::Error;
use std::f64::consts::PI;

use rand::{self, RngExt};


pub struct GeneratorConfig {
    pub config_obj: Map<String, Value>,
}

pub fn debug_generator(_params: &GeneratorConfig) -> Result<Body, Error> {
    Ok(Body {
        q: Vec3 {x: 0.0, y: 0.0, z: 0.0},
        p: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        mass: 1e6,
        metallicity: 1e-2,
    })
}


pub fn exponential_disk_generator(params: &GeneratorConfig) -> Result<Body, Error> {

    let seed_parameters = params.config_obj["seed_parameters"].as_object().unwrap();

    let scale_radius = seed_parameters["scale_radius"].as_f64().unwrap();
    let scale_height = seed_parameters["scale_height"].as_f64().unwrap();

    let max_tries = 2000;

    let n0: f64 = 1.0 / (4.0 * PI * scale_height * scale_radius * scale_height);

    let mut r = 0.0;
    let mut z = 0.0;

    let mut rng = rand::rng();
    let mut success = false;
    // Run Monte Carlo to get suitable R and z values given the exponential distribution
    
    for _ in 0..max_tries {
        r = rand::random_range(0.0 .. 3.0 * scale_radius);
        z = rand::random_range(-2.0 * scale_height .. 2.0 * scale_height);

        let p = n0 * (-r / scale_radius).exp() * (-z.abs() / scale_height).exp();

        if p >= rng.random() { 
            success = true; 
            break 
        };
    }

    if !success {
        return Err(Error);
    }

    // Now that we have R and z, build theta
    let theta = rand::random_range(0.0 .. 2.0 * PI);

    let q = Vec3 {
        x: r * theta.cos(),
        y: r * theta.sin(),
        z: z,
    };

    let p = Vec3::new_zero();

    Ok(Body{
        q: q, 
        p: p,
        mass: 1e6,
        metallicity: 0.02,
    })
}


