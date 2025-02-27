pub fn concat(a: &str, b: &str) -> String {
    return format!("{}{}", a, b);
}

pub fn length(a: &str) -> usize {
    return a.len();
}

pub fn reverse(a: &str) -> String {
    return a.chars().rev().collect();
}

pub fn replace(a: &str, b: &str, c: &str) -> String {
    return a.replace(b, c);
}

pub fn split(a: &str, b: &str) -> Vec<String> {
    return a.split(b).map(|s| s.to_string()).collect();
}
