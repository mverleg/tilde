use crate::op::Op;

/// Encode a positive integer, using static width of 1 byte each, and
/// do not allow modifiers in the first byte.
pub fn encode_positive_int_static_width_avoid_modifiers(nr: u64) -> Vec<Op> {
    todo!();
}

/// Inverse of [encode_pos_int_static_width_avoid_modifiers].
pub fn decode_positive_int_static_width_avoid_modifiers(ops: &[Op]) -> Option<u64> {
    if ops.is_empty() {
        return None;
    }
    todo!();
}

// //TODO @mark: variable length ints
// /// Read a variable length integer for the first string lookup.
// /// * First digit cannot be string delimiter, and CANNOT be a variable token, for 10 options.
// /// * Subsequent digits cannot be string delimiter, but CAN be a variable token, for 15 options.
// pub fn read_first_str_number() {
//     todo!(); //TODO @mark: TEMPORARY! REMOVE THIS!
// }
//TODO @mark: ^

#[cfg(test)]
mod static_width {
    use super::*;

    #[test]
    fn positive_int_without_avoided_modifiers() {
        todo!();
    }

    #[test]
    fn positive_int_avoid_modifiers_empty_input() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert!(decode.is_none());
    }

    #[test]
    fn positive_int_with_avoided_modifiers() {
        let decode = decode_positive_int_static_width_avoid_modifiers(&[]);
        assert!(decode.is_none());
        todo!();
    }
}
