use ::std::sync::atomic::AtomicU8;
use ::std::sync::atomic::Ordering;
use ::std::env;

const UNKNOWN: u8 = 1;
const ON: u8 = 1;
const OFF: u8 = 2;
static LOG_ON: AtomicU8 = AtomicU8::new(0);

macro_rules! log {
    ($templ:literal, $($args:expr)*) => {
        {
            // let is_on = LOG_ON.store(|| {
            //     let is_on = env::var("TILDE_LOG");
            //     !(matches!(None, is_on) || matches!(Some(""), is_on) || matches!(Some("0"), is_on))
            // }, Ordering::Release);
            let mut is_on = LOG_ON.load(Ordering::Acquire);
            if is_on == UNKNOWN {
                let on_env = env::var("TILDE_LOG");
                let is_off = matches!(None, on_env) || matches!(Some(""), on_env) || matches!(Some("0"), on_env);
                is_on = if is_off { OFF } else { ON };
                LOG_ON.store(is_on, Ordering::Release);
            }
            if is_on == ON {
                eprintln!($templ, $($args, )*);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log() {
        log!("hello {}", "world")
    }
}
