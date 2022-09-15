
macro_rules! log {
    ($templ:literal, $($args:expr)*) => {
        {
            use ::std::sync::atomic::AtomicU8;
            use ::std::sync::atomic::Ordering;
            use ::std::env;
            const UNKNOWN: u8 = 1;
            const ON: u8 = 1;
            const OFF: u8 = 2;
            static LOG_ON: AtomicU8 = AtomicU8::new(0);
            let mut is_on = LOG_ON.load(Ordering::Acquire);
            if is_on == UNKNOWN {
                let on_env = env::var("TILDE_LOG");
                let is_off = matches!(on_env, Err(_)) || matches!(on_env.as_deref(), Ok("")) || matches!(on_env.as_deref(), Ok("0"));
                is_on = if is_off { OFF } else { ON };
                LOG_ON.store(is_on, Ordering::Release);
            }
            if is_on == ON {
                eprintln!($templ, $($args, )*);
            }
        }
    }
}

pub(crate) use log;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_log() {
        log!("hello {}", "world")
    }
}
