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

fn read_lines(filename: &String) -> io::Result<Box<BufRead>> {
    if filename == "-" {
        Ok(Box::new(io::BufReader::new(io::stdin())))
    } else {
        let f = File::open(filename)?;
        let reader = io::BufReader::new(f);
        Ok(Box::new(reader))
    }
}

fn _output_file(name: &String, fmt: &mut DisplayFormat, writer: &mut impl std::io::Write) -> Result<(), io::Error>{
    let reader = read_lines(name)?;
    for line in reader.lines() {
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
    fn write_line(&mut self, line: String, writer: &mut std::io::Write) -> Result<(), io::Error> {
        if self.line_numbers == NumberLineOption::All || (self.line_numbers == NumberLineOption::OnlyNoneEmpty && !line.is_empty()) {
            writeln!(writer, "{n:>6}  {line}", n=self.current_line, line=line);
            self.current_line += 1;
            Ok(())
        } else {
            writeln!(writer, "{}", line)
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
