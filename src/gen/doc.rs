use crate::gen::input::gen_inputs;

pub fn gen_docs() {
    for group in gen_inputs() {
        let f;
        write!(f, "input: ");
        group.fmt_chars(f);
        write!(f, ", bytes: ");
        group.fmt_bytes(f);
        //println!("names: {}", group.names());

        //TODO @mverleg: parse the group and if successful, print description
    }
    todo!(); //TODO @mverleg: TEMPORARY! REMOVE THIS!
}
