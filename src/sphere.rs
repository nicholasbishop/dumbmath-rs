use vector;

pub struct Sphere3f {
    pub center: vector::Vec3f,
    pub radius: f32
}

impl Sphere3f {
    pub fn new(center: vector::Vec3f, radius: f32) -> Sphere3f {
        Sphere3f {
            center: center,
            radius: radius
        }
    }
}

#[test]
fn test_sphere3f_create() {
    Sphere3f::new(vector::vec3f(1, 2, 3), 4.0);
}
