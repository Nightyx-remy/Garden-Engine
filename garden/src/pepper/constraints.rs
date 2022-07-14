use crate::{
    garlic::{
        Axis, 
        SideX, 
        SideY
    }, 
    ui::{
        ElementRef, 
        get_system, 
        ParentComponent, 
        ScreenProperty
    }, 
    potato::Assume, 
    warn, 
    mem::MutRef, 
    critical
};

const DEFAULT_POS: f32 = 0f32;
const DEFAULT_SIZE: f32 = 1.0f32;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Constraint X                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum ConstraintX {
    Pixel {
        value: f32,
        relative: Option<(Axis, ElementRef)>,
        from: (SideX, ElementRef),
    },
    PixelCentered {
        value: f32,
        relative: Option<(Axis, ElementRef)>,
        from: ElementRef
    },
    Percent {
        value: f32,
        relative: (Axis, ElementRef),
        from: (SideX, ElementRef),
    },
    PercentCentered {
        value: f32,
        relative: (Axis, ElementRef),
        from: ElementRef
    }
}

impl Default for ConstraintX {
    fn default() -> Self {
        return ConstraintX::pixel_in_left(0f32, ElementRef::Screen);
    }
}

impl ConstraintX {

    // Pixel

    pub fn pixel(value: f32, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::Pixel {
            value,
            relative: None,
            from,
        }
    }

    pub fn pixel_in_left(value: f32, from: ElementRef) -> ConstraintX {
        return ConstraintX::pixel(value, (SideX::InLeft, from));
    }

    pub fn pixel_in_right(value: f32, from: ElementRef) -> ConstraintX {
        return ConstraintX::pixel(value, (SideX::InRight, from));
    }

    pub fn pixel_from_left(value: f32, from: ElementRef) -> ConstraintX {
        return ConstraintX::pixel(value, (SideX::FromLeft, from));
    }

    pub fn pixel_from_right(value: f32, from: ElementRef) -> ConstraintX {
        return ConstraintX::pixel(value, (SideX::FromRight, from));
    }

    pub fn pixel_relative(value: f32, from: (SideX, ElementRef), relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::Pixel {
            value,
            relative: Some(relative),
            from,
        }
    }

