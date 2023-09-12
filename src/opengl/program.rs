/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    shader.rs
@brief   Part of OpenGL support library
 */

//a Documentation

/*!

A shader program consists of a number of [GlShader]s linked together

!*/

//a Imports
use std::ffi::CString;

use super::utils;
use super::Shader;
use crate::UniformId;

//a Program
//tp Program
/// A shader program, with its 'known' attributes and uniforms
pub struct Program {
    /// The GL ID of the program
    id: gl::types::GLuint,
    /// attribute names
    attributes: Vec<(gl::types::GLuint, model3d_base::VertexAttr)>,
    /// attribute names
    uniforms: Vec<(gl::types::GLint, UniformId)>,
}

///ip Program
impl Program {
    //fp link_program
    /// Compile a program from a slice of kind/source pairs
    pub fn link_program(shaders: &[&Shader]) -> Result<Program, String> {
        let program_id = unsafe {
            let program_id = gl::CreateProgram();
            for shader in shaders {
                gl::AttachShader(program_id, shader.id());
            }
            gl::LinkProgram(program_id);
            program_id
        };

        if utils::get_programiv(program_id, gl::LINK_STATUS) == 0 {
            let err = utils::get_shader_error(
                program_id,
                |id| utils::get_programiv(id, gl::INFO_LOG_LENGTH),
                |id, len, buf| unsafe { gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buf) },
            );
            Err(format!(
                "Unable to create shader program, linking error {}",
                err
            ))?;
        }
        utils::check_errors().expect("Linked");

        //         for shader in shaders {
        //             unsafe {
        //                 gl::DetachShader(program_id, shader.id());
        //                 // Don't delete the shader - that happens when the shader is dropped
        //             }
        //         }

        let attributes = Vec::new();
        let uniforms = Vec::new();
        Ok(Program {
            id: program_id,
            attributes,
            uniforms,
        })
    }

    //mp add_attr_name
    /// Add an attribute to the [Program] from its name (that should be in the shader source)
    pub fn add_attr_name(
        &mut self,
        name: &str,
        vertex_attr: model3d_base::VertexAttr,
    ) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let attr_index = unsafe { gl::GetAttribLocation(self.id, name_c.as_ptr()) };
        if attr_index < 0 {
            Err(format!("Unable to find attribute {} in program", name))
        } else {
            self.attributes
                .push((attr_index as gl::types::GLuint, vertex_attr));
            Ok(self)
        }
    }

    //mp add_uniform_name
    /// Add a uniform to the [Program] from its name (that should be in the shader source)
    pub fn add_uniform_name(
        &mut self,
        name: &str,
        uniform_id: UniformId,
    ) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let uniform_index = unsafe { gl::GetUniformLocation(self.id, name_c.as_ptr()) };
        if uniform_index == (gl::INVALID_INDEX as i32) {
            Err(format!("Unable to find uniform {} in program", name))
        } else {
            self.uniforms
                .push((uniform_index as gl::types::GLint, uniform_id));
            Ok(self)
        }
    }

    //mp add_uniform_buffer_name
    /// Add a uniform buffer (or 'block') to the [Program] from its name (that should be in the shader source)
    pub fn add_uniform_buffer_name(&mut self, name: &str, id: usize) -> Result<&mut Self, String> {
        let name_c = CString::new(name).unwrap();
        let uniform_index = unsafe { gl::GetUniformBlockIndex(self.id, name_c.as_ptr()) };
        if uniform_index == gl::INVALID_INDEX {
            Err(format!("Unable to find uniform block {} in program", name))
        } else {
            self.uniforms
                .push((uniform_index as gl::types::GLint, UniformId::Buffer(id)));
            Ok(self)
        }
    }

    //fp id
    /// Get the program id
    #[inline]
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    //fp set_used
    /// Use the program
    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }
}

//ip Drop for Program
impl Drop for Program {
    //fp drop
    /// Drop requires the GLProgram to be deleted
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    //zz All done
}

//ip GlProgram for Program
impl crate::GlProgram for Program {
    type GlAttrId = gl::types::GLuint;
    type GlUniformId<'a> = gl::types::GLint;
    fn attributes(&self) -> &[(gl::types::GLuint, model3d_base::VertexAttr)] {
        &self.attributes
    }
    fn uniform(&self, uniform_id: UniformId) -> Option<gl::types::GLint> {
        for (gl_id, u) in &self.uniforms {
            if *u == uniform_id {
                return Some(*gl_id);
            }
        }
        None
    }
}
