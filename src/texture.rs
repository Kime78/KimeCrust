use std::fs::File;
use std::{mem::{size_of, self}, ffi::c_void};
use image::io::Reader as ImageReader;
use std::io::Cursor;
pub struct Texture2D {
    id: u32
}   

impl Texture2D {
    pub fn new(path: &str) -> Self {
        unsafe {
            let mut texture: u32 = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
           
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            //let mut file = File::open("/home/kime/Documents/projects/KimeCrust/res/grass.png").unwrap();
            //let mut img = stb::image::stbi_load_from_reader(&mut file, stb::image::Channels::Rgb).unwrap().1.as_slice();
            
            let img = ImageReader::open("/home/kime/Documents/projects/KimeCrust/res/grass.png").unwrap().decode().unwrap();
            let byt = img.as_rgb8().unwrap().as_ptr() as *const c_void;
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 32, 32, 0, gl::RGB, gl::UNSIGNED_BYTE, byt);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            Self {
                id: texture
            }

        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

