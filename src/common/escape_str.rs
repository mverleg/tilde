
/// Escape double quotes, newlines and backslashes.
pub fn escape_for_string(input: impl Into<String>) -> String {
    let input = input.into();
    let mut result = String::with_capacity(input.len() + 4);
    for ch in input.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\n' => result.push_str("\\n"),
            '\\' => result.push_str("\\\\"),
            safe => result.push(safe),
        }
    }
    result
    //TODO @mverleg: prevent reallocating if nothing to escape?
}

pub fn is_safe_for_string(input: &str) -> bool {
    if input.contains('"') {
        return false
    }
    if input.contains('\\') {
        return false
    }
    if input.contains('\n') {
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
        assert_eq!("hello\\\\\\\"there", escaped);
    }

    #[test]
    fn leading_backslash() {
        let text = "\\hi";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\\hi", escaped);
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

    #[test]
    fn just_newline() {
        let text = "\n";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\n", escaped);
    }

    #[test]
    fn quoted_newline() {
        let text = "\"\n\"";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\"\\n\\\"", escaped);
    }

    #[test]
    fn nightmare() {
        // \\"hi\\"+\"\"
        let text = "\\\\\"hi\\\\\"\n+\n\\\"\\\"";
        assert!(!is_safe_for_string(text));
        let escaped = escape_for_string(text);
        assert_eq!("\\\\\\\\\\\"hi\\\\\\\\\\\"\\n+\\n\\\\\\\"\\\\\\\"", escaped);
    }
}
