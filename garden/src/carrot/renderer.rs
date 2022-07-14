use cgmath::{ortho, vec2};

use crate::{
    error,
    garlic::{Color, IBatch, QuadGroup, Shader},
    mem::MutRef,
    warn,
};

use super::{ShapeBatch, ShapeData, SHAPE_BATCH_ID};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Batch Type                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq)]
enum BatchType {
    ShapeBatch,
    None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Renderer2D                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Renderer2D {
    batches: Vec<Box<dyn IBatch>>,
    shape_batch_shader: Option<Shader>,
    batch_type: BatchType,
    z_index: i32,
}

impl Renderer2D {
    pub fn new() -> Renderer2D {
        return Renderer2D {
            batches: Vec::new(),
            shape_batch_shader: None,
            batch_type: BatchType::None,
            z_index: 0,
        };
    }

    pub fn init(&mut self) {
        if self.shape_batch_shader.is_none() {
            self.shape_batch_shader = Some(Shader::load("res/shaders/shape_batch"));
        } else {
            error!("Carrot", "Renderer2D already initialized!");
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let projection = ortho(0f32, width as f32, height as f32, 0f32, 0f32, 1f32);

        self.bind_shader(BatchType::ShapeBatch);
        self.shape_batch_shader
            .as_mut()
            .unwrap()
            .load_mat4("uProjection", projection);
        self.shape_batch_shader
            .as_mut()
            .unwrap()
            .load_vec2_f32("uScreenSize", vec2(width as f32, height as f32));
        self.bind_shader(BatchType::None);
    }

    fn push_shape(&mut self, shape: ShapeData) {
        for batch in self.batches.iter_mut() {
            if batch.id() == SHAPE_BATCH_ID
                && (batch.z_index() == self.z_index || batch.is_empty())
                && batch.has_space()
            {
                if batch.z_index() != self.z_index {
                    *batch.z_index_mut() = self.z_index;
                }
                let shape_batch = batch.as_any_mut().downcast_mut::<ShapeBatch>().unwrap();
                shape_batch.push(shape);
                return;
            }
        }

        let mut batch = ShapeBatch::new();
        *batch.z_index_mut() = self.z_index;
        batch.push(shape);
        self.batches.push(Box::new(batch));
    }

    pub fn fill_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        self.push_shape(ShapeData {
            x,
            y,
            width,
            height,
            color: QuadGroup::single(color),
            thickness: 0.0,
            roundness: QuadGroup::single(0.0),
        });
    }

    fn bind_shader(&mut self, batch_type: BatchType) {
        if batch_type != self.batch_type {
            if self.batch_type != BatchType::None {
                Shader::unbind();
            }
            match batch_type {
                BatchType::ShapeBatch => self.shape_batch_shader.as_mut().unwrap().bind(),
                BatchType::None => {}
            }
            self.batch_type = batch_type;
        }
    }

    pub fn render(&mut self) {
        self.batches.sort();

        let mut batches = MutRef::from(&mut self.batches);
        for batch in batches.iter_mut() {
            if batch.id() == SHAPE_BATCH_ID {
                self.bind_shader(BatchType::ShapeBatch);
                batch.render(self.shape_batch_shader.as_mut().unwrap());
            } else {
                warn!("Carrot", "Unknown batch id {}!", batch.id());
            }
        }

        // Unbind Shader
        self.bind_shader(BatchType::None);
    }
}
