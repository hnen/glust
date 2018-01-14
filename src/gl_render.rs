extern crate gl;
use std::ptr;

use gl_shaders::GlShader;
use gl_shaders::GlShaderUniform;
use gl_vertex_array::GlVertexArray;
use gl_vertex_array::HasGlVertexArrayHandle;
use gl_buffer::GlBufferRaw;
use gl_framebuffer::GlFramebuffer;
use gl_err::*;
use gl::types::*;

pub enum RenderTarget<'a> {
    Framebuffer(&'a GlFramebuffer),
    Screen(usize, usize)
}

pub fn clear(r : GLfloat, g : GLfloat, b : GLfloat, a : GLfloat, fb : Option<&GlFramebuffer>) -> Result<()> {
    unsafe {
        if let Some(fb) = fb {
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb.handle);
        } else {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        gl::ClearColor(r,g,b,a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
    validate_gl()
}

fn set_state() -> Result<()> {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::DepthFunc(gl::LEQUAL);
        gl::Enable(gl::CULL_FACE);
    }
    validate_gl()
}

fn apply_uniforms(shader : &GlShader, uniforms : &[(&str, GlShaderUniform)]) -> Result<()> {
    let mut texture_counter = 0i32;
    for &(name, ref uniform) in uniforms {
        let result = match uniform {
            &GlShaderUniform::TextureHandle(handle) => {
                unsafe {
                    gl::ActiveTexture((gl::TEXTURE0 as i32 + texture_counter) as GLenum);
                    gl::BindTexture(gl::TEXTURE_2D, handle);
                }
                let result = shader.set_uniform(name, &GlShaderUniform::Int(texture_counter));
                texture_counter += 1;
                result
            }
            uniform => {
                shader.set_uniform(name, uniform)
            }
        };
        match result {
            Ok(()) => (),
            Err(code) => {
                return Err(GlError::new(format!("Error setting {:?} to {:?}: {:?}", name, uniform, code)));
            }
        }
    }
    validate_gl()
}

pub fn render<V>(shader     : &GlShader,
              vertex_array  : &V,
              vertex_count  : i32,
              rendertarget  : &RenderTarget,
              uniforms      : &[(&str, GlShaderUniform)]) -> Result<()> where V : HasGlVertexArrayHandle {
    set_state()?;
    apply_uniforms(shader, uniforms)?;
    unsafe {
        match *rendertarget {
            RenderTarget::Framebuffer(fb) => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fb.handle);
                gl::Viewport(0,0,fb.w as GLsizei,fb.h as GLsizei);
            },
            RenderTarget::Screen(w,h) => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::Viewport(0,0,w as GLsizei,h as GLsizei);
            }
        }

        gl::BindVertexArray(vertex_array.gl_vao_handle());
        gl::UseProgram(shader.program_handle);
        gl::DrawArrays(gl::TRIANGLES, 0, vertex_count);
    }
    validate_gl()
}

pub fn render_indexed<V>(shader        : &GlShader,
                      vertex_array  : &V,
                      index_buffer  : &GlBufferRaw,
                      rendertarget  : &RenderTarget,
                      uniforms      : &[(&str, GlShaderUniform)]) -> Result<()> where V : HasGlVertexArrayHandle {
    set_state()?;
    apply_uniforms(shader, uniforms)?;

    unsafe {
        match *rendertarget {
            RenderTarget::Framebuffer(fb) => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, fb.handle);
                gl::Viewport(0,0,fb.w as GLsizei,fb.h as GLsizei);
            }
            RenderTarget::Screen(w, h) => {
                gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
                gl::Viewport(0,0,w as GLsizei,h as GLsizei);
            }
        }

        gl::BindVertexArray(vertex_array.gl_vao_handle());
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer.buffer_id);
        gl::UseProgram(shader.program_handle);
        gl::DrawElements(gl::TRIANGLES, (index_buffer.el_count * index_buffer.component_count) as i32, index_buffer.gl_type_enum, ptr::null());
    }
    validate_gl()
}





