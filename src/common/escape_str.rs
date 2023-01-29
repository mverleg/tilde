
/// Escape double quotes and trailing backslashes.
pub fn escape_for_string(input: impl Into<String>) -> String {
    let input = input.into();
    if ! is_safe_for_string(&input) {
        todo!("json escaping");  //TODO @mverleg:
    }
    input
}

pub fn is_safe_for_string(input: &str) -> bool {
    if input.contains('"') {
        return false
    }
    if input.ends_with('\\') {
        //TODO @mverleg: should maybe allow even numbers of backslashes
        return false
    }
    true
}
