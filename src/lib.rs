use std::fmt;

pub mod argparse;

pub trait OrExit<T> {
    fn unwrap_or_exit<M: fmt::Display>(self, with_msg_prefix: Option<M>) -> T;
}

impl<T, E: fmt::Display> OrExit<T> for Result<T, E> {
    fn unwrap_or_exit<M: fmt::Display>(self, with_msg_prefix: Option<M>) -> T {
        match self {
            Err(why) => {
                match with_msg_prefix {
                    None => eprintln!("{}", why),
                    Some(msg) => eprintln!("{}: {}", msg, why),
                };
                std::process::exit(1);
            }
            Ok(value) => value,
        }
    }
}

impl<T> OrExit<T> for Option<T> {
    fn unwrap_or_exit<M: fmt::Display>(self, with_msg_prefix: Option<M>) -> T {
        match self {
            None => {
                match with_msg_prefix {
                    None => eprintln!("expected the Option to have some value"),
                    Some(msg) => eprintln!("{}", msg),
                };
                std::process::exit(1);
            }
            Some(value) => value,
        }
    }
}

#[macro_export]
macro_rules! print_named_vars {
    ($($id:ident), +) => {
        $(
            println!("{}", format_args!("{} {}", stringify!($id), $id));
        )+
    };
}

#[macro_export]
macro_rules! debug_print_named_vars {
    ($($id:ident), +) => {
        $(
            println!("{}", format_args!("{} {:?}", stringify!($id), $id));
        )+
    };
}

#[macro_export]
macro_rules! eprint_named_vars {
    ($($id:ident), +) => {
        $(
            eprintln!("{}", format_args!("{} {}", stringify!($id), $id));
        )+
    };
}

#[macro_export]
macro_rules! debug_eprint_named_vars {
    ($($id:ident), +) => {
        $(
            eprintln!("{}", format_args!("{} {:?}", stringify!($id), $id));
        )+
    };
}
