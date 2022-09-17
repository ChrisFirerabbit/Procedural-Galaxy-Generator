extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate shader_version;

mod render;

use std::convert::TryInto;
// A Frames Per Second counter.

use std::collections::VecDeque;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Measures Frames Per Second (FPS).
#[derive(Debug)]
pub struct FPSCounter {
    /// The last registered frames.
    last_second_frames: VecDeque<Instant>
}

impl Default for FPSCounter {
    fn default() -> Self {
        FPSCounter::new()
    }
}

impl FPSCounter {
    /// Creates a new FPSCounter.
    pub fn new() -> FPSCounter {
        FPSCounter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }

    /// Updates the FPSCounter and returns number of frames.
    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);

        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}

//----------------------------------------
// Cube associated data

gfx_vertex_struct!( Vertex {
    a_pos: [f32; 4] = "a_pos",
    a_tex_coord: [i8; 2] = "a_tex_coord",
});

impl Vertex {
    fn new(pos: [f32; 3], tc: [i8; 2]) -> Vertex {
        Vertex {
            a_pos: [pos[0] as f32, pos[1] as f32, pos[2] as f32, 1.0 as f32],
            a_tex_coord: tc,
        }
    }
}

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    t_color: gfx::TextureSampler<[f32; 4]> = "t_color",
    out_color: gfx::RenderTarget<::gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

//----------------------------------------

fn main() {
    use piston_window::*;
    use gfx::traits::*;
    use shader_version::Shaders;
    use shader_version::glsl::GLSL;
    use camera_controllers::{
        FirstPersonSettings,
        FirstPerson,
        CameraPerspective,
        model_view_projection
    };

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new("MineStar", [1920, 1080]) //[640, 480]
        .exit_on_esc(true)
        .samples(4)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_capture_cursor(true);

    let mut factory = window.factory.clone();

    // point to cube
    let mut vertex_data: Vec<Vertex> = Vec::new();
    let mut index_data: Vec<u32> = Vec::new();

    for x in 0..2 {
        let (vertex_data_temp, index_data_temp) = point_to_cube([x as f32,x as f32,x as f32]);

        let init_len: u32 = vertex_data.len() as u32;
        println!("len: {:?}",vertex_data.len());

        for points in vertex_data_temp{
            vertex_data.push(points);
        }

        for index_value in index_data_temp.iter() {
            index_data.push(init_len + index_value);
        }
    }
    //for x in index_data_temp {
    //    index_data[index_data.len()-1] = x;
    //}

    let index_data_slice: &[u32] = &index_data;

    let (vbuf, slice) = factory.create_vertex_buffer_with_slice
        (&vertex_data, index_data_slice);


    //let vbuf = factory.create_vertex_buffer(&vertex_data);


    let texels = [
        [0xff, 0xff, 0xff, 0x00],
        [0xff, 0x00, 0x00, 0x00],
        [0x00, 0xff, 0x00, 0x00],
        [0x00, 0x00, 0xff, 0x00]
    ];
    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        gfx::texture::Kind::D2(2, 2, gfx::texture::AaMode::Single),
        gfx::texture::Mipmap::Provided,
        &[&texels]).unwrap();

    let sinfo = gfx::texture::SamplerInfo::new(
        gfx::texture::FilterMethod::Bilinear,
        gfx::texture::WrapMode::Clamp);

    let glsl = opengl.to_glsl();
    let pso = factory.create_pipeline_simple(
            Shaders::new()
                .set(GLSL::V1_20, include_str!("../assets/cube_120.glslv"))
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslv"))
                .get(glsl).unwrap().as_bytes(),
            Shaders::new()
                .set(GLSL::V1_20, include_str!("../assets/cube_120.glslf"))
                .set(GLSL::V1_50, include_str!("../assets/cube_150.glslf"))
                .get(glsl).unwrap().as_bytes(),
            pipe::new()
        ).unwrap();

    let get_projection = |w: &PistonWindow| {
        let draw_size = w.window.draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&window);
    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let mut data = pipe::Data {
        vbuf,
        u_model_view_proj: [[0.0; 4]; 4],
        t_color: (texture_view, factory.create_sampler(sinfo)),
        out_color: window.output_color.clone(),
        out_depth: window.output_stencil.clone(),
    };

    let mut last_epoch = get_epoch_ms();

    while let Some(e) = window.next() {
        first_person.event(&e);

        window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

            data.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            window.encoder.draw(&slice, &pso, &data);
        });

        if e.resize_args().is_some() {
            projection = get_projection(&window);
            data.out_color = window.output_color.clone();
            data.out_depth = window.output_stencil.clone();
        }

        /*let mut a = FPSCounter::default();
        a.tick();
        println!("{:?}", a);*/
        
        
        /*let epoch: u128 = get_epoch_ms();
        let dif = epoch - last_epoch;
        if dif != 0 {
            println!("{:?}", 1000/dif);
            last_epoch = epoch;
        }*/
    }
}











fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}



fn point_to_cube(point: [f32; 3]) -> (Vec<Vertex>, Vec<u32>) {
    let x = point[0];
    let y = point[1];
    let z = point[2];

    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([x-0.5, y-0.5, z+0.5], [0, 0]),
        Vertex::new([x+0.5, y-0.5, z+0.5], [1, 0]),
        Vertex::new([x+0.5, y+0.5, z+0.5], [1, 1]),
        Vertex::new([x-0.5, y+0.5, z+0.5], [0, 1]),
        //bottom (0, 0, -0.5)
        Vertex::new([x+0.5, y+0.5, z-0.5], [0, 0]),
        Vertex::new([x-0.5, y+0.5, z-0.5], [1, 0]),
        Vertex::new([x-0.5, y-0.5, z-0.5], [1, 1]),
        Vertex::new([x+0.5, y-0.5, z-0.5], [0, 1]),
        //right (0.5, 0, 0)
        Vertex::new([x+0.5, y-0.5, z-0.5], [0, 0]),
        Vertex::new([x+0.5, y+0.5, z-0.5], [1, 0]),
        Vertex::new([x+0.5, y+0.5, z+0.5], [1, 1]),
        Vertex::new([x+0.5, y-0.5, z+0.5], [0, 1]),
        //left (-0.5, 0, 0)
        Vertex::new([x-0.5, y+0.5, z+0.5], [0, 0]),
        Vertex::new([x-0.5, y-0.5, z+0.5], [1, 0]),
        Vertex::new([x-0.5, y-0.5, z-0.5], [1, 1]),
        Vertex::new([x-0.5, y+0.5, z-0.5], [0, 1]),
        //front (0, 0.5, 0)
        Vertex::new([x-0.5, y+0.5, z-0.5], [0, 0]),
        Vertex::new([x+0.5, y+0.5, z-0.5], [1, 0]),
        Vertex::new([x+0.5, y+0.5, z+0.5], [1, 1]),
        Vertex::new([x-0.5, y+0.5, z+0.5], [0, 1]),
        //back (0, -0.5, 0)
        Vertex::new([x+0.5, y-0.5, z+0.5], [0, 0]),
        Vertex::new([x-0.5, y-0.5, z+0.5], [1, 0]),
        Vertex::new([x-0.5, y-0.5, z-0.5], [1, 1]),
        Vertex::new([x+0.5, y-0.5, z-0.5], [0, 1]),
    ];

    let index_data: Vec<u32> = vec![
        0,  1,  2,  2,  3,  0, // top
        4,  6,  5,  6,  4,  7, // bottom
        8,  9, 10, 10, 11,  8, // right
       12, 14, 13, 14, 12, 15, // left
       16, 18, 17, 18, 16, 19, // front
       20, 21, 22, 22, 23, 20, // back
   ];

    return (vertex_data, index_data);
}

// Check if side of block detects non-absolute solid, e.g. air, wedge, torch, e.g
// if it is non-absolute solid, create face


/*let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)+1.0], [0, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)+1.0], [1, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)+1.0], [1, 1]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)+1.0], [0, 1]),
        //bottom (0, 0, -1.0)
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)-1.0], [0, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)-1.0], [1, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)-1.0], [1, 1]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)-1.0], [0, 1]),
        //right (1.0, 0, 0)
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)-1.0], [0, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)-1.0], [1, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)+1.0], [1, 1]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)+1.0], [0, 1]),
        //left (-1.0, 0, 0)
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)+1.0], [0, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)+1.0], [1, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)-1.0], [1, 1]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)-1.0], [0, 1]),
        //front (0, 1.0, 0)
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)-1.0], [0, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)-1.0], [1, 0]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)+1.0, (2.0*z)+1.0], [1, 1]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)+1.0, (2.0*z)+1.0], [0, 1]),
        //back (0, -1.0, 0)
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)+1.0], [0, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)+1.0], [1, 0]),
        Vertex::new([(2.0*x)-1.0, (2.0*y)-1.0, (2.0*z)-1.0], [1, 1]),
        Vertex::new([(2.0*x)+1.0, (2.0*y)-1.0, (2.0*z)-1.0], [0, 1]),
    ];

    let index_data: Vec<u32> = vec![
        0,  1,  2,  2,  3,  0, // top
        4,  6,  5,  6,  4,  7, // bottom
        8,  9, 10, 10, 11,  8, // right
       12, 14, 13, 14, 12, 15, // left
       16, 18, 17, 18, 16, 19, // front
       20, 21, 22, 22, 23, 20, // back
   ];

    return (vertex_data, index_data);
}*/