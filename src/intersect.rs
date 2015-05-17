// Copyright 2015 Nicholas Bishop
//
// Intersection code adapted from "Real-Time Collision Detection" by
// Christer Ericson, published by Morgan Kaufmann Publishers,
// Copyright 2005 Elsevier Inc
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sphere::Sphere3f;
use vec3f::Vec3f;

/// Calculate closest point on triangle to an input point
fn triangle_closest_point_to_point(triangle: [Vec3f; 3], p: Vec3f) -> Vec3f {
    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    // Check if P in vertex region outside A
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let d1 = ab.dot(ap);
    let d2 = ac.dot(ap);
    if d1 <= 0.0f32 && d2 <= 0.0f32 {
        return a; // barycentric coordinates (1,0,0)
    }
    // Check if P in vertex region outside B
    let bp = p - b;
    let d3 = ab.dot(bp);
    let d4 = ac.dot(bp);
    if d3 >= 0.0f32 && d4 <= d3 {
        return b; // barycentric coordinates (0,1,0)
    }

    // Check if P in edge region of AB, if so return projection of P onto AB
    let vc = d1*d4 - d3*d2;
    if vc <= 0.0f32 && d1 >= 0.0f32 && d3 <= 0.0f32 {
        let v = d1 / (d1 - d3);
        return a + ab * v; // barycentric coordinates (1-v,v,0)
    }
    // Check if P in vertex region outside C
    let cp = p - c;
    let d5 = ab.dot(cp);
    let d6 = ac.dot(cp);
    if d6 >= 0.0f32 && d5 <= d6 {
        return c; // barycentric coordinates (0,0,1)
    }

    // Check if P in edge region of AC, if so return projection of P onto AC
    let vb = d5*d2 - d1*d6;
    if vb <= 0.0f32 && d2 >= 0.0f32 && d6 <= 0.0f32 {
        let w = d2 / (d2 - d6);
        return a + ac * w; // barycentric coordinates (1-w,0,w)
    }
    // Check if P in edge region of BC, if so return projection of P onto BC
    let va = d3*d6 - d5*d4;
    if va <= 0.0f32 && (d4 - d3) >= 0.0f32 && (d5 - d6) >= 0.0f32 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return b + (c - b) * w; // barycentric coordinates (0,1-w,w)
    }
    // P inside face region. Compute Q through its barycentric coordinates (u,v,w)
    let denom = 1.0f32 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    return a + ab * v + ac * w; // = u*a + v*b + w*c, u = va * denom = 1.0f - v - w
}

// Test for intersection between sphere and triangle
//
// Returns None if the inputs do not intersect, otherwise returns the
// point on the triangle closest to the sphere center.
pub fn intersect_sphere_triangle(sphere: &Sphere3f,
                                 triangle: [Vec3f; 3]) -> Option<Vec3f> {
    // Find point P on triangle ABC closest to sphere center
    let p = triangle_closest_point_to_point(triangle, sphere.center);

    // Sphere and triangle intersect if the (squared) distance from
    // sphere center to point p is less than the (squared) sphere
    // radius
    let v = p - sphere.center;
    if v.magnitude_squared() <= sphere.radius_squared() {
        Some(p)
    }
    else {
        None
    }
}

#[test]
fn test_intersect_sphere_triangle() {
    use sphere;
    use vec3f::{vec3f, ZERO_3F};

    let triangle = [
        vec3f(0, 0, 0),
        vec3f(1, 0, 0),
        vec3f(0, 1, 0)];

    assert!(intersect_sphere_triangle(&sphere::UNIT,
                                      triangle) == Some(ZERO_3F));

    assert!(intersect_sphere_triangle(&Sphere3f::new(vec3f(10, 10, 10), 1.0),
                                      triangle) == None);
}
