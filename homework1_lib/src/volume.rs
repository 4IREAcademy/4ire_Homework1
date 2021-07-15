use std::f32::consts::PI;

pub enum VolumeOf {
    Sphere{radius: f32},
    Parallelepiped{a:f32, b:f32, height:f32},
}

pub fn find_volume(figure: VolumeOf) -> f32 {
    match figure {
        VolumeOf::Sphere{radius}=>(4.0/3.0)*radius*radius*radius*PI,
        VolumeOf::Parallelepiped{a,b,height}=>a*b*height,
    }
}