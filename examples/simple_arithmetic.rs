use ::tilde::tilde_strs;

fn main() {
    let res = tilde_strs("8 2div 1 2 plus minus", "");
    assert_eq!(res.unwrap(), "1")
}
