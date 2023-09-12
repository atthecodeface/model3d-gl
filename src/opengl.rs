use crate::{Gl, GlProgram, GlShaderType, Mat4};

mod shader;
pub mod utils;
pub use shader::Shader;
mod program;
pub use program::Program;

mod buffer;

mod vao;
use vao::Vao;

//a Model3DOpenGL
//tp Model3DOpenGL
#[derive(Debug)]
pub struct Model3DOpenGL {}

//ip Model3DOpenGL
impl Model3DOpenGL {
    pub fn new() -> Self {
        Self {}
    }
}

//ip Gl for Model3DOpenGL
impl Gl for Model3DOpenGL {
    type Id = u32;
    type Shader = Shader;
    type Program = Program;
    type Buffer = buffer::Buffer;
    type Vao = vao::Vao;

    //fp link_program
    /// Create a program from a list of compiled shaders
    fn link_program(
        &self,
        srcs: &[&Self::Shader],
        named_attrs: &[(&str, model3d_base::VertexAttr)],
        named_uniforms: &[(&str, crate::UniformId)],
        named_uniform_buffers: &[(&str, usize)],
    ) -> Result<Self::Program, String> {
        let mut program = Program::link_program(srcs)?;
        for (name, attr) in named_attrs {
            program.add_attr_name(name, *attr)?;
        }
        for (name, uniform) in named_uniforms {
            program.add_uniform_name(name, *uniform)?;
        }
        for (name, uniform) in named_uniform_buffers {
            program.add_uniform_buffer_name(*name, *uniform)?;
        }
        Ok(program)
    }

    //fp compile_shader
    /// Compile a shader
    fn compile_shader(
        &self,
        shader_type: GlShaderType,
        source: &str,
    ) -> Result<Self::Shader, String> {
        Shader::compile(source, shader_type)
    }

    //fp use_program
    /// Use the program
    fn use_program(&self, program: Option<&Self::Program>) {
        if let Some(program) = program {
            program.set_used();
        } else {
            unsafe {
                gl::UseProgram(0);
            }
        }
    }
    //fp init_buffer_of_indices
    fn init_buffer_of_indices(
        &mut self,
        buffer: &mut <Self as Gl>::Buffer,
        view: &model3d_base::BufferView<Self>,
    ) {
        buffer.of_indices(view);
    }

    //fp vao_create_from_indices
    fn vao_create_from_indices(&mut self, indices: &crate::IndexBuffer<Self>) -> Result<Vao, ()> {
        Vao::create_from_indices(self, indices)
    }

    //fp buffer_bind_to_vao_attr
    fn buffer_bind_to_vao_attr(
        &mut self,
        buffer: &<Self as Gl>::Buffer,
        attr_id: &<Program as GlProgram>::GlAttrId,
        count: u32,
        ele_type: model3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    ) {
        buffer.bind_to_vao_attr(*attr_id, count, ele_type, byte_offset, stride);
    }
    //fp program_set_uniform_mat4
    fn program_set_uniform_mat4(&mut self, program: &Program, id: crate::UniformId, mat4: &Mat4) {
        if let Some(u) = program.uniform(id) {
            unsafe {
                gl::UniformMatrix4fv(u, 1, gl::FALSE, mat4.as_ptr());
            }
        }
    }
    //fp draw_primitive
    fn draw_primitive(&mut self, vaos: &[Vao], primitive: &model3d_base::Primitive) {
        // (if p.vertices_index different to last)
        vaos[primitive.vertices_index()].bind_vao();
        use model3d_base::PrimitiveType::*;
        let gl_type = match primitive.primitive_type() {
            Points => gl::POINTS,
            Lines => gl::LINES,
            LineLoop => gl::LINE_LOOP,
            LineStrip => gl::LINE_STRIP,
            Triangles => gl::TRIANGLES,
            TriangleFan => gl::TRIANGLE_FAN,
            TriangleStrip => gl::TRIANGLE_STRIP,
        };
        unsafe {
            gl::DrawElements(
                gl_type,
                primitive.index_count() as i32,
                gl::UNSIGNED_BYTE, // index_type,
                std::mem::transmute(primitive.byte_offset()),
            );
        }
    }
    //fp bind_vao
    fn bind_vao(&mut self, vao: Option<&Self::Vao>) {
        if let Some(vao) = vao {
            vao.bind_vao();
        } else {
            unsafe {
                gl::BindVertexArray(0);
            }
        }
    }
}

//ip model3d_base::Renderable for Model3DOpenGL
impl model3d_base::Renderable for Model3DOpenGL {
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
            client.of_data(buffer_data)
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
        _client: &mut Self::Material,
        _material: &dyn model3d_base::Material<Self>,
    ) {
    }
    //zz All done
}
