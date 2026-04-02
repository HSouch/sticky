use super::{Potential, LoadablePotential, G_GALAXY, PI, Vec3};


pub struct BurkertPotential{
    pub mass: f64,
    pub r0: f64,
}


impl Potential for BurkertPotential {
    fn name(&self) -> String {"Burkert Potential".to_string()}

    fn acceleration(&self, q: &Vec3, _t: f64) -> Vec3 {
        let r = q.magnitude();
        
        // Do a check on r to make sure it's not 0
        if r < 1e-6 { return Vec3::new_zero(); }

        let rhat = *q / r;
        let x = r / self.r0;
        let c = (1.0 + x).ln() - x / (1.0 + x);

        rhat * - 4.0 * PI * G_GALAXY * self.mass * self.r0.powi(3) / r.powi(2) * c

    }

    fn pot(&self, _q: &Vec3, _t: f64) -> f64 {
        0.0
    }

    
}

impl LoadablePotential for BurkertPotential {

    fn default() -> Self {
        BurkertPotential { mass: 1.0e12, r0: 8.0 }
    }

}