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
impl Gl for Model3DWebGL {
    type Id = u32;
    type Shader = Shader;
    type Program = Program;
    type Buffer = buffer::Buffer;

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
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &model3d_base::BufferView<Self>,
    ) {
        buffer.of_indices(view, self);
    }
}

//ip model3d_base::Renderable for Model3DWebGL
impl model3d_base::Renderable for Model3DWebGL {
    type Buffer = buffer::Buffer;
    type View = crate::BufferView<Self>;
    type Texture = crate::Texture;
    type Material = crate::Material;
    type Vertices = crate::Vertices<Self>;

    /// Initialize a BufferData client
    ///
    /// This may be called multiple times for the same [BufferData]; if the
    /// gl buffer is 0 then create, else it already exists with the same data
    fn init_buffer_data_client(
        &mut self,
        client: &mut Self::Buffer,
        buffer_data: &model3d_base::BufferData<Self>,
    ) {
        if client.is_none() {
            client.of_data(buffer_data, self)
        }
    }

    /// Initialize a buffer view client
    fn init_buffer_view_client(
        &mut self,
        client: &mut Self::View,
        buffer_view: &model3d_base::BufferView<Self>,
        attr: model3d_base::VertexAttr,
    ) {
        client.init_buffer_view_client(buffer_view, attr, self);
    }
    fn create_vertices_client(
        &mut self,
        vertices: &model3d_base::Vertices<Self>,
    ) -> Self::Vertices {
        Self::Vertices::create(vertices, self)
    }

    fn init_material_client(
        &mut self,
        client: &mut Self::Material,
        material: &dyn model3d_base::Material<Self>,
    ) {
    }
    //zz All done
}
