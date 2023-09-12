//a Imports
use super::{Model3DWebGL, Program};
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
    //    gl_vao: u32,
}

//ip Vao
impl Vao {
    //fp new
    pub fn new(
        context: &mut Model3DWebGL,
        program: &Program,
        vertices: &Vertices<Model3DWebGL>,
    ) -> Self {
        Self {}
    }

    //fp bind_vao
    pub fn bind_vao(&self) {}

    //fp create_from_indices
    pub fn create_from_indices(
        _context: &Model3DWebGL,
        indices: &crate::IndexBuffer<Model3DWebGL>,
    ) -> Result<Self, String> {
        Ok(Self {})
    }
}

//ip GlVao for Vao
impl crate::GlVao for Vao {}
