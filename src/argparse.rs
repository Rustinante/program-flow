use std::str::FromStr;

use clap::ArgMatches;

pub fn extract_str_arg(matches: &ArgMatches, arg_name: &str) -> String {
    match matches.value_of(arg_name) {
        Some(filename) => filename.to_string(),
        None => {
            eprintln!("the argument {} is required", arg_name);
            std::process::exit(1);
        }
    }
}

pub fn extract_optional_numeric_arg<T: FromStr>(
    matches: &ArgMatches,
    arg_name: &str,
) -> Result<Option<T>, String>
    where <T as std::str::FromStr>::Err: std::fmt::Display {
    match matches.value_of(arg_name) {
        None => Ok(None),
        Some(s) => match s.parse::<T>() {
            Ok(val) => Ok(Some(val)),
            Err(why) => Err(format!("failed to parse {}: {}", arg_name, why))
        }
    }
}

pub fn extract_numeric_arg<T: FromStr>(
    matches: &ArgMatches,
    arg_name: &str,
) -> Result<T, String>
    where <T as std::str::FromStr>::Err: std::fmt::Display {
    match extract_optional_numeric_arg(matches, arg_name)? {
        None => Err(format!("{} is not provided", arg_name)),
        Some(val) => Ok(val)
    }
}

pub fn extract_optional_str_arg(
    matches: &ArgMatches,
    arg_name: &str,
) -> Option<String> {
    match matches.value_of(arg_name) {
        None => None,
        Some(v) => Some(v.to_string())
    }
}

pub fn extract_optional_str_vec_arg(
    matches: &ArgMatches,
    arg_name: &str,
) -> Option<Vec<String>> {
    match matches.values_of(arg_name) {
        None => None,
        Some(v) => Some(v.map(|s| s.to_string()).collect())
    }
}

pub fn extract_str_vec_arg(
    matches: &ArgMatches,
    arg_name: &str,
) -> Result<Vec<String>, String> {
    match extract_optional_str_vec_arg(matches, arg_name) {
        None => Err(format!("{} is not provided", arg_name)),
        Some(v) => Ok(v)
    }
}
