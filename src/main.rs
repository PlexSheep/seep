use std::{
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
};

use clap::Parser;

const TTY: &str = "/dev/tty";
const BUFSIZ: usize = 2 << 8;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    targets: Vec<PathBuf>,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    // /dev/tty points the currently opened terminal. If we write to it directly, we bypass piping
    // to the next process
    let tty = std::fs::File::options()
        .write(true)
        .read(false)
        // try to use $TTY set by the terminal, otherwise use the default tty
        .open(std::env::var("TTY").unwrap_or(String::from(TTY)))
        .inspect_err(|err| eprintln!("could not open tty: {err}"))?;

    // fill our targets
    // any file or stream that we want our stdin to go to can be a target. The only condition is
    // that we can write to it.
    let mut targets: Vec<Box<dyn Write>> = vec![Box::new(tty), Box::new(io::stdout())];
    for path in cli.targets {
        match File::options()
            .write(true)
            .read(false)
            .create(true)
            .open(path)
        {
            Ok(f) => targets.push(Box::new(f)),
            Err(err) => {
                eprintln!("error while opening file: {err}");
            }
        }
    }

    // fill the buffer with data from stdin, then write the read data to all targets
    let mut buf = [0; BUFSIZ];
    let mut stdin = io::stdin();
    let mut read_amount = buf.len();
    while read_amount == buf.len() {
        // first we read data from the stdin
        read_amount = stdin.read(&mut buf).inspect_err(|err| eprintln!("{err}"))?;

        // now we just write it to our targets
        for target in targets.iter_mut() {
            target
                .write_all(&buf[..read_amount])
                .inspect_err(|err| eprintln!("{err}"))?;
        }
    }

    // append a newline to the tty
    // otherwise weird wrapped together lines may happen
    targets[0]
        .write(b"\n")
        .inspect_err(|err| eprintln!("{err}"))?;

    // flush all targets
    for target in targets.iter_mut() {
        target.flush().inspect_err(|err| eprintln!("{err}"))?; // make sure it comes there
    }

    Ok(())
}
