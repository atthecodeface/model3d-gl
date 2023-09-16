//a Imports
use crate::{Gl, GlProgram, UniformId, Vertices};

//a ShaderInstantiable
//tp ShaderInstantiable
/// This is a shader-specific instantiable built from the vertices of an [model3d_base::Instantiable]
///
/// A shader requires a VAO that maps *some* of the vertex attribute
/// buffers to particular attribute UIDs in the shader program
///
/// It requires mapping of textures to texture things
///
/// Possibly it will also require some particullar Uniforms
///
/// An [model3d_base::Instance] can be renderd with a shader by using the RenderRecipe
/// from the [model3d_base::Instantiable], using the matrix and bone positions in the
/// Instance, and using the VAOs and other data in the
/// [ShaderInstantiable].
///
/// It borrows from the [model3d_base::Instantiable] and so does not need to its own GlBuffers
pub struct ShaderInstantiable<'a, G>
where
    G: Gl,
{
    instantiable: &'a model3d_base::Instantiable<G>,
    // vaos is 1-to-1 with instantiable::vertices, specific to this shader (class)
    vaos: Vec<G::Vao>,
    // The program NEED NOT be borrowed, if the program's uniforms
    // required for the draw are recorded during 'new_vao'
    program: &'a G::Program,
}

//ip ShaderInstantiable
impl<'a, G> ShaderInstantiable<'a, G>
where
    G: Gl,
{
    //fi new_vao
    fn new_vao(
        context: &mut G,
        program: &G::Program,
        vertices: &Vertices<G>,
    ) -> Result<G::Vao, ()> {
        let (indices, position, attrs) = vertices.borrow();
        let mut gl_vao = context.vao_create_from_indices(indices)?;
        for (index, vertex_attr) in program.attributes() {
            if *vertex_attr == model3d_base::VertexAttr::Position {
                position.bind_to_vao_attr(context, index);
            } else {
                for (va, buffer) in attrs {
                    if *vertex_attr == *va {
                        buffer.bind_to_vao_attr(context, index);
                    }
                }
            }
        }
        context.bind_vao(None);
        Ok(gl_vao)
    }

    //fp new
    /// Create a new [ShaderInstantiable]
    pub fn new(
        context: &mut G,
        program: &'a G::Program,
        instantiable: &'a model3d_base::Instantiable<G>,
    ) -> Result<Self, ()> {
        let mut vaos = Vec::new();
        for v in &instantiable.vertices {
            vaos.push(Self::new_vao(context, program, v)?);
        }
        Ok(Self {
            instantiable,
            vaos,
            program,
        })
    }

    //fp gl_draw
    /// Draw this [ShaderInstantiable] given an [model3d_base::Instance] data
    pub fn gl_draw(&self, context: &mut G, instance: &model3d_base::Instance<G>) {
        // shader camera matrix (already set?)
        /*
            // for bone_set_pose in instance.bone_set_poses {
            //  bone_set_pose.update(tick)
            // }
                //for (t,m,b) in self.meshes:
                //if b>=0:
                //bma = self.bone_set_poses[b]
                //program.set_uniform_if("uBonesMatrices",
                //lambda u:GL.glUniformMatrix4fv(u, bma.max_index, False, bma.data))
                //program.set_uniform_if("uBonesScale",
                //lambda u: GL.glUniform1f(u, 1.0) )
                //pass
            //else:
                //program.set_uniform_if("uBonesScale",
                //lambda u: GL.glUniform1f(u, 0.0) )
                //pass
                # Provide mesh matrix and material uniforms
                program.set_uniform_if("uMeshMatrix",
                                       lambda u: GL.glUniformMatrix4fv(u, 1, False, t.mat4()) )

        instance bone matrices
        instance model matrix
        for (i, p) in render_recipe.primitives.iter().enumerate() {
         */
        context.program_set_uniform_mat4(
            self.program,
            UniformId::ModelMatrix,
            &instance.transformation.mat4(),
        );
        for (i, p) in self
            .instantiable
            .render_recipe
            .primitives
            .iter()
            .enumerate()
        {
            // set MeshMatrix (if different to last)
            // Optimization using mesh uniform buffer
            // Bind a mat4-sized range of the matrices arrays to the Matrix uniform binding point
            let m = self.instantiable.render_recipe.matrix_for_primitives[i];
            context.program_set_uniform_mat4(
                self.program,
                UniformId::MeshMatrix,
                &self.instantiable.render_recipe.matrices[m],
            );
            // set material info to that for shader_instantiable p.material_index,(if different to last)
            context.draw_primitive(&self.vaos, p);
        }
    }

    //zz All done
}
