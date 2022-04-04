mod shaders;
mod draw;
mod texture;

extern crate glfw;
extern crate gl;

use glfw::*;
use glm::Vec2;
use shaders::*;
use draw::*;
use texture::Texture2D;
//use gl::types::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));

    let (mut window, events) = glfw.create_window(1024, 720, "KimeCrust", glfw::WindowMode::Windowed)
    .expect("Failed to create GLFW window.");

    gl::load_with(|s| window.get_proc_address(s));
    
    window.set_key_polling(true);
    window.make_current();
    let test = include_bytes!("../res/grass.png");
    let text = Texture2D::new("/home/kime/Documents/Projects/KimeCrust/res/grass.png");
    let mut vert_src = include_str!("triangle.vs").to_string();
    let mut frag_src = include_str!("triangle.fs").to_string();
    let vert = Shader::new(vert_src, ShaderType::Vertex);
    let frag = Shader::new(frag_src, ShaderType::Fragment);

    let mut model = glm::mat4(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0, 
    );

    //let m = &model;
    
    //model = glm::ext::rotate(mo, glm::radians(-55.0), glm::vec3(1.0, 0.0, 0.0));

    let mut projection = glm::mat4(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0, 
    );

    projection = glm::ext::perspective(glm::radians(45.0), 800.0 / 600.0, 0.1, 100.0);

    let mut view = glm::mat4(
        1.0,0.0,0.0,0.0,
        0.0,1.0,0.0,0.0,
        0.0,0.0,1.0,0.0,
        0.0,0.0,0.0,1.0, 
    );

    let m = &view;
    view = glm::ext::translate(m, glm::vec3(0.0, 0.0, -3.0)); 
    
    let prog = ShaderProgram::new(vert, frag);
    let mut d = DrawEngine::new();
    unsafe { gl::Viewport(0, 0, 1024, 720); }
    let mut is_fullscreen = false;
    let mut last_pos = (0, 0);
    let mut last_size = (0, 0);

    while !window.should_close() {
        let m = &model;
        model = glm::ext::rotate(m, (glfw.get_time() * glm::radians(50.0)) as f32, glm::vec3(0.5, 1.0, 0.0));  

        prog.upload_mat4(view, "view".to_string());
        prog.upload_mat4(projection, "projection".to_string());
        prog.upload_mat4(model, "model".to_string());

        glfw.get_time();
        window.swap_buffers();
        prog.use_program();
        text.bind();
        d.draw();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
        
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
                glfw::WindowEvent::Key(Key::F10, _, Action::Press, _) => {
                    d.change_wireframe();
                }
                _ => {}
            }
            handle_window_event(&mut window, event);
        }
    }
    d.delete();
    prog.delete();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event { 
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


