use ::tilde::tilde_strs;

fn main() {
    // pythagoras, implicit apply
    let prog = "arg dup mul arg dup mul sqrt";
    let res = tilde_strs(prog, "4 3").unwrap();
    assert_eq!(res, "5");

    // capture order
    let prog = "arg arg sub 3 1 apply apply";
    let res = tilde_strs(prog, "").unwrap();
    assert_eq!(res, "2");
}
