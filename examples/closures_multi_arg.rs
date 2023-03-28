use ::tilde::tilde_strs;

fn main() {
    // pythagoras, implicit apply
    //TODO @mark: there is a bug here, which may be hard to solve: dup has two return values, but
    //TODO @mark: Tilde does not know that while building the closure, so `arg dup` is one value,
    //TODO @mark: making the next binary op (`mul`) take the closure and the prev value
    let prog = "flatten arg dup mul arg dup mul plus sqrt";
    let res = tilde_strs(prog, "4\n3").unwrap();
    assert_eq!(res, "5");

    // capture order
    let prog = "3 1 arg arg sub apply apply";
    let res = tilde_strs(prog, "").unwrap();
    assert_eq!(res, "2");
    //TODO @mark: is this indeed the desirable order? or -2?
}
