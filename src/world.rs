use hashbrown::HashMap;
use rand::Rng;
use ndarray::{arr1, arr2, arr3, Array1};
use std::thread;
use std::thread::sleep;
use std::sync::{mpsc, Arc, Mutex};

pub mod grid;
pub mod blocks;
pub mod freebodies;

pub struct Universe {
    // Have to divide into "sectors," if player is in a sector we stream the data to them
    // 1) Check if sector is in hashmap (memory)
    // 2) If not, check if sector is in storage and load if it is in storage and attempt to load nearby sectors into memory
    // 3) If not in storage generate new sector
    //
    // Fetching Sector and Moving Planet Problem
    //  If the planet orbits around a Star, it is always going to be changing sectors
    //  Sector has a flag if a planet passes through it?
    //  Sector has list of path of planet and planet path as function of time
    //  --> or global list of all planet positions as function of time????
    //  ----> PROBLEM: what happens when a entity is in the path of a planet??? what happens when
    //                  the sector loads and the entity is INSIDE the planet? --> Mark Problematic sectors have have special cases?
    // When sector loads in path of planet, run distance calculations
    // if freebody is inside planet, destroy
    // if close to planet, make child
    // 
    // Planets shouldnt be stored in secto_map, but on a star chart
    sector_map: HashMap<[u64; 3], Sector>,
    seed: u64,
    star_chart: Vec<Galaxy>,
    // Planet Chart?
    // calc stars realtime, calc planets as func of time
}

impl Universe{
    // Universe Tree Structure
    //
    //                            A (Universe)
    //                           / \
    //                 (Galaxy) B   C (Galaxy)
    //                ______________|___________
    //                |  |          |          |
    //          (Sun) D  E (Planet) F (Planet) G (Planet)
    //                     _________|_____
    //                     |  |          |      
    //   (Satellite/ Moon) H  I (Chunk)  J (Ships)
    //                     |   \
    //                     |
    //            _________|_____
    //
    //                     
    //                     |  |          |      
    //   (Satellite/ Moon) H  I (Chunk)  J (Ships)
    //             (entity) K         K (entity)
    //

    pub fn new() -> Universe {
        //let mut universe = 
        Universe {//freebodies: Vec::new(),
                  //fixedfreebodies: Vec::new()}
                  sector_map: HashMap::with_capacity(100),//::new(),
                  seed: 4,
                  star_chart: Vec::new()}
        //universe.sector_map.reserve(10);
        //return univers;
    }

    pub fn generate_universe(&mut self) {
        let thread_count: usize = 5;
        let (tx, rx) = mpsc::channel();

        println!("Starting {} Threads", thread_count);

        for i in 0..thread_count {
            let tx1 = tx.clone();
            thread::spawn(move || {
                let mut vals = Vec::new();
                for x in 0..1 { //0..10 galaxy count
                    let galaxy: Galaxy = Galaxy::new();
                    vals.push(galaxy);
                }
                tx1.send(vals).unwrap();
            });
        }

        // https://doc.rust-lang.org/std/sync/mpsc/index.html
        // Drop base tx so it is not waited on
        drop(tx); 

        for received in rx {
            for galaxy in received {
                self.star_chart.push(galaxy);
            }
        }

        for galaxy in self.star_chart.iter() {
            println!("");
            println!("Galaxy Coords: ");
            for solar_sys in galaxy.solar_system.iter() {
                print!("{:?},", &solar_sys.sun.position);
            }
            println!("");
            println!("--------------------------------------------");
        }

        println!("Threads Joined, Universe Generated");
    }

    pub fn generate_galaxy(&mut self) {
        self.star_chart.push(Galaxy::new());
    }

    /*pub fn generate_planet(&mut self, sector: [u64; 3]) -> usize {
        let planet: freebodies::FixedFreeBody = freebodies::FixedFreeBody::new_planet(5);
        //self.sector_map.get_mut().fixedfreebodies.push(planet);

        //self.get_sector(sector).add_fixedfreebodies(planet);
        
        //self.sector_map[&sector];
        //if let Some(x) = self.sector_map.get_mut(&sector) {
        //    *x = "b";
        //}

        match self.get_sector(sector) {
            Some(s) => s.add_fixedfreebodies(planet),
            None => println!("Error No Value"),
        }
        
        return 0;//self.fixedfreebodies.len() - 1;
    }*/

    fn get_sector(&mut self, sector: [u64; 3]) -> Option<&mut Sector> {
        if !self.sector_map.contains_key(&sector) {
            self.sector_map.insert(sector, Sector::new());
            println!("Created New Sector at {:?}", &sector);
        }
        return self.sector_map.get_mut(&sector);
    }
}

pub struct Galaxy {
    //center: []
    solar_system: Vec<SolarSystem>,
}

