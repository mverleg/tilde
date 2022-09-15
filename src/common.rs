use ::std::cell::OnceCell;

static LOG_ON: OnceCell<bool> = OnceCell::new();

macro_rules! log {
    ($templ:literal, $($args:expr)*) => {
        {
            let is_on = LOG_ON.get_or_init(|| {
                let is_on = env::var("TILDE_LOG");
                !(matches!(None, is_on) || matches!(Some(""), is_on) || matches!(Some("0"), is_on))
            });
            if is_on {
                eprintln!($templ);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn log() {
        log!("hello {}", "world")
    }
}
