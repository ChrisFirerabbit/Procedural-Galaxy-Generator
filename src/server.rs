#![allow(unused_imports)]
#![allow(dead_code)]
//export RUSTFLAGS='-Ctarget-cpu=native'
extern crate noise;
extern crate approx; 
extern crate nalgebra as na;
extern crate hashbrown;
extern crate rand;
extern crate ndarray;
use na::{Vector3, Rotation3};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::thread::sleep;
use std::sync::{mpsc, Arc, Mutex};

mod world;
mod actions;
mod keep_mathematizing;
mod physics;

// https://tokio.rs/tokio/tutorial
// 1 thread, 1 area/ player? idk
// 1 thread per chunk? idk
static BLOCK_PHYSICS_TICK_PER_MS:u128 = 200; // tick every n milliseconds -> (1000/ticks_per_second)
static FLUID_PHYSICS_TICK_PER_MS:u128 = 200; // tick every n milliseconds
static FREEBODY_PHYSICS_TICK_PER_MS:u128 = 200; // tick every n milliseconds
static OUT_OF_BOUNDS_PHYSICS_TICK_PER_MS: u128 = 2000; // ms

fn main() {
    let mut universe: world::Universe = world::Universe::new();
    let mut tick_counter: [u128; 4] = [get_epoch_ms(); 4];
    let mut now = SystemTime::now();

    universe.generate_universe();
    println!("{:?}", now.elapsed());
    
    //now = SystemTime::now();
    //universe.generate_universe_fallback();
    //println!("---------------------------------------------------{:?}", now.elapsed());

    /*std::thread::sleep(std::time::Duration::from_millis(1000));
    loop {
        now = SystemTime::now();
        let epoch: u128 = get_epoch_ms();

        if epoch > tick_counter[0] + BLOCK_PHYSICS_TICK_PER_MS {
            tick_counter[0] = epoch; physics::run_physics();}
        if epoch > tick_counter[1] + FLUID_PHYSICS_TICK_PER_MS {
            tick_counter[1] = epoch; physics::run_physics();}
        if epoch > tick_counter[2] + FREEBODY_PHYSICS_TICK_PER_MS {
            tick_counter[2] = epoch; physics::run_physics();}
        if epoch > tick_counter[3] + OUT_OF_BOUNDS_PHYSICS_TICK_PER_MS {
            tick_counter[3] = epoch; physics::run_physics();}

        println!("----------TEST--------------");
        now = SystemTime::now();
        
        universe.generate_galaxy();
        //universe.generate_planet([9,9,9]);

        println!("{:?}", now.elapsed());
        now = SystemTime::now();
        println!("----------END_--------------");
        //let five_seconds = Duration::new(5, 0);
        // both declarations are equivalent
        //assert_eq!(Duration::new(5, 0), Duration::from_secs(5));
        //println!("seconds")

        std::thread::sleep(std::time::Duration::from_millis(1000));
    }*/
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn gen_test() {
    let mut universe: world::Universe = world::Universe::new();
    let mut now = SystemTime::now();
    let mut elapsed = now.elapsed();
    const PLANETS_GEN: usize = 1;
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    use std::collections::HashMap;

    println!(
        "{:?}",
        (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()
    );

    let x = [(); std::usize::MAX];
    println!("Hello, world! {}", x.len());

    println!("----------TEST--------------");
    now = SystemTime::now();
    /*for x in 0..PLANETS_GEN {
        println!("Index of Planet: {:?}", universe.generate_planet([9,9,9]));
    }*/
    //elapsed = now.elapsed();
    //assert!(3 < elapsed.unwrap().as_millis(), "we are testing addition with {:?} and {:?}", 3, elapsed);
    println!("{:?} Planets Generated in {:?}", PLANETS_GEN, now.elapsed());
    now = SystemTime::now();
    println!("----");
    //thread_examples();
    println!("----");
    println!("----------END_--------------");
    /*
    let y = if 12 * 15 > 150 {
        "Bigger"
    } else {
        "Smaller"
    };
    assert_eq!(y, "Bigger");
    */
}



// if vertex overlaps delete them/ combine