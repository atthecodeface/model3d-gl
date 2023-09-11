use crate::Gl;
use crate::GlShaderType;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};

mod shader;
pub use shader::Shader;
mod program;
pub use program::Program;

mod buffer;

//a Model3DWebGL
//tp Model3DWebGL
#[derive(Debug)]
pub struct Model3DWebGL {
    context: WebGl2RenderingContext,
}

//ip Model3DWebGL
impl Model3DWebGL {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        Self { context }
    }
    pub fn context(&self) -> &WebGl2RenderingContext {
        &self.context
    }
}

//ip Deref for Model3DWebGL
impl std::ops::Deref for Model3DWebGL {
    type Target = WebGl2RenderingContext;
    fn deref(&self) -> &WebGl2RenderingContext {
        &self.context
    }
}

//ip Gl for Model3DWebGL
//ip model3d_base::Renderable for Model3DWebGL
impl model3d_base::Renderable for Model3DWebGL {
    type Context = Self;
    type Buffer = buffer::Buffer;
    type View = crate::BufferView<Self, buffer::Buffer>;
    type Texture = crate::Texture;
    type Material = crate::Material;
    type Vertices = crate::Vertices<Self, buffer::Buffer>;
}
impl Gl for Model3DWebGL {
    type Id = u32;
    type Shader = Shader;
    type Program = Program;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(&self, srcs: &[&Shader]) -> Result<Program, String> {
        Program::link_program(&self.context, srcs)
    }

    //fp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String> {
        Shader::compile(&self.context, source, shader_type)
    }

    //fp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>) {
        if let Some(program) = program {
            program.set_used(&self.context);
        } else {
            self.context.use_program(None);
        }
    }
}
