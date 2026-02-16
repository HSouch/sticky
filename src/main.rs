
mod clouds;
mod core;
mod config;
mod generators;
mod snapshot;
mod static_potential;
use std::env;

use serde_json::Value;

use crate::{clouds::BodyArray, config::SimSetup};


fn initialize_bodyarray(sim_params: &SimSetup) -> BodyArray {
    let generator_params = sim_params.params["particles"].as_object().unwrap().clone();
    let generator_config = generators::GeneratorConfig{config_obj: generator_params};

    let bodyarray = clouds::BodyArray::seed(
        generators::exponential_disk_generator, &generator_config).unwrap();
    bodyarray
}


fn main()  {
    let args : Vec<String> = env::args().collect();    
    assert!(args.len() > 1, "Need to specify a config file!");
    
    let sim_params = config::load_params(&args[1]).unwrap();
    
    if sim_params.verbose {
        println!("Running Sticky NBody Stuff (in Rust!)");
        if sim_params.debug {
            println!("(Running in Debug Mode)")
        }
    }

    let snapshot = sim_params.params["snapshot"].clone();

    let bodyarray: BodyArray = match snapshot {
        Value::Null => {
            println!("No snapshot specified. Initializing array based on user parameters.");
            initialize_bodyarray(&sim_params)
        },  
        Value::String(s) => {
            println!("Snapshot specified: [{}]", s);
            snapshot::from_csv(&s.as_str()).unwrap()
        },
        _ => panic!("Expected Null or String for snapshot."),
    };


    bodyarray.printout();

}
