use std::any::Any;
use std::cmp::Ordering;

use super::Shader;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             IBatch                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait IBatch {
    fn new() -> Self where Self: Sized;

    fn render(&mut self, shader: &mut Shader);

    fn dispose(&mut self);

    fn has_space(&self) -> bool;
    fn is_empty(&self) -> bool;

    fn z_index(&self) -> i32;
    fn z_index_mut(&mut self) -> &mut i32;

    fn id(&self) -> usize;

    fn as_any_mut(&mut self) -> &mut dyn Any;

}

impl Eq for dyn IBatch {}

impl PartialEq<Self> for dyn IBatch {

    fn eq(&self, other: &Self) -> bool {
        return self.z_index() == other.z_index();
    }

}

impl PartialOrd<Self> for dyn IBatch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.z_index().partial_cmp(&other.z_index());
    }
}

impl Ord for dyn IBatch {

    fn cmp(&self, other: &Self) -> Ordering {
        return self.z_index().cmp(&other.z_index());
    }

}