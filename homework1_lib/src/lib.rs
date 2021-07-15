pub mod area;
pub mod volume;

#[cfg(test)]
mod tests {
    use crate::area::{find_area, AreaOf};
    use std::f32::consts::PI;
    use crate::volume::{find_volume, VolumeOf};

    #[test]
    fn area_works() {
        assert_eq!(find_area(AreaOf::Circle {radius:2.0}), 4.0*PI);
        assert_eq!(find_area(AreaOf::Rectangle{width:2.1,height:4.0}), 8.4);
    }

    #[test]
    fn volume_works() {
        assert_eq!(find_volume(VolumeOf::Sphere{radius:2.0}), 32.0*PI/3.0);
        assert_eq!(find_volume(VolumeOf::Parallelepiped{a:2.1,b:1.0,height:4.0}), 8.4);
    }

}
