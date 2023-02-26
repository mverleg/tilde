use ::tilde::tilde_strs;

fn main() {
    let res = tilde_strs("1 2 3drop+/", "");
    assert_eq!(res.unwrap(), "2")
}
