use cgmath::Vector2;
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use std::any::Any;
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;

use crate::garlic::{Color, IBatch, QuadGroup, Shader};

const SHAPE_BATCH_SIZE: usize = 200;
pub const SHAPE_BATCH_ID: usize = 0x101;

// ============ Data Layout =============
// Roundness:   f32
const ROUNDNESS_SIZE: usize = 1;
const ROUNDNESS_OFFSET: usize = 0;
// Color:       4 x f32
const COLOR_SIZE: usize = 4;
const COLOR_OFFSET: usize = ROUNDNESS_OFFSET + ROUNDNESS_SIZE * size_of::<GLfloat>();
// Total:       5 x f32
const VERTEX_SIZE: usize = ROUNDNESS_SIZE + COLOR_SIZE;
const VERTEX_SIZE_BYTES: usize = VERTEX_SIZE * size_of::<GLfloat>();

// =========== Uniform Layout ===========
// Position:    2 x f32
// Size:        2 x f32
// Thickness:   1 x f32

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Shape Data                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ShapeData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: QuadGroup<Color>,
    pub thickness: f32,
    pub roundness: QuadGroup<f32>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Shape Batch                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ShapeBatch {
    z_index: i32,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    data: [f32; SHAPE_BATCH_SIZE * VERTEX_SIZE * 4],
    u_position: [Vector2<f32>; SHAPE_BATCH_SIZE],
    u_size: [Vector2<f32>; SHAPE_BATCH_SIZE],
    u_thickness: [f32; SHAPE_BATCH_SIZE],
    count: usize,
}

impl IBatch for ShapeBatch {
    fn new() -> Self {
        unsafe {
            // Create data
            let data = [0.0; SHAPE_BATCH_SIZE * VERTEX_SIZE * 4];

            // Generate Vertex Array
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Generate indices
            let mut indices: [i32; 6 * SHAPE_BATCH_SIZE] = [0; 6 * SHAPE_BATCH_SIZE];
            for i in 0..(SHAPE_BATCH_SIZE as usize) {
                let index = (i * 4) as i32;
                indices[i * 6] = index + 3;
                indices[i * 6 + 1] = index + 2;
                indices[i * 6 + 2] = index;
                indices[i * 6 + 3] = index;
                indices[i * 6 + 4] = index + 2;
                indices[i * 6 + 5] = index + 1;
            }

            // Generate Element Buffer Object
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (SHAPE_BATCH_SIZE * 6 * size_of::<GLint>()) as GLsizeiptr,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Generate Vertex Buffer Object
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_SIZE_BYTES * 4 * SHAPE_BATCH_SIZE) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            // Bind Roundness Attribute
            gl::VertexAttribPointer(
                0,
                ROUNDNESS_SIZE as GLint,
                gl::FLOAT,
                gl::FALSE,
                VERTEX_SIZE_BYTES as GLsizei,
                ROUNDNESS_OFFSET as *const c_void,
            );
            gl::EnableVertexAttribArray(0);

            // Bind Color Attribute
            gl::VertexAttribPointer(
                1,
                COLOR_SIZE as GLint,
                gl::FLOAT,
                gl::FALSE,
                VERTEX_SIZE_BYTES as GLsizei,
                COLOR_OFFSET as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // Unbind Everything
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            // Initialize Uniforms
            let u_position = [Vector2::<f32>::new(0.0, 0.0); SHAPE_BATCH_SIZE];
            let u_size = [Vector2::<f32>::new(0.0, 0.0); SHAPE_BATCH_SIZE];
            let u_thickness = [0f32; SHAPE_BATCH_SIZE];

            return ShapeBatch {
                z_index: 0,
                vao,
                vbo,
                ebo,
                data,
                u_position,
                u_size,
                u_thickness,
                count: 0,
            };
        }
    }

    fn render(&mut self, shader: &mut Shader) {
        unsafe {
            // Load data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (self.count * VERTEX_SIZE_BYTES * 4) as GLsizeiptr,
                &self.data[0] as *const f32 as *const c_void,
            );

            // Load Uniforms
            shader.load_vec2_f32_arr("uPos", self.u_position);
            shader.load_vec2_f32_arr("uSize", self.u_size);
            shader.load_f32_arr("uThickness", self.u_thickness);

            // Render Batch
            gl::BindVertexArray(self.vao);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::DrawElements(
                gl::TRIANGLES,
                (self.count * 6) as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::BindVertexArray(0);
        }

        self.count = 0;
    }

    fn dispose(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }

    fn has_space(&self) -> bool {
        return self.count < SHAPE_BATCH_SIZE;
    }

    fn is_empty(&self) -> bool {
        return self.count == 0;
    }

    fn z_index(&self) -> i32 {
        return self.z_index;
    }

    fn z_index_mut(&mut self) -> &mut i32 {
        return &mut self.z_index;
    }

    fn id(&self) -> usize {
        return SHAPE_BATCH_ID;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ShapeBatch {
    pub fn push(&mut self, data: ShapeData) {
        // Push Uniforms
        self.u_position[self.count] = Vector2::new(data.x, data.y);
        self.u_size[self.count] = Vector2::new(data.width, data.height);
        self.u_thickness[self.count] = data.thickness;

        // Push VBO Data
        let mut ptr = self.count * VERTEX_SIZE * 4;
        for vertex in 0..4 {
            self.data[ptr] = data.roundness[vertex];

            let color = &data.color[vertex];
            self.data[ptr + 1] = color.red;
            self.data[ptr + 2] = color.green;
            self.data[ptr + 3] = color.blue;
            self.data[ptr + 4] = color.alpha;

            ptr += VERTEX_SIZE;
        }

        self.count += 1;
    }
}
