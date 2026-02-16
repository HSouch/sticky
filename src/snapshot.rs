//! Code to handle reading and writing snapshot files to data. 
//! Since this n-body code is reasonably lightweight, it's acceptable to just use CSV files.

use crate::clouds::{Body, BodyArray};
use serde::{Serialize, Deserialize};

use csv::{Writer, Reader};
use std::{fs::File};

use lin_alg::f64::Vec3;


#[derive(Serialize, Deserialize)]
struct FlatBody {
    x: f64, 
    y: f64, 
    z: f64,
    vx: f64, 
    vy: f64, 
    vz: f64,
    mass: f64,
    metallicity: f64
}

impl FlatBody {
    fn from_body(body: &Body) -> Self {
        FlatBody {
            x: body.q.x, 
            y: body.q.y, 
            z: body.q.z, 
            vx: body.p.x, 
            vy: body.p.y, 
            vz: body.p.z, 
            mass: body.mass, 
            metallicity: body.metallicity 
        }
    }

    fn to_body(&self) -> Body {
        Body { q: Vec3 { x: self.x, y: self.y, z: self.z }, 
               p: Vec3 { x: self.vx, y: self.vy, z: self.vz },  
               mass: self.mass, 
               metallicity: self.metallicity }
    }
}


/// Contains functionality for reading and saving snapshots.
/// Takes in a BodyArray and associated metadata.
/// For now we will save to CSV files, but obviously this lacks good metadata saving
/// Ideally would like associated headers to be saved.
/// 
pub fn save_csv(bodyarray: &BodyArray, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;

    let mut writer = Writer::from_writer(file);

    for i in 0..bodyarray.bodies.len() {
        let row = FlatBody::from_body(&bodyarray.bodies[i]);

        writer.serialize(row)?;
    }

    Ok(())

}


pub fn from_csv(path: &str) -> Result<BodyArray, Box<dyn std::error::Error>> {

    let mut reader = Reader::from_path(path)?;

    let bodies = reader
        .deserialize::<FlatBody>()
        .map(|row| Ok(row?.to_body()))
        .collect::<Result<Vec<_>, csv::Error>>()?;


    Ok(BodyArray{bodies: bodies})
}
