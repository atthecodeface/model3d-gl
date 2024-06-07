//a Imports
use super::utils;
use super::{Model3DOpenGL};
use crate::{GlProgram, Vertices};

//a Vao
//tp Vao
/// The [Vao] *must* be owned by a [ShaderInstantiable], which borrows
/// from the Instantiable, which owns the GL buffers for the indices
/// and vertices etc
///
/// Because of this the [Vao] cannot outlive the [ShaderInstantiable], which
/// cannot outlive the GL buffer for the vertices and indices etc
pub struct Vao {
    gl_vao: u32,
}

//ip Vao
impl Vao {
    //fp bind_vao
    pub fn bind_vao(&self) {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }
    }

    //fp create_from_indices
    pub fn create_from_indices(
        _context: &Model3DOpenGL,
        indices: &crate::IndexBuffer<Model3DOpenGL>,
    ) -> Result<Self, ()> {
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices.gl_buffer().gl_buffer());
            println!("VAO {} {:?}", gl_vao, indices);
        }
        utils::check_errors().expect("Added indices to VAO");
        Ok(Self { gl_vao })
    }
}

//ip GlVao for Vao
impl crate::GlVao for Vao {}
