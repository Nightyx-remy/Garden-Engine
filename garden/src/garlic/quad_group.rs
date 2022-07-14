use std::ops::{Index, IndexMut};

use super::Interpolate;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Quad Group                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct QuadGroup<T> {
    pub top_left: T,
    pub top_right: T,
    pub bottom_left: T,
    pub bottom_right: T,
}

impl<T> QuadGroup<T> {

    pub fn new(top_left: T, top_right: T, bottom_left: T, bottom_right: T) -> QuadGroup<T> {
        return QuadGroup {
            top_left,
            top_right,
            bottom_left,
            bottom_right
        }
    }

    pub fn flip_x(&mut self) {
        std::mem::swap(&mut self.top_left, &mut self.top_right);
        std::mem::swap(&mut self.bottom_left, &mut self.bottom_right);
    }

    pub fn flip_y(&mut self) {
        std::mem::swap(&mut self.top_left, &mut self.bottom_left);
        std::mem::swap(&mut self.top_right, &mut self.bottom_right);
    }

    pub fn flip(&mut self) {
        std::mem::swap(&mut self.top_left, &mut self.bottom_right);
        std::mem::swap(&mut self.top_right, &mut self.bottom_left);
    }

}

impl<T: Clone> QuadGroup<T> {

    pub fn single(data: T) -> QuadGroup<T> {
        return QuadGroup::new(data.clone(), data.clone(), data.clone(), data.clone());
    }

    pub fn vertical(top: T, bottom: T) -> QuadGroup<T> {
        return QuadGroup::new(top.clone(), top.clone(), bottom.clone(), bottom.clone());
    }

    pub fn horizontal(left: T, right: T) -> QuadGroup<T> {
        return QuadGroup::new(left.clone(), right.clone(), left.clone(), right.clone());
    }

}

impl<T: Clone> Clone for QuadGroup<T> {

    fn clone(&self) -> Self {
        return Self {
            top_left: self.top_left.clone(),
            top_right: self.top_right.clone(),
            bottom_left: self.bottom_left.clone(),
            bottom_right: self.bottom_right.clone()
        }
    }

}

impl<T: Copy> Copy for QuadGroup<T> {}

impl<T: PartialEq> PartialEq for QuadGroup<T> {

    fn eq(&self, other: &Self) -> bool {
        return self.top_left == other.top_left &&
            self.bottom_left == other.bottom_left &&
            self.bottom_right == other.bottom_right &&
            self.top_right == other.top_right;
    }
    
}

impl<T: Eq + PartialEq> Eq for QuadGroup<T> {

}

impl<T> Index<usize> for QuadGroup<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return match index % 4 {
            0 => &self.bottom_right,
            1 => &self.top_right,
            2 => &self.top_left,
            3 => &self.bottom_left,
            _ => panic!("Cannot happen!")
        }
    }
}

impl <T> IndexMut<usize> for QuadGroup<T> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return match index % 4 {
            0 => &mut self.bottom_right,
            1 => &mut self.top_right,
            2 => &mut self.top_left,
            3 => &mut self.bottom_left,
            _ => panic!("Cannot happen!")
        }
    }

}

impl<O, T: Interpolate<O, T>> Interpolate<QuadGroup<O>, QuadGroup<T>> for QuadGroup<T> {

    fn interpolate(start: QuadGroup<T>, target: QuadGroup<T>, value: f64) -> QuadGroup<O> {
        return QuadGroup::<O>::new(
            T::interpolate(start.top_left, target.top_left, value),
            T::interpolate(start.top_right, target.top_right, value),
            T::interpolate(start.bottom_left, target.bottom_left, value),
            T::interpolate(start.bottom_right, target.bottom_right, value)
        );
    }

}