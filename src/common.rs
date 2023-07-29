use std::f64::consts::PI;

pub fn Clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

pub fn Saturate(x:f64) -> f64 {
    Clamp(x, 0.0, 1.0)
}

pub fn DegreesToRadians(degrees:f64) -> f64 {
    degrees * PI / 180.0
}