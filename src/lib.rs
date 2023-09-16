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
pub trait GlBuffer: Default + Clone + std::fmt::Debug + model3d_base::BufferClient {}

//tt GlVao
/// The GlVao correlates to an OpenGl VAO buffer for a ShaderInstantiable mesh + GlProgram
pub trait GlVao: Sized {}

//tt Gl
/// This must provide Debug as Rust requires a type that is generic on
/// a type of trait [Gl] to have that generic support Debug in order
/// to derive Debug on the type.
///
/// The same is true of Clone, but that is too burdensome for Gl
pub trait Gl:
    model3d_base::Renderable<
        Buffer = <Self as Gl>::Buffer,
        Vertices = Vertices<Self>,
        View = BufferView<Self>,
    > + std::fmt::Debug
{
    // Lose Id?
    type Id: Sized;
    type Shader: GlShader;
    type Program: GlProgram;
    type Buffer: GlBuffer;
    type Vao: GlVao;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(
        &self,
        srcs: &[&Self::Shader],
        named_attrs: &[(&str, model3d_base::VertexAttr)],
        named_uniforms: &[(&str, UniformId)],
        named_uniform_buffers: &[(&str, usize)],
    ) -> Result<Self::Program, String>;

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

    //mp init_buffer_of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &model3d_base::BufferView<Self>,
    );

    //mp uniform_buffer_create
    /// Create a uniform buffer (a GlBuffer in the GPU bound to GlUniformBuffer)
    ///
    /// Fill the data; if is_dynamic is true then make it dynamic draw
    fn uniform_buffer_create<F: Sized>(
        &mut self,
        _data: &[F],
        _is_dynamic: bool,
    ) -> Result<UniformBuffer<Self>, ()>;

    //mp uniform_buffer_update_data
    /// Update (a portion) of a uniform GlBuffer
    fn uniform_buffer_update_data<F: std::fmt::Debug>(
        &mut self,
        _uniform_buffer: &UniformBuffer<Self>,
        _data: &[F],
        _byte_offset: u32,
    );

    //mp uniform_index_of_range
    /// Set the GPU's UniformBlockMatrix index N to a range of a UniformBuffer
    fn uniform_index_of_range(
        &mut self,
        _uniform_buffer: &UniformBuffer<Self>,
        _gl_uindex: u32,
        _byte_offset: usize,
        _byte_length: usize,
    );

    //fp vao_create_from_indices
    /// Create a VAO, add the indices as its element array buffer, and
    /// leave it bound
    fn vao_create_from_indices(
        &mut self,
        indices: &crate::IndexBuffer<Self>,
    ) -> Result<Self::Vao, ()>;

    //fp buffer_bind_to_vao_attr
    /// With the currently bound VAO add this view of the specified
    /// buffer as an attribute of the program, if the program has that
    /// attribute
    fn buffer_bind_to_vao_attr(
        &mut self,
        buffer: &<Self as Gl>::Buffer,
        attr_id: &<<Self as Gl>::Program as GlProgram>::GlAttrId,
        count: u32,
        ele_type: model3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    );

    //fp program_set_uniform_mat4
    fn program_set_uniform_mat4(
        &mut self,
        program: &Self::Program,
        id: crate::UniformId,
        mat4: &Mat4,
    );

    //mp program_bind_uniform_index
    fn program_bind_uniform_index(
        &mut self,
        program: &<Self as Gl>::Program,
        uniform_buffer_id: usize,
        gl_uindex: u32,
    ) -> Result<(), ()>;

    //fp draw_primitive
    /// Draw the specified primitive using its VAO index into the vaos slice
    fn draw_primitive(&mut self, vaos: &[Self::Vao], primitive: &model3d_base::Primitive);

    //fp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>);
}

//a Submodules
mod material;
mod texture;
pub use material::Material;
pub use texture::Texture;

mod buffer;
pub use buffer::{BufferView, IndexBuffer, UniformBuffer, VertexBuffer};

mod vertices;
pub use vertices::Vertices;

mod shader_instantiable;
pub use shader_instantiable::ShaderInstantiable;

// mod program;
// mod shader;
// pub use program::Program as GlProgram;
// pub use program::UniformId;
// pub use shader::GlShader;

// mod renderable;
// mod shader_instantiable;

//a Model3DWebGL
#[cfg(feature = "webgl")]
mod webgl;
#[cfg(feature = "webgl")]
mod webgl_log;
#[cfg(feature = "webgl")]
pub use webgl::Model3DWebGL;

//a Model3DOpenGL
#[cfg(feature = "opengl")]
mod opengl;
#[cfg(feature = "opengl")]
pub use opengl::utils as opengl_utils;
#[cfg(feature = "opengl")]
pub use opengl::Model3DOpenGL;
