extern crate gl;
use gl::types::*;
use gl_err::*;

use std::ptr;
use std::ffi::CString;

#[derive(Debug, Clone, Copy)]
pub enum GlShaderUniform {
    Mat4x4([f32;16]),
    Int(i32),
    TextureHandle(GLuint),
    Vec2([f32;2]),
    Vec3([f32;3]),
    Vec4([f32;4]),
    Float(f32)
}

#[derive(Debug)]
pub struct GlShader {
    pub program_handle : gl::types::GLuint
}

#[derive(Debug)]
pub struct UniformInfo {
    pub name : String,
    pub datatype : GLenum,
    pub size : i32
}

#[derive(Debug)]
pub struct AttribInfo {
    pub name : String,
    pub datatype : GLenum,
    pub size : i32,
    pub location : i32
}

#[derive(Debug)]
pub struct FragOutputInfo {
    pub name : String,
    pub datatype : GLenum,
    pub location : i32,
    pub index : GLuint
}

impl GlShader {

    pub fn compile(vs_source : &str, fs_source : &str) -> Result<GlShader> {

        let vs_handle = load_shader_prog(gl::VERTEX_SHADER, vs_source)?;
        let fs_handle = load_shader_prog(gl::FRAGMENT_SHADER, fs_source)?;

        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vs_handle);
            gl::AttachShader(program_id, fs_handle);
            gl::LinkProgram(program_id);

