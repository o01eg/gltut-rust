
#![doc = "Common stuff for controls."]

use matrix::{Vector3f, Matrix4f};

use sdl;

use sdl2;
use sdl2::mouse::MouseWheelDirection;
use sdl2::keyboard::Scancode;

use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Mul};

#[doc = "Control stuff."]
pub struct Controls {

    #[doc = "Projection matrix"]
    pub projection : Matrix4f,

    #[doc = "View matrix"]
    pub view : Matrix4f,

    position : Vector3f,
    horizontal_angle : f32,
    vertical_angle : f32,
    initial_fov : f32,
    speed : f32,
    mouse_speed : f32,
    last_time : u32,
    ts : sdl2::TimerSubsystem,
}

impl Controls {
    #[doc = "Create controls."]
    pub fn new(mut ts : sdl2::TimerSubsystem) -> Controls {
        Controls {
            position : Vector3f(0f32, 0f32, 5f32),
            horizontal_angle : FRAC_PI_2,
            vertical_angle : 0.,
            initial_fov : 45.0,
            speed : 0.0005,
            mouse_speed : 0.0005,
            // sdl2::TimerSubsystem::ticks() is called only once, the first time this function is called
            last_time : ts.ticks(),
            ts : ts,
            projection : Matrix4f::perspective(45.0, 4.0 / 3.0, 0.1, 100.0),
            view : Matrix4f::look_at(
                &Vector3f(4.0, 3.0, 3.0), // Camera is at (4,3,3), in World Space
                &Vector3f(0.0, 0.0, 0.0), // and looks at the origin
                &Vector3f(0.0, 1.0, 0.0) // Head is up (set to 0,-1,0 to look upside-down)
            ),
        }
    }

    #[doc = "Mouse wheel handler"]
    pub fn process_wheel(&mut self, x : i32, y : i32, direction : MouseWheelDirection) {
        self.initial_fov -= y as f32 * 5f32;
    }

    #[doc = "update controls data."]
    pub fn update(&mut self, e : &sdl2::EventPump) {

        // Compute time difference between current and last frame
        let current_time = self.ts.ticks();
        let delta_time = current_time - self.last_time;
        // For the next frame, the "last time" will be "now"
        self.last_time = current_time;

        // Get mouse position
        let mouse_state = e.relative_mouse_state();
        let xpos = mouse_state.x();
        let ypos = mouse_state.y();
        
        self.horizontal_angle -= self.mouse_speed * delta_time as f32 * xpos as f32;
        self.vertical_angle -= self.mouse_speed * delta_time as f32 * ypos as f32;

        let direction = Vector3f(
            self.vertical_angle.cos() * self.horizontal_angle.sin(),
            self.vertical_angle.sin(),
            self.vertical_angle.cos() * self.horizontal_angle.cos()
        );

        let right = Vector3f(
            (self.horizontal_angle - FRAC_PI_2).sin(),
            0.,
            (self.horizontal_angle - FRAC_PI_2).cos()
        );

        let up = right.cross(&direction);

        let keyboard_state = e.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Up) {
            self.position = &self.position + &(&direction * (delta_time as f32 * self.speed));
        }
        if keyboard_state.is_scancode_pressed(Scancode::Down) {
            self.position = &self.position - &(&direction * (delta_time as f32 * self.speed));
        }
        if keyboard_state.is_scancode_pressed(Scancode::Left) {
            self.position = &self.position - &(&right * (delta_time as f32 * self.speed));
        }
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            self.position = &self.position + &(&right * (delta_time as f32 * self.speed));
        }

        // Projection matrix : 45&deg; Field of View, 4:3 ratio, display range : 0.1 unit <-> 100 units
        self.projection = Matrix4f::perspective(self.initial_fov, 4. / 3., 0.1, 100.);

        self.view = Matrix4f::look_at(&self.position, // Camera is here
                                            &self.position.add(&direction), // and looks here : at the same position, plus "direction"
                                            &up); // Head is up (set to 0,-1,0 to look upside-down)
    }
}

