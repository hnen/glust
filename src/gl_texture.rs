extern crate gl;
use gl::types::*;
use std::os::raw::c_void;
use gl_err::*;
use std::ptr;

pub struct GlTexture {
    pub handle : GLuint
}

impl GlTexture {

    pub fn new_rgba8_empty(width : usize, height : usize) -> Result<GlTexture> {
        Self::new_empty(width, height, gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE)
    }
    pub fn new_f32_empty(width : usize, height : usize) -> Result<GlTexture> {
        Self::new_empty(width, height, gl::RED, gl::RED, gl::FLOAT)
    }

    pub fn new_rgba8(width : usize, height : usize, data_rgba : &[u8]) -> Result<GlTexture> {
        let mut tex_id : GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, width as i32, height as i32, 0,
                           gl::RGBA, gl::UNSIGNED_BYTE, data_rgba.as_ptr() as *const c_void );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        }

        match validate_gl() {
            Err(s) => Err(s),
            _ => Ok(GlTexture{ handle: tex_id }),
        }
    }

    fn new_empty(width : usize, height : usize, internalformat : GLenum, format : GLenum, txtype : GLenum) -> Result<GlTexture> {
        let mut tex_id : GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut tex_id);
            gl::BindTexture(gl::TEXTURE_2D, tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, internalformat as i32, width as i32, height as i32, 0,
                           format, txtype, ptr::null() );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            //gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        match validate_gl() {
            Err(s) => Err(s),
            _ => Ok(GlTexture{ handle: tex_id }),
        }
    }

    pub fn mipmapped(self) -> Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        }
        validate_gl().unwrap();
        self
    }

    pub fn _uv_clamp(self) -> Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }
        validate_gl().unwrap();
        self
    }

    pub fn _uv_repeat(self) -> Self {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.handle);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
        validate_gl().unwrap();
        self
    }

}

impl Drop for GlTexture {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
        validate_gl().unwrap();
        self.handle  = 0;
    }
}