impl Galaxy {
    // https://arxiv.org/ftp/arxiv/papers/0908/0908.0892.pdf
    pub fn new() -> Galaxy {
        let mut rng = rand::thread_rng();
        let d: usize = 3; // float accuracy, distance between each planet
        let s: f64 = rng.gen_range(0.1..0.4);//scatter
        let a: f64 = rng.gen_range(1.0..6.0); // scale (make sure larger has same density of stars)
        let n: f64 = rng.gen_range(2.0..10.0); // spiral tail length 8.0
        //let b: f32 = 0.5;//rng.gen_range(0.05..5.0); // curvature
        let b: f64 = ((rng.gen_range(1.05..2.1) as f64).powf(3.0))-1.0; // curvature
        let arms: u8 = rng.gen_range(1..7);
        let arm_angle: f64 = 360.0/(arms as f64);
        let precision: f64 = 2.0;
        let precision_center: f64 = 100.0;
        let limit: u32 = ((270.0/((b.powf(1.5))+(4.0/n)))*precision) as u32; // 1.5 PI
        //let mut coordinates: Vec<[f32; 2]> = Vec::new();

        let mut coordinates: HashMap<String, [f64; 3]> = HashMap::new();
        let mut galaxy: Galaxy = Galaxy { solar_system: Vec::with_capacity(((limit as usize)*2)+360) };

        for angle in 0..limit { // USE THREADING, Parameter to generate multiple arms
            let theta = ((angle as f64)/precision)*(std::f64::consts::PI/180.0);
            let r = a/(( b * (( theta/(2.0*n) ).tan()) ).log10());
            let mut x = r * theta.cos();
            let mut y = r * theta.sin();
            //let dist = ((1.0/(r+0.00001)) * s * a * (1.0-((angle as f32)/(limit as f32)))) + 0.001;
            let dist = s * a * (1.0-((angle as f64)/(limit as f64)));

            for arm in 1..(arms+1) {
                let f_arm_ang = ((arm as f64) * arm_angle)*(std::f64::consts::PI/180.0);
                let xa = rng.gen_range((x-dist)..(x+dist));
                let ya = rng.gen_range((y-dist)..(y+dist));
                let xf = (xa * (f_arm_ang.cos())) - (ya * (f_arm_ang.sin())); // Or toss in matrix
                let yf = (xa * (f_arm_ang.sin())) + (ya * (f_arm_ang.cos()));
                //println!("({:?}, {:?}),", xf,yf);
                //coordinates.push([xf, yf]);
                let hash_key = format!("{:?},{:?}", convert(xf, d), convert(yf, d));

                if !coordinates.contains_key(&hash_key) {
                    coordinates.insert(hash_key, [xf, yf, 0.0]);
                }
            }
        }

        let h = a/(4.0*b); // 7.0, 2.5, 5 core size
        let w = (1.0 + ((b.powf(2.0))/n)) * h;
    
        // Using polar, then convert cartesian

        for angle in 0..360 { //.step_by(2)
            let theta = ((angle as f64))*(std::f64::consts::PI/180.0);
            let mut r = (w * h)/((h * theta.cos().powf(2.0)) + (w * theta.sin().powf(2.0))).sqrt();
            r = rng.gen_range(1.0..(r + 1.0)).powf(2.5);
            r -= 0.95;
            let mut x = r * theta.cos();
            let mut y = r * theta.sin();
            //println!("({:?}, {:?}),", x,y);
            //coordinates.push([x, y]);
            let hash_key = format!("{:?},{:?}", convert(x, d), convert(y, d));

            if !coordinates.contains_key(&hash_key) {
                coordinates.insert(hash_key, [x, y, 0.0]);
            }
        }

        for (key, val) in coordinates.iter() {
            //println!("key: {:?} val: {:?}", key, val);
            //println!("{:?}", val);
            galaxy.solar_system.push(SolarSystem::new(*val));
        }

        println!("-> Created Galaxy with: arms:{:?}, a:{:?}, n:{:?}, b:{:?}", arms, a, n, b); 

        return galaxy;
    }

    /*pub fn simulate_orbit(x: f32, y: f32) {
        let r = ((x.powf(2.0))+(y.powf(2.0))).sqrt(); // radius
        let v = 1.0-(r/100.0); // speed multipier
        let rot = 0.0174533 * v;

        let vector = arr1(&[x, y, 1.0]);

        let matrix = arr3(&[[rot.cos(), -rot.sin(), 0.0],
                            [rot.sin(), rot.cos(), 0.0],
                            [0.0, 0.0, 1.0]]);

        let new_matrix = matrix.dot(&vector);
        println!("{}", new_matrix);
    }*/
}

pub struct SolarSystem {
    sun: freebodies::FixedFreeBody,
    planets: Vec<Planets>,
}

impl SolarSystem {
    pub fn new(position: [f64; 3]) -> SolarSystem {
        SolarSystem { sun: freebodies::FixedFreeBody::new_sun(position, [0.0,0.0,0.0]), planets: Vec::new() }
    }
}

pub struct Planets {
    planet: freebodies::FixedFreeBody,
    satellites: Vec<freebodies::FixedFreeBody>,
}

impl Planets {
    pub fn new() -> Planets {
        Planets { planet: freebodies::FixedFreeBody::new([0.0,0.0,0.0], [0.0,0.0,0.0]), satellites: Vec::new() }
    }
}



pub struct Sector {
    freebodies: Vec<freebodies::FreeBody>,
    fixedfreebodies: Vec<freebodies::FixedFreeBody>, 
    // Maybe planets should be handled differently and not be stored in "sectors?"
    // Because planets move with a function of time
}

impl Sector {
    pub fn new() -> Sector {
        Sector { freebodies: Vec::new(), fixedfreebodies: Vec::new() }
    }

    pub fn add_freebodies(&mut self, freebody: freebodies::FreeBody) {
        self.freebodies.push(freebody);
    }

    pub fn add_fixedfreebodies(&mut self, fixedfreebodies: freebodies::FixedFreeBody) {
        self.fixedfreebodies.push(fixedfreebodies);
    }
}

fn convert(val: f64, precision: usize) -> String {
    format!("{:.prec$}", val, prec = precision)
}