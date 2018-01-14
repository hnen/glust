extern crate gl;
use gl::types::*;
use gl_buffer::*;
use gl_err::*;
use std::ptr;

pub struct GlVertexArray {
    pub gl_handle : GLuint,
    pub vertex_count : i32,
    _vbs : Vec<GlBufferRaw>
}

pub struct GlVertexArrayTmp<'a> {
    pub gl_handle : GLuint,
    pub vertex_count : i32,
    _vbs : Vec<&'a GlBufferRaw>
}

pub trait HasGlVertexArrayHandle {
    fn gl_vao_handle(&self) -> GLuint;
}

impl GlVertexArray {
    pub fn new(vbs : Vec<GlBufferRaw>) -> Result<GlVertexArray> {
        let gl_handle = {
            let vbs_ref: Vec<_> = vbs.iter().map(|a| a).collect();
            gen_va(&vbs_ref[..])?
        };
        Ok(GlVertexArray {
                gl_handle: gl_handle,
                vertex_count: vbs[0].el_count as i32,
                _vbs: vbs
        })
    }
}

impl<'a> GlVertexArrayTmp<'a> {
    pub fn new(vbs : Vec<&'a GlBufferRaw>) -> Result<GlVertexArrayTmp<'a>> {
        let gl_handle = gen_va(&vbs[..])?;
        Ok(GlVertexArrayTmp {
            gl_handle: gl_handle,
            vertex_count: vbs[0].el_count as i32,
            _vbs: vbs
        })
    }

}

impl HasGlVertexArrayHandle for GlVertexArray {
    fn gl_vao_handle(&self) -> GLuint {
        self.gl_handle
    }
}

impl<'a> HasGlVertexArrayHandle for GlVertexArrayTmp<'a> {
    fn gl_vao_handle(&self) -> GLuint {
        self.gl_handle
    }
}

impl Drop for GlVertexArray {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.gl_handle);
        }
        validate_gl().unwrap();
        self.gl_handle = 0;
    }
}

impl<'a> Drop for GlVertexArrayTmp<'a> {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.gl_handle);
        }
        validate_gl().unwrap();
        self.gl_handle = 0;
    }
}

fn gen_va(vbs : &[&GlBufferRaw]) -> Result<GLuint> {
    unsafe {
        let mut gl_handle : GLuint = 0;
        gl::GenVertexArrays(1, &mut gl_handle);

        gl::BindVertexArray(gl_handle);
        for (i,vb) in vbs.iter().enumerate() {
            gl::EnableVertexAttribArray(i as u32);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb.buffer_id);
            gl::VertexAttribPointer(i as u32, vb.component_count as i32,
                                    vb.gl_type_enum, gl::FALSE, 0, ptr::null());
        }

        match validate_gl() {
            Err(s) => Err(s),
            _ => Ok(gl_handle)
        }
    }
}

