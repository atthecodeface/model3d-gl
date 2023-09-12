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

//a Imports
use super::utils;
use crate::GlShaderType;
use gl;
use std;
use std::ffi::CString;

//a Shader
//tp Shader
/// An OpenGL shader, of any kind, which can be created from source.
///
/// A number of shaders are linked together to make a program; once
/// the program has been linked, the shader can be dropped.
pub struct Shader {
    /// The GL ID of the shader
    id: gl::types::GLuint,
}

//ip Shader
impl Shader {
    //fp compile
    /// Create a shader of a particular kind from source
    pub fn compile(source: &str, shader_type: GlShaderType) -> Result<Self, String> {
        let source = CString::new(source).unwrap();
        let (shader_type, shader_kind) = match shader_type {
            GlShaderType::Fragment => (gl::FRAGMENT_SHADER, "fragment"),
            GlShaderType::Vertex => (gl::VERTEX_SHADER, "vertex"),
        };
        let id = unsafe {
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            id
        };

        if utils::get_shaderiv(id, gl::COMPILE_STATUS) == 0 {
            let err = utils::get_shader_error(
                id,
                |id| utils::get_shaderiv(id, gl::INFO_LOG_LENGTH),
                |id, len, buf| unsafe { gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buf) },
            );
            Err(format!("Error compiling {} shader {}", shader_kind, err))
        } else {
            Ok(Self { id })
        }
    }

    //fp id
    /// Get the shader program id
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

//ip Drop for Shader
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
//ip GlShader for Shader
impl crate::GlShader for Shader {
    type Id<'a> = gl::types::GLuint;
    //fp id
    /// Get the shader program id
    fn id<'a>(&'a self) -> gl::types::GLuint {
        self.id
    }
}
