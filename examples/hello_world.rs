use tilde::tilde_strs;

fn main() {
    let res = tilde_strs("", ",hello world");
    assert_eq!(res.unwrap(), "hello world")
}
