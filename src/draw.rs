use std::{mem::{size_of, self}, ffi::c_void};
use std::fs::File;

pub struct DrawEngine { //idk how tf to call this
    vao: u32,
    vbo: u32,
    wireframe: bool
}
impl DrawEngine {
    pub fn new() -> Self {
        let vertices: [f32; 180] = [
            // positions       // texture coords
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,   1.0, 0.0,
            0.5,  0.5, -0.5,   1.0, 1.0,
            0.5,  0.5, -0.5,   1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,   1.0, 0.0,
            0.5,  0.5,  0.5,   1.0, 1.0,
            0.5,  0.5,  0.5,   1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,   1.0, 0.0,
            0.5,  0.5, -0.5,   1.0, 1.0,
            0.5, -0.5, -0.5,   0.0, 1.0,
            0.5, -0.5, -0.5,   0.0, 1.0,
            0.5, -0.5,  0.5,   0.0, 0.0,
            0.5,  0.5,  0.5,   1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        // let elements: [u32; 36] = [
        //     /*Above ABC,BCD*/
        //     0,1,2,
        //     1,2,3,
    
        //     /*Following EFG,FGH*/
        //     4,5,6,
        //     5,6,7,
        //     /*Left ABF,AEF*/
        //     0,1,5,
        //     0,4,5,
        //     /*Right side CDH,CGH*/
        //     2,3,7,
        //     2,6,7,
        //     /*ACG,AEG*/
        //     0,2,6,
        //     0,4,6,
        //     /*Behind BFH,BDH*/
        //     1,5,7,
        //     1,3,7
        // ];

        let mut VBO: u32 = 0;
        let mut VAO: u32 = 0;
        //let mut EBO: u32 = 0;

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

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as i32, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const c_void);
            // gl::EnableVertexAttribArray(1);

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as i32, (3 * mem::size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(1); 

            //let asd = &elements;
            //let p = asd as *const u32;
            //let dat = p as *const c_void;

            // gl::GenBuffers(1, &mut EBO);
	        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
	        // gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (elements.len() * size_of::<u32>()) as isize, dat, gl::STATIC_DRAW);
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
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
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
