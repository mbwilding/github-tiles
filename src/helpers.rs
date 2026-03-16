/// Format a number with a suffix for large values.
pub fn format_number(n: u32) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f32 / 1_000_000.0)
    } else if n >= 1000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        n.to_string()
    }
}

/// Splits a comma-separated string slice into a Vec of string slices, trimming whitespace from each element.
pub fn comma_str_to_vec_slice<'a>(s: &'a str) -> Vec<&'a str> {
    s.split(',').map(|s| s.trim()).collect::<Vec<&'a str>>()
}
