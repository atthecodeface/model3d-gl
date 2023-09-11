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

@file    gl_buffer.rs
@brief   An OpenGL Buffer representation
 */

//a Imports
use std::cell::RefCell;
use std::rc::Rc;

use model3d_base::{BufferClient, BufferData, BufferElementType};

use super::Model3DWebGL;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

//a Buffer
//tp Buffer
/// A simple structure provides a reference-counted OpenGl buffer;
/// when the last reference is dropped it will drop the OpenGl buffer
/// that it contains, if any
///
/// Its actual buffer is created from vertex data or from indices;
/// from vertex data it is created *only* on the first invocation
/// (from a [model3d_rs::BufferData]) as subsequent 'creations' will be
/// duplicates - the reference count should ont be changed either as
/// it is the *same* BufferData instance that is invoking the creation
///
/// For indices a buffer is created for the [model3d_rs::BufferView], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The WebGL Buffer if this has been set
    gl: Rc<RefCell<Option<WebGlBuffer>>>,
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        let gl = Rc::new(RefCell::new(None));
        Self { gl }
    }
}

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "WebGlBuf",)
    }
}

//ip BufferClient for Buffer
impl BufferClient<Model3DWebGL> for Buffer {
    /// Create a client
    ///
    /// This may be called multiple times for the same [BufferData]; if the
    /// gl buffer is 0 then create, else it already exists with the same data
    fn create(&mut self, data: &BufferData<Model3DWebGL>, render_context: &mut Model3DWebGL) {
        if self.is_none() {
            self.of_data(data, render_context)
        }
    }
}

//ip Buffer
impl Buffer {
    //ap is_none
    pub fn is_none(&self) -> bool {
        self.gl.borrow().is_none()
    }

    //mp of_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_data(&mut self, data: &BufferData<Model3DWebGL>, render_context: &Model3DWebGL) {
        assert!(self.is_none());
        let gl = render_context.create_buffer().unwrap();
        render_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&gl));
        render_context.buffer_data_with_u8_array(
            WebGl2RenderingContext::ARRAY_BUFFER,
            data.as_slice(),
            WebGl2RenderingContext::STATIC_DRAW,
        );
        render_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        *self.gl.borrow_mut() = Some(gl);
    }

    //mp of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_indices(
        &mut self,
        view: &model3d_base::BufferView<Model3DWebGL>,
        render_context: &Model3DWebGL,
    ) {
        assert!(self.is_none());
        let ele_size = {
            use BufferElementType::*;
            match view.ele_type {
                Int8 => 1,
                Int16 => 2,
                Int32 => 4,
                _ => panic!("Indices BufferView must have an int element type"),
            }
        };
        let byte_length = ele_size * view.count;
        // stops the indices messing up other VAO
        render_context.bind_vertex_array(None);
        let data = view.data.as_slice();
        let gl = render_context.create_buffer().unwrap();
        render_context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&gl));
        render_context.buffer_data_with_u8_array(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            data,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        render_context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        *self.gl.borrow_mut() = Some(gl);
    }

    //zz All done
}

//a Stuff that needs to be webgled
// for vertex buffer
//fp gl_element_type
// fn gl_element_type(&self) -> gl::types::GLuint {
//     use BufferElementType::*;
//     match self.ele_type {
//         Float32 => gl::FLOAT,
//         Float16 => gl::HALF_FLOAT,
//         Int8 => gl::BYTE,
//         Int16 => gl::SHORT,
//         Int32 => gl::INT,
//     }
// }

//fp bind_to_vao
// Bind the buffer as a vertex attribute to the current VAO
// pub fn bind_to_vao(&self, attr_index: gl::types::GLuint) {
//     unsafe {
//         gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer());
//         gl::EnableVertexAttribArray(attr_index);
//         gl::VertexAttribPointer(
//             attr_index,
//             self.count as i32, // size
//             self.gl_element_type(),
//             gl::FALSE,          // normalized
//             self.stride as i32, // stride
//             std::mem::transmute::<usize, *const std::os::raw::c_void>(self.byte_offset as usize), // ptr
//         );
//     }
// }
// For index buffer
//fp bind_to_vao
// Bind the index buffer to the current VAO
//     pub fn bind_to_vao(&self) {
//         unsafe {
//             gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_buffer());
//         }
//     }
