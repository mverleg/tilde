use ::tilde::tilde_strs;

fn main() {
    let prog = "arg 1 plus map sum";
    let res = tilde_strs(prog, "1\n2\n3\n4\n5").unwrap();
    assert_eq!(res, "20");
}
