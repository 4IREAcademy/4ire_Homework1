use std::f32::consts::PI;

pub enum AreaOf {
    Circle {radius: f32},
    Rectangle{width:f32, height:f32},
}

pub fn find_area(figure: AreaOf) -> f32 {
    match figure {
        AreaOf::Circle {radius}=>radius*radius*PI,
        AreaOf::Rectangle{width,height}=>width*height,
    }
}