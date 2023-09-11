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

@file    buffer.rs
@brief   An OpenGL buffer representation Part of geometry library
 */

//a Imports
use std::marker::PhantomData;

use model3d_base::{BufferElementType, VertexAttr, ViewClient};

use crate::{Gl, GlBuffer};

//a VertexBuffer
//tp VertexBuffer
///
/// A subset of a data buffer for use with OpenGL vertex data.
///
/// A data buffer may contain a lot of data per vertex, such as
/// position, normal, tangent, color etc.  A [VertexBuffer] is
/// then a subset of this data - perhaps picking out just the
/// position, for example, for a set of vertices
///
/// OpenGL will have one copy of the data for all the [VertexBuffer]
#[derive(Debug, Clone)]
pub struct VertexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    /// Ref-counted buffer
    gl_buffer: B,
    /// Number of elements per vertex - 1 to 4
    pub count: u32,
    /// The type of each element
    pub ele_type: model3d_base::BufferElementType,
    /// Offset from start of buffer to first byte of data
    pub byte_offset: u32,
    /// Stride of data in the buffer - 0 for count*sizeof(ele_type)
    pub stride: u32,
    phantom: PhantomData<R>,
}

//ip VertexBuffer
impl<R, B> VertexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //ap gl_buffer
    /// Get the gl_buffer associated with the data, assuming its
    /// `gl_create` method has been invoked at least once
    pub fn gl_buffer(&self) -> &B {
        &self.gl_buffer
    }

    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(&mut self, view: &model3d_base::BufferView<R>, render_context: &mut R::Context) {
        view.data.create_client(render_context);
        self.count = view.count;
        self.ele_type = view.ele_type;
        self.byte_offset = view.byte_offset;
        self.stride = view.stride;
        self.gl_buffer = view.data.borrow_client().clone();
    }

    //zz All done
}

//ip Default for VertexBuffer
impl<R, B> Default for VertexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    fn default() -> Self {
        let gl_buffer = B::default();
        let count = 0;
        let ele_type = BufferElementType::Float32;
        let byte_offset = 0;
        let stride = 0;
        Self {
            gl_buffer,
            count,
            ele_type,
            byte_offset,
            stride,
            phantom: PhantomData,
        }
    }
}

//ip Display for VertexBuffer
impl<R, B> std::fmt::Display for VertexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Vert({:?}+{}:#{} {:?} @{})",
            self.gl_buffer, self.byte_offset, self.count, self.ele_type, self.stride
        )
    }
}

//ip DefaultIndentedDisplay for VertexBuffer
impl<R, B> indent_display::DefaultIndentedDisplay for VertexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
}

//a IndexBuffer
//tp IndexBuffer
///
/// A subset of a data buffer for use with OpenGL index data.
///
/// An IndexBuffer directly owns the OpenGL buffer which is an
/// ElementArray rather than vertex data
#[derive(Debug, Clone)]
pub struct IndexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    /// Ref-counted buffer
    gl_buffer: B,
    /// Number of elements per index - 1 to 4
    pub count: u32,
    /// The type of each element
    pub ele_type: BufferElementType,
    phantom: PhantomData<R>,
}

//ip Default for IndexBuffer
impl<R, B> Default for IndexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    fn default() -> Self {
        let gl_buffer = B::default();
        let count = 0;
        let ele_type = BufferElementType::Int8;
        Self {
            gl_buffer,
            count,
            ele_type,
            phantom: PhantomData,
        }
    }
}

//ip IndexBuffer
impl<R, B> IndexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //mp of_view
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn of_view(view: &model3d_base::BufferView<R>, _render_context: &mut R::Context) -> Self {
        let mut gl_buffer = B::default();
        // gl_buffer.of_indices(view);
        let count = view.count;
        let ele_type = view.ele_type;
        println!(
            "Create indices buffer {} of view {:?}#{}",
            gl_buffer, view.ele_type, view.count
        );
        Self {
            gl_buffer,
            count,
            ele_type,
            phantom: PhantomData,
        }
    }

    //zz All done
}

//ip Display for IndexBuffer
impl<R, B> std::fmt::Display for IndexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "Ind({:?}#{} {:?})",
            self.gl_buffer, self.count, self.ele_type,
        )
    }
}

//ip DefaultIndentedDisplay for IndexBuffer
impl<R, B> indent_display::DefaultIndentedDisplay for IndexBuffer<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
}

//a BufferView
//tp BufferView
///
/// A view of data with either vertices of indices
#[derive(Debug, Clone)]
pub enum BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    /// Vertex buffer
    VertexBuffer(VertexBuffer<R, B>),
    /// Index buffer
    IndexBuffer(IndexBuffer<R, B>),
}

//ip Default for BufferView<G>
impl<R, B> Default for BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    fn default() -> Self {
        Self::VertexBuffer(VertexBuffer::default())
    }
}

//ip BufferView
impl<R, B> BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //fp as_index_buffer
    /// Return the [IndexBuffer] that this [BufferView] is of - if it
    /// is not a view of indices then panic
    pub fn as_index_buffer(&self) -> &IndexBuffer<R, B> {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer,
            _ => panic!("Attempt to borrow a VertexBuffer as an IndexBuffer"),
        }
    }

    //fp as_vertex_buffer
    /// Return the [VertexBuffer] that this [BufferView] is of - if it
    /// is not a view of vertex attributess then panic
    pub fn as_vertex_buffer(&self) -> &VertexBuffer<R, B> {
        match self {
            Self::VertexBuffer(vertex_buffer) => vertex_buffer,
            _ => panic!("Attempt to borrow an IndexBuffer as an VertexBuffer"),
        }
    }
}

//ip ViewClient for BufferView
impl<R, B> ViewClient<R> for BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B, View = Self> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    //mp create
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    fn create(
        &mut self,
        view: &model3d_base::BufferView<R>,
        attr: VertexAttr,
        render_context: &mut R::Context,
    ) {
        if attr == VertexAttr::Indices {
            let index_buffer = IndexBuffer::of_view(view, render_context);
            *self = BufferView::IndexBuffer(index_buffer);
        } else {
            match self {
                BufferView::IndexBuffer(_) => panic!("Vertex buffer is already an index buffer"),
                BufferView::VertexBuffer(vb) => {
                    vb.of_view(view, render_context);
                }
            }
        }
    }

    //zz All done
}

//ip Display for BufferView
impl<R, B> std::fmt::Display for BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::IndexBuffer(index_buffer) => index_buffer.fmt(f),
            Self::VertexBuffer(vertex_buffer) => vertex_buffer.fmt(f),
        }
    }
}

//ip DefaultIndentedDisplay for BufferView
impl<R, B> indent_display::DefaultIndentedDisplay for BufferView<R, B>
where
    R: model3d_base::Renderable<Buffer = B> + std::fmt::Debug,
    B: model3d_base::BufferClient<R> + Clone,
{
}