    pub fn pixel_relative_in_left(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::pixel_relative(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_in_right(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::pixel_relative(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_from_left(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::pixel_relative(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_from_right(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::pixel_relative(value, (SideX::FromRight, from), relative);
    }

    pub fn pixel_relative_x(value: f32, from: (SideX, ElementRef), relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative(value, from, (Axis::X, relative));
    }

    pub fn pixel_relative_x_in_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_x(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_x_in_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_x(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_x_from_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_x(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_x_from_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_x(value, (SideX::FromRight, from), relative);
    }

    pub fn pixel_relative_y(value: f32, from: (SideX, ElementRef), relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative(value, from, (Axis::Y, relative));
    }

    pub fn pixel_relative_y_in_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_y(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_y_in_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_y(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_y_from_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_y(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_y_from_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_y(value, (SideX::FromRight, from), relative);
    }

    pub fn pixel_relative_min(value: f32, from: (SideX, ElementRef), relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative(value, from, (Axis::Min, relative));
    }

    pub fn pixel_relative_min_in_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_min(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_min_in_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_min(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_min_from_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_min(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_min_from_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_min(value, (SideX::FromRight, from), relative);
    }

    pub fn pixel_relative_max(value: f32, from: (SideX, ElementRef), relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative(value, from, (Axis::Max, relative));
    }

    pub fn pixel_relative_max_in_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_max(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_max_in_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_max(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_max_from_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_max(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_max_from_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_max(value, (SideX::FromRight, from), relative);
    }

    pub fn pixel_relative_avg(value: f32, from: (SideX, ElementRef), relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative(value, from, (Axis::Average, relative));
    }

    pub fn pixel_relative_avg_in_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_avg(value, (SideX::InLeft, from), relative);
    }

    pub fn pixel_relative_avg_in_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_avg(value, (SideX::InRight, from), relative);
    }

    pub fn pixel_relative_avg_from_left(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_avg(value, (SideX::FromLeft, from), relative);
    }

    pub fn pixel_relative_avg_from_right(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_avg(value, (SideX::FromRight, from), relative);
    }

    // Percent

    pub fn percent(value: f32, relative: (Axis, ElementRef), from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::Percent {
            value,
            relative,
            from
        }
    }

    pub fn percent_in_left(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintX {
        return ConstraintX::percent(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_in_right(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintX {
        return ConstraintX::percent(value, relative, (SideX::InRight, from));
    }

    pub fn percent_from_left(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintX {
        return ConstraintX::percent(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_from_right(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintX {
        return ConstraintX::percent(value, relative, (SideX::FromRight, from));
    }

    pub fn percent_x(value: f32, relative: ElementRef, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::percent(value, (Axis::X, relative), from);
    }

    pub fn percent_x_in_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_x(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_x_in_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_x(value, relative, (SideX::InRight, from));
    }

    pub fn percent_x_from_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_x(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_x_from_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_x(value, relative, (SideX::FromRight, from));
    }

    pub fn percent_y(value: f32, relative: ElementRef, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::percent(value, (Axis::Y, relative), from);
    }

    pub fn percent_y_in_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_x(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_y_in_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_y(value, relative, (SideX::InRight, from));
    }

    pub fn percent_y_from_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_y(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_y_from_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_y(value, relative, (SideX::FromRight, from));
    }

    pub fn percent_min(value: f32, relative: ElementRef, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::percent(value, (Axis::Min, relative), from);
    }

    pub fn percent_min_in_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_min(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_min_in_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_min(value, relative, (SideX::InRight, from));
    }

    pub fn percent_min_from_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_min(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_min_from_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_min(value, relative, (SideX::FromRight, from));
    }

    pub fn percent_max(value: f32, relative: ElementRef, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::percent(value, (Axis::Max, relative), from);
    }

    pub fn percent_max_in_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_max(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_max_in_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_max(value, relative, (SideX::InRight, from));
    }

    pub fn percent_max_from_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_max(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_max_from_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_max(value, relative, (SideX::FromRight, from));
    }

    pub fn percent_avg(value: f32, relative: ElementRef, from: (SideX, ElementRef)) -> ConstraintX {
        return ConstraintX::percent(value, (Axis::Average, relative), from);
    }

    pub fn percent_avg_in_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_avg(value, relative, (SideX::InLeft, from));
    }

    pub fn percent_avg_in_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_avg(value, relative, (SideX::InRight, from));
    }

    pub fn percent_avg_from_left(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_avg(value, relative, (SideX::FromLeft, from));
    }

    pub fn percent_avg_from_right(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintX {
        return ConstraintX::percent_avg(value, relative, (SideX::FromRight, from));
    }

    // Pixel Centered

    pub fn pixel_centered(value: f32, from: ElementRef) -> ConstraintX {
        return ConstraintX::PixelCentered {
            value,
            relative: None,
            from
        }
    }

    pub fn pixel_relative_centered(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::PixelCentered {
            value,
            relative: Some(relative),
            from
        }
    }

    pub fn pixel_relative_x_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_centered(value, from, (Axis::X, relative));
    }

    pub fn pixel_relative_y_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_centered(value, from, (Axis::Y, relative));
    }

    pub fn pixel_relative_min_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_centered(value, from, (Axis::Min, relative));
    }

    pub fn pixel_relative_max_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_centered(value, from, (Axis::Max, relative));
    }

    pub fn pixel_relative_avg_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::pixel_relative_centered(value, from, (Axis::Average, relative));
    }

    // Percent Centered

    pub fn percent_centered(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintX {
        return ConstraintX::PercentCentered {
            value,
            relative,
            from
        }
    }

    pub fn percent_x_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::percent_centered(value, from, (Axis::X, relative));
    }

    pub fn percent_y_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::percent_centered(value, from, (Axis::Y, relative));
    }

    pub fn percent_min_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::percent_centered(value, from, (Axis::Min, relative));
    }

    pub fn percent_max_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::percent_centered(value, from, (Axis::Max, relative));
    }

    pub fn percent_avg_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintX {
        return ConstraintX::percent_centered(value, from, (Axis::Average, relative));
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                          Constraint Y                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum ConstraintY {
    Pixel {
        value: f32,
        relative: Option<(Axis, ElementRef)>,
        from: (SideY, ElementRef),
    },
    PixelCentered {
        value: f32,
        relative: Option<(Axis, ElementRef)>,
        from: ElementRef
    },
    Percent {
        value: f32,
        relative: (Axis, ElementRef),
        from: (SideY, ElementRef),
    },
    PercentCentered {
        value: f32,
        relative: (Axis, ElementRef),
        from: ElementRef
    }
}

impl Default for ConstraintY {
    fn default() -> Self {
        return ConstraintY::pixel(0f32, (SideY::InTop, ElementRef::Screen));
    }
}

impl ConstraintY {

    // Pixel

    pub fn pixel(value: f32, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::Pixel {
            value,
            relative: None,
            from,
        }
    }

    pub fn pixel_in_top(value: f32, from: ElementRef) -> ConstraintY {
        return ConstraintY::pixel(value, (SideY::InTop, from));
    }

    pub fn pixel_in_bottom(value: f32, from: ElementRef) -> ConstraintY {
        return ConstraintY::pixel(value, (SideY::InBottom, from));
    }

    pub fn pixel_from_top(value: f32, from: ElementRef) -> ConstraintY {
        return ConstraintY::pixel(value, (SideY::FromTop, from));
    }

    pub fn pixel_from_bottom(value: f32, from: ElementRef) -> ConstraintY {
        return ConstraintY::pixel(value, (SideY::FromBottom, from));
    }

    pub fn pixel_relative(value: f32, from: (SideY, ElementRef), relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::Pixel {
            value,
            relative: Some(relative),
            from,
        }
    }

    pub fn pixel_relative_in_top(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::pixel_relative(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_in_bottom(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::pixel_relative(value, (SideY::InBottom, from), relative);
    }

    pub fn pixel_relative_from_top(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::pixel_relative(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_from_bottom(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::pixel_relative(value, (SideY::FromBottom, from), relative);
    }

    pub fn pixel_relative_x(value: f32, from: (SideY, ElementRef), relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative(value, from, (Axis::X, relative));
    }

    pub fn pixel_relative_x_in_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_x(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_x_in_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_x(value, (SideY::InBottom, from), relative);
    }

    pub fn pixel_relative_x_from_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_x(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_x_from_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_x(value, (SideY::FromBottom, from), relative);
    }

    pub fn pixel_relative_y(value: f32, from: (SideY, ElementRef), relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative(value, from, (Axis::Y, relative));
    }

    pub fn pixel_relative_y_in_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_y(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_y_in_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_y(value, (SideY::InBottom, from), relative);
    }

    pub fn pixel_relative_y_from_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_y(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_y_from_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_y(value, (SideY::FromBottom, from), relative);
    }

    pub fn pixel_relative_min(value: f32, from: (SideY, ElementRef), relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative(value, from, (Axis::Min, relative));
    }

    pub fn pixel_relative_min_in_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_min(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_min_in_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_min(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_min_from_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_min(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_min_from_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_min(value, (SideY::FromBottom, from), relative);
    }

    pub fn pixel_relative_max(value: f32, from: (SideY, ElementRef), relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative(value, from, (Axis::Max, relative));
    }

    pub fn pixel_relative_max_in_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_max(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_max_in_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_max(value, (SideY::InBottom, from), relative);
    }

    pub fn pixel_relative_max_from_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_max(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_max_from_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_max(value, (SideY::FromBottom, from), relative);
    }

    pub fn pixel_relative_avg(value: f32, from: (SideY, ElementRef), relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative(value, from, (Axis::Average, relative));
    }

    pub fn pixel_relative_avg_in_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_avg(value, (SideY::InTop, from), relative);
    }

    pub fn pixel_relative_avg_in_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_avg(value, (SideY::InBottom, from), relative);
    }

    pub fn pixel_relative_avg_from_top(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_avg(value, (SideY::FromTop, from), relative);
    }

    pub fn pixel_relative_avg_from_bottom(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_avg(value, (SideY::FromBottom, from), relative);
    }

    // Percent

    pub fn percent(value: f32, relative: (Axis, ElementRef), from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::Percent {
            value,
            relative,
            from
        }
    }

    pub fn percent_in_top(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintY {
        return ConstraintY::percent(value, relative, (SideY::InTop, from));
    }

    pub fn percent_in_bottom(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintY {
        return ConstraintY::percent(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_from_top(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintY {
        return ConstraintY::percent(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_from_bottom(value: f32, relative: (Axis, ElementRef), from: ElementRef) -> ConstraintY {
        return ConstraintY::percent(value, relative, (SideY::FromBottom, from));
    }

    pub fn percent_x(value: f32, relative: ElementRef, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::percent(value, (Axis::X, relative), from);
    }

    pub fn percent_x_in_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_x(value, relative, (SideY::InTop, from));
    }

    pub fn percent_x_in_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_x(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_x_from_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_x(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_x_from_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_x(value, relative, (SideY::FromBottom, from));
    }

    pub fn percent_y(value: f32, relative: ElementRef, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::percent(value, (Axis::Y, relative), from);
    }

    pub fn percent_y_in_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_x(value, relative, (SideY::InTop, from));
    }

    pub fn percent_y_in_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_y(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_y_from_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_y(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_y_from_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_y(value, relative, (SideY::FromBottom, from));
    }

    pub fn percent_min(value: f32, relative: ElementRef, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::percent(value, (Axis::Min, relative), from);
    }

    pub fn percent_min_in_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_min(value, relative, (SideY::InTop, from));
    }

    pub fn percent_min_in_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_min(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_min_from_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_min(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_min_from_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_min(value, relative, (SideY::FromBottom, from));
    }

    pub fn percent_max(value: f32, relative: ElementRef, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::percent(value, (Axis::Max, relative), from);
    }

    pub fn percent_max_in_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_max(value, relative, (SideY::InTop, from));
    }

    pub fn percent_max_in_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_max(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_max_from_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_max(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_max_from_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_max(value, relative, (SideY::FromBottom, from));
    }

    pub fn percent_avg(value: f32, relative: ElementRef, from: (SideY, ElementRef)) -> ConstraintY {
        return ConstraintY::percent(value, (Axis::Average, relative), from);
    }

    pub fn percent_avg_in_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_avg(value, relative, (SideY::InTop, from));
    }

    pub fn percent_avg_in_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_avg(value, relative, (SideY::InBottom, from));
    }

    pub fn percent_avg_from_top(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_avg(value, relative, (SideY::FromTop, from));
    }

    pub fn percent_avg_from_bottom(value: f32, relative: ElementRef, from: ElementRef) -> ConstraintY {
        return ConstraintY::percent_avg(value, relative, (SideY::FromBottom, from));
    }

    // Pixel Centered

    pub fn pixel_centered(value: f32, from: ElementRef) -> ConstraintY {
        return ConstraintY::PixelCentered {
            value,
            relative: None,
            from
        }
    }

    pub fn pixel_relative_centered(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::PixelCentered {
            value,
            relative: Some(relative),
            from
        }
    }

    pub fn pixel_relative_x_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_centered(value, from, (Axis::X, relative));
    }

    pub fn pixel_relative_y_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_centered(value, from, (Axis::Y, relative));
    }

    pub fn pixel_relative_min_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_centered(value, from, (Axis::Min, relative));
    }

    pub fn pixel_relative_max_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_centered(value, from, (Axis::Max, relative));
    }

    pub fn pixel_relative_avg_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::pixel_relative_centered(value, from, (Axis::Average, relative));
    }

    // Percent Centered

    pub fn percent_centered(value: f32, from: ElementRef, relative: (Axis, ElementRef)) -> ConstraintY {
        return ConstraintY::PercentCentered {
            value,
            relative,
            from
        }
    }

    pub fn percent_x_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::percent_centered(value, from, (Axis::X, relative));
    }

    pub fn percent_y_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::percent_centered(value, from, (Axis::Y, relative));
    }

    pub fn percent_min_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::percent_centered(value, from, (Axis::Min, relative));
    }

    pub fn percent_max_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::percent_centered(value, from, (Axis::Max, relative));
    }

    pub fn percent_avg_centered(value: f32, from: ElementRef, relative: ElementRef) -> ConstraintY {
        return ConstraintY::percent_centered(value, from, (Axis::Average, relative));
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        Constraint Width                                        //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum ConstraintWidth {
    Pixel {
        value: f32,
        relative: Option<(Axis, ElementRef)>
    },
    Percent {
        value: f32,
        relative: (Axis, ElementRef)
    },
}

impl Default for ConstraintWidth {

    fn default() -> Self {
        return ConstraintWidth::pixel(1f32);
    }

}

impl ConstraintWidth {

    pub fn pixel(value: f32) -> ConstraintWidth {
        return ConstraintWidth::Pixel {
            value,
            relative: None
        }
    }

    pub fn pixel_relative(value: f32, relative: (Axis, ElementRef)) -> ConstraintWidth {
        return ConstraintWidth::Pixel {
            value,
            relative: Some(relative)
        }
    }

    pub fn pixel_relative_x(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::pixel_relative(value, (Axis::X, relative));
    }

    pub fn pixel_relative_y(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::pixel_relative(value, (Axis::Y, relative));
    }

    pub fn pixel_relative_min(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::pixel_relative(value, (Axis::Min, relative));
    }

    pub fn pixel_relative_max(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::pixel_relative(value, (Axis::Max, relative));
    }

    pub fn pixel_relative_avg(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::pixel_relative(value, (Axis::Average, relative));
    }

    pub fn percent(value: f32, relative: (Axis, ElementRef)) -> ConstraintWidth {
        return ConstraintWidth::Percent {
            value,
            relative
        }
    }

    pub fn percent_x(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::percent(value, (Axis::X, relative));
    }

    pub fn percent_y(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::percent(value, (Axis::Y, relative));
    }

    pub fn percent_min(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::percent(value, (Axis::Min, relative));
    }

    pub fn percent_max(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::percent(value, (Axis::Max, relative));
    }

    pub fn percent_avg(value: f32, relative: ElementRef) -> ConstraintWidth {
        return ConstraintWidth::percent(value, (Axis::Average, relative));
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        Constraint Height                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub enum ConstraintHeight {
    Pixel {
        value: f32,
        relative: Option<(Axis, ElementRef)>
    },
    Percent {
        value: f32,
        relative: (Axis, ElementRef)
    },
}

impl Default for ConstraintHeight {
    fn default() -> Self {
        return ConstraintHeight::pixel(1f32);
    }
}

impl ConstraintHeight {

    pub fn pixel(value: f32) -> ConstraintHeight {
        return ConstraintHeight::Pixel {
            value,
            relative: None
        }
    }

    pub fn pixel_relative(value: f32, relative: (Axis, ElementRef)) -> ConstraintHeight {
        return ConstraintHeight::Pixel {
            value,
            relative: Some(relative)
        }
    }

    pub fn pixel_relative_x(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::X, relative));
    }

    pub fn pixel_relative_y(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Y, relative));
    }

    pub fn pixel_relative_min(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Min, relative));
    }

    pub fn pixel_relative_max(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Max, relative));
    }

    pub fn pixel_relative_avg(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Average, relative));
    }

    pub fn percent(value: f32, relative: (Axis, ElementRef)) -> ConstraintHeight {
        return ConstraintHeight::Percent {
            value,
            relative
        }
    }

    pub fn percent_relative_x(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::X, relative));
    }

    pub fn percent_relative_y(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Y, relative));
    }

    pub fn percent_relative_min(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Min, relative));
    }

