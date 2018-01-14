#![feature(try_trait)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate gl;

mod gl_shaders;
mod gl_buffer;
mod gl_framebuffer;
mod gl_vertex_array;
mod gl_texture;
mod gl_err;
mod shader;
mod gl_render;

pub use gl_shaders::AttribInfo;
pub use gl_shaders::UniformInfo;
pub use gl_shaders::GlShader;
pub use gl_shaders::GlShaderUniform;

pub use gl_buffer::GlBufferRaw;
pub use gl_buffer::GlBufferElementType;
pub use gl_buffer::GlBuffer;
pub use gl_buffer::GlIndexBuffer;

pub use gl_framebuffer::GlFramebuffer;
pub use gl_vertex_array::GlVertexArray;
pub use gl_vertex_array::GlVertexArrayTmp;
pub use gl_vertex_array::HasGlVertexArrayHandle;
pub use gl_texture::GlTexture;

pub use gl_render::RenderTarget;
pub use gl_render::render;
pub use gl_render::render_indexed;
pub use gl_render::clear;

pub use gl_err::validate_gl;
pub use gl_err::GlError;

pub use shader::Shader;
