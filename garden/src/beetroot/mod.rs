use gl::types::GLenum;

use crate::error;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           GL Errors                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

fn gl_error_to_string(error: GLenum) -> String {
    return match error {
        gl::INVALID_ENUM => "Invalid Enum",
        gl::INVALID_VALUE => "Invalid Value",
        gl::INVALID_OPERATION => "Invalid Operation",
        gl::STACK_OVERFLOW => "Stack Overflow",
        gl::STACK_UNDERFLOW => "Stack Underflow",
        gl::OUT_OF_MEMORY => "Out Of Memory",
        gl::INVALID_FRAMEBUFFER_OPERATION => "Invalid Frame Buffer Operation",
        gl::CONTEXT_LOST => "Context Lost",
        _ => "Unknown"
    }.to_string();
}

pub fn clear_errors() {
    unsafe {
        let mut error = gl::GetError();
        while error != gl::NO_ERROR {
            error = gl::GetError();
        }
    }
}

pub fn get_errors() {
    unsafe {
        let mut error = gl::GetError();
        while error != gl::NO_ERROR {
            error!("OpenGL", "{}", gl_error_to_string(error));
            error = gl::GetError();
        }
    }
}