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

@file    primitive.rs
@brief   Part of OpenGL library
 */

//a Imports
use std::rc::Rc;

use crate::{Gl, IndexBuffer, VertexBuffer};

//a Vertices
//tp Vertices
/// This is a set of OpenGL vertices with [crate::GlBuffer] for all of its contents
///
/// This is part of the [RenderContext], and so has a different
/// lifetime to the model3d objects and vertices. It is created by
/// invoking create_client on a [model3d_rs::Object]
#[derive(Debug)]
pub struct Vertices<G>
where
    G: Gl,
{
    indices: Rc<IndexBuffer<G>>,
    position: Rc<VertexBuffer<G>>,
    attrs: Rc<Vec<(model3d_base::VertexAttr, VertexBuffer<G>)>>,
}

//ip Clone for Vertices
impl<G> Clone for Vertices<G>
where
    G: Gl,
{
    fn clone(&self) -> Self {
        let indices = self.indices.clone();
        let position = self.position.clone();
        let attrs = self.attrs.clone();
        Self {
            indices,
            position,
            attrs,
        }
    }
}

//ip Vertices
impl<G> Vertices<G>
where
    G: Gl,
{
    //mp create
    /// Create based on [model3d_rs::Vertices]
    pub fn create(vertices: &model3d_base::Vertices<G>, _renderer: &mut G) -> Self {
        let indices = vertices
            .borrow_indices()
            .borrow_client()
            .as_index_buffer()
            .clone()
            .into();
        let position = vertices
            .borrow_position()
            .borrow_client()
            .as_vertex_buffer()
            .clone()
            .into();
        let mut attrs = Vec::new();
        for (attr, buffer) in vertices.iter_attrs() {
            attrs.push((*attr, buffer.borrow_client().as_vertex_buffer().clone()));
        }
        let attrs = attrs.into();
        Self {
            indices,
            position,
            attrs,
        }
    }
    //fp borrow
    /// Borrow the indices, positions, and the array of other attributes
    pub fn borrow(
        &self,
    ) -> (
        &IndexBuffer<G>,
        &VertexBuffer<G>,
        &Vec<(model3d_base::VertexAttr, VertexBuffer<G>)>,
    ) {
        (&self.indices, &self.position, &self.attrs)
    }
}

//ip Display for Vertices
impl<G> std::fmt::Display for Vertices<G>
where
    G: Gl,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "ind:{}", self.indices)?;
        writeln!(fmt, "pos:{}", self.position)
    }
}

//ip Default for Vertices
impl<G> Default for Vertices<G>
where
    G: Gl,
{
    /// Create a none
    fn default() -> Self {
        let indices = IndexBuffer::default().into();
        let position = VertexBuffer::default().into();
        let attrs = Vec::new().into();
        Self {
            indices,
            position,
            attrs,
        }
    }
}

//ip VerticesClient for Vertices
impl<G> model3d_base::VerticesClient for Vertices<G> where G: Gl {}
