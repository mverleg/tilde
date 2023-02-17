use ::std::fmt::Debug;

mod literal;

pub trait Op: Debug {

    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Option<()>;

    fn golf_code(&self) -> Option<()>;

    //TODO @mark: evaluation methods
}

pub fn all_non_literals() -> [&'static dyn Op; 1] {
    [&Dummy]
}


#[derive(Debug, PartialEq, Eq)]
struct Dummy;

impl Op for Dummy {

    fn name(&self) -> &'static str {
        "dummy"
    }

    fn description(&self) -> &'static str {
        todo!()
    }

    fn long_code(&self) -> Option<()> {
        todo!()
    }

    fn golf_code(&self) -> Option<()> {
        todo!()
    }
}

//TODO @mark: long and gold not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
