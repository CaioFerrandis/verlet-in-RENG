mod vertex;
mod mesh;
mod shader;
mod macros;
mod window;
mod game_object;
mod transform;
mod shapes;
mod camera;
mod texture;
mod light;
mod line;

use std::collections::HashMap;

use game_object::GameObject;
use glam::{vec2, vec3, vec4, Quat, Vec2, Vec3, Vec4};

use glfw::{Action, Key};
use image::imageops::colorops;
use light::{Light, LIGHTS};
use line::Line;
use mesh::Mesh;
use texture::make_tex;
use window::Window;

// settings
const W: u32 = 800;
const H: u32 = 600;

pub fn main() {
    let mut window = Window::new(W, H);
    window.set_caption("ulala babe babe cmon");

    let mut texture_pack: HashMap<i32, u32> = HashMap::default();
    texture_pack.insert(0, make_tex("src/textures/container.jpg"));
    texture_pack.insert(1, make_tex("src/textures/default_tex.png"));

    unsafe{
        LIGHTS.push(Light { position: vec3(0., 0., 0.), color: vec3(1., 1., 1.) });
    }

    let grid_size: i32 = 5;
    let spacing: i32 = 2;

    let mut particles: Vec<GameObject<Mesh>> = vec![];

    for x in 0..grid_size{
        for y in 0..grid_size{
            let mut m = GameObject::<Mesh>::new(Mesh::empty());
            m.set_shape(shapes::Shapes::Sphere);
            m.set_texture(texture_pack[&1]);
            m.setup_mesh();

            m.set_position(vec3((-grid_size + x*spacing) as f32, (-grid_size + y*spacing) as f32, -3.));
            m.scale(0.4);
            m.set_color(vec4(1., 0., 0., 1.));

            particles.push(m);
        }
    }

    let mut lines: Vec<GameObject<Line>> = vec![];
    let mut lines_indexes: Vec<Vec2> = vec![];

    for (index, mesh) in particles.iter().enumerate(){
        if index < particles.len() - 1 && (index + grid_size as usize) % grid_size as usize != (grid_size - 1) as usize{
            let mut line1 = GameObject::<Line>::new(mesh.transform.position, particles[index+1].transform.position, false);
            line1.setup_mesh();

            lines.push(line1);
            lines_indexes.push(vec2(index as f32, index as f32+1.));
        }

        if index as i32 + grid_size < particles.len() as i32{
            let mut line2 = GameObject::<Line>::new(mesh.transform.position, particles[index+grid_size as usize].transform.position, false);
            line2.setup_mesh();

            lines.push(line2);
            lines_indexes.push(vec2(index as f32, index as f32+grid_size as f32));
        }
    }

    let mut new_click = false;
    let mut closest_particle: usize = 0;
    let mouse_follow_speed = 8.;

    while !window.should_close() {
        let view_position = window.camera.position;
        window.clear_screen();

        window.camera.movement(window.keyboard.clone(), window.dt);

        unsafe{
            LIGHTS[0].position = window.camera.position;
        }

        if window.keyboard[&Key::LeftAlt] == Action::Press{
            window.lock_cursor();
        }

        if window.mouse_buttons[0] == true{
            if new_click{
                closest_particle = get_closest_particle_to_mouse(&particles, window.mouse_pos);
                new_click = false;
            }
            particles[closest_particle].translate(vec3((window.mouse_pos.x - window.last_mouse_pos.x)*mouse_follow_speed/W as f32, -(window.mouse_pos.y - window.last_mouse_pos.y)*mouse_follow_speed/H as f32, 0.));
        }
        else{
            new_click = true;
        }

        for obj in particles.iter(){
            obj.draw(view_position);
        }
        for (index, line) in lines.iter_mut().enumerate(){
            line.set_begin(particles[lines_indexes[index].x as usize].transform.position);
            line.set_end(particles[lines_indexes[index].y as usize].transform.position);
            line.draw(view_position);
        }

        window.update();
    }
}

pub fn get_closest_particle_to_mouse(particles: &Vec<GameObject<Mesh>>, mouse_pos: Vec2) -> usize{
    let mut answer = 0;
    for (index, particle) in particles.iter().enumerate(){
        if particle.transform.position.distance(vec3(mouse_pos.x, mouse_pos.y, 0.)) < particles[answer].transform.position.distance(vec3(mouse_pos.x, mouse_pos.y, 0.)){
            answer = index;
        }
    }
    answer
}
