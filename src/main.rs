use std::io::{self, prelude::*};
const TTY: &str = "/dev/tty";

fn main() -> Result<(), io::Error> {
    // /dev/tty points the currently opened terminal. If we write to it directly, we bypass piping
    // to the next process
    let mut tty = std::fs::File::options()
        .write(true)
        .read(false)
        // try to use $TTY set by the terminal, otherwise use the default tty
        .open(std::env::var("TTY").unwrap_or(String::from(TTY)))
        .inspect_err(|err| eprintln!("could not open tty: {err}"))?;
    // we want to write to the stdout too
    let mut stdout = io::stdout();
    // sadly, we need to store the content of `stdin` in a buffer and cannot just `io::copy()` it
    // multiple times. `tee` from the GNU coreutils does this too.
    let mut buf = Vec::new();
    io::stdin()
        // FIXME: we don't want to always read to end! If we want to seep a network socket for
        // example.
        .read_to_end(&mut buf)
        .inspect_err(|err| eprintln!("{err}"))?;

    // now we just write to our targets
    tty.write_all(&buf).inspect_err(|err| eprintln!("{err}"))?;
    tty.flush().inspect_err(|err| eprintln!("{err}"))?; // make sure it comes there
    stdout
        .write_all(&buf)
        .inspect_err(|err| eprintln!("{err}"))?;
    stdout.flush().inspect_err(|err| eprintln!("{err}"))?; // make sure it comes there
    Ok(())
}
