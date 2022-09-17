struct Position3 {
    x: u32,
    y: u32,
    z: u32,
}

//e = (rmax-rmin)/(rmax+rmin)
//r = P/1+e*cos(90)

/*pub fn eliptic_orbit(time: f64) -> [f32; 3]{
    let a = 2
    let b = 1
    let o = 90
    let r = (a*(1-(e*e)))/(1+(e*o.cos()))
    let r = 1/(a+b*o.cos())
}

pub fn circular_orbit(time: f64) -> [f32; 3]{
    [3.0, 3.0, 3.0]
}*/