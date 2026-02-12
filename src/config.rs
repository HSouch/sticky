
use serde_json;
use std::fs;
use std::io::Error;

use crate::static_potential::{BurkertPotential, LoadablePotential, NFWPotential, Potential, PotentialType};


pub struct SimSetup {
    pub verbose: bool,
    pub debug: bool, 

    pub potential: Box<dyn Potential>,
}

/// Load a JSON configuration file using serde.
pub fn load_config_file(filename: &String) -> Result<serde_json::Value, Error> {


    let contents = fs::read_to_string(filename)
        .expect("Filename not found {filename}");

    let json: serde_json::Value = serde_json::from_str(&contents).expect("Failure in opening file {filename}");
    
    Ok(json)
}

/// 
/// Load the simulation setup based on an input JSON comfig file.
/// 
pub fn load_params(filename: &String) -> Result<SimSetup, Error > {

    let json_data = load_config_file(&filename).unwrap();

    let potential_info = json_data["potential"].as_object().unwrap();
    let potential = load_potential(&potential_info);


    let simsetup = SimSetup {
        verbose: json_data["verbose"].as_bool().unwrap(),
        debug: json_data["debug"].as_bool().unwrap(),
        
        potential: potential,

    };

    Ok(simsetup)
}


pub fn load_potential(potential_info: &serde_json::Map<String, serde_json::Value>)
    -> Box<dyn Potential> {

    let name = potential_info["name"].as_str().unwrap();
    let potential_params = potential_info["parameters"].as_object().unwrap();


    let kind: PotentialType = match name {
        "NFW" | "NFWPotential" => PotentialType::NFWPotential,
        "Burkert" | "BurkertPotential" => PotentialType::BurkertPotential,
        _ => PotentialType::NFWPotential, 
    };

    match kind {
            PotentialType::NFWPotential => Box::new(NFWPotential::default()),
            PotentialType::BurkertPotential => Box::new(BurkertPotential::default()),
        }
    }




