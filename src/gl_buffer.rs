extern crate gl;

use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use gl_err::*;
use std::marker::PhantomData;

///
/// An OpenGL buffer of arbitrary type.
///
pub struct GlBufferRaw {
    pub buffer_id : gl::types::GLuint,
    pub el_count : usize,
    pub component_count : usize,
    pub gl_type_enum : u32
}

///
/// Typed OpenGL buffer.
///
pub struct GlBuffer<T>(pub GlBufferRaw, PhantomData<T>) where T : GlBufferElementType;

///
/// Typed OpenGL buffer initialized with GL_ELEMENT_ARRAY_BUFFER
///
pub struct GlIndexBuffer<T>(pub GlBufferRaw, PhantomData<T>) where T : GlBufferElementType;

///
/// Trait for types that can be contained in GlBufferRaw or GlBuffer<T>
///
pub trait GlBufferElementType {
    fn gl_type_enum() -> u32;
    fn components_per_element() -> usize;
}

impl GlBufferElementType for GLfloat {
    fn gl_type_enum() -> u32 { gl::FLOAT }
    fn components_per_element() -> usize { 1 }
}
impl GlBufferElementType for u32 {
    fn gl_type_enum() -> u32 { gl::UNSIGNED_INT }
    fn components_per_element() -> usize { 1 }
}
impl GlBufferElementType for [u32;3] {
    fn gl_type_enum() -> u32 { gl::UNSIGNED_INT }
    fn components_per_element() -> usize { 3 }
}
impl GlBufferElementType for u16 {
    fn gl_type_enum() -> u32 { gl::UNSIGNED_SHORT }
    fn components_per_element() -> usize { 1 }
}
impl GlBufferElementType for [f32;2] {
    fn gl_type_enum() -> u32 { gl::FLOAT }
    fn components_per_element() -> usize { 2 }
}
impl GlBufferElementType for [f32;3] {
    fn gl_type_enum() -> u32 { gl::FLOAT }
    fn components_per_element() -> usize { 3 }
}
impl GlBufferElementType for [f32;4] {
    fn gl_type_enum() -> u32 { gl::FLOAT }
    fn components_per_element() -> usize { 4 }
}

impl GlBufferRaw {
    pub fn new<T: GlBufferElementType>(data : &[T], component_count : usize) -> Result<GlBufferRaw> {
        unsafe {
            Self::new_impl_raw(data.as_ptr() as *const c_void,
                               (mem::size_of::<T>() * data.len()) as isize,
                               (T::components_per_element() as usize) * data.len() / component_count,
                               component_count, gl::ARRAY_BUFFER, T::gl_type_enum())
        }
    }
    pub fn new_index<T: GlBufferElementType>(data : &[T]) -> Result<GlBufferRaw> {
        unsafe {
            Self::new_impl_raw(data.as_ptr() as *const c_void,
                         (mem::size_of::<T>() * data.len()) as isize,
                         data.len(), T::components_per_element(), gl::ELEMENT_ARRAY_BUFFER, T::gl_type_enum())
        }
    }

    unsafe fn new_impl_raw(data : *const c_void,
                           data_size : isize,
                           element_count : usize,
                           component_count : usize,
                           buffer_type : GLenum,
                           buffer_element_type : GLenum) -> Result<GlBufferRaw> {
        let mut buffer_id : GLuint = 0;
        gl::GenBuffers(1, &mut buffer_id);
        gl::BindBuffer(buffer_type, buffer_id);
        gl::BufferData(buffer_type, data_size, data, gl::STATIC_DRAW);

        match validate_gl() {
            Err(s) => Err(s),
            Ok(()) => Ok(GlBufferRaw {
                buffer_id: buffer_id,
                el_count: element_count,
                component_count: component_count,
                gl_type_enum: buffer_element_type
            })
        }
    }


}

impl<T> GlBuffer<T> where T : GlBufferElementType {
    pub fn new(data : &[T]) -> Result<GlBuffer<T>> {
        Ok(GlBuffer::<T>(GlBufferRaw::new(data, T::components_per_element())?, PhantomData::<T>))
    }
}

impl<T> GlIndexBuffer<T> where T : GlBufferElementType {
    pub fn new(data : &[T]) -> Result<GlIndexBuffer<T>> {
        Ok(GlIndexBuffer::<T>(GlBufferRaw::new_index(data)?, PhantomData::<T>))
    }
}

impl Drop for GlBufferRaw {
    fn drop (&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.buffer_id);
        }
        validate_gl().unwrap();
        self.buffer_id = 0;
    }
}
