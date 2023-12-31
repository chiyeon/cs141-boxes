use macroquad::prelude::*;
use macroquad::time::*;
use macroquad::rand;

use noise::{NoiseFn, Perlin, Simplex, Seedable};
// use glam::vec3;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;
const CHUNK_SIZE: usize = 16;

type Voxel = u8;

const AIR: Voxel = 0;
const GRASS: Voxel = 1;
const DIRT: Voxel = 2;
const WOOD: Voxel = 3;
const LEAF: Voxel = 4;
const LEAF_RED: Voxel = 5;
const LEAF_GREEN: Voxel = 6;
const SAND: Voxel = 7;

const YP: i32 = 0;
const YN: i32 = 1;
const XP: i32 = 2;
const XN: i32 = 3;
const ZP: i32 = 4;
const ZN: i32 = 5;

#[derive(Copy, Clone)]
struct Chunk {
    blocks: [[[Voxel; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2],
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Chunk {
    fn default() -> Chunk {
        Chunk {
            blocks: [[[AIR; CHUNK_SIZE + 2]; CHUNK_SIZE + 2]; CHUNK_SIZE + 2],
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
                //if y == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y+1][x][z];
            },
            YN => {
                if y == 1  { return DIRT; }
                return self.blocks[y-1][x][z];
            },
            XP => {
                //if x == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y][x+1][z];
            },
            XN => {
                //if x == 0 { return AIR; }
                return self.blocks[y][x-1][z];
            },
            ZP => {
                //if z == CHUNK_SIZE - 1 { return AIR; }
                return self.blocks[y][x][z+1];
            },
            ZN => {
                //if z == 0 { return AIR; }
                return self.blocks[y][x][z-1];
            },
            default => return AIR,
        }
    }

    fn fill_layer(&mut self, layer: usize, voxel: Voxel) {
        for x in 1..CHUNK_SIZE-1 {
            for z in 1..CHUNK_SIZE-1 {
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
    let xf: f32 = (chunk.x * CHUNK_SIZE as i32 + x as i32) as f32 - 2.0;
    let zf: f32 = (chunk.z * CHUNK_SIZE as i32 + z as i32) as f32 - 2.0;
    let yf: f32 = (chunk.y * CHUNK_SIZE as i32 + y as i32) as f32 - 2.0;

    if voxel == AIR { return; }

    let color: Vec3;

    match voxel {
        GRASS => color = vec3(144., 224., 72.),
        DIRT => color = vec3(79., 48., 43.),
        WOOD => color = vec3(85., 51., 17.),
        LEAF => color = vec3(252., 186., 3.),
        LEAF_RED => color = vec3(223.,57.,8.),
        LEAF_GREEN => color = vec3(167., 159., 15.),
        SAND => color = vec3(233., 225., 194.),
        default => color = vec3(0., 0., 0.),
    }

    let color_top = color_u8!(color.x, color.y, color.z, 255);
    let color_bottom = color_u8!(color.x * 0.6, color.y * 0.6, color.z * 0.6, 255);
    let color_x = color_u8!(color.x * 0.9, color.y * 0.9, color.z * 0.9, 255);
    let color_z = color_u8!(color.x * 0.7, color.y * 0.7, color.z * 0.7, 255);


    /*
    // if there is any voxel in our "line of view" just don't render us
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
// camera stuff
let mut freecam = false;
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

    
    let mut grabbed = false;
    set_cursor_grab(grabbed);
    show_mouse(!grabbed);
    

    let mut chunks: Vec<Chunk> = vec![];
    rand::srand(macroquad::miniquad::date::now() as _);
    let perlin = Perlin::new(rand::gen_range(0, 1000000));

    let chunk_start: i32 = -2;
    let chunk_end: i32 = 2;
    let island_radius = 26;
    let island_radius_roughness = 6; // roughness at which edges of island circle are
    let island_blend_start = 0.6;  // start blending terrain to 0 at X% from center
    let island_beach_start = 0.7;  // switch to beach biome (sand) at X% from center
    let island_beach_edge_randomness = 1.5; // number of blocks (forwards AND backwards) to variate
    for x in chunk_start..chunk_end {
        for z in chunk_start..chunk_end {
            //if (x == -2 && z == -2) || (x == 1 && z == 1) { continue; }
            //if x == z { continue; }
            

            let mut chunk = Chunk {
                x,
                z,
                ..Default::default()
            };
            for i in 1..CHUNK_SIZE + 1 {
                for j in 1..CHUNK_SIZE + 1 {
                    let mut voxel: Voxel = GRASS;
                    let fx = x as i32 * CHUNK_SIZE as i32 + i as i32;
                    let fz = z as i32 * CHUNK_SIZE as i32 + j as i32;
                    let magnitude = ((fx * fx + fz * fz) as f32).sqrt();

                    // shape our island into a rough circle
                    if fx.pow(2) + fz.pow(2) > (island_radius * island_radius) + rand::gen_range(-island_radius_roughness, island_radius_roughness) { continue; }

                    //println!("{}", perlin.get([(i * 10) as f64, (j * 10) as f64]));
                    let mut ny = ((perlin.get([
                        fx as f64 / 10., 
                        fz as f64 / 10.
                    ]) * 4.0 + 4.0));

                    if magnitude > island_radius as f32 * island_blend_start {
                        ny *= (island_radius as f32 - magnitude) as f64 / (island_radius as f32 * island_blend_start) as f64;
                        
                        // start beaches on edges, slightly more inward than when we start blending
                        // terrain
                        if magnitude + rand::gen_range(-island_beach_edge_randomness, island_beach_edge_randomness) > island_radius as f32 * island_beach_start {
                            voxel = SAND;
                        }
                    }

                    let y = ny.floor() as usize;

                    if y == 0 { continue; }

                    chunk.blocks[y][i][j] = voxel;
                    for z in 1..y {
                        chunk.blocks[z][i][j] = DIRT;
                    }

                    let min_tree_height = 4;
                    let max_tree_height = 10;
                    if rand::gen_range(0, 50) == 0 && voxel == GRASS {
                        let tree_height = rand::gen_range(min_tree_height, max_tree_height);
                        for z in (y+1)..(y+tree_height) {
                            chunk.blocks[z][i][j] = WOOD;
                        }
                        
                        let colors = [LEAF, LEAF_GREEN, LEAF_RED];
                        let tree_color = rand::gen_range(0, 3);

                        chunk.blocks[y+tree_height][i][j] = colors[tree_color];
                        if i != CHUNK_SIZE { chunk.blocks[y+tree_height-1][i+1][j] = colors[tree_color]; }
                        if i != 1 { chunk.blocks[y+tree_height-1][i-1][j] = colors[tree_color]; }
                        if j != CHUNK_SIZE { chunk.blocks[y+tree_height-1][i][j+1] = colors[tree_color]; }
                        if j != 1 { chunk.blocks[y+tree_height-1][i][j-1] = colors[tree_color]; }
                    }
                }
            }

            chunks.push(chunk);
        }
    }

    // occlude edges of chunks for better performance. minimal improvement.
    /*
    for x in chunk_start..chunk_end {
        for z in chunk_start..chunk_end {
            let index: usize = (x * (chunk_end - chunk_start) + z) as usize;
            if index >= chunks.len() { continue; }

            for i in 1..CHUNK_SIZE { 
                for j in 0..CHUNK_SIZE {
                    //if x != chunk_start { chunks[index].blocks[i][0][j] = chunks[((x + 1) * (chunk_end - chunk_start) + z) as usize].blocks[i][1][j]; }
                    //if x != chunk_end - 1 { chunks[index].blocks[i][CHUNK_SIZE+1][j] = chunks[((x - 1) * (chunk_end - chunk_start) + z) as usize].blocks[i][CHUNK_SIZE][j]; }
                    
                    chunks[index].blocks[i][0][j] = DIRT;
                    chunks[index].blocks[i][CHUNK_SIZE+1][j] = DIRT;
                    chunks[index].blocks[i][j][0] = DIRT;
                    chunks[index].blocks[i][j][CHUNK_SIZE+1] = DIRT;
                }
            }
        }
    }
    */

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) && freecam {
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

        if is_key_pressed(KeyCode::Tab) {
            freecam = !freecam;
            if freecam { 
                position = vec3(0., 10., 0.);
                grabbed = true;
                set_cursor_grab(grabbed);
                show_mouse(!grabbed);
            } else {
                grabbed = false;
                set_cursor_grab(grabbed);
                show_mouse(!grabbed);
            }
        }

        clear_background(color_u8!(135, 206, 235, 255));

        // Going 3d!

/*
        position = vec3(100., 100., 100.);
        set_camera(&Camera3D {
            position: position,
            up: Vec3::Y,
            projection: Projection::Orthographics,
            fovy: 24.,
            ..Default::default()
        });
        */

        /*
        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });
        */

        if freecam {
            set_camera(&Camera3D {
                position: position,
                up: up,
                target: position + front,
                ..Default::default()
            });
        } else {
            position = vec3(150., 150., 150.);
            set_camera(&Camera3D {
                position: position,
                up: Vec3::Y,
                projection: Projection::Orthographics,
                fovy: 42.,
                ..Default::default()
            });
        }

        for chunk in chunks.clone() {
            for x in 1..CHUNK_SIZE+1 {
                for z in 1..CHUNK_SIZE+1 {
                    for y in 1..CHUNK_SIZE+1 {
                        draw_voxel(
                            x, y, z,
                            chunk
                        );                      
                    }
                }
            }
        }

        // draw water
        draw_plane(vec3(0., -1., 0.), vec2(100., 100.), None, color_u8!(116,204,244,180));

        set_default_camera();
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            20.0,
            30.0,
            BLACK,
        );
        draw_text(
            "TAB to switch cameras",
            10.0,
            48.0,
            30.0,
            BLACK,
        );
        draw_text(
            "ESC to free mouse",
            10.0,
            48.0 + 28.,
            30.0,
            BLACK
        );

        next_frame().await
    }
}
