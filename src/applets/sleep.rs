use clap::{App, SubCommand, Arg, ArgMatches};
use std::thread::sleep;
use std::time::Duration;

pub fn subcommand() -> App<'static, 'static>  {
    SubCommand::with_name("sleep")
        .about("Print the current environment")
        .help("Sleep for the total time of all arguments")
        .arg(
            Arg::with_name("times").multiple(true).index(1).required(true)
        )
}

fn parse(x: &str) -> Result<u32, String> {
    str::parse(x).or(Err(format!("Failed to parse {}", x)))
}

fn get_value_from(x: &str) -> Result<u32, String> {
    let mut t2 = x.to_string();
    match t2.pop().unwrap().to_ascii_lowercase() {
        's' => Ok(parse(&t2)?),
        'm' => Ok(parse(&t2)? * 60),
        'h' => Ok(parse(&t2)? * 60 * 60),
        'd' => Ok(parse(&t2)? * 60 * 60 * 24),
        '0'..='9' => parse(x),
        bad_chr => Err(format!("Invalid char {} in arg {}", bad_chr, x))
    }
}

fn sum_time_safe<'a>(mut values: &mut impl Iterator<Item = &'a str>) -> Result<u32, String> {
    values.try_fold(0, |acc, x: &str| -> Result<u32, String> {
        Ok(acc + get_value_from(x)?)
    })
}

fn get_time_to_sleep(matches: &ArgMatches) -> Result<u32, String> {
    sum_time_safe(&mut matches.values_of("times").unwrap())
}

pub fn sleep_main(matches: Option<&ArgMatches>) -> Result<(), String>{
    // We have required arguments so this won't be none
    let time = get_time_to_sleep(matches.unwrap())?;
    sleep(Duration::from_secs(time.into()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sum_time_safe;
    #[test]
    fn test_calculate_sleep_time() {
    }
}