#[macro_export]
macro_rules! trait_component {
    ($name: ident) => {
        impl dyn $name {

            pub fn downcast_ref<Comp: $name>(&self) -> &Comp {
                unsafe {
                    return &*(self as *const dyn $name as *const Comp);
                }
            }

            pub fn downcast_mut<Comp: $name>(&mut self) -> &mut Comp {
                unsafe {
                    return &mut *(self as *mut dyn $name as *mut Comp);
                }
            }
            
        }
    };
}