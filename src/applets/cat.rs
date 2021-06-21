use clap::{App, SubCommand, Arg, ArgMatches};
use std::io;
use std::fs::File;
use std::io::BufRead;

pub fn subcommand() -> App<'static, 'static>  {
    SubCommand::with_name("cat")
        .about("concate FILEs to stdout")
        .arg(
            Arg::with_name("number-lines").short("-n").help("number output lines").overrides_with("number-none-empty-lines")
        )
        .arg(
            Arg::with_name("number-none-empty-lines").short("-b").help("number none-empty output lines").overrides_with("number-lines")
        )
        .arg(
            Arg::with_name("files").multiple(true).index(1)
        )
}
struct FullLines<B> {
    buf: B,
}

impl<B: BufRead> Iterator for FullLines<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

fn get_reader(filename: &String) -> io::Result<Box<dyn BufRead>> {
    if filename == "-" {
        Ok(Box::new(io::BufReader::new(io::stdin())))
    } else {
        let f = File::open(filename)?;
        let reader = io::BufReader::new(f);
        Ok(Box::new(reader))
    }
}

fn _output_file(name: &String, fmt: &mut DisplayFormat, writer: &mut impl std::io::Write) -> Result<(), io::Error>{
    let reader = get_reader(name)?;
    let lines_reader = FullLines { buf: reader };
    for line in lines_reader {
        if let Ok(line) = line {
            fmt.write_line(line, writer)?;
        }
        // TODO: Handle errors
    }
    Ok(())
}

#[derive(PartialEq)]
enum NumberLineOption {
    None,
    OnlyNoneEmpty,
    All,
}

struct DisplayFormat {
    line_numbers: NumberLineOption,
    current_line: u32,
}

impl DisplayFormat {
    fn build(matches: &Option<&ArgMatches>) -> DisplayFormat {
        if let Some(matches) = matches {
            DisplayFormat {
                line_numbers: match (matches.is_present("number-lines"), matches.is_present("number-none-empty-lines")) {
                    (true, _) => NumberLineOption::All,
                    (false, true) => NumberLineOption::OnlyNoneEmpty,
                    (false, false) => NumberLineOption::None,
                },
                current_line: 1
            }
        } else {
            DisplayFormat {
                line_numbers: NumberLineOption::None,
                current_line: 1
            }
        }
    }
    fn write_line(&mut self, line: String, writer: &mut dyn std::io::Write) -> Result<(), io::Error> {
        if self.line_numbers == NumberLineOption::All || (self.line_numbers == NumberLineOption::OnlyNoneEmpty && !line.trim_end_matches("\n").is_empty()) {
            write!(writer, "{n:>6}  {line}", n=self.current_line, line=line)?;
            self.current_line += 1;
            Ok(())
        } else {
            write!(writer, "{}", line)
        }
    }
}

fn get_files(matches: &Option<&ArgMatches>) -> Vec<String> {
    if let Some(matches) = matches {
        if let Some(files) = matches.values_of("files") {
            files.map(|res| res.to_string()).collect()
        } else {
            vec!["-".to_string()]
        }
    } else {
        vec!["-".to_string()]
    }
}

fn _cat_main(matches: Option<&ArgMatches>, writer: &mut impl std::io::Write) -> Result<(), String> {
    let mut fmt = DisplayFormat::build(&matches);
    let files = get_files(&matches);
    for filename in files {
        match _output_file(&filename, &mut fmt, writer).err() {
            Some(x) => {
                return Err(format!("{file}: {err}", file=filename, err=x))
            },
            None => ()
        }
    }
    Ok(())
}

pub fn cat_main(matches: Option<&ArgMatches>) -> Result<(), String>{
    _cat_main(matches, &mut std::io::stdout())
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use super::{subcommand, _cat_main};
    use std::process::Command;

    fn create_file(name: &str, content: &str) {
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo -n '{content}' > {name}", name=name, content=content))
            .output()
            .expect("failed to execute process");
    }

    fn run_get_output(args: Vec<&OsStr>) -> Vec<u8> {
        let mut s : Vec<u8> = Vec::new();
        let cmd = subcommand();
        let matches = cmd.get_matches_from(args.iter());
        _cat_main(Some(&matches), &mut s);
        return s;
    }

    fn run_cat_test_case(name: &str, content: &str, args: Vec<&OsStr>, expected_output: &str) {
        create_file(name, content);
        let s = run_get_output(args);
        assert_eq!(s, expected_output.as_bytes());
    }

    #[test]
    fn test_cat_basic() {
        let name = "/tmp/rustybox-cat-test1";
        let content = "why can I not do this like a human person";
        let args = vec![OsStr::new("cat"), OsStr::new(name)];
        run_cat_test_case(name, content, args, content);
    }

    #[test]
    fn test_cat_all_numbers() {
        let name = "/tmp/rustybox-cat-test2";
        let content = "why can I not do this like a human person\nThis is another line\n\nWe got past the empty line\n";
        let expected_output = "     1  why can I not do this like a human person\n     2  This is another line\n     3  \n     4  We got past the empty line\n";
        let args = vec![OsStr::new("cat"), OsStr::new("-n"), OsStr::new(name)];
        run_cat_test_case(name, content, args, expected_output);
    }

    #[test]
    fn test_cat_some_numbers() {
        let name = "/tmp/rustybox-cat-test3";
        let content = "why can I not do this like a human person\nThis is another line\n\nWe got past the empty line\n";
        let expected_output = "     1  why can I not do this like a human person\n     2  This is another line\n\n     3  We got past the empty line\n";
        let args = vec![OsStr::new("cat"), OsStr::new("-b"), OsStr::new(name)];
        run_cat_test_case(name, content, args, expected_output);
    }

    #[test]
    fn test_whitespace_is_empty() {
        let name = "/tmp/rustybox-cat-test4";
        let content = "why can I not do this like a human person\nThis is another line\n  \nWe got past the almost-empty line\n";
        let expected_output = "     1  why can I not do this like a human person\n     2  This is another line\n     3    \n     4  We got past the almost-empty line\n";
        let args = vec![OsStr::new("cat"), OsStr::new("-b"), OsStr::new(name)];
        run_cat_test_case(name, content, args, expected_output);
    }
}
