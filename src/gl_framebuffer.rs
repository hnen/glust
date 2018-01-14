extern crate gl;
use gl::types::*;
use gl_err::*;
use gl_texture::GlTexture;

pub struct GlFramebuffer {
    pub handle : GLuint,
    pub textures : Vec<GlTexture>,
    pub w : usize,
    pub h : usize,
    _depth_handle : Option<GLuint>
}

impl GlFramebuffer {
    pub fn new_with_depth(w : usize, h : usize, textures : Vec<GlTexture>) -> Result<GlFramebuffer> {
        unsafe {
            let mut fb_handle : GLuint = 0;
            gl::GenFramebuffers(1, &mut fb_handle);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fb_handle);

            let depth = {
                let mut depth_handle : GLuint = 0;
                gl::GenRenderbuffers(1, &mut depth_handle);
                gl::BindRenderbuffer(gl::RENDERBUFFER, depth_handle);
                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, w as GLsizei, h as GLsizei);
                depth_handle
            };
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth);

            let mut attachments = Vec::new();
            for (i,texture) in textures.iter().enumerate() {
                let attachment_enum = (gl::COLOR_ATTACHMENT0 as usize + i) as GLenum;
                gl::BindTexture(gl::TEXTURE_2D, texture.handle);
                //gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment_enum, gl::TEXTURE_2D, textures[i].handle, 0);
                gl::FramebufferTexture(gl::FRAMEBUFFER, attachment_enum, texture.handle, 0);
                attachments.push(attachment_enum);
            }
            gl::DrawBuffers(attachments.len() as GLsizei, attachments.as_ptr());

            match validate_gl() {
                Ok(()) => match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                    gl::FRAMEBUFFER_COMPLETE => {
                        Ok(GlFramebuffer{
                            handle : fb_handle,
                            _depth_handle: None,
                            textures: textures,
                            w : w,
                            h : h
                        })
                    },
                    fb_incomplete_state => Err(GlError::new(format!("Framebuffer status not complete: {}", fb_incomplete_state)))
                },
                Err(x) => Err(x)
            }
        }
    }
}

impl Drop for GlFramebuffer {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.handle);
        }
        validate_gl().unwrap();
        self.handle  = 0;
    }
}