            let mut link_result : gl::types::GLint = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut link_result);

            let mut log_length = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_length);
            let error_str = if log_length > 0 {
                let mut error_msg = Vec::with_capacity((log_length+1) as usize);
                for _ in 1..log_length { error_msg.push(0); }
                let error_msg_ptr = error_msg.as_mut_ptr();
                gl::GetProgramInfoLog(program_id, log_length, ptr::null_mut(), error_msg_ptr);
                let s = String::from_utf8(error_msg.iter().map(|&c| c as u8).collect())?;
                println!("Info log: {:?}", s);
                s
            } else {
                String::from("")
            };

            if !error_str.is_empty() {
                Err(GlError::new(error_str))
            } else {
                match link_result {
                    0 => {
                        Err(GlError::new(error_str))
                    }
                    _ => {
                        Ok(program_id)
                    }
                }
            }
        }?;

        unsafe {
            gl::DetachShader(program_id, vs_handle);
            gl::DetachShader(program_id, fs_handle);

            gl::DeleteShader(vs_handle);
            gl::DeleteShader(fs_handle);
        }

        let shader = GlShader{ program_handle: program_id };
        //println!("Created program, uniforms: {}", shader.get_number_of_uniforms());

        Ok(shader)
    }

    pub fn set_uniform(&self, uniform_name : &str, uniform : &GlShaderUniform) -> Result<()> {
        let uniform_name_c = CString::new(uniform_name)?;
        let uniform_id = unsafe {
            gl::GetUniformLocation(self.program_handle, uniform_name_c.as_ptr())
        };

        if uniform_id == -1 {
            return Err(GlError::new(format!("Error getting uniform location: {:?}, note that unused uniforms are stripped out", uniform_name)));
        }

        unsafe {
            gl::UseProgram(self.program_handle);
        }

        match *uniform {
            GlShaderUniform::Mat4x4(data) =>
                unsafe {
                    gl::UniformMatrix4fv(uniform_id, 1, gl::TRUE, data.as_ptr());
                },
            GlShaderUniform::Vec3(data) =>
                unsafe {
                    gl::Uniform3fv(uniform_id, 1, data.as_ptr());
                },
            GlShaderUniform::Int(data) =>
                unsafe {
                    gl::Uniform1i(uniform_id, data);
                },
            GlShaderUniform::TextureHandle(_) => {
                return Err(GlError::new("Can't set texture handle here. Needs renderer.".to_string()))
                ;
            },
            _ => return Err(GlError::new(format!("unimplemented datatype {:?}", uniform)))
        };
        validate_gl()
    }

    pub fn get_uniform_infos(&self) -> Result<Vec<UniformInfo>> {
        let num_uniforms = self.get_number_of_uniforms()?;
        let mut ret = Vec::with_capacity(num_uniforms as usize);
        for i in 0..num_uniforms {
            unsafe {
                let mut uniform_name_buf = Vec::with_capacity(1024 as usize);
                for _ in 1..1024 { uniform_name_buf.push(0); }
                let mut uniform_name_len : i32 = 0;
                let mut uniform_size : i32 = 0;
                let mut uniform_type : GLenum = 0 as GLenum;
                gl::GetActiveUniform(self.program_handle, i, 1024, &mut uniform_name_len, &mut uniform_size, &mut uniform_type,
                                     uniform_name_buf.as_mut_ptr());
                uniform_name_buf.resize(uniform_name_len as usize, 0);
                let name_str = String::from_utf8(uniform_name_buf.iter().map(|&c| c as u8).collect())?;
                ret.push(UniformInfo {
                    name: name_str,
                    datatype: uniform_type,
                    size: uniform_size
                });
            }
        }
        Ok(ret)
    }

    fn get_number_of_uniforms(&self) -> Result<u32>
     {
        unsafe {
            let mut num_uniforms : GLint = 0;
            gl::GetProgramiv(self.program_handle, gl::ACTIVE_UNIFORMS, &mut num_uniforms);
            validate_gl()?;
            Ok(num_uniforms as u32)
        }
    }

    pub fn get_attrib_infos_sorted(&self) -> Result<Vec<AttribInfo>> {
        let num_attribs = self.get_number_of_attributes()?;
        let mut ret = Vec::with_capacity(num_attribs as usize);
        for i in 0..num_attribs {
            unsafe {
                let mut attrib_name_buf = Vec::with_capacity(1024 as usize);
                for _ in 1..1024 { attrib_name_buf.push(0); }
                let mut attrib_name_len : i32 = 0;
                let mut attrib_size : i32 = 0;
                let mut attrib_type : GLenum = 0 as GLenum;
                gl::GetActiveAttrib(self.program_handle, i, 1024, &mut attrib_name_len, &mut attrib_size, &mut attrib_type,
                                     attrib_name_buf.as_mut_ptr());
                attrib_name_buf.resize(attrib_name_len as usize, 0);
                let name_str = String::from_utf8(attrib_name_buf.iter().map(|&c| c as u8).collect())?;
                let location = gl::GetAttribLocation(self.program_handle, attrib_name_buf.as_ptr());
                ret.push(AttribInfo {
                    name: name_str,
                    datatype: attrib_type,
                    size: attrib_size,
                    location: location
                });
            }
        }

        ret.sort_by(|a,b| a.location.cmp(&b.location) );

        Ok(ret)
    }

    fn get_number_of_attributes(&self) -> Result<u32> {
        unsafe {
            let mut num_attribs : GLint = 0;
            gl::GetProgramiv(self.program_handle, gl::ACTIVE_ATTRIBUTES, &mut num_attribs);
            validate_gl()?;
            Ok(num_attribs as u32)

        }
    }

    pub fn get_frag_outputs_sorted(&self) -> Result<Vec<FragOutputInfo>> {
        let num_outputs = self.get_number_of_frag_outputs()?;

        let mut outputs = Vec::new();

        unsafe {
            for i in 0..num_outputs {
                let mut name_buf = Vec::with_capacity(1024 as usize);
                for _ in 1..1024 { name_buf.push(0); }
                let mut name_len = 0;

                println!("getProgramResourceName");
                gl::GetProgramResourceName(self.program_handle, gl::PROGRAM_OUTPUT, i, 1024, &mut name_len, name_buf.as_mut_ptr());

                println!("getProgramResourceIndex");
                let index = gl::GetProgramResourceIndex(self.program_handle, gl::PROGRAM_OUTPUT, name_buf.as_ptr());

                let name_str = String::from_utf8(name_buf.iter().map(|&c| c as u8).collect())?;

                outputs.push(FragOutputInfo {
                    name: name_str,
                    datatype: 0,
                    location: -1,
                    index: index
                });

            }
        }
        Ok(outputs)
    }

    fn get_number_of_frag_outputs(&self) -> Result<u32> {
        unsafe {
            let mut num_frag_outputs : GLint = 0;
            println!("getProgramInteraceiv");
            gl::GetProgramInterfaceiv(self.program_handle, gl::PROGRAM_OUTPUT, gl::ACTIVE_RESOURCES, &mut num_frag_outputs);
            validate_gl()?;
            Ok(num_frag_outputs as u32)
        }
    }

}

impl Drop for GlShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_handle);
        }
        validate_gl().unwrap();
        self.program_handle = 0;
    }
}

fn load_shader_prog(shader_type : gl::types::GLenum, source : &str) -> Result<gl::types::GLuint> {
    unsafe {
        let id_shader = gl::CreateShader(shader_type);

        let vs_source_c = CString::new(source).unwrap();
        let vs_source_arr_c = Box::into_raw(Box::new(vs_source_c.as_ptr()));
        gl::ShaderSource(id_shader, 1, vs_source_arr_c, ptr::null());
        gl::CompileShader(id_shader);

        let mut result : gl::types::GLint = 0;

        gl::GetShaderiv(id_shader, gl::COMPILE_STATUS, &mut result);
        match result {
            0 => {
                let mut log_length = 0;
                gl::GetShaderiv(id_shader, gl::INFO_LOG_LENGTH, &mut log_length);
                let mut error = Vec::with_capacity((log_length+1) as usize);
                for _ in 1..log_length { error.push(0); }
                let error_ptr = error.as_mut_ptr();
                gl::GetShaderInfoLog(id_shader, log_length, &mut log_length, error_ptr);
                let error_str = String::from_utf8(error.iter().map(|&c| c as u8).collect()).unwrap();
                Err(GlError::new(error_str))
            }
            _ => {
                Ok(id_shader)
            }
        }
    }
}
