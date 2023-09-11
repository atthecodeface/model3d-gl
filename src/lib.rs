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

@file    lib.rs
@brief   OpenGL/WebGL model and shader program abstraction library
 */

//a Documentation
// #![warn(missing_docs)]
// Document code examples cannot be executed
// so don't require them right now
//
// #![warn(rustdoc::missing_doc_code_examples)]

/*!
# OpenGL/WeblGL Model / Shader Program abstraction library

This library provides structures for OpenGL shaders ...
!*/

//a Imports and exports
pub use model3d_base::{Mat3, Mat4, Quat, Transformation, Vec3, Vec4};

//tp UniformId
/// An enumeration of uniforms - that this crate particularly cares about
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UniformId {
    /// The view matrix uniform - once per framebuffer render
    ViewMatrix,
    /// The model matrix uniform - once per model instance
    ModelMatrix,
    /// The mesh matrix uniform - once per model mesh
    MeshMatrix,
    /// The Bone data uniform - once per model
    BoneScale,
    /// The Bone data uniform - once per model
    BoneMatrices,
    /// User uniform - dependent on the program.
    User(usize),
    /// User uniform buffer - dependent on the program.
    Buffer(usize),
}

//tp GlShader
pub trait GlShader: Sized {
    type Id<'a>: Sized + 'a
    where
        Self: 'a;
    //fp id
    /// Get the shader program id
    fn id<'a>(&'a self) -> Self::Id<'a>;
}

//tt GlProgram
pub trait GlProgram: Sized {
    type GlAttrId: Sized;
    // type Context;
    type GlUniformId<'a>: Sized + 'a
    where
        Self: 'a;
    /// Borrow a slice of attribute / program attribute location pairings
    fn attributes(&self) -> &[(Self::GlAttrId, model3d_base::VertexAttr)];

    /// Attempt to retrieve a uniform from a [UniformId] - return None
    /// if the shader program does not have that uniform
    fn uniform<'a>(&'a self, uniform_id: UniformId) -> Option<Self::GlUniformId<'a>>;
}

//tt GlShaderType
pub enum GlShaderType {
    Vertex,
    Fragment,
}

//tt GlBuffer
/// The GlBuffer is something that is the Gl context's static draw
/// copy of a [u8] that forms the values for vertices and indices etc.
///
/// A single GlBuffer will be cloned for different
/// model3d_base::BufferView of the same BufferData (by the
/// [VertexBuffer] type)
pub trait GlBuffer<G>: Default + Clone + std::fmt::Debug + model3d_base::BufferClient<G> {}

//tt Gl
pub trait Gl {
    type Id: Sized;
    type Shader: GlShader;
    type Program: GlProgram;
    type Buffer: GlBuffer<Self>;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(&self, srcs: &[&Self::Shader]) -> Result<Self::Program, String>;

    //fp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String>;

    //fp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>);
}

//a Model3DWebGL
mod webgl;
pub use webgl::Model3DWebGL;

mod material;
mod texture;
pub use material::Material;
pub use texture::Texture;

mod buffer;
pub use buffer::{BufferView, IndexBuffer, VertexBuffer};

mod vertices;
pub use vertices::Vertices;

// mod gl_buffer;
// mod renderable;
// mod shader_instantiable;
// // mod traits;
// // mod utils;

// pub use gl_buffer::GlBuffer;
// pub use material::Material;
// pub use renderable::{RenderContext, Renderable};
// pub use shader_instantiable::ShaderInstantiable;
// pub use texture::Texture;
// pub use traits::ShaderClass;
// pub use utils::{check_errors, get_programiv, get_shader_error, get_shaderiv};

// mod program;
// mod shader;
// pub use program::Program as GlProgram;
// pub use program::UniformId;
// pub use shader::GlShader;
