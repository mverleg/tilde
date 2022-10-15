fn main() {
    let res = run_tilde(",hello world");
    assert_eq!(res.unwrap(), vec!["hello world"])
}
