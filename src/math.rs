const EPSILON: f64 = 1e-10; // highest value that will be considered zero

pub fn is_approx_zero(val: f64) -> bool
{
    val.abs() < EPSILON
}

// fn is_approx_integer(val: f64) -> bool
// {
//    val.fract().abs() < EPSILON
// }
