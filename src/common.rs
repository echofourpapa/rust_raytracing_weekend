use std::f64::consts::PI;

pub fn clamp(x:f64, min:f64, max:f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

pub fn saturate(x:f64) -> f64 {
    clamp(x, 0.0, 1.0)
}

pub fn degrees_to_radians(degrees:f64) -> f64 {
    degrees * PI / 180.0
}

pub fn seconds_to_hhmmss(s:f64) -> String {
    let seconds = s % 60.0;
    let minutes = (s as i32 / 60) % 60;
    let hours = (s as i32 / 60) / 60;

    let s_string = format!("{:.2}s", seconds);    
    let m_string = format!("{}m", minutes);
    let h_string = format!("{}h", hours);

    if hours > 0 {
        return format!("{} {} {}", h_string, m_string, s_string);
    }

    if minutes > 0 {        
        return format!("{} {}", m_string, s_string);
    }

    return s_string;
}