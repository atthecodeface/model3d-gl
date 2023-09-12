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
use super::Model3DOpenGL;
use model3d_base::{BufferClient, BufferData, BufferElementType};
use std::rc::Rc;

//a Buffer
//tp Buffer
/// A simple structure provides a reference-counted OpenGl buffer;
/// when the last reference is dropped it will drop the OpenGl buffer
/// that it contains, if any
///
/// Its actual buffer is created from vertex data or from indices;
/// from vertex data it is created *only* on the first invocation
/// (from a [model3d_base::BufferData]) as subsequent 'creations' will be
/// duplicates - the reference count should ont be changed either as
/// it is the *same* BufferData instance that is invoking the creation
///
/// For indices a buffer is created for the [model3d_base::BufferView], as
/// the buffer in this case must be an OpenGL ELEMENT_ARRAY_BUFFER;
/// this could perhaps be optimized to reduce the number of OpenGL
/// buffers with much more code.
#[derive(Debug, Clone)]
pub struct Buffer {
    /// The OpenGL Buffer
    gl: Rc<gl::types::GLuint>,
}

//ip Default for Buffer
impl Default for Buffer {
    fn default() -> Self {
        let gl = Rc::new(0);
        Self { gl }
    }
}

//ip BufferClient for Buffer
impl BufferClient for Buffer {}

//ip Buffer
impl Buffer {
    //ap gl_buffer
    /// Get the gl_buffer associated with the data
    pub fn gl_buffer(&self) -> gl::types::GLuint {
        *self.gl
    }

    //ap as_ptr
    /// Get a pointer to this gl
    pub fn as_ptr(&self) -> *const gl::types::GLuint {
        Rc::as_ptr(&self.gl)
    }

    //mp is_none
    /// Return true if the buffer is not initialized
    pub fn is_none(&self) -> bool {
        *self.gl == 0
    }

    //mp of_data
    /// Create the OpenGL ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_data(&mut self, data: &BufferData<Model3DOpenGL>) {
        assert!(self.is_none());
        let mut gl: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                data.byte_length as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind to protect
        }
        self.gl = Rc::new(gl);
    }

    //mp of_indices
    /// Create the OpenGL ELEMENT_ARRAY_BUFFER buffer using STATIC_DRAW - this copies the data in to OpenGL
    pub fn of_indices(&mut self, view: &model3d_base::BufferView<Model3DOpenGL>) {
        assert!(self.is_none());
        let mut gl: gl::types::GLuint = 0;
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
        unsafe {
            // stops the indices messing up other VAO
            gl::BindVertexArray(0);
            let buffer = view.data.as_ptr().add(view.byte_offset as usize);
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gl);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                byte_length as gl::types::GLsizeiptr,
                buffer as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // unbind to protect
        }
        self.gl = Rc::new(gl);
    }

    //mp uniform_buffer
    /// Create the OpenGL
    pub fn uniform_buffer<F: Sized>(&mut self, data: &[F]) {
        assert!(self.is_none());
        let buffer = data.as_ptr();
        let byte_length = std::mem::size_of::<F>() * data.len();
        let mut gl: gl::types::GLuint = 0;
        unsafe {
            gl::BindVertexArray(0);
            gl::GenBuffers(1, (&mut gl) as *mut gl::types::GLuint);
            gl::BindBuffer(gl::UNIFORM_BUFFER, gl);
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                byte_length as gl::types::GLsizeiptr,
                buffer as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0); // unbind to protect
            println!("Uniform buffer {} bound @{:?}+{}", gl, buffer, byte_length);
        }
        self.gl = Rc::new(gl);
    }

    //fp bind_to_vao_attr
    /// Bind the buffer as a vertex attribute to the current VAO
    pub fn bind_to_vao_attr(
        &self,
        attr_id: gl::types::GLuint,
        count: u32,
        ele_type: model3d_base::BufferElementType,
        byte_offset: u32,
        stride: u32,
    ) {
        let ele_type = {
            use BufferElementType::*;
            match ele_type {
                Float32 => gl::FLOAT,
                Float16 => gl::HALF_FLOAT,
                Int8 => gl::BYTE,
                Int16 => gl::SHORT,
                Int32 => gl::INT,
            }
        };
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_buffer());
            gl::EnableVertexAttribArray(attr_id);
            gl::VertexAttribPointer(
                attr_id,
                count as i32, // size
                ele_type,
                gl::FALSE,     // normalized
                stride as i32, // stride
                std::mem::transmute::<usize, *const std::os::raw::c_void>(byte_offset as usize), // ptr
            );
        }
    }

    //zz All done
}

//ip Drop for Buffer
impl Drop for Buffer {
    //fp drop
    /// If an OpenGL buffer has been created for this then delete it
    fn drop(&mut self) {
        if Rc::strong_count(&self.gl) == 1 && !self.is_none() {
            unsafe {
                gl::DeleteBuffers(1, self.as_ptr());
            }
        }
    }
}

//ip GlBuffer for Buffer
impl crate::GlBuffer for Buffer {}

//ip Display for Buffer
impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "GL({})", self.gl)
    }
}