    pub fn percent_relative_max(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Max, relative));
    }

    pub fn percent_relative_avg(value: f32, relative: ElementRef) -> ConstraintHeight {
        return ConstraintHeight::pixel_relative(value, (Axis::Average, relative));
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        Usefull Functions                                       //
////////////////////////////////////////////////////////////////////////////////////////////////////

mod util {
    use crate::{garlic::Axis, mem::{MutRef, Ref}, ui::{Constraints, ScreenProperty, ElementRef}};

    pub fn from_left(value: f32, element_x: f32, self_width: f32) -> f32 {
        return element_x - self_width - value;
    }

    pub fn from_right(value: f32, element_x: f32, element_width: f32) -> f32 {
        return element_x + element_width + value;
    }

    pub fn in_left(value: f32, element_x: f32) -> f32 {
        return element_x + value;
    }

    pub fn in_right(value: f32, element_x: f32, element_width: f32, self_width: f32) -> f32 {
        return element_x + element_width - self_width - value;
    }

    pub fn centered(offset: f32, element_x: f32, element_width: f32, self_width: f32) -> f32 {
        return element_x + (element_width - self_width) / 2.0 + offset;
    }

    pub fn pixel_relative(value: f32, axis: Axis, mut relative_constraints: MutRef<Constraints>) -> f32 {
        match axis {
            Axis::X => value * relative_constraints.get_scale_x(),
            Axis::Y => value * relative_constraints.get_scale_y(),
            Axis::Min => value * relative_constraints.get_scale_x().min(relative_constraints.get_scale_y()),
            Axis::Max => value * relative_constraints.get_scale_x().max(relative_constraints.get_scale_y()),
            Axis::Average => value * (relative_constraints.get_scale_x() + relative_constraints.get_scale_y()) / 2.0,
        }
    }

    pub fn screen_pixel_relative(value: f32, axis: Axis, screen: Ref<ScreenProperty>) -> f32 {
        match axis {
            Axis::X => value * screen.scale_x,
            Axis::Y => value * screen.scale_y,
            Axis::Min => value * screen.scale_x.min(screen.scale_y),
            Axis::Max => value * screen.scale_x.max(screen.scale_y),
            Axis::Average => value * (screen.scale_x + screen.scale_y) / 2.0,
        }
    }

    pub fn percent(value: f32, axis: Axis, mut relative_constraints: MutRef<Constraints>) -> f32 {
        match axis {
            Axis::X => value * relative_constraints.get_width(),
            Axis::Y => value * relative_constraints.get_height(),
            Axis::Min => value * relative_constraints.get_width().min(relative_constraints.get_height()),
            Axis::Max => value * relative_constraints.get_width().max(relative_constraints.get_height()),
            Axis::Average => value * (relative_constraints.get_width() + relative_constraints.get_height()) / 2.0,
        }
    }

    pub fn screen_percent(value: f32, axis: Axis, screen: Ref<ScreenProperty>) -> f32 {
        match axis {
            Axis::X => value * screen.width as f32,
            Axis::Y => value * screen.height as f32,
            Axis::Min => value * screen.width.min(screen.height) as f32,
            Axis::Max => value * screen.width.max(screen.height) as f32,
            Axis::Average => value * (screen.width + screen.height) as f32 / 2.0,
        }
    }

    pub fn check_relative(self_id: String, parent: Option<String>, element: &mut ElementRef) {
        if *element == ElementRef::Parent && parent.is_none() {
            *element = ElementRef::Screen;
        } else if let ElementRef::Other(e_id) = element.clone() {
            if e_id == self_id {
                *element = ElementRef::This;
            } else if let Some(parent) = parent.clone() {
                if parent == e_id {
                    *element = ElementRef::Parent;
                }
            }
        } 
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                        Constraint State                                        //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq)]
pub enum ConstraintState {
    Unknown,
    Calculating,
    Calculated(f32)
}

impl ConstraintState {


    pub fn unwrap(self) -> f32 {
        if let ConstraintState::Calculated(value) = self {
            return value;
        } else {
            panic!("Failed to unwrap Constraint State!");
        }
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Constraints                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Constraints {
    parent: Option<String>,
    pub constraint_x: ConstraintX,
    pub constraint_y: ConstraintY,
    pub constraint_width: ConstraintWidth,
    pub constraint_height: ConstraintHeight,
    last_width: ConstraintState,
    last_height: ConstraintState,
    x: ConstraintState,
    y: ConstraintState,
    width: ConstraintState,
    height: ConstraintState,
    scale_x: f32,
    scale_y: f32,
}

impl Default for Constraints {
    fn default() -> Self {
        return Constraints {
            parent: None,
            constraint_x: Default::default(),
            constraint_y: Default::default(),
            constraint_width: Default::default(),
            constraint_height: Default::default(),
            last_width: ConstraintState::Unknown,
            last_height: ConstraintState::Unknown,
            x: ConstraintState::Unknown,
            y: ConstraintState::Unknown,
            width: ConstraintState::Unknown,
            height: ConstraintState::Unknown,
            scale_x: 1.0,
            scale_y: 1.0
        }
    }
}

impl Constraints {

    pub fn new(constraint_x: ConstraintX, constraint_y: ConstraintY, constraint_width: ConstraintWidth, constraint_height: ConstraintHeight) -> Constraints {
        return Constraints {
            parent: None,
            constraint_x,
            constraint_y,
            constraint_width,
            constraint_height,
            last_width: ConstraintState::Unknown,
            last_height: ConstraintState::Unknown,
            x: ConstraintState::Unknown,
            y: ConstraintState::Unknown,
            width: ConstraintState::Unknown,
            height: ConstraintState::Unknown,
            scale_x: 1.0,
            scale_y: 1.0
        }
    }

    pub(crate) fn apply_to(&mut self, parent: String) {
        self.parent = Some(parent);
    }

    pub fn prepare(&mut self) {
        self.last_width = self.width;
        self.last_height = self.height;
        self.x = ConstraintState::Unknown;
        self.y = ConstraintState::Unknown;
        self.width = ConstraintState::Unknown;
        self.height = ConstraintState::Unknown;
    }

    fn calculate_x(&mut self) {
        self.x = ConstraintState::Calculating;
        let mut system = get_system();

        let x = if self.parent.is_none() {
            DEFAULT_POS
        } else {
            let screen = system.get_attribute::<ScreenProperty>("screen").assume("Pepper/Constraints", "Screen Property not defined!");
            let mut this = system.get_entity_mut(self.parent.clone().unwrap());
            let parent = this.get_component_mut::<ParentComponent>().assume("Pepper/Constraints", format!("Parent Component not defined for '{}'!", self.parent.clone().unwrap()).as_str()).0.clone();

            match self.constraint_x.clone() {
                ConstraintX::Pixel { value, relative, from: (side, mut from)} => {
                    // If no parent then parent == screen
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    if let Some((axis, mut relative)) = relative {
                        // If no parent then parent == screen
                        util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut relative);

                        match side {
                            SideX::FromLeft => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), self.get_width())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), 0.0, self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), 0.0, self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), 0.0, self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), 0.0, self.get_width())
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), self.get_width())
                                            }
                                        }
                                    }
                                }
                            },
                            SideX::FromRight => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Parent => {
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), 0.0, screen.width as f32)
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32)
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), 0.0, screen.width as f32)
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32)
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Parent => {
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width())
                                            }
                                        }
                                    }
                                }
                            }
                            SideX::InLeft => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), element_constraints.get_x())
                                            }
                                            ElementRef::Parent => {
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x())
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), 0.0)
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), 0.0)
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), 0.0)
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), 0.0)
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), element_constraints.get_x())
                                            }
                                            ElementRef::Parent => {
                                                let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x())
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x())
                                            }
                                        }
                                    }
                                }
                            }
                            SideX::InRight => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), 0.0, screen.width as f32, self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), 0.0, screen.width as f32, self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Parent => {
                                                let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    } else {
                        match side {
                            SideX::FromLeft => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::from_left(value, element_constraints.get_x(), self.get_width())
                                    }
                                    ElementRef::Screen => util::from_left(value, 0.0, self.get_width()),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::from_left(value, element_constraints.get_x(), self.get_width())
                                    }
                                }
                            }
                            SideX::FromRight => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::from_right(value, element_constraints.get_x(), element_constraints.get_width())
                                    }
                                    ElementRef::Screen => util::from_right(value, 0.0, screen.width as f32),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::from_right(value, element_constraints.get_x(), element_constraints.get_width())
                                    }
                                }
                            }
                            SideX::InLeft => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::in_left(value, element_constraints.get_x())
                                    }
                                    ElementRef::Screen => util::in_left(value, 0.0),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::in_left(value, element_constraints.get_x())
                                    }
                                }
                            }
                            SideX::InRight => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::in_right(value, element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Screen => util::in_right(value, 0.0, screen.width as f32, self.get_width()),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::in_right(value, element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                }
                            }
                        }
                    }
                }
                ConstraintX::PixelCentered { value, relative, mut from } => {
                    // If no parent then parent == screen
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    if let Some((axis, relative)) = relative {
                        match from {
                            ElementRef::This => {
                                warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_POS
                            }
                            ElementRef::Parent => {
                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Parent => {
                                        util::centered(util::pixel_relative(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element2 = system.get_entity_mut(id.clone());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                }
                            }
                            ElementRef::Screen => {
                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), 0.0, screen.width as f32, self.get_width())
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), 0.0, screen.width as f32, self.get_width())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                    }
                                }
                            }
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());

                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Parent => {
                                        let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element2 = system.get_entity_mut(id.clone());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                    }
                                }
                            }
                        }
                    } else {
                        match from {
                            ElementRef::This => {
                                warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_POS
                            }
                            ElementRef::Parent => {
                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                util::centered(value, element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                            }
                            ElementRef::Screen => util::centered(value, 0.0, screen.width as f32, self.get_width()),
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                util::centered(value, element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                            }
                        }
                    }
                }
                ConstraintX::Percent { value, relative: (axis, mut relative), from: (side, mut from) } => {
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut relative);

                    match side {
                        SideX::FromLeft => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::from_left(util::percent(value, axis, element_constraints), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints2), element_constraints.get_x(), self.get_width())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), 0.0, self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints), 0.0, self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), 0.0, self.get_width())
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints), 0.0, self.get_width())
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::from_left(util::percent(value, axis, element_constraints), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), element_constraints.get_x(), self.get_width())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints2), element_constraints.get_x(), self.get_width())
                                        },
                                    }
                                }
                            }
                        }
                        SideX::FromRight => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::from_right(util::percent(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), 0.0, screen.width as f32)
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints), 0.0, screen.width as f32)
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), 0.0, screen.width as f32)
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints), 0.0, screen.width as f32)
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::from_right(util::percent(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width())
                                        },
                                    }
                                }
                            }
                        }
                        SideX::InLeft => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), element_constraints.get_x())
                                        },
                                        ElementRef::Parent => {
                                            util::in_left(util::percent(value, axis, element_constraints), element_constraints.get_x())
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), element_constraints.get_x())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints2), element_constraints.get_x())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), 0.0)
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints), 0.0)
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), 0.0)
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints), 0.0)
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), element_constraints.get_x())
                                        },
                                        ElementRef::Parent => {
                                            util::in_left(util::percent(value, axis, element_constraints), element_constraints.get_x())
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), element_constraints.get_x())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints2), element_constraints.get_x())
                                        },
                                    }
                                }
                            }
                        }
                        SideX::InRight => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::in_right(util::percent(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), 0.0, screen.width as f32, self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), 0.0, screen.width as f32, self.get_width())
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Parent => {
                                            util::in_right(util::percent(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                ConstraintX::PercentCentered { value, relative: (axis, relative), from } => {
                    match from {
                        ElementRef::This => {
                            warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                            DEFAULT_POS
                        }
                        ElementRef::Parent => {
                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                            let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Parent => {
                                    util::centered(util::percent(value, axis, element_constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Other(id) => {
                                    let mut element2 = system.get_entity_mut(id.clone());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                            }
                        }
                        ElementRef::Screen => {
                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), 0.0, screen.width as f32, self.get_width())
                                }
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), 0.0, screen.width as f32, self.get_width())
                                }
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints), 0.0, screen.width as f32, self.get_width())
                                }
                            }
                        }
                        ElementRef::Other(id) => {
                            let mut element = system.get_entity_mut(id.clone());
                            let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());

                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Parent => {
                                    let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                                ElementRef::Other(id) => {
                                    let mut element2 = system.get_entity_mut(id.clone());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_x(), element_constraints.get_width(), self.get_width())
                                }
                            }
                        }
                    }
                }
            }
        };

        self.x = ConstraintState::Calculated(x);
    }

    fn calculate_y(&mut self) {
        self.y = ConstraintState::Calculating;
        let mut system = get_system();

        let y = if self.parent.is_none() {
            DEFAULT_POS
        } else {
            let screen = system.get_attribute::<ScreenProperty>("screen").assume("Pepper/Constraints", "Screen Property not defined!");
            let mut this = system.get_entity_mut(self.parent.clone().unwrap());
            let parent = this.get_component_mut::<ParentComponent>().assume("Pepper/Constraints", format!("Parent Component not defined for '{}'!", self.parent.clone().unwrap()).as_str()).0.clone();

            match self.constraint_y.clone() {
                ConstraintY::Pixel { value, relative, from: (side, mut from)} => {
                    // If no parent then parent == screen
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    if let Some((axis, mut relative)) = relative {
                        // If no parent then parent == screen
                        util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut relative);

                        match side {
                            SideY::FromTop => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), self.get_height())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), 0.0, self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), 0.0, self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), 0.0, self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), 0.0, self.get_height())
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_left(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                util::from_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::from_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), self.get_height())
                                            }
                                        }
                                    }
                                }
                            },
                            SideY::FromBottom => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Parent => {
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), 0.0, screen.height as f32)
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32)
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), 0.0, screen.height as f32)
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32)
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::from_right(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Parent => {
                                                util::from_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::from_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::from_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height())
                                            }
                                        }
                                    }
                                }
                            }
                            SideY::InTop => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), element_constraints.get_y())
                                            }
                                            ElementRef::Parent => {
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y())
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), 0.0)
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), 0.0)
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), 0.0)
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints), 0.0)
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_left(util::pixel_relative(value, axis, constraints), element_constraints.get_y())
                                            }
                                            ElementRef::Parent => {
                                                let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y())
                                            }
                                            ElementRef::Screen => {
                                                util::in_left(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_left(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y())
                                            }
                                        }
                                    }
                                }
                            }
                            SideY::InBottom => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                        }
                                    }
                                    ElementRef::Screen => {
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), 0.0, screen.height as f32, self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), 0.0, screen.height as f32, self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element = system.get_entity_mut(id.clone());
                                                let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                            }
                                        }
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        match relative {
                                            ElementRef::This => {
                                                let constraints = MutRef::new(self as *mut Constraints);
                                                util::in_right(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Parent => {
                                                let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Screen => {
                                                util::in_right(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                            ElementRef::Other(id) => {
                                                let mut element2 = system.get_entity_mut(id.clone());
                                                let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                                util::in_right(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                            }
                                        }
                                    }
                                }
                            },
                        }
                    } else {
                        match side {
                            SideY::FromTop => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::from_left(value, element_constraints.get_y(), self.get_height())
                                    }
                                    ElementRef::Screen => util::from_left(value, 0.0, self.get_height()),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::from_left(value, element_constraints.get_y(), self.get_height())
                                    }
                                }
                            }
                            SideY::FromBottom => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::from_right(value, element_constraints.get_y(), element_constraints.get_height())
                                    }
                                    ElementRef::Screen => util::from_right(value, 0.0, screen.height as f32),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::from_right(value, element_constraints.get_y(), element_constraints.get_height())
                                    }
                                }
                            }
                            SideY::InTop => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::in_left(value, element_constraints.get_y())
                                    }
                                    ElementRef::Screen => util::in_left(value, 0.0),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::in_left(value, element_constraints.get_y())
                                    }
                                }
                            }
                            SideY::InBottom => {
                                match from {
                                    ElementRef::This => {
                                        warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                        DEFAULT_POS
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::in_right(value, element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Screen => util::in_right(value, 0.0, screen.height as f32, self.get_height()),
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                        util::in_right(value, element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                }
                            }
                        }
                    }
                }
                ConstraintY::PixelCentered { value, relative, mut from } => {
                    // If no parent then parent == screen
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    if let Some((axis, relative)) = relative {
                        match from {
                            ElementRef::This => {
                                warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_POS
                            }
                            ElementRef::Parent => {
                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Parent => {
                                        util::centered(util::pixel_relative(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element2 = system.get_entity_mut(id.clone());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                }
                            }
                            ElementRef::Screen => {
                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), 0.0, screen.height as f32, self.get_height())
                                    }
                                    ElementRef::Parent => {
                                        let mut element = system.get_entity_mut(parent.clone().unwrap());
                                        let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), 0.0, screen.height as f32, self.get_height())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element = system.get_entity_mut(id.clone());
                                        let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                    }
                                }
                            }
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());

                                match relative {
                                    ElementRef::This => {
                                        let constraints = MutRef::new(self as *mut Constraints);
                                        util::centered(util::pixel_relative(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Parent => {
                                        let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Screen => {
                                        util::centered(util::screen_pixel_relative(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                    ElementRef::Other(id) => {
                                        let mut element2 = system.get_entity_mut(id.clone());
                                        let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                        util::centered(util::pixel_relative(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                    }
                                }
                            }
                        }
                    } else {
                        match from {
                            ElementRef::This => {
                                warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_POS
                            }
                            ElementRef::Parent => {
                                let mut element = system.get_entity_mut(parent.clone().unwrap());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                util::centered(value, element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                            }
                            ElementRef::Screen => util::centered(value, 0.0, screen.height as f32, self.get_height()),
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());
                                util::centered(value, element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                            }
                        }
                    }
                }
                ConstraintY::Percent { value, relative: (axis, mut relative), from: (side, mut from) } => {
                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut from);

                    util::check_relative(self.parent.clone().unwrap(), parent.clone(), &mut relative);

                    match side {
                        SideY::FromTop => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::from_left(util::percent(value, axis, element_constraints), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints2), element_constraints.get_y(), self.get_height())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), 0.0, self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints), 0.0, self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), 0.0, self.get_height())
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints), 0.0, self.get_height())
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_left(util::percent(value, axis, constraints), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::from_left(util::percent(value, axis, element_constraints), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::from_left(util::screen_percent(value, axis, screen), element_constraints.get_y(), self.get_height())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_left(util::percent(value, axis, element_constraints2), element_constraints.get_y(), self.get_height())
                                        },
                                    }
                                }
                            }
                        }
                        SideY::FromBottom => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::from_right(util::percent(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), 0.0, screen.height as f32)
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints), 0.0, screen.height as f32)
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), 0.0, screen.height as f32)
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints), 0.0, screen.height as f32)
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::from_right(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::from_right(util::percent(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::from_right(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::from_right(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height())
                                        },
                                    }
                                }
                            }
                        }
                        SideY::InTop => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), element_constraints.get_y())
                                        },
                                        ElementRef::Parent => {
                                            util::in_left(util::percent(value, axis, element_constraints), element_constraints.get_y())
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), element_constraints.get_y())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints2), element_constraints.get_y())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), 0.0)
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints), 0.0)
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), 0.0)
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints), 0.0)
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_left(util::percent(value, axis, constraints), element_constraints.get_y())
                                        },
                                        ElementRef::Parent => {
                                            util::in_left(util::percent(value, axis, element_constraints), element_constraints.get_y())
                                        },
                                        ElementRef::Screen => {
                                            util::in_left(util::screen_percent(value, axis, screen), element_constraints.get_y())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_left(util::percent(value, axis, element_constraints2), element_constraints.get_y())
                                        },
                                    }
                                }
                            }
                        }
                        SideY::InBottom => {
                            match from {
                                ElementRef::This => {
                                    warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_POS
                                },
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::in_right(util::percent(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Other(id) => {
                                            let mut element2 = system.get_entity_mut(id.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                    }
                                },
                                ElementRef::Screen => {
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), 0.0, screen.height as f32, self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), 0.0, screen.height as f32, self.get_height())
                                        }
                                        ElementRef::Other(id) => {
                                            let mut element = system.get_entity_mut(id.clone());
                                            let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                        }
                                    }
                                },
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());   
                                    match relative {
                                        ElementRef::This => {
                                            let constraints = MutRef::new(self as *mut Constraints);
                                            util::in_right(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Parent => {
                                            util::in_right(util::percent(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Screen => {
                                            util::in_right(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                        ElementRef::Other(id2) => {
                                            let mut element2 = system.get_entity_mut(id2.clone());
                                            let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());   
                                            util::in_right(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                ConstraintY::PercentCentered { value, relative: (axis, relative), from } => {
                    match from {
                        ElementRef::This => {
                            warn!("Pepper/Constraints", "Recursive x constraint for '{}'", self.parent.clone().unwrap());
                            DEFAULT_POS
                        }
                        ElementRef::Parent => {
                            let mut element = system.get_entity_mut(parent.clone().unwrap());
                            let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Parent => {
                                    util::centered(util::percent(value, axis, element_constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Other(id) => {
                                    let mut element2 = system.get_entity_mut(id.clone());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                            }
                        }
                        ElementRef::Screen => {
                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), 0.0, screen.height as f32, self.get_height())
                                }
                                ElementRef::Parent => {
                                    let mut element = system.get_entity_mut(parent.clone().unwrap());
                                    let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), 0.0, screen.height as f32, self.get_height())
                                }
                                ElementRef::Other(id) => {
                                    let mut element = system.get_entity_mut(id.clone());
                                    let element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints), 0.0, screen.height as f32, self.get_height())
                                }
                            }
                        }
                        ElementRef::Other(id) => {
                            let mut element = system.get_entity_mut(id.clone());
                            let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id).as_str());

                            match relative {
                                ElementRef::This => {
                                    let constraints = MutRef::new(self as *mut Constraints);
                                    util::centered(util::percent(value, axis, constraints), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Parent => {
                                    let mut element2 = system.get_entity_mut(parent.clone().unwrap());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", parent.clone().unwrap()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Screen => {
                                    util::centered(util::screen_percent(value, axis, screen), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                                ElementRef::Other(id) => {
                                    let mut element2 = system.get_entity_mut(id.clone());
                                    let element_constraints2 = element2.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'", id.clone()).as_str());
                                    util::centered(util::percent(value, axis, element_constraints2), element_constraints.get_y(), element_constraints.get_height(), self.get_height())
                                }
                            }
                        }
                    }
                }
            }
        };

        self.y = ConstraintState::Calculated(y);
    }

    fn calculate_width(&mut self) {
        self.width = ConstraintState::Calculating;
        let mut system = get_system();

        let width = if self.parent.is_none() {
            DEFAULT_SIZE
        } else {
            let screen = system.get_attribute::<ScreenProperty>("screen").assume("Pepper/Constraints", "Screen Property not defined!");
            let mut this = system.get_entity_mut(self.parent.clone().unwrap());
            let parent = this.get_component_mut::<ParentComponent>().assume("Pepper/Constraints", format!("Parent Component not defined for '{}'!", self.parent.clone().unwrap()).as_str()).0.clone();

            match self.constraint_width.clone() {
                ConstraintWidth::Pixel { value, relative } => {
                    if let Some((axis, mut element)) = relative {
                        // If no parent then parent == screen
                        if element == ElementRef::Parent && parent.is_none() {
                            element = ElementRef::Screen;
                        } else if let ElementRef::Other(id) = element.clone() {
                            if id == self.parent.clone().unwrap() {
                                element = ElementRef::This;
                            } else if let Some(parent) = parent.clone() {
                                if parent == id {
                                    element = ElementRef::Parent;
                                }
                            }
                        }

                        match element {
                            ElementRef::This => {
                                if axis == Axis::Y {
                                    value * self.get_scale_y()
                                } else {
                                    warn!("Pepper/Constraints", "Recursive width constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_SIZE
                                }
                            }
                            ElementRef::Parent => {
                                let mut parent_constraints = system.get_entity_mut(parent.clone().unwrap()).get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'!", parent.clone().unwrap()).as_str());
                                match axis {
                                    Axis::X => parent_constraints.get_scale_x() * value,
                                    Axis::Y => parent_constraints.get_scale_y() * value,
                                    Axis::Min => parent_constraints.get_scale_x().min(parent_constraints.get_scale_y()) * value,
                                    Axis::Max => parent_constraints.get_scale_x().max(parent_constraints.get_scale_y()) * value,
                                    Axis::Average => (parent_constraints.get_scale_x() + parent_constraints.get_scale_y()) / 2.0 * value,
                                }
                            }
                            ElementRef::Screen => {
                                match axis {
                                    Axis::X => screen.scale_x * value,
                                    Axis::Y => screen.scale_y * value,
                                    Axis::Min => screen.scale_x.min(screen.scale_y) * value,
                                    Axis::Max => screen.scale_x.max(screen.scale_y) * value,
                                    Axis::Average => (screen.scale_x + screen.scale_y) / 2.0 * value
                                }
                            }
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'!", id).as_str());
                                match axis {
                                    Axis::X => element_constraints.get_scale_x() * value,
                                    Axis::Y => element_constraints.get_scale_y() * value,
                                    Axis::Min => element_constraints.get_scale_x().min(element_constraints.get_scale_y()) * value,
                                    Axis::Max => element_constraints.get_scale_x().max(element_constraints.get_scale_y()) * value,
                                    Axis::Average => (element_constraints.get_scale_y() + element_constraints.get_scale_y()) / 2.0 * value,
                                }
                            }
                        }
                    } else {
                        value
                    }
                }
                ConstraintWidth::Percent { value, relative } => {
                    let (axis, mut element) = relative;
                    // If no parent then parent == screen
                    if element == ElementRef::Parent && parent.is_none() {
                        element = ElementRef::Screen;
                    } else if let ElementRef::Other(id) = element.clone() {
                        if id == self.parent.clone().unwrap() {
                            element = ElementRef::This;
                        } else if let Some(parent) = parent.clone() {
                            if parent == id {
                                element = ElementRef::Parent;
                            }
                        }
                    }

                    match element {
                        ElementRef::This => {
                            if axis == Axis::Y {
                                value * self.get_height()
                            } else {
                                warn!("Pepper/Constraints", "Recursive width constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_SIZE
                            }
                        }
                        ElementRef::Parent => {
                            let mut parent_constraints = system.get_entity_mut(parent.clone().unwrap()).get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'", parent.clone().unwrap()).as_str());
                            match axis {
                                Axis::X => parent_constraints.get_width() * value,
                                Axis::Y => parent_constraints.get_height() * value,
                                Axis::Min => parent_constraints.get_width().min(parent_constraints.get_height()) * value,
                                Axis::Max => parent_constraints.get_width().max(parent_constraints.get_height()) * value,
                                Axis::Average => (parent_constraints.get_width() + parent_constraints.get_height()) / 2.0 * value,
                            }
                        }
                        ElementRef::Screen => {
                            match axis {
                                Axis::X => screen.width as f32 * value,
                                Axis::Y => screen.height as f32 * value,
                                Axis::Min => screen.width.min(screen.height) as f32 * value,
                                Axis::Max => screen.width.max(screen.height) as f32 * value,
                                Axis::Average => (screen.width + screen.height) as f32 / 2.0 * value
                            }
                        }
                        ElementRef::Other(id) => {
                            let mut parent = system.get_entity_mut(id.clone());
                            let mut parent_constraints = parent.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'!", id).as_str());
                            match axis {
                                Axis::X => parent_constraints.get_width() as f32 * value,
                                Axis::Y => parent_constraints.get_height() as f32 * value,
                                Axis::Min => parent_constraints.get_width().min(parent_constraints.get_height()) as f32 * value,
                                Axis::Max => parent_constraints.get_height().max(parent_constraints.get_width()) as f32 * value,
                                Axis::Average => (parent_constraints.get_width() + parent_constraints.get_height()) as f32 / 2.0 * value
                            }
                        }
                    }
                }
            }
        };

        // Update Width, Last Width and Scale
        if let ConstraintState::Calculated(last_width) = self.width {
            self.scale_x *= width / last_width;
        }
        self.last_width = self.width;
        self.width = ConstraintState::Calculated(width);
    }

    fn calculate_height(&mut self) {
        let mut system = get_system();

        let height = if self.parent.is_none() {
            DEFAULT_SIZE
        } else {
            let screen = system.get_attribute::<ScreenProperty>("screen").assume("Pepper/Constraints", "Screen Property not defined!");
            let mut this = system.get_entity_mut(self.parent.clone().unwrap());
            let parent = this.get_component_mut::<ParentComponent>().assume("Pepper/Constraints", format!("Parent Component not defined for '{}'!", self.parent.clone().unwrap()).as_str()).0.clone();

            match self.constraint_height.clone() {
                ConstraintHeight::Pixel { value, relative } => {
                    if let Some((axis, mut element)) = relative {
                        // If no parent then parent == screen
                        if element == ElementRef::Parent && parent.is_none() {
                            element = ElementRef::Screen;
                        } else if let ElementRef::Other(id) = element.clone() {
                            if id == self.parent.clone().unwrap() {
                                element = ElementRef::This;
                            } else if let Some(parent) = parent.clone() {
                                if parent == id {
                                    element = ElementRef::Parent;
                                }
                            }
                        }

                        match element {
                            ElementRef::This => {
                                if axis == Axis::X {
                                    value * self.get_scale_x()
                                } else {
                                    warn!("Pepper/Constraints", "Recursive height constraint for '{}'", self.parent.clone().unwrap());
                                    DEFAULT_SIZE
                                }
                            }
                            ElementRef::Parent => {
                                let mut parent_constraints = system.get_entity_mut(parent.clone().unwrap()).get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraint Component not defined for '{}'!", parent.clone().unwrap()).as_str());
                                match axis {
                                    Axis::X => parent_constraints.get_scale_x() * value,
                                    Axis::Y => parent_constraints.get_scale_y() * value,
                                    Axis::Min => parent_constraints.get_scale_x().min(parent_constraints.get_scale_y()) * value,
                                    Axis::Max => parent_constraints.get_scale_x().max(parent_constraints.get_scale_y()) * value,
                                    Axis::Average => (parent_constraints.get_scale_x() + parent_constraints.get_scale_y()) / 2.0 * value,
                                }
                            }
                            ElementRef::Screen => {
                                match axis {
                                    Axis::X => screen.scale_x * value,
                                    Axis::Y => screen.scale_y * value,
                                    Axis::Min => screen.scale_x.min(screen.scale_y) * value,
                                    Axis::Max => screen.scale_x.max(screen.scale_y) * value,
                                    Axis::Average => (screen.scale_x + screen.scale_y) / 2.0 * value
                                }
                            }
                            ElementRef::Other(id) => {
                                let mut element = system.get_entity_mut(id.clone());
                                let mut element_constraints = element.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'!", id).as_str());
                                match axis {
                                    Axis::X => element_constraints.get_scale_x() * value,
                                    Axis::Y => element_constraints.get_scale_y() * value,
                                    Axis::Min => element_constraints.get_scale_x().min(element_constraints.get_scale_y()) * value,
                                    Axis::Max => element_constraints.get_scale_x().max(element_constraints.get_scale_y()) * value,
                                    Axis::Average => (element_constraints.get_scale_y() + element_constraints.get_scale_y()) / 2.0 * value,
                                }
                            }
                        }
                    } else {
                        value
                    }
                }
                ConstraintHeight::Percent { value, relative } => {
                    let (axis, mut element) = relative;
                    // If no parent then parent == screen
                    if element == ElementRef::Parent && parent.is_none() {
                        element = ElementRef::Screen;
                    } else if let ElementRef::Other(id) = element.clone() {
                        if id == self.parent.clone().unwrap() {
                            element = ElementRef::This;
                        } else if let Some(parent) = parent.clone() {
                            if parent == id {
                                element = ElementRef::Parent;
                            }
                        }
                    }

                    match element {
                        ElementRef::This => {
                            if axis == Axis::X {
                                value * self.get_width()
                            } else {
                                warn!("Pepper/Constraints", "Recursive height constraint for '{}'", self.parent.clone().unwrap());
                                DEFAULT_SIZE
                            }
                        }
                        ElementRef::Parent => {
                            let parent_constraints = system.get_entity_mut(parent.clone().unwrap()).get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'", parent.clone().unwrap()).as_str());
                            util::percent(value, axis, parent_constraints)
                        }
                        ElementRef::Screen => {
                            util::screen_percent(value, axis, screen)
                        }
                        ElementRef::Other(id) => {
                            let mut parent = system.get_entity_mut(id.clone());
                            let parent_constraints = parent.get_component_mut::<Constraints>().assume("Pepper/Constraints", format!("Constraints Component not defined for '{}'!", id).as_str());
                            util::percent(value, axis, parent_constraints)
                        }
                    }
                }
            }
        };

        // Update Height, Last Height and Scale
        if let ConstraintState::Calculated(last_height) = self.height {
            self.scale_y *= height / last_height;
        }
        self.last_height = self.height;
        self.height = ConstraintState::Calculated(height);
    }

    pub fn get_x(&mut self) -> f32 {
        if self.x == ConstraintState::Unknown {
            self.calculate_x();
        } else if self.x == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive x constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.x.unwrap();
    }

    pub fn get_y(&mut self) -> f32 {
        if self.y == ConstraintState::Unknown {
            self.calculate_y();
        } else if self.y == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive y constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.y.unwrap();
    }

    pub fn get_width(&mut self) -> f32 {
        if self.width == ConstraintState::Unknown {
            self.calculate_width();
        } else if self.width == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive width constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.width.unwrap();
    }

    pub fn get_height(&mut self) -> f32 {
        if self.height == ConstraintState::Unknown {
            self.calculate_height();
        } else if self.height == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive height constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.height.unwrap();
    }

    pub fn get_scale_x(&mut self) -> f32 {
        if self.width == ConstraintState::Unknown {
            self.calculate_width();
        } else if self.width == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive width constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.scale_x;
    }

    pub fn get_scale_y(&mut self) -> f32 {
        if self.height == ConstraintState::Unknown {
            self.calculate_height();
        } else if self.height == ConstraintState::Calculating {
            critical!("Pepper/Constraints", "Recursive height constraint for '{}'!", self.parent.clone().unwrap());
        }
        return self.scale_y;
    }

}
