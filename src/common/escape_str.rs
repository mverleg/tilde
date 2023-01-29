
/// Escape double quotes and backslashes.
pub fn escape_for_string(input: impl Into<String>) -> String {
    let input = input.into();
    if ! is_safe_for_string(&input) {
        todo!("json escaping ({})", &input);  //TODO @mverleg:
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let text = "";
        assert!(is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("", escaped);
    }

    #[test]
    fn already_safe() {
        let text = "hello 'world'! spam@local";
        assert!(is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("hello 'world'! spam@local", escaped);
    }

    #[test]
    fn single_quote() {
        let text = "'";
        assert!(is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("'", escaped);
    }

    #[test]
    fn just_double_quote() {
        let text = "\"";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\"", escaped);
    }

    #[test]
    fn quoted_string() {
        let text = "\"hello\"";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\"hello\\\"", escaped);
    }

    #[test]
    fn escaped_quote() {
        let text = "hello\\\"there";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("hello\\\\\"there", escaped);
    }

    #[test]
    fn trailing_backslash() {
        let text = "hi\\";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("hi\\\\", escaped);
    }

    #[test]
    fn mixed_backslashes() {
        let text = "\\\\hi\\";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\\\\\\hi\\\\", escaped);
    }

    //TODO @mverleg: more fancy escape combinations
}
