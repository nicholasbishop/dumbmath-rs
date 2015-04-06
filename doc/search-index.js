var searchIndex = {};
searchIndex['dumbmath'] = {"items":[[0,"","dumbmath","",null,null],[0,"aabb","","",null,null],[3,"Aabb3f","dumbmath::aabb","",null,null],[12,"min","","",0,null],[12,"max","","",0,null],[11,"new","","Create an empty Aabb3f with min initialized to +infinity and\nmax is initialized to -infinity.",0,{"inputs":[{"name":"aabb3f"}],"output":{"name":"aabb3f"}}],[11,"from_point","","",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"aabb3f"}}],[11,"contains_point","","True if the point intersects the box",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[0,"intersect","dumbmath","",null,null],[5,"intersect_sphere_triangle","dumbmath::intersect","",null,null],[0,"range","dumbmath","",null,null],[3,"InclusiveRange","dumbmath::range","Inclusive range from min to max",null,null],[12,"min","","",1,null],[12,"max","","",1,null],[5,"range_clamp","","Create range covering the overlap between two ranges, or None if\nthere is no overlap.",null,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"option"}}],[17,"RANGE_0_1_F32","","",null,null],[11,"eq","","",1,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"inclusiverange"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"inclusiverange"}}],[11,"new","","Create an InclusiveRange range from min to max",1,{"inputs":[{"name":"inclusiverange"},{"name":"t"},{"name":"t"}],"output":{"name":"inclusiverange"}}],[11,"expand","","",1,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":null}],[0,"segment","dumbmath","",null,null],[3,"Segment3f","dumbmath::segment","Line segment between two points",null,null],[12,"start","","",2,null],[12,"end","","",2,null],[11,"eq","","",2,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"segment3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"segment3f"}],"output":{"name":"segment3f"}}],[11,"new","","Create a segment between two points",2,{"inputs":[{"name":"segment3f"},{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"segment3f"}}],[11,"length","","Length of the line segment",2,{"inputs":[{"name":"segment3f"}],"output":{"name":"f32"}}],[11,"distance_to_parametric_delta","","Convert a distance in coordinate space to a distance in the\nline segment's parametric space. The sign of the input is\nkept.",2,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"f32"}}],[11,"closest_point_to_point","","Find the point on the segment closest to the input point. The\nreturn value contains both the parametric and actual location\nof the closest point.",2,null],[0,"sphere","dumbmath","",null,null],[3,"Sphere3f","dumbmath::sphere","Sphere represented by a center and radius",null,null],[12,"center","","",3,null],[12,"radius","","",3,null],[17,"UNIT","","Sphere centered at (0.0, 0.0, 0.0) with a radius of 1.0",null,null],[11,"eq","","",3,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"clone","","",3,{"inputs":[{"name":"sphere3f"}],"output":{"name":"sphere3f"}}],[11,"new","","Create a Sphere3f from center point and radius",3,{"inputs":[{"name":"sphere3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"from_radius","","Create a Sphere3f centered at zero with given radius",3,{"inputs":[{"name":"sphere3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"radius_squared","","Squared radius of the sphere",3,{"inputs":[{"name":"sphere3f"}],"output":{"name":"f32"}}],[0,"vector","dumbmath","",null,null],[3,"Vec3f","dumbmath::vector","Vector with three f32 components",null,null],[12,"x","","",4,null],[12,"y","","",4,null],[12,"z","","",4,null],[5,"vec3f","","Create a Vec3f from x, y, and z inputs",null,{"inputs":[{"name":"x"},{"name":"y"},{"name":"z"}],"output":{"name":"vec3f"}}],[5,"dot3","","Inner product of two Vec3f inputs",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[5,"cross","","Cross product of two Vec3f inputs",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[5,"distance3","","Distance between two points",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[5,"lerp3","","Linearly interpolate between two points by the factor `t`. When\n`t` is zero the result is `p0`, and when `t` is one the result is\n`p1`. The range of `t` is not clamped.",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[17,"ZERO_3F","","Vec3f(0.0, 0.0, 0.0)",null,null],[8,"CastF32","","Convert a numeric type to an f32",null,null],[10,"as_f32","","",5,{"inputs":[{"name":"castf32"}],"output":{"name":"f32"}}],[11,"eq","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"vec3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"vec3f"}],"output":{"name":"vec3f"}}],[11,"new","","Create a Vec3f from three components",4,{"inputs":[{"name":"vec3f"},{"name":"f32"},{"name":"f32"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"from_scalar","","Create a Vec3f with all components set to the same scalar",4,{"inputs":[{"name":"vec3f"},{"name":"t"}],"output":{"name":"vec3f"}}],[11,"magnitude_squared","","Squared length of the vector",4,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"magnitude","","Length of the vector",4,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[6,"Output","","",null,null],[11,"add","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"div","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",4,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"sub","","",4,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}]],"paths":[[3,"Aabb3f"],[3,"InclusiveRange"],[3,"Segment3f"],[3,"Sphere3f"],[3,"Vec3f"],[8,"CastF32"]]};
initSearch(searchIndex);
