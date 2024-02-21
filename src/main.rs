use std::io::{self, prelude::*};

fn main() -> Result<(), io::Error> {
    // TODO: check if stdin is empty and show usage
    let mut tty = std::fs::File::options()
        .write(true)
        .read(false)
        .open("/dev/tty")
        .inspect_err(|err| eprintln!("{err}"))?;
    // FIXME: only works with cargo run?
    io::copy(&mut io::stdin(), &mut tty).inspect_err(|err| eprintln!("{err}"))?;
    tty.flush().inspect_err(|err| eprintln!("{err}"))?;
    io::copy(&mut io::stdin(), &mut io::stdout()).inspect_err(|err| eprintln!("{err}"))?;
    Ok(())
}
