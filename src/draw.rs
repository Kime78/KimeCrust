use std::{mem::{size_of, self}, ffi::c_void};

pub struct DrawEngine { //idk how tf to call this
    vao: u32,
    vbo: u32,
    wireframe: bool
}
impl DrawEngine {
    pub fn new() -> Self {
        let vertices: [f32; 60] = [
            // positions         // colors
            -0.4,   0.125,  0.0,  0.4, 0.521, 0.960, 
            -0.125, 0.125,  0.0,  0.490, 0.443, 0.956,
             0.0,   0.5,    0.0,  0.686, 0.443, 0.956, 
             0.125, 0.125,  0.0,  0.917, 0.443, 0.956,  
             0.4,   0.125,  0.0,  0.807, 0.317, 0.250,  
             0.13, -0.125,  0.0,  0.807, 0.250, 0.682,
             0.29, -0.6,    0.0,  0.956, 0.631, 0.443,
             0.0,  -0.29,   0.0,  0.956, 0.843, 0.443,
            -0.29, -0.6,    0.0,  0.862, 0.956, 0.443,
            -0.13, -0.125,  0.0,  0.584, 0.956, 0.443
        ];

        let elements: [u32; 24] = [
            0, 1, 9,   1, 2, 3,
            3, 4, 5,   5, 6, 7,
            7, 8, 9,   9, 5, 7,
            9, 1, 3,   9, 3, 5
        ];

        let mut VBO: u32 = 0;
        let mut VAO: u32 = 0;
        let mut EBO: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut VAO);
            gl::GenBuffers(1, &mut VBO);

            gl::BindVertexArray(VAO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

            //https://stackoverflow.com/questions/64309656/how-to-convert-a-rust-array-to-pointer
            let asd = &vertices;
            let p = asd as *const f32;
            let dat = p as *const c_void;

            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * size_of::<f32>()) as isize, dat, gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            let asd = &elements;
            let p = asd as *const u32;
            let dat = p as *const c_void;

            gl::GenBuffers(1, &mut EBO);
	        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
	        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (elements.len() * size_of::<u32>()) as isize, dat, gl::STATIC_DRAW);
        }   

        Self {
            vao: VAO,
            vbo: VBO,
            wireframe: false
        }
    }

    pub fn draw(&self) {
        unsafe {
            if self.wireframe {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            }
            else {
                gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
            }

            gl::ClearColor(0.0 / 255.0, 0.0 / 255.0, 0.0 /255.0, 0.0 / 255.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 24, gl::UNSIGNED_INT, 0 as *const c_void);
        }
    }
    pub fn change_wireframe(&mut self) {
        self.wireframe = true ^ self.wireframe;
    }
    pub fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
