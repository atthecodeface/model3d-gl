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
use model3d_base::VertexAttr;
use std::rc::Rc;

use crate::{IndexBuffer, VertexBuffer};

//a Vertices
//tp Vertices
/// This is a set of OpenGL vertices with [crate::GlBuffer] for all of its contents
///
/// This is part of the [RenderContext], and so has a different
/// lifetime to the model3d objects and vertices. It is created by
/// invoking create_client on a [model3d_rs::Object]
#[derive(Debug)]
pub struct Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    indices: Rc<IndexBuffer<R, B>>,
    position: Rc<VertexBuffer<R, B>>,
    attrs: Rc<Vec<(model3d_base::VertexAttr, VertexBuffer<R, B>)>>,
}

//ip Clone for Vertices
impl<R, B> Clone for Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
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
impl<R, B> Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //fp borrow
    /// Borrow the indices, positions, and the array of other attributes
    pub fn borrow(
        &self,
    ) -> (
        &IndexBuffer<R, B>,
        &VertexBuffer<R, B>,
        &Vec<(model3d_base::VertexAttr, VertexBuffer<R, B>)>,
    ) {
        (&self.indices, &self.position, &self.attrs)
    }
}

//ip Display for Vertices
impl<R, B> std::fmt::Display for Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "ind:{}", self.indices)?;
        writeln!(fmt, "pos:{}", self.position)
    }
}

//ip Default for Vertices
impl<R, B> Default for Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
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
impl<R, B> model3d_base::VerticesClient<R> for Vertices<R, B>
where
    R: model3d_base::Renderable<Buffer = B, Vertices = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //mp create
    /// Create based on [model3d_rs::Vertices]
    fn create(vertices: &model3d_base::Vertices<R>, _render_context: &mut R::Context) -> Self {
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
}
