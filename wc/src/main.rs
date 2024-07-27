use std::{
    env::args,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

/*
    wc unix codeutils implementation in rust.

    reference - https://github.com/coreutils/coreutils/blob/master/src/wc.c

    TODO
        1. with argument `-c` it should output the numbers of bytes
            output - 342190 test.txt
        2. with argument `-l` should output the number of lines
            output - 7145 test.txt
        3. with argument `-w` should output the number of words
            output - 58164 test.txt
        4. with argument `-m` should output the number of chars
            output - 339292 test.txt
        5. without any argument, it should output all three
            output - 7145 58164 342190 test.txt
        6. it should be able to take stdout stream as input (pipe) if no filename is specified
            `cat test.txt | ccwc -l` should give output - 7145

        - make it as fast as possible
*/
enum Command {
    BytesCount,
    LinesCount,
    WordsCount,
    CharsCount,
}

struct Args {
    command: Option<Command>,
    file_path: Option<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut args = args().skip(1);

    let command = match args.next().as_deref() {
        Some("-c") => Command::BytesCount,
        Some("-l") => Command::LinesCount,
        Some("-w") => Command::WordsCount,
        Some("-m") => Command::CharsCount,
        Some(option) if option.starts_with('-') => return Err("Invalid option".to_string()),
        Some(file) => {
            return Ok(Args {
                command: None,
                file_path: Some(file.to_string()),
            });
        }
        _ => {
            return Ok(Args {
                command: None,
                file_path: None,
            })
        }
    };

    let file_name: Option<String> = match args.next() {
        Some(file) => Some(file),
        None => None,
    };

    return Ok(Args {
        command: Some(command),
        file_path: file_name,
    });
}

/*
    count bytes

    1. https://llogiq.github.io/2016/09/27/count.html
    2. https://llogiq.github.io/2016/09/24/newline.html
*/
fn _count_bytes_v1(file_path: &str) -> u64 {
    /*
       perf -
       real    0m0.050s
       user    0m0.034s
       sys     0m0.016s
    */
    File::open(&file_path).unwrap().metadata().unwrap().len()
}

fn _count_bytes_v3(file_path: &str) -> usize {
    /*
       perf -
       real    0m0.056s
       user    0m0.036s
       sys     0m0.020s
    */
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);

    reader.bytes().count()
}

fn count_bytes(file_path: &str) -> usize {
    /*
       - not significant difference
       perf -
       real    0m0.049s
       user    0m0.036s
       sys     0m0.012s
    */
    // metadata(&file_path).unwrap().len() v2
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);

    reader.bytes().count()
}

fn count_lines(file_path: &str) -> usize {
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().count()
}

fn count_words(file_path: &str) -> usize {
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().split_whitespace().count())
        .sum()
}

fn is_continous_byte(byte: &u8) -> bool {
    (byte & 0b1100_0000) == 0b1000_0000
}

/**
 * note: number of chars is not equal to number of bytes, UTF-8 encoding :)
 */
fn count_chars(file_path: &str) -> usize {
    let file = File::open(&file_path).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for byte in reader.bytes() {
        count += if !is_continous_byte(&byte.unwrap()) {
            1
        } else {
            0
        };
    }

    count
}

fn count_bytes_from_stdin(buffer: &String) -> usize {
    buffer.bytes().count()
}

fn count_lines_from_stdin(buffer: &String) -> usize {
    buffer.split('\n').filter(|line| !line.is_empty()).count()
}

fn count_words_from_stdin(buffer: &String) -> usize {
    buffer.split_whitespace().count()
}

fn count_chars_from_stdin(buffer: &String) -> usize {
    let mut count = 0;
    for byte in buffer.bytes() {
        if !is_continous_byte(&byte) {
            count += 1
        }
    }
    count
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let file_path = args.file_path;

    match file_path {
        Some(file_path) => match args.command {
            Some(Command::BytesCount) => {
                println!("{} {}", count_bytes(&file_path), file_path);
            }
            Some(Command::LinesCount) => {
                println!("{} {}", count_lines(&file_path), file_path);
            }
            Some(Command::WordsCount) => {
                println!("{} {}", count_words(&file_path), file_path);
            }
            Some(Command::CharsCount) => {
                println!("{} {}", count_chars(&file_path), file_path);
            }
            None => {
                let lines = count_lines(&file_path);
                let words = count_words(&file_path);
                let bytes = count_bytes(&file_path);

                println!("{} {} {} {}", lines, words, bytes, file_path);
            }
        },
        _ => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();

            match args.command {
                Some(Command::BytesCount) => {
                    println!("{}", count_bytes_from_stdin(&buffer));
                }
                Some(Command::LinesCount) => {
                    println!("{}", count_lines_from_stdin(&buffer));
                }
                Some(Command::WordsCount) => {
                    println!("{}", count_words_from_stdin(&buffer));
                }
                Some(Command::CharsCount) => {
                    println!("{}", count_chars_from_stdin(&buffer));
                }
                None => {
                    let lines = count_lines_from_stdin(&buffer);
                    let words = count_words_from_stdin(&buffer);
                    let bytes = count_bytes_from_stdin(&buffer);

                    println!("{} {} {}", lines, words, bytes);
                }
            }
        }
    }
}
