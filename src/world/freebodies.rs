// Also free floating debris and ships
use hashbrown::HashMap;
use noise::{NoiseFn, Perlin};

#[path = "grid.rs"]
mod grid;

// use grid hashing in 3D to speed up detection of nearby targets?
// update voxel hashing only on moving freebodies

// Store entity in chunks, and freebodies like ships in fixedfreebody

pub struct FreeBody {
    id: u128,
    center_of_mass: [u32; 3],
    velocity: [f64; 3],
    position: [f64; 3],
    rotation: [f64; 3], // using Euler Angles
    mass: u32,
    chunk_grid: grid::ChunkGrid,
}

impl FreeBody {
    fn new(id: u128, center_of_mass: [u32; 3], velocity: [f64; 3], position: [f64; 3], rotation: [f64; 3], mass: u32, chunk_grid: grid::ChunkGrid) -> FreeBody {
        FreeBody { id: 0,
                   center_of_mass: center_of_mass, 
                   velocity: velocity, 
                   position: position,
                   rotation: rotation,
                   mass: mass,
                   chunk_grid: chunk_grid }
    }
    // splits off from other freebody or planets

    // user creates a freefloating freebody

    // shifting center of mass should move thet freebody

    // inertia -> spin
}

pub struct FixedFreeBody<> {
    velocity: [f64; 3],
    pub position: [f64; 3],
    rotation: [f64; 3],
    mass: u32, //calculate grav effect from mass
    atmosphere_radius: u8,
    chunk_grid: Vec<grid::ChunkGrid>, //6
    //children_entity: Vec<Entity>,
    //children_fixedfreebody: Vec<FixedFreeBody>,
    //children_freebody: Vec<FreeBody>,
    // uses circular grid
}

impl FixedFreeBody {
    pub fn new(position: [f64; 3], rotation: [f64; 3]) -> FixedFreeBody{
        FixedFreeBody {
            velocity: [0.0,0.0,3.0],
            position: position,
            rotation: rotation,
            mass: 2, 
            atmosphere_radius: 2,
            chunk_grid: Vec::new(),
            //children_entity: Vec::new(),
            //children_fixedfreebody: Vec::new(),
            //children_freebody: Vec::new(),
        }

        //planet.chunk_grid.push(grid::ChunkGrid::new());   

        //println!{"Sun Generated with Grid Size: {:?}", planet.chunk_grid.len()};
        //return planet;
    }

    pub fn new_sun(position: [f64; 3], rotation: [f64; 3]) -> FixedFreeBody{
        FixedFreeBody {
            velocity: [0.0,0.0,3.0],
            position: position,
            rotation: rotation,
            mass: 2, 
            atmosphere_radius: 2,
            chunk_grid: Vec::with_capacity(1),
            //children_entity: Vec::new(),
            //children_fixedfreebody: Vec::new(),
            //children_freebody: Vec::new(),
        }
    }

    pub fn new_planet(radius: u8) -> FixedFreeBody{
        let mut planet = FixedFreeBody {
            velocity: [0.0,0.0,3.0],
            position: [0.0,0.0,3.0],
            rotation: [0.0,0.0,3.0],
            mass: 2, 
            atmosphere_radius: 10, // in chunks
            chunk_grid: Vec::with_capacity(6),
            //children_entity: Vec::new(),
            //children_fixedfreebody: Vec::new(),
            //children_freebody: Vec::new(),
        };

        for x in 0..6 {
            planet.chunk_grid.push(grid::ChunkGrid::new());   
        }

        let perlin = Perlin::new();
        let val = perlin.get([420000.4, 35.7, 2.8]);
        //println!{"Planet Generated with Grid Size: {:?}", val};
        println!{"Planet Generated with Grid Size: {:?}", planet.chunk_grid.len()};
        return planet;
    }
}

// players, animals, monsters, etc.
pub struct Entity {
    x: u32,
    y: u32,
    z: u32,
}

impl Entity {
    fn new(x: u32, y: u32) -> Entity {
        Entity { x: x, y: y, z: 0 }
    }
}
