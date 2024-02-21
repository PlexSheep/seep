use std::io::{self, prelude::*};
const TTY: &str = "/dev/tty";
const BUFSIZ: usize = 2 << 8;

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
    let mut buf = [0; BUFSIZ];
    let mut stdin = io::stdin();
    let mut read_amount = buf.len();
    while read_amount == buf.len() {
        read_amount = stdin.read(&mut buf).inspect_err(|err| eprintln!("{err}"))?;

        // now we just write to our targets
        tty.write_all(&buf[..read_amount])
            .inspect_err(|err| eprintln!("{err}"))?;
        stdout
            .write_all(&buf[..read_amount])
            .inspect_err(|err| eprintln!("{err}"))?;
    }
    tty.write(b"\n").inspect_err(|err| eprintln!("{err}"))?; // otherwise weird wrapped together
                                                             // lines may happen
    tty.flush().inspect_err(|err| eprintln!("{err}"))?; // make sure it comes there
    stdout.flush().inspect_err(|err| eprintln!("{err}"))?; // make sure it comes there
    Ok(())
}
