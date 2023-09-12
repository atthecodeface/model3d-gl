//a Imports
use super::utils;
use super::{Model3DOpenGL, Program};
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
    //fp new
    pub fn new(
        context: &mut Model3DOpenGL,
        program: &Program,
        vertices: &Vertices<Model3DOpenGL>,
    ) -> Self {
        let (indices, position, attrs) = vertices.borrow();
        utils::check_errors().unwrap();
        let mut gl_vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::BindVertexArray(gl_vao);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indices.gl_buffer().gl_buffer());
            println!("VAO {} {:?}", gl_vao, indices);
        }
        utils::check_errors().expect("Added indices to VAO");
        for (index, vertex_attr) in program.attributes() {
            if *vertex_attr == model3d_base::VertexAttr::Position {
                println!(".. posn {} {}", *index, position);
                position.bind_to_vao_attr(context, index);
                utils::check_errors().unwrap();
            } else {
                for (va, buffer) in attrs {
                    if *vertex_attr == *va {
                        println!(".. {:?} {} {}", *vertex_attr, *index, buffer);
                        buffer.bind_to_vao_attr(context, index);
                    }
                    utils::check_errors().unwrap();
                }
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }
        utils::check_errors().unwrap();
        Self { gl_vao }
    }

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
