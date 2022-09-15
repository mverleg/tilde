
macro_rules! log {
    ($templ:literal, $($args:expr)*) => {
        {
            use ::std::sync::atomic::AtomicU8;
            use ::std::sync::atomic::Ordering;
            use ::std::env;
            use ::std::time::SystemTime;
            const UNKNOWN: u8 = 0;
            const ON: u8 = 1;
            const OFF: u8 = 2;
            static LOG_ON: AtomicU8 = AtomicU8::new(UNKNOWN);
            let mut is_on = LOG_ON.load(Ordering::Acquire);
            if is_on == UNKNOWN {
                eprintln!("initializing IS_LOG, now {is_on}");  //TODO @mverleg: TEMPORARY! REMOVE THIS!
                let on_env = env::var("TILDE_LOG");
                let is_off = matches!(on_env, Err(_)) || matches!(on_env.as_deref(), Ok("")) || matches!(on_env.as_deref(), Ok("0"));
                is_on = if is_off { OFF } else { ON };
                LOG_ON.store(is_on, Ordering::Release);
            }
            eprintln!("initialized IS_LOG, now {is_on}");  //TODO @mverleg: TEMPORARY! REMOVE THIS!
            if is_on == ON {
                // add time without external dependencies?
                eprint!("# ");
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
