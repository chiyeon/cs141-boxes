use macroquad::prelude::*;
use macroquad::time::*;
use macroquad::rand;

use noise::{NoiseFn, Perlin, Simplex, Seedable};
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;
const CHUNK_SIZE: usize = 16;

type Voxel = i32;

const AIR: Voxel = 0;
const GRASS: Voxel = 1;
const DIRT: Voxel = 2;

const YP: i32 = 0;
const YN: i32 = 1;
const XP: i32 = 2;
const XN: i32 = 3;
const ZP: i32 = 4;
const ZN: i32 = 5;

#[derive(Copy, Clone)]
struct Chunk {
    blocks: [[[i32; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Chunk {
    fn default() -> Chunk {
        Chunk {
            blocks: [[[AIR; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

impl Chunk {
    fn get_neighbor(&self, x: usize, y: usize, z: usize, loc: i32) -> Voxel {
        match loc {
            YP => {
                if y == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y+1][x][z];
            },
            YN => {
                if y == 0 { return AIR; }
                return self.blocks[y-1][x][z];
            },
            XP => {
                if x == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y][x+1][z];
            },
            XN => {
                if x == 0 { return AIR; }
                return self.blocks[y][x-1][z];
            },
            ZP => {
                if z == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y][x][z+1];
            },
            ZN => {
                if z == 0 { return AIR; }
                return self.blocks[y][x][z-1];
            },
            default => return AIR,
        }
    }

    fn fill_layer(&mut self, layer: usize, voxel: Voxel) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                self.blocks[layer][x][z] = voxel;
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

fn draw_voxel(x: usize, y: usize, z: usize, chunk: Chunk) {
    let voxel = chunk.blocks[y][x][z];

    // x y and z are local. also calc our final pos
    let xf: f32 = (chunk.x * CHUNK_SIZE as i32 + x as i32) as f32;
    let zf: f32 = (chunk.z * CHUNK_SIZE as i32 + z as i32) as f32;
    let yf: f32 = (chunk.y * CHUNK_SIZE as i32 + y as i32) as f32;

    if voxel == AIR { return; }

    let color: Vec3;

    match voxel {
        GRASS => color = vec3(144., 224., 72.),
        DIRT => color = vec3(79., 48., 43.),
        default => color = vec3(0., 0., 0.),
    }

    let color_top = color_u8!(color.x, color.y, color.z, 255);
    let color_bottom = color_u8!(color.x * 0.6, color.y * 0.6, color.z * 0.6, 255);
    let color_x = color_u8!(color.x * 0.9, color.y * 0.9, color.z * 0.9, 255);
    let color_z = color_u8!(color.x * 0.7, color.y * 0.7, color.z * 0.7, 255);


    // if there is any voxel in our "line of view" just don't render us
    /*
    let mut x2 = x + 1;
    let mut z2 = z + 1;
    let mut y2 = y + 1;
    while x2 < CHUNK_SIZE && y2 < CHUNK_SIZE && z2 < CHUNK_SIZE {
        if chunk.blocks[y2][x2][z2] != AIR { return; } 
        x2 += 1;
        z2 += 1;
        y2 += 1;
    }
    */

    // top
    if chunk.get_neighbor(x, y, z, YP) == AIR { 
        draw_affine_parallelogram(
            vec3(xf, yf+0.5, zf),
            Vec3::X,
            Vec3::Z,
            None,
            color_top
        );
    }

    // Z
    if chunk.get_neighbor(x, y, z, ZN) == AIR { 
        draw_affine_parallelogram(
            vec3(xf, yf-0.5, zf),
            Vec3::X,
            Vec3::Y,
            None,
            color_z
        );
    }
    
    if chunk.get_neighbor(x, y, z, ZP) == AIR { 
        draw_affine_parallelogram(
            vec3(xf, yf-0.5, zf+1.),
            Vec3::X,
            Vec3::Y,
            None,
            color_z
        );
    }

    // X
    if chunk.get_neighbor(x, y, z, XN) == AIR {
        draw_affine_parallelogram(
            vec3(xf, yf-0.5, zf),
            Vec3::Z,
            Vec3::Y,
            None,
            color_x
        );
    }
    
    if chunk.get_neighbor(x, y, z, XP) == AIR { 
        draw_affine_parallelogram(
            vec3(xf+1., yf-0.5, zf),
            Vec3::Z,
            Vec3::Y,
            None,
            color_x
        );
    }
    
    // bottom
    if chunk.get_neighbor(x, y, z, YN) == AIR { 
        draw_affine_parallelogram(
            vec3(xf, yf-0.5, zf),
            Vec3::X,
            Vec3::Z,
            None,
            color_bottom
        );
    }
}

#[macroquad::main(conf)]
async fn main() {
let mut x = 0.0;
let mut switch = false;
let bounds = 8.0;


let world_up = vec3(0.0, 1.0, 0.0);
let mut yaw: f32 = 1.18;
let mut pitch: f32 = 0.0;

let mut front = vec3(
yaw.cos() * pitch.cos(),
pitch.sin(),
yaw.sin() * pitch.cos(),
)
.normalize();
let mut right = front.cross(world_up).normalize();
let mut up;

let mut position = vec3(0.0, 10.0, 0.0);
let mut last_mouse_position: Vec2 = mouse_position().into();

    
    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);
    

    let mut chunks: Vec<Chunk> = vec![];
    let perlin = Simplex::new(1);
        
    for x in -2..2 {
        for z in -2..2 {
            if (x == -2 && z == -2) || (x == 1 && z == 2) { continue; }
            

            let mut chunk = Chunk {
                x,
                z,
                ..Default::default()
            };
            for i in 0..CHUNK_SIZE {
                for j in 0..CHUNK_SIZE {
                    //println!("{}", perlin.get([(i * 10) as f64, (j * 10) as f64]));
                    let y: usize = (perlin.get([
                        (x as f64 * CHUNK_SIZE as f64 + i as f64), 
                        (z as f64 * CHUNK_SIZE as f64 + j as f64)
                    ]) * 4.0 +2.0).floor() as usize;
                    chunk.blocks[y][i][j] = GRASS;
                    for z in 0..y {
                        chunk.blocks[z][i][j] = DIRT;
                    }
                }
            }

            chunks.push(chunk);
        }
    }

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_position - last_mouse_position;
        last_mouse_position = mouse_position;

        yaw += mouse_delta.x * delta * LOOK_SPEED;
        pitch += mouse_delta.y * delta * -LOOK_SPEED;

        pitch = if pitch > 1.5 { 1.5 } else { pitch };
        pitch = if pitch < -1.5 { -1.5 } else { pitch };

        front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();

        right = front.cross(world_up).normalize();
        up = right.cross(front).normalize();

        x += if switch { 0.04 } else { -0.04 };
        if x >= bounds || x <= -bounds {
            switch = !switch;
        }

        clear_background(LIGHTGRAY);

        // Going 3d!

        position = vec3(100., 100., 100.);
        set_camera(&Camera3D {
            position: position,
            up: Vec3::Y,
            projection: Projection::Orthographics,
            fovy: 21.,
            ..Default::default()
        });

        /*
        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });
        */

        draw_grid(20, 1., BLACK, GRAY);

        for chunk in chunks.clone() {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    for y in 0..CHUNK_SIZE {
                        draw_voxel(
                            x, y, z,
                            chunk.clone()
                        );                      
                    }
                }
            }
        }

        // Back to screen space, render some text

        set_default_camera();
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            20.0,
            30.0,
            BLACK,
        );

        next_frame().await
    }
}
