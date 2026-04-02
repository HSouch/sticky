use super::{Potential, LoadablePotential, G_GALAXY, RHO_CRIT, PI, Vec3};


pub struct NFWPotential {
    /// A (spherical) NFW profile
    pub m200: f64,      
    pub r200: f64,
    pub c: f64,
}


impl NFWPotential {
    fn default() -> Self {
        NFWPotential { m200: 1.0, r200: 1.0, c: 6.0}
    }

    fn new() -> Self {
        Self::default()
    }


    fn from_m200_c(m200: f64, c: f64) -> Self {
        let r200: f64 = (3.0 * m200 / (800.0 * PI * RHO_CRIT)).cbrt();                
        NFWPotential { m200: m200, r200: r200,  c: c}

    }


    fn rs(&self) -> f64 {
        self.r200 / self.c
    }


    fn a_nfw(&self) -> f64 {
        (1.0 + self.c).ln() - (self.c / (1.0 + self.c)).ln()
    }

    fn rho_s(&self) -> f64 {
        self.m200 / (4.0 * PI * self.rs().powi(3) * self.a_nfw())
    }

}


impl Potential for NFWPotential {

    fn name(&self) -> String {"NFW Potential".to_string()}


    fn acceleration(&self, q: &Vec3, _t: f64) -> Vec3 {
        
        let r = q.magnitude();
        
        // Do a check on r to make sure it's not 0
        if r < 1e-8 { return Vec3::new_zero(); }
        
        let x = r / self.rs();        
        let x_rhs = (1.0 + x).ln() - (x / (1.0 + x));

        -(*q) * G_GALAXY * self.m200 / self.a_nfw() * x_rhs / r.powi(3)

    }

    fn pot(&self, q: &Vec3, _t: f64) -> f64 {
        let r = q.magnitude();

        - G_GALAXY * self.m200 / r * (1.0 + r / self.rs()).ln()
        
    }


    fn printout(&self) {
        println!("{} with M200 of {} Msun and Virial radius of {}", 
            self.name(), self.m200, self.r200);
    }


}

impl LoadablePotential for NFWPotential {

    fn default() -> Self {
        NFWPotential::from_m200_c(1.0e12, 6.0)
    }

}