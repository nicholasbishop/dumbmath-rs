var searchIndex = {};
searchIndex['dumbmath'] = {"items":[[0,"","dumbmath","",null,null],[0,"aabb","","",null,null],[3,"Aabb3f","dumbmath::aabb","",null,null],[12,"min","","",0,null],[12,"max","","",0,null],[11,"new","","Create an empty Aabb3f with min initialized to +infinity and\nmax is initialized to -infinity.",0,{"inputs":[{"name":"aabb3f"}],"output":{"name":"aabb3f"}}],[11,"from_point","","",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"aabb3f"}}],[11,"contains_point","","True if the point intersects the box",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[0,"angle","dumbmath","",null,null],[3,"Degrees","dumbmath::angle","",null,null],[3,"Radians","","",null,null],[11,"partial_cmp","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"option"}}],[11,"lt","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"le","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"gt","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"ge","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"eq","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"degrees"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"degrees"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"option"}}],[11,"lt","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"le","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"gt","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"ge","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"radians"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"radians"}}],[11,"value","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"f32"}}],[11,"to_radians","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"radians"}}],[11,"value","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"f32"}}],[11,"to_degrees","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"degrees"}}],[0,"intersect","dumbmath","",null,null],[5,"intersect_sphere_triangle","dumbmath::intersect","",null,null],[0,"line","dumbmath","",null,null],[3,"Line3f","dumbmath::line","Line of infinite length represented by two distinct points it\npasses through.",null,null],[12,"points","","",3,null],[11,"eq","","",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"line3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"line3f"}],"output":{"name":"line3f"}}],[11,"new","","Create a new line. Return None if the input points are\nidentical.",3,{"inputs":[{"name":"line3f"},{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"option"}}],[11,"closest_points_between_lines","","Find the closest points between two lines. The result is a\npair of parametric points, the first for `self` and the second\nfor `line`. If the lines are parallel then None is\nreturned.",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"option"}}],[0,"range","dumbmath","",null,null],[3,"InclusiveRange","dumbmath::range","Inclusive range from min to max",null,null],[12,"min","","",4,null],[12,"max","","",4,null],[5,"range_clamp","","Create range covering the overlap between two ranges, or None if\nthere is no overlap.",null,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"option"}}],[5,"range_combine","","Create range covering both ranges (and any gap between them).",null,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"inclusiverange"}}],[6,"Rangef","","Convenience type: InclusiveRange<f32>",null,null],[17,"RANGE_0_1_F32","","",null,null],[11,"eq","","",4,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"inclusiverange"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"inclusiverange"}}],[11,"new","","Create an InclusiveRange from min to max. Asserts that min is\nless than max.",4,{"inputs":[{"name":"inclusiverange"},{"name":"t"},{"name":"t"}],"output":{"name":"inclusiverange"}}],[11,"from_sorting","","Create an InclusiveRange by sorting the inputs.",4,{"inputs":[{"name":"inclusiverange"},{"name":"t"},{"name":"t"}],"output":{"name":"inclusiverange"}}],[11,"empty","","True if `min == max`, false otherwise.",4,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"length","","Distance between self.min and self.max",4,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"t"}}],[11,"expand","","Expand `self` as needed to include another range.",4,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":null}],[0,"segment","dumbmath","",null,null],[3,"Segment3f","dumbmath::segment","Line segment between two points",null,null],[12,"start","","",5,null],[12,"end","","",5,null],[11,"eq","","",5,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"segment3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"segment3f"}],"output":{"name":"segment3f"}}],[11,"new","","Create a segment between two points",5,{"inputs":[{"name":"segment3f"},{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"segment3f"}}],[11,"to_vec3f","","Get vector from start to end (not normalized)",5,{"inputs":[{"name":"segment3f"}],"output":{"name":"vec3f"}}],[11,"length","","Length of the line segment",5,{"inputs":[{"name":"segment3f"}],"output":{"name":"f32"}}],[11,"reversed","","Create a copy of `self` with endpoints reversed.",5,{"inputs":[{"name":"segment3f"}],"output":{"name":"segment3f"}}],[11,"distance_to_parametric_delta","","Convert a distance in coordinate space to a distance in the\nline segment's parametric space. The sign of the input is\nkept.",5,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"f32"}}],[11,"distance_from_parametric_delta","","Convert a parametric delta to coordinate space. The sign of\nthe input is kept.",5,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"f32"}}],[11,"point_from_parametric","","Linearly interpolate between the segment's endpoints by the\nfactor `t`. When `t` is zero the result is `self.start`, and\nwhen `t` is one the result is `self.end`. The range of `t` is\nnot clamped.",5,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"segment_from_parametric_range","","Treat the range's start and end as parametric coords. Use\n`point_from_parametric` to interpolate the range into a new\nsegment. The output segment is not clamped.",5,{"inputs":[{"name":"segment3f"},{"name":"rangef"}],"output":{"name":"segment3f"}}],[11,"project_segment_as_range","","Project another segment onto `self`. The result is a\nparametric range of `self` clamped to [0, 1].",5,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"rangef"}}],[11,"point_distance_squared","","Return the squared distance from this segment to the input\n`point`.",5,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"point_distance","","Return the distance from this segment to the input `point`.",5,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"closest_point_to_point","","Find the point on the segment closest to the input point. The\nreturn value contains both the parametric and actual location\nof the closest point.",5,null],[6,"Output","","",null,null],[11,"add","","",5,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"output"}}],[0,"sphere","dumbmath","",null,null],[3,"Sphere3f","dumbmath::sphere","Sphere represented by a center and radius",null,null],[12,"center","","",6,null],[12,"radius","","",6,null],[17,"UNIT","","Sphere centered at (0.0, 0.0, 0.0) with a radius of 1.0",null,null],[11,"eq","","",6,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"clone","","",6,{"inputs":[{"name":"sphere3f"}],"output":{"name":"sphere3f"}}],[11,"new","","Create a Sphere3f from center point and radius",6,{"inputs":[{"name":"sphere3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"from_radius","","Create a Sphere3f centered at zero with given radius",6,{"inputs":[{"name":"sphere3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"radius_squared","","Squared radius of the sphere",6,{"inputs":[{"name":"sphere3f"}],"output":{"name":"f32"}}],[0,"vector","dumbmath","",null,null],[3,"Vec3f","dumbmath::vector","Vector with three f32 components",null,null],[12,"x","","",7,null],[12,"y","","",7,null],[12,"z","","",7,null],[5,"vec3f","","Create a Vec3f from x, y, and z inputs",null,{"inputs":[{"name":"x"},{"name":"y"},{"name":"z"}],"output":{"name":"vec3f"}}],[5,"dot3","","Inner product of two Vec3f inputs",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[5,"cross","","Cross product of two Vec3f inputs",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[5,"distance3","","Distance between two points",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[5,"lerp3","","Linearly interpolate between two points by the factor `t`. When\n`t` is zero the result is `p0`, and when `t` is one the result is\n`p1`. The range of `t` is not clamped.",null,{"inputs":[{"name":"vec3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[17,"ZERO_3F","","Vec3f(0.0, 0.0, 0.0)",null,null],[8,"CastF32","","Convert a numeric type to an f32",null,null],[10,"as_f32","","",8,{"inputs":[{"name":"castf32"}],"output":{"name":"f32"}}],[11,"eq","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"ne","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"fmt","","",7,{"inputs":[{"name":"vec3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"vec3f"}],"output":{"name":"vec3f"}}],[11,"new","","Create a Vec3f from three components",7,{"inputs":[{"name":"vec3f"},{"name":"f32"},{"name":"f32"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"from_scalar","","Create a Vec3f with all components set to the same scalar",7,{"inputs":[{"name":"vec3f"},{"name":"t"}],"output":{"name":"vec3f"}}],[11,"magnitude_squared","","Squared length of the vector",7,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"magnitude","","Length of the vector",7,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"normalized","","Create a normalized copy, or None if the magnitude is zero",7,{"inputs":[{"name":"vec3f"}],"output":{"name":"option"}}],[11,"project_onto","","Projection of `self` into `v`, or None if the magnitude of\n`b` is zero.",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"option"}}],[6,"Output","","",null,null],[11,"add","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"add","","",7,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"div","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",7,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"sub","","",7,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"sub","","",7,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}]],"paths":[[3,"Aabb3f"],[3,"Degrees"],[3,"Radians"],[3,"Line3f"],[3,"InclusiveRange"],[3,"Segment3f"],[3,"Sphere3f"],[3,"Vec3f"],[8,"CastF32"]]};
initSearch(searchIndex);
