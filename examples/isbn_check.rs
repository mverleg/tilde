
// https://code.golf/isbn

use ::tilde::tilde_strs;

fn main() {
    let prog = "arg print arg \"-\" neq filter 11 range rev zip arg mul map sum 11 mod 10 eq return";
    assert_eq!(tilde_strs(prog, "5-3614111-3-"), Ok("5-3614111-3-2".to_owned()));
    assert_eq!(tilde_strs(prog, "5-80635-550-"), Ok("5-80635-550-0".to_owned()));
    assert_eq!(tilde_strs(prog, "4-57803-516-"), Ok("4-57803-516-6".to_owned()));
    assert_eq!(tilde_strs(prog, "5-36723-465-"), Ok("5-36723-465-8".to_owned()));
    assert_eq!(tilde_strs(prog, "0-821-99799-"), Ok("0-821-99799-8".to_owned()));
}
