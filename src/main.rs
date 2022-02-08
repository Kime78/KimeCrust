mod shaders;

extern crate glfw;
extern crate gl;

use glfw::*;
//use gl::types::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(1024, 720, "KimeCrust", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");
    //let joystick = glfw.get_joystick(glfw::JoystickId::Joystick1);
    gl::load_with(|s| window.get_proc_address(s));
    
    window.set_key_polling(true);
    window.make_current();

    unsafe { gl::Viewport(0, 0, 1024, 720); }
    let mut is_fullscreen = false;
    let mut last_pos = (0, 0);
    let mut last_size = (0, 0);
    while !window.should_close() {
        unsafe {
            gl::ClearColor(250.0 / 255.0, 119.0 / 255.0, 155.0 /255.0, 110.0 / 255.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            //println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::F11, _, Action::Press, _) => {
                    if is_fullscreen {
                        window.set_monitor(
                            glfw::WindowMode::Windowed,
                            last_pos.0,
                            last_pos.1,
                            last_size.0 as u32,
                            last_size.1 as u32,
                            None,
                        );
                        println!(
                            "Window restored to {:?} at location {:?}",
                            last_size, last_pos
                        );
                    } else {
                        last_pos = window.get_pos();
                        last_size = window.get_size();
        
                        glfw.with_primary_monitor(|_: &mut _, m: Option<&glfw::Monitor>| {
                            let monitor = m.unwrap();
        
                            let mode = monitor.get_video_mode().unwrap();
        
                            window.set_monitor(
                                glfw::WindowMode::FullScreen(&monitor),
                                0,
                                0,
                                mode.width,
                                mode.height,
                                Some(mode.refresh_rate),
                            );
        
                            println!(
                                "{}x{} fullscreen enabled at {}Hz on monitor {}",
                                mode.width,
                                mode.height,
                                mode.refresh_rate,
                                monitor.get_name().unwrap()
                            );
                        });
                    }
        
                    is_fullscreen = !is_fullscreen;
                }
                _ => {}
            }
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        // glfw::JoystickEvent::Key(GamepadButton::ButtonA, _, Action::Press, _) => {

        // }
    
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) | glfw::WindowEvent::Key(Key::A, _, Action::Repeat, _) => {
            window.set_pos(window.get_pos().0 - 1, window.get_pos().1);
        }
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) | glfw::WindowEvent::Key(Key::D, _, Action::Repeat, _) => {
            window.set_pos(window.get_pos().0 + 1, window.get_pos().1);
        }
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) | glfw::WindowEvent::Key(Key::W, _, Action::Repeat, _) => {
            window.set_pos(window.get_pos().0, window.get_pos().1 - 1);
        }
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) | glfw::WindowEvent::Key(Key::S, _, Action::Repeat, _) => {
            window.set_pos(window.get_pos().0, window.get_pos().1 + 1);
        }
        _ => {}
    }
}


