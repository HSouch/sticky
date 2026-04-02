use super::{Potential, LoadablePotential, Vec3};


pub struct MiyamotoNagaiPotential{
    pub mass: f64,

    pub a: f64,
    pub b: f64,
}


impl Potential for MiyamotoNagaiPotential {
    fn name(&self) -> String {"Miyamoto Nagai Potential".to_string()}

    fn acceleration(&self, _q: &Vec3, _t: f64) -> Vec3 {
        Vec3::new_zero()
    }


    fn pot(&self, _q: &Vec3, _t: f64) -> f64 {
        0.0
    }

    
}


impl LoadablePotential for MiyamotoNagaiPotential {

    fn default() -> Self {
        MiyamotoNagaiPotential { mass: 1e10, a: 2.5, b: 0.5 } 
    }

}

