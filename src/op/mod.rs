use ::std::fmt::Debug;

trait Op: Debug {
    fn id(&self) -> u8;

    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn long_code(&self) -> Option<()>;

    fn golf_code(&self) -> Option<()>;

    //TODO @mark: evaluation methods
}

//TODO @mark: long and gold not both empty
//TODO @mark: id unique and sequential
//TODO @mark: name unique and identifier-safe
