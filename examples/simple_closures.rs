use ::tilde::tilde_strs;

fn main() {
    // map and reduce array
    let prog = "arg 1 plus plus apply";
    let res = tilde_strs(prog, "1\n2\n3\n4\n5").unwrap();
    assert_eq!(res, "20");

    // compare x-3 and 3-x (scalar apply)
    assert_eq!(tilde_strs("arg 3 sub 5 apply", "").unwrap(), "2");
    assert_eq!(tilde_strs("3 arg sub 5 apply", "").unwrap(), "-2");
}
