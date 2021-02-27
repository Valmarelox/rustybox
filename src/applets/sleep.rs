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

fn sum_time_safe<'a>(mut values: &'a mut impl Iterator<Item = &'a str>) -> Result<u32, String> {
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
    use std::slice::Iter;

    fn test<'a>(mut a: &mut impl Iterator<Item = &'a &'a str>) {

    }
    struct myvec<'a> {
        iter: Iter<'a, &'a str>
    }

    // TODO: I miss something in how the generics work because for the love of god I wasn't able to
    // get array.iter() to be passedd into sum_time_safe - something with &&str and &str
    impl<'a> Iterator for myvec<'a> {
        type Item = &'a str;

        fn next(&mut self) -> Option<&'a str> {
            match self.iter.next() {
                Some(x) => Some(*x),
                None => None,
            }
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }

    #[test]
    fn test_calculate_sleep_time() {
        let mut a: [&'static str; 1] = ["1s"];
        let mut vecy = myvec { iter: a.iter()};
        assert_eq!(sum_time_safe(&mut vecy).unwrap(), 1);

        let mut a: [&'static str; 3] = ["1m", "5s", "6"];
        let mut vecy = myvec { iter: a.iter()};
        assert_eq!(sum_time_safe(&mut vecy).unwrap(), 71);

        let mut a: [&'static str; 1] = ["1k"];
        let mut vecy = myvec { iter: a.iter()};
        assert!(sum_time_safe(&mut vecy).is_err());

        let mut a: [&'static str; 1] = ["LOL"];
        let mut vecy = myvec { iter: a.iter()};
        assert!(sum_time_safe(&mut vecy).is_err());
    }
}