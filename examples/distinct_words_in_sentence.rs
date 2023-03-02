use ::tilde::tilde_strs;

fn main() {
    let res = tilde_strs(
        "lookup base-words unique count",
        "To be, or not to be, that is the question");
    assert_eq!(res.unwrap(), "8")
}
