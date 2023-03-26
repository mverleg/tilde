
pub trait Fork {
    fn fork(&self) -> Self;
}

impl <T: Fork> Fork for Vec<T> {
    fn fork(&self) -> Self {
        //TODO @mark: eventually don't ues vec but persistent collection
        self.into_iter().map(|e| e.fork()).collect()
    }
}
