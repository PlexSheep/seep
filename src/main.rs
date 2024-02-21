use std::io::{self, prelude::*};

fn main() -> Result<(), io::Error> {
    let mut tty = std::fs::File::options()
        .write(true)
        .read(false)
        .open("/dev/tty")?;
    io::copy(&mut io::stdin(), &mut tty)?;
    tty.flush()?;
    io::copy(&mut io::stdin(), &mut io::stdout())?;
    Ok(())
}
