
mod config;
mod static_potential;
use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();    
    assert!(args.len() > 1, "Need to specify a config file!");
    
    let sim_params = config::load_params(&args[1]).unwrap();

    if sim_params.verbose {
        println!("Running Sticky NBody Stuff (in Rust!)");
    }


    // sim_params.potential.printout();


    
    
}
