use std::{
    env::args, 
    fs::File, 
    io::{self, BufRead, BufReader, Read}
};

/**
 *  cat unix codeutils implementation in rust.
 *
 *  reference : https://github.com/coreutils/coreutils/blob/master/src/cat.c
 *
 *  Plan :
 *      1. `cat some-text.txt` outputs the text file contents
 *      2. `cat -n some-text.txt` outputs with line number
 *      3. `cat -` should take stdin stream as input
 *      4. `cat file.txt data.txt` concats both file contents and outputs
 */

enum Command {
    NumberLines,
    Stream,
}

struct Args {
    command: Option<Command>,
    file_paths: Vec<String>,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = args().skip(1).collect();

    if args.len() == 0 {
        return Err("missing arguments".to_string());
    };

    let first_arg = &args[0];
    let command: Command = match first_arg.as_str() {
        "-n" => Command::NumberLines,
        "-" => Command::Stream,
        _ => return Ok(Args {
            command: None,
            file_paths: args.to_vec(),
        }),
    };

    return Ok(Args {
        command: Some(command),
        file_paths: args[1..].to_vec(),
    })
}

fn process_file(file_path: &str, print_line: bool) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // imp: .lines() allocates a new `String` per line. so its not recommended to use it otherwise needed [todo - find a better way]
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if print_line {
            println!("{} {}", index, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(());
        }
    }; 

    /*
     * to read a file line by line I've multiple options 
     * - direct read_to_string -> but not good for memory 
     * - using bufReader which reads in chunks 
    */

    match args.command {
        Some(Command::NumberLines) => {
            for path in args.file_paths {
                process_file(&path, true)?;
            }
        },
        Some(Command::Stream) => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;

            for line in buffer.lines() {
                println!("{}", line);
            }
        },
        None => {
            for path in args.file_paths {
                process_file(&path, false)?;
            }
        }
    }

    Ok(())
}
