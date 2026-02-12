
use lin_alg::f64::Vec3;
use barnes_hut::BodyModel;


pub struct Body {
    q: Vec3,
    p: Vec3,
    mass: f64,
    metallicity: f64,
}


/// This implementation is to ensure compliance with the Barnes Hut Tree algorithm.
impl BodyModel for Body {
    fn posit(&self) -> Vec3 {
        self.q    
    }
    
    fn mass(&self) -> f64 {
        self.mass
    }
}
