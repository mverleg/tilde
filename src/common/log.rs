
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($templ:literal $(, $args:expr)*) => {
        {
            use ::std::sync::atomic::AtomicU8;
            use ::std::sync::atomic::Ordering;
            static LOG_ON: AtomicU8 = AtomicU8::new(0);
            use ::std::env;
            const UNKNOWN: u8 = 0;
            const ON: u8 = 1;
            const OFF: u8 = 2;
            let mut is_on = LOG_ON.load(Ordering::Acquire);
            if is_on == UNKNOWN {
                let on_env = env::var("TILDE_LOG");
                let is_off = matches!(on_env, Result::Err(_)) || matches!(on_env.as_deref(), Result::Ok("")) || matches!(on_env.as_deref(), Result::Ok("0"));
                is_on = if is_off { OFF } else { ON };
                LOG_ON.store(is_on, Ordering::Release);
            }
            if is_on == ON {
                // add time without external dependencies?
                eprint!("# {}:{} ", file!(), line!());
                eprintln!($templ, $($args, )*);
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($templ:literal $(, $args:expr)*) => {
        {}
    }
}

pub use log;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_log() {
        log!("hello {}", "world")
    }
}
