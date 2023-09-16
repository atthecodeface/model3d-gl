//a Imports
use super::{Model3DWebGL, Program};
use crate::{GlProgram, Vertices};
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use crate::console_log;
use crate::webgl_log::log_gl_vao;

//a Vao
//tp Vao
/// The [Vao] *must* be owned by a [ShaderInstantiable], which borrows
/// from the Instantiable, which owns the GL buffers for the indices
/// and vertices etc
///
/// Because of this the [Vao] cannot outlive the [ShaderInstantiable], which
/// cannot outlive the GL buffer for the vertices and indices etc
///
/// The VAO is Boxed as then &WebGlVertexArrayObject is constant for debug
#[derive(Debug)]
pub struct Vao {
    gl_vao: Box<WebGlVertexArrayObject>,
}

//ip Vao
impl Vao {
    //fp bind_vao
    pub fn bind_vao(&self, render_context: &Model3DWebGL) {
        render_context.bind_vertex_array(Some(&self.gl_vao));
        log_gl_vao(render_context, Some(&self.gl_vao), "Vao::bind_vao");
    }

    //fp create_from_indices
    /// This creates a VAO, and attaches the indices, leaving the VAO bound
    pub fn create_from_indices(
        render_context: &Model3DWebGL,
        indices: &crate::IndexBuffer<Model3DWebGL>,
    ) -> Result<Self, ()> {
        let gl_vao = render_context.create_vertex_array().unwrap().into();
        let vao = Self { gl_vao };
        render_context.bind_vertex_array(Some(&vao.gl_vao));
        indices
            .gl_buffer()
            .bind_to_context_buffer(render_context, WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER);
        log_gl_vao(
            render_context,
            Some(&vao.gl_vao),
            &format!("Vao::create_from_indices {indices}"),
        );
        Ok(vao)
    }
}

//ip GlVao for Vao
impl crate::GlVao for Vao {}
