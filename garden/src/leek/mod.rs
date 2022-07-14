use std::ops::{Deref, DerefMut};

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Reference                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Ref<T> {
    ptr: *const T
}

impl<T> Ref<T> {

    pub fn new(ptr: *const T) -> Ref<T> {
        return Ref {
            ptr
        }
    }

}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            return &*self.ptr;
        }
    }
}

impl<T> From<&T> for Ref<T> {

    fn from(value: &T) -> Self {
        return Ref::new(value as *const T);
    }

}

impl<T> Clone for Ref<T> {

    fn clone(&self) -> Self {
        return Self::new(self.ptr.clone())
    }

}

impl<T> Copy for Ref<T> {
    
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        Mutable Reference                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MutRef<T> {
    ptr: *mut T
}

impl<T> MutRef<T> {

    pub fn new(ptr: *mut T) -> MutRef<T> {
        return MutRef {
            ptr
        }
    }

}

impl<T> Deref for MutRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            return &*self.ptr;
        }
    }
}

impl<T> DerefMut for MutRef<T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            return &mut *self.ptr;
        }
    }

}

impl<T> From<&mut T> for MutRef<T> {

    fn from(value: &mut T) -> Self {
        return MutRef::new(value as *mut T);
    }

}

impl<T> Clone for MutRef<T> {

    fn clone(&self) -> Self {
        return Self::new(self.ptr.clone())
    }

}

impl<T> Copy for MutRef<T> {

}