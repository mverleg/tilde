use ::tilde::tilde_strs;

fn main() {
    let res = tilde_strs("6 2div", "");
    assert_eq!(res.unwrap(), "3")
}
