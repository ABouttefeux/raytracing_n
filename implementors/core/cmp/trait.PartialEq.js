(function() {var implementors = {
"raytracing_n":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/utils/struct.Normed.html\" title=\"struct raytracing_n::utils::Normed\">Normed</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/utils/struct.Normed.html\" title=\"struct raytracing_n::utils::Normed\">Normed</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"raytracing_n/utils/trait.Norm.html\" title=\"trait raytracing_n::utils::Norm\">Norm</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Div.html\" title=\"trait core::ops::arith::Div\">Div</a>&lt;&lt;T as <a class=\"trait\" href=\"raytracing_n/utils/trait.Norm.html\" title=\"trait raytracing_n::utils::Norm\">Norm</a>&gt;::<a class=\"associatedtype\" href=\"raytracing_n/utils/trait.Norm.html#associatedtype.Output\" title=\"type raytracing_n::utils::Norm::Output\">Output</a>, Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,</span>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.Plane.html\" title=\"struct raytracing_n::object::Plane\">Plane</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.Plane.html\" title=\"struct raytracing_n::object::Plane\">Plane</a>&lt;F, N&gt;"],["impl&lt;const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/direction/struct.Direction.html\" title=\"struct raytracing_n::direction::Direction\">Direction</a>&lt;N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/direction/struct.Direction.html\" title=\"struct raytracing_n::direction::Direction\">Direction</a>&lt;N&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/utils/struct.NonNegative.html\" title=\"struct raytracing_n::utils::NonNegative\">NonNegative</a>&lt;F&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/utils/struct.NonNegative.html\" title=\"struct raytracing_n::utils::NonNegative\">NonNegative</a>&lt;F&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.FloatConst.html\" title=\"trait num_traits::float::FloatConst\">FloatConst</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/angle/struct.Angle.html\" title=\"struct raytracing_n::angle::Angle\">Angle</a>&lt;F&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/angle/struct.Angle.html\" title=\"struct raytracing_n::angle::Angle\">Angle</a>&lt;F&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.OriginPlanePlaneWithOffset.html\" title=\"struct raytracing_n::object::OriginPlanePlaneWithOffset\">OriginPlanePlaneWithOffset</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.OriginPlanePlaneWithOffset.html\" title=\"struct raytracing_n::object::OriginPlanePlaneWithOffset\">OriginPlanePlaneWithOffset</a>&lt;F, N&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialOrd.html\" title=\"trait core::cmp::PartialOrd\">PartialOrd</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/utils/struct.Positive.html\" title=\"struct raytracing_n::utils::Positive\">Positive</a>&lt;F&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/utils/struct.Positive.html\" title=\"struct raytracing_n::utils::Positive\">Positive</a>&lt;F&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/transformation/struct.Translation.html\" title=\"struct raytracing_n::transformation::Translation\">Translation</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/transformation/struct.Translation.html\" title=\"struct raytracing_n::transformation::Translation\">Translation</a>&lt;F, N&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/vector/struct.Vector.html\" title=\"struct raytracing_n::vector::Vector\">Vector</a>&lt;T, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/vector/struct.Vector.html\" title=\"struct raytracing_n::vector::Vector\">Vector</a>&lt;T, N&gt;"],["impl&lt;F, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/vector/ray/struct.Ray.html\" title=\"struct raytracing_n::vector::ray::Ray\">Ray</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/vector/ray/struct.Ray.html\" title=\"struct raytracing_n::vector::ray::Ray\">Ray</a>&lt;F, N&gt;<span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,\n    <a class=\"struct\" href=\"raytracing_n/vector/struct.Vector.html\" title=\"struct raytracing_n::vector::Vector\">Vector</a>&lt;F, N&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/ops/arith/trait.Div.html\" title=\"trait core::ops::arith::Div\">Div</a>&lt;&lt;<a class=\"struct\" href=\"raytracing_n/vector/struct.Vector.html\" title=\"struct raytracing_n::vector::Vector\">Vector</a>&lt;F, N&gt; as <a class=\"trait\" href=\"raytracing_n/utils/trait.Norm.html\" title=\"trait raytracing_n::utils::Norm\">Norm</a>&gt;::<a class=\"associatedtype\" href=\"raytracing_n/utils/trait.Norm.html#associatedtype.Output\" title=\"type raytracing_n::utils::Norm::Output\">Output</a>, Output = <a class=\"struct\" href=\"raytracing_n/vector/struct.Vector.html\" title=\"struct raytracing_n::vector::Vector\">Vector</a>&lt;F, N&gt;&gt;,</span>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.FloatConst.html\" title=\"trait num_traits::float::FloatConst\">FloatConst</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/transformation/struct.Rotation.html\" title=\"struct raytracing_n::transformation::Rotation\">Rotation</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/transformation/struct.Rotation.html\" title=\"struct raytracing_n::transformation::Rotation\">Rotation</a>&lt;F, N&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"enum\" href=\"raytracing_n/utils/enum.ErrorSet.html\" title=\"enum raytracing_n::utils::ErrorSet\">ErrorSet</a>&gt; for <a class=\"enum\" href=\"raytracing_n/utils/enum.ErrorSet.html\" title=\"enum raytracing_n::utils::ErrorSet\">ErrorSet</a>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.OriginPlane.html\" title=\"struct raytracing_n::object::OriginPlane\">OriginPlane</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.OriginPlane.html\" title=\"struct raytracing_n::object::OriginPlane\">OriginPlane</a>&lt;F, N&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.FloatConst.html\" title=\"trait num_traits::float::FloatConst\">FloatConst</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.Camera.html\" title=\"struct raytracing_n::object::Camera\">Camera</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.Camera.html\" title=\"struct raytracing_n::object::Camera\">Camera</a>&lt;F, N&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"enum\" href=\"raytracing_n/object/enum.PlaneBuildingError.html\" title=\"enum raytracing_n::object::PlaneBuildingError\">PlaneBuildingError</a>&gt; for <a class=\"enum\" href=\"raytracing_n/object/enum.PlaneBuildingError.html\" title=\"enum raytracing_n::object::PlaneBuildingError\">PlaneBuildingError</a>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.FloatConst.html\" title=\"trait num_traits::float::FloatConst\">FloatConst</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.OrientedOriginPlane.html\" title=\"struct raytracing_n::object::OrientedOriginPlane\">OrientedOriginPlane</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.OrientedOriginPlane.html\" title=\"struct raytracing_n::object::OrientedOriginPlane\">OrientedOriginPlane</a>&lt;F, N&gt;"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.Triangle.html\" title=\"struct raytracing_n::object::Triangle\">Triangle</a>&lt;F, N&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.Triangle.html\" title=\"struct raytracing_n::object::Triangle\">Triangle</a>&lt;F, N&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"enum\" href=\"raytracing_n/utils/enum.Never.html\" title=\"enum raytracing_n::utils::Never\">Never</a>&gt; for <a class=\"enum\" href=\"raytracing_n/utils/enum.Never.html\" title=\"enum raytracing_n::utils::Never\">Never</a>"],["impl&lt;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/float/trait.Float.html\" title=\"trait num_traits::float::Float\">Float</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>, const H: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.70.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.70.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>&lt;<a class=\"struct\" href=\"raytracing_n/object/struct.HyperPlane.html\" title=\"struct raytracing_n::object::HyperPlane\">HyperPlane</a>&lt;F, N, H&gt;&gt; for <a class=\"struct\" href=\"raytracing_n/object/struct.HyperPlane.html\" title=\"struct raytracing_n::object::HyperPlane\">HyperPlane</a>&lt;F, N, H&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()