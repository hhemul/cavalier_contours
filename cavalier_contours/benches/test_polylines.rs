use cavalier_contours::{
    core::{
        math::{point_on_circle, Vector2},
        traits::Real,
    },
    pline_closed,
    polyline::Polyline,
};

pub fn pathological1<T>(vertex_count: usize) -> Polyline<T>
where
    T: Real,
{
    let radius = T::from(40.0).unwrap();
    let center = Vector2::zero();

    let mut result = Polyline::new_closed();

    for i in 0..vertex_count {
        let angle = T::from(i).unwrap() * T::tau() / T::from(vertex_count).unwrap();
        let point = point_on_circle(radius, center, angle);
        let bulge = if i % 2 == 0 { T::one() } else { -T::one() };
        result.add(point.x, point.y, bulge);
    }

    result
}

pub fn pathological1_no_arcs<T>(vertex_count: usize) -> Polyline<T>
where
    T: Real,
{
    pathological1::<T>(vertex_count)
        .arcs_to_approx_lines(T::from(0.01).unwrap())
        .unwrap()
}

pub fn profile1() -> Polyline<f64> {
    pline_closed![
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 1.0),
        (10.0, 0.0, -0.5),
        (10.0, 10.0, 0.5),
        (14.0, 20.0, -0.5),
        (0.0, 20.0, 0.0)
    ]
}

pub fn profile1_no_arcs() -> Polyline<f64> {
    profile1().arcs_to_approx_lines(0.01).unwrap()
}

pub fn profile2() -> Polyline<f64> {
    pline_closed![
        (0.0, 25.0, 1.0),
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 1.0),
        (10.0, 0.0, -0.5),
        (8.0, 9.0, 0.374794619217547),
        (21.0, 0.0, 0.0),
        (23.0, 0.0, 1.0),
        (32.0, 0.0, -0.5),
        (28.0, 0.0, 0.5),
        (39.0, 21.0, 0.0),
        (28.0, 12.0, 0.0)
    ]
}

pub fn profile2_no_arcs() -> Polyline<f64> {
    profile2().arcs_to_approx_lines(0.01).unwrap()
}
