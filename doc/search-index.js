var searchIndex = {};
searchIndex['dumbmath'] = {"items":[[0,"","dumbmath","",null,null],[0,"aabb","","",null,null],[3,"Aabb3f","dumbmath::aabb","",null,null],[12,"min","","",0,null],[12,"max","","",0,null],[11,"new","","Create an empty Aabb3f with min initialized to +infinity and\nmax is initialized to -infinity.",0,{"inputs":[{"name":"aabb3f"}],"output":{"name":"aabb3f"}}],[11,"from_point","","",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"aabb3f"}}],[11,"contains_point","","True if the point intersects the box",0,{"inputs":[{"name":"aabb3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[0,"angle","dumbmath","",null,null],[3,"Degrees","dumbmath::angle","",null,null],[3,"Radians","","",null,null],[11,"partial_cmp","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"option"}}],[11,"lt","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"le","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"gt","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"ge","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"eq","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"degrees"},{"name":"degrees"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"degrees"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"degrees"}}],[11,"partial_cmp","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"option"}}],[11,"lt","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"le","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"gt","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"ge","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"eq","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"radians"},{"name":"radians"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"radians"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"radians"}}],[11,"value","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"f32"}}],[11,"to_radians","","",1,{"inputs":[{"name":"degrees"}],"output":{"name":"radians"}}],[11,"value","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"f32"}}],[11,"to_degrees","","",2,{"inputs":[{"name":"radians"}],"output":{"name":"degrees"}}],[0,"intersect","dumbmath","",null,null],[5,"intersect_sphere_triangle","dumbmath::intersect","",null,null],[0,"line","dumbmath","",null,null],[3,"Line3f","dumbmath::line","Line of infinite length represented by two distinct points it\npasses through.",null,null],[12,"points","","",3,null],[11,"eq","","",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"line3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"line3f"}],"output":{"name":"line3f"}}],[11,"new","","Create a new line. Return None if the input points are\nidentical.",3,{"inputs":[{"name":"line3f"},{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"option"}}],[11,"closest_points_between_lines","","Find the closest points between two lines. The result is a\npair of parametric points, the first for `self` and the second\nfor `line`. If the lines are parallel then None is\nreturned.",3,{"inputs":[{"name":"line3f"},{"name":"line3f"}],"output":{"name":"option"}}],[0,"quad","dumbmath","",null,null],[3,"Quad2f","dumbmath::quad","2D Quadrilateral",null,null],[12,"points","","",4,null],[4,"IBLerpResult","","Return type for Quad2f::iblerp",null,null],[13,"NoSolution","","",5,null],[13,"OneSolution","","One solutions (as parametric coordinate)",5,null],[13,"TwoSolutions","","Two solutions (as parametric coordinates)",5,null],[13,"ManySolutions","","",5,null],[4,"InvBilerpResult","","Return type for Quad2f::inv_bilerp_u",null,null],[13,"NoSolution","","",6,null],[13,"OneSolution","","One solutions (as parametric coordinate)",6,null],[13,"TwoSolutions","","Two solutions (as parametric coordinates)",6,null],[13,"ManySolutions","","",6,null],[11,"eq","","",4,{"inputs":[{"name":"quad2f"},{"name":"quad2f"}],"output":{"name":"bool"}}],[11,"ne","","",4,{"inputs":[{"name":"quad2f"},{"name":"quad2f"}],"output":{"name":"bool"}}],[11,"fmt","","",4,{"inputs":[{"name":"quad2f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"quad2f"}],"output":{"name":"quad2f"}}],[11,"eq","","",5,{"inputs":[{"name":"iblerpresult"},{"name":"iblerpresult"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"iblerpresult"},{"name":"iblerpresult"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"iblerpresult"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",5,{"inputs":[{"name":"iblerpresult"}],"output":{"name":"iblerpresult"}}],[11,"eq","","",6,{"inputs":[{"name":"invbilerpresult"},{"name":"invbilerpresult"}],"output":{"name":"bool"}}],[11,"ne","","",6,{"inputs":[{"name":"invbilerpresult"},{"name":"invbilerpresult"}],"output":{"name":"bool"}}],[11,"fmt","","",6,{"inputs":[{"name":"invbilerpresult"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",6,{"inputs":[{"name":"invbilerpresult"}],"output":{"name":"invbilerpresult"}}],[11,"new","","",4,{"inputs":[{"name":"quad2f"},{"name":"vec2f"},{"name":"vec2f"},{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"quad2f"}}],[11,"iblerp","","Inverse bilinear interpolation",4,{"inputs":[{"name":"quad2f"},{"name":"vec2f"}],"output":{"name":"iblerpresult"}}],[11,"inv_bilerp_u","","Calculate horizontal parametric coordinate from cartesian point.",4,{"inputs":[{"name":"quad2f"},{"name":"vec2f"}],"output":{"name":"invbilerpresult"}}],[11,"lerp_bottom","","",4,{"inputs":[{"name":"quad2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"lerp_top","","",4,{"inputs":[{"name":"quad2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"lerp_left","","",4,{"inputs":[{"name":"quad2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"lerp_right","","",4,{"inputs":[{"name":"quad2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"blerp","","",4,{"inputs":[{"name":"quad2f"},{"name":"vec2f"}],"output":{"name":"vec2f"}}],[0,"prelude","dumbmath","",null,null],[0,"range","","",null,null],[3,"InclusiveRange","dumbmath::range","Inclusive range from min to max",null,null],[12,"min","","",7,null],[12,"max","","",7,null],[5,"range_clamp","","Create range covering the overlap between two ranges, or None if\nthere is no overlap.",null,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"option"}}],[5,"range_combine","","Create range covering both ranges (and any gap between them).",null,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"inclusiverange"}}],[6,"Rangef","","Convenience type: InclusiveRange<f32>",null,null],[17,"RANGE_0_1_F32","","",null,null],[11,"eq","","",7,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"ne","","",7,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"fmt","","",7,{"inputs":[{"name":"inclusiverange"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",7,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"inclusiverange"}}],[11,"new","","Create an InclusiveRange from min to max. Asserts that min is\nless than max.",7,{"inputs":[{"name":"inclusiverange"},{"name":"t"},{"name":"t"}],"output":{"name":"inclusiverange"}}],[11,"from_sorting","","Create an InclusiveRange by sorting the inputs.",7,{"inputs":[{"name":"inclusiverange"},{"name":"t"},{"name":"t"}],"output":{"name":"inclusiverange"}}],[11,"empty","","True if `min == max`, false otherwise.",7,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"bool"}}],[11,"length","","Distance between self.min and self.max",7,{"inputs":[{"name":"inclusiverange"}],"output":{"name":"t"}}],[11,"expand","","Expand `self` as needed to include another range.",7,{"inputs":[{"name":"inclusiverange"},{"name":"inclusiverange"}],"output":null}],[0,"segment","dumbmath","",null,null],[3,"Segment3f","dumbmath::segment","Line segment between two points",null,null],[12,"start","","",8,null],[12,"end","","",8,null],[11,"eq","","",8,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"ne","","",8,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"bool"}}],[11,"fmt","","",8,{"inputs":[{"name":"segment3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",8,{"inputs":[{"name":"segment3f"}],"output":{"name":"segment3f"}}],[11,"new","","Create a segment between two points",8,{"inputs":[{"name":"segment3f"},{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"segment3f"}}],[11,"to_vec3f","","Get vector from start to end (not normalized)",8,{"inputs":[{"name":"segment3f"}],"output":{"name":"vec3f"}}],[11,"length","","Length of the line segment",8,{"inputs":[{"name":"segment3f"}],"output":{"name":"f32"}}],[11,"reversed","","Create a copy of `self` with endpoints reversed.",8,{"inputs":[{"name":"segment3f"}],"output":{"name":"segment3f"}}],[11,"distance_to_parametric_delta","","Convert a distance in coordinate space to a distance in the\nline segment's parametric space. The sign of the input is\nkept.",8,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"f32"}}],[11,"distance_from_parametric_delta","","Convert a parametric delta to coordinate space. The sign of\nthe input is kept.",8,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"f32"}}],[11,"point_from_parametric","","Linearly interpolate between the segment's endpoints by the\nfactor `t`. When `t` is zero the result is `self.start`, and\nwhen `t` is one the result is `self.end`. The range of `t` is\nnot clamped.",8,{"inputs":[{"name":"segment3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"segment_from_parametric_range","","Treat the range's start and end as parametric coords. Use\n`point_from_parametric` to interpolate the range into a new\nsegment. The output segment is not clamped.",8,{"inputs":[{"name":"segment3f"},{"name":"rangef"}],"output":{"name":"segment3f"}}],[11,"project_segment_as_range","","Project another segment onto `self`. The result is a\nparametric range of `self` clamped to [0, 1].",8,{"inputs":[{"name":"segment3f"},{"name":"segment3f"}],"output":{"name":"rangef"}}],[11,"point_distance_squared","","Return the squared distance from this segment to the input\n`point`.",8,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"point_distance","","Return the distance from this segment to the input `point`.",8,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"closest_point_to_point","","Find the point on the segment closest to the input point. The\nreturn value contains both the parametric and actual location\nof the closest point.",8,null],[6,"Output","","",null,null],[11,"add","","",8,{"inputs":[{"name":"segment3f"},{"name":"vec3f"}],"output":{"name":"output"}}],[0,"sphere","dumbmath","",null,null],[3,"Sphere3f","dumbmath::sphere","Sphere represented by a center and radius",null,null],[12,"center","","",9,null],[12,"radius","","",9,null],[17,"UNIT","","Sphere centered at (0.0, 0.0, 0.0) with a radius of 1.0",null,null],[11,"eq","","",9,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"ne","","",9,{"inputs":[{"name":"sphere3f"},{"name":"sphere3f"}],"output":{"name":"bool"}}],[11,"clone","","",9,{"inputs":[{"name":"sphere3f"}],"output":{"name":"sphere3f"}}],[11,"new","","Create a Sphere3f from center point and radius",9,{"inputs":[{"name":"sphere3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"from_radius","","Create a Sphere3f centered at zero with given radius",9,{"inputs":[{"name":"sphere3f"},{"name":"f32"}],"output":{"name":"sphere3f"}}],[11,"radius_squared","","Squared radius of the sphere",9,{"inputs":[{"name":"sphere3f"}],"output":{"name":"f32"}}],[0,"util","dumbmath","",null,null],[8,"CastF32","dumbmath::util","Convert a numeric type to an f32",null,null],[10,"as_f32","","",10,{"inputs":[{"name":"castf32"}],"output":{"name":"f32"}}],[0,"vec2f","dumbmath","",null,null],[3,"Vec2f","dumbmath::vec2f","Vector with two f32 components",null,null],[12,"x","","",11,null],[12,"y","","",11,null],[3,"Line2f","","",null,null],[12,"points","","",12,null],[5,"detf_2x2","","",null,{"inputs":[{"name":"f32"},{"name":"f32"},{"name":"f32"},{"name":"f32"}],"output":{"name":"f32"}}],[5,"vec2f","","Create a Vec2f from x and y inputs",null,{"inputs":[{"name":"x"},{"name":"y"}],"output":{"name":"vec2f"}}],[11,"eq","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"bool"}}],[11,"ne","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"bool"}}],[11,"fmt","","",11,{"inputs":[{"name":"vec2f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",11,{"inputs":[{"name":"vec2f"}],"output":{"name":"vec2f"}}],[11,"new","","Create a Vec2f from two components",11,{"inputs":[{"name":"vec2f"},{"name":"f32"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"cross","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"lerp","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"dot","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"vec3f","","",11,{"inputs":[{"name":"vec2f"}],"output":{"name":"vec3f"}}],[11,"distance_squared","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"distance","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"magnitude_squared","","",11,{"inputs":[{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"magnitude","","",11,{"inputs":[{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"normalized","","",11,{"inputs":[{"name":"vec2f"}],"output":{"name":"option"}}],[6,"Output","","",null,null],[11,"add","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"vec2f"}}],[6,"Output","","",null,null],[11,"sub","","",11,{"inputs":[{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"vec2f"}}],[6,"Output","","",null,null],[11,"mul","","",11,{"inputs":[{"name":"vec2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[6,"Output","","",null,null],[11,"div","","",11,{"inputs":[{"name":"vec2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"eq","","",12,{"inputs":[{"name":"line2f"},{"name":"line2f"}],"output":{"name":"bool"}}],[11,"ne","","",12,{"inputs":[{"name":"line2f"},{"name":"line2f"}],"output":{"name":"bool"}}],[11,"fmt","","",12,{"inputs":[{"name":"line2f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",12,{"inputs":[{"name":"line2f"}],"output":{"name":"line2f"}}],[11,"new","","",12,{"inputs":[{"name":"line2f"},{"name":"vec2f"},{"name":"vec2f"}],"output":{"name":"line2f"}}],[11,"closest_parametric_point","","",12,{"inputs":[{"name":"line2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[11,"cart_from_para","","",12,{"inputs":[{"name":"line2f"},{"name":"f32"}],"output":{"name":"vec2f"}}],[11,"orient","","If result is greater than zero `point` lies to the left of the\nline. If result is less than zero `point` lies to the\nright. If result is zero, the point is on the line.",12,{"inputs":[{"name":"line2f"},{"name":"vec2f"}],"output":{"name":"f32"}}],[0,"vec3f","dumbmath","",null,null],[3,"Vec3f","dumbmath::vec3f","Vector with three f32 components",null,null],[12,"x","","",13,null],[12,"y","","",13,null],[12,"z","","",13,null],[5,"vec3f","","Create a Vec3f from x, y, and z inputs",null,{"inputs":[{"name":"x"},{"name":"y"},{"name":"z"}],"output":{"name":"vec3f"}}],[17,"ZERO_3F","","Vec3f(0.0, 0.0, 0.0)",null,null],[11,"eq","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"ne","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"bool"}}],[11,"fmt","","",13,{"inputs":[{"name":"vec3f"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",13,{"inputs":[{"name":"vec3f"}],"output":{"name":"vec3f"}}],[11,"new","","Create a Vec3f from three components",13,{"inputs":[{"name":"vec3f"},{"name":"f32"},{"name":"f32"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"from_scalar","","Create a Vec3f with all components set to the same scalar",13,{"inputs":[{"name":"vec3f"},{"name":"t"}],"output":{"name":"vec3f"}}],[11,"dot","","Inner product",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"cross","","Cross product",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[11,"distance","","Distance to another point",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"lerp","","Linearly interpolate between two points by the factor\n`t`. When `t` is zero the result is `self`, and when `t` is\none the result is `p`. The range of `t` is not clamped.",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[11,"magnitude_squared","","Squared length of the vector",13,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"magnitude","","Length of the vector",13,{"inputs":[{"name":"vec3f"}],"output":{"name":"f32"}}],[11,"normalized","","Create a normalized copy, or None if the magnitude is zero",13,{"inputs":[{"name":"vec3f"}],"output":{"name":"option"}}],[11,"project_onto","","Projection of `self` into `v`, or None if the magnitude of\n`b` is zero.",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"option"}}],[6,"Output","","",null,null],[11,"add","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"add","","",13,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"div","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"mul","","",13,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"sub","","",13,{"inputs":[{"name":"vec3f"},{"name":"vec3f"}],"output":{"name":"vec3f"}}],[6,"Output","","",null,null],[11,"sub","","",13,{"inputs":[{"name":"vec3f"},{"name":"f32"}],"output":{"name":"vec3f"}}]],"paths":[[3,"Aabb3f"],[3,"Degrees"],[3,"Radians"],[3,"Line3f"],[3,"Quad2f"],[4,"IBLerpResult"],[4,"InvBilerpResult"],[3,"InclusiveRange"],[3,"Segment3f"],[3,"Sphere3f"],[8,"CastF32"],[3,"Vec2f"],[3,"Line2f"],[3,"Vec3f"]]};
initSearch(searchIndex);
