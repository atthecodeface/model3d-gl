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

@file    program.rs
@brief   Part of WebGL support library
 */

//a Documentation

/*!

A shader program consists of a number of [webgl::Shader]s linked together

The attributes and uniforms can be accessed in a simple common manner
!*/

//a Imports
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};

use super::Shader;
use crate::{GlProgram, GlShader, UniformId};

//a Program
//tp Program
/// A shader program, with its 'known' attributes and uniforms
pub struct Program {
    /// The GL ID of the program
    program: WebGlProgram,
    /// attribute names
    attributes: Vec<(i32, model3d_base::VertexAttr)>,
    /// attribute names
    uniforms: Vec<(WebGlUniformLocation, UniformId)>,
}

///ip Program
impl Program {
    //fp compile_program
    /// Compile a program from a slice of kind/source pairs
    pub fn link_program(
        context: &WebGl2RenderingContext,
        shaders: &[&Shader],
    ) -> Result<Program, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader program"))?;

        for shader in shaders {
            context.attach_shader(&program, shader.id());
        }
        context.link_program(&program);

        if !context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")));
        }

        let attributes = Vec::new();
        let uniforms = Vec::new();
        Ok(Program {
            program,
            attributes,
            uniforms,
        })
    }

    //mp add_attr_name
    /// Add an attribute to the [Program] from its name (that should be in the shader source)
    pub fn add_attr_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        vertex_attr: model3d_base::VertexAttr,
    ) -> Result<&mut Self, String> {
        let attr_index = context.get_attrib_location(&self.program, name);
        if attr_index < 0 {
            Err(format!("Unable to find attribute {} in program", name))
        } else {
            self.attributes.push((attr_index, vertex_attr));
            Ok(self)
        }
    }

    //mp add_uniform_name
    /// Add a uniform to the [Program] from its name (that should be in the shader source)
    pub fn add_uniform_name(
        &mut self,
        context: &WebGl2RenderingContext,
        name: &str,
        uniform_id: UniformId,
    ) -> Result<&mut Self, String> {
        if let Some(uniform_index) = context.get_uniform_location(&self.program, name) {
            self.uniforms.push((uniform_index, uniform_id));
            Ok(self)
        } else {
            Err(format!("Unable to find uniform {} in program", name))
        }
    }

    //mp add_uniform_buffer_name
    // Add a uniform buffer (or 'block') to the [Program] from its name (that should be in the shader source)
    // pub fn add_uniform_buffer_name(
    //     &mut self,
    //     context: &WebGl2RenderingContext,
    //     name: &str,
    //     id: usize,
    // ) -> Result<&mut Self, String> {
    //     let uniform_index: str = context.get_uniform_block_index(&self.program, name);
    //     if uniform_index == gl::INVALID_INDEX {
    //         Err(format!("Unable to find uniform block {} in program", name))
    //     } else {
    //         self.uniforms.push((0, UniformId::Buffer(id)));
    //         Ok(self)
    //     }
    // }

    //fp set_used
    pub fn set_used(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }
}

//ip GlProgram for Program
impl GlProgram for Program {
    type GlAttrId = i32;
    type GlUniformId<'a> = &'a WebGlUniformLocation;
    // type Context = WebGl2RenderingContext;
    fn attributes(&self) -> &[(i32, model3d_base::VertexAttr)] {
        &self.attributes
    }
    fn uniform<'a>(&'a self, uniform_id: UniformId) -> Option<Self::GlUniformId<'a>> {
        for (gl_id, u) in &self.uniforms {
            if *u == uniform_id {
                return Some(gl_id);
            }
        }
        None
    }
}
