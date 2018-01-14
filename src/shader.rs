
use gl_vertex_array::GlVertexArray;
use gl_render::RenderTarget;
use gl_buffer::GlBufferRaw;
use gl_err::*;

pub trait Shader<'a> {
    type Attribs;
    type Uniforms;

    fn new() -> Self;
    fn create_va(attribs : Self::Attribs) -> Result<GlVertexArray>;
    fn render(&self,
              vertex_array     : &GlVertexArray,
              rendertarget     : &RenderTarget,
              uniforms         : Self::Uniforms) -> Result<()>;
    fn render_indexed(&self,
                      vertex_array     : &GlVertexArray,
                      index_buffer     : &GlBufferRaw,
                      rendertarget     : &RenderTarget,
                      uniforms         : Self::Uniforms) -> Result<()>;

}

