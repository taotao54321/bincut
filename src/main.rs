use std::fs;
use std::io::{self, Read, Write};
use std::str;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Extract byte-range from input.")]
struct Opt {
    #[structopt(short, long, parse(try_from_str = parse_int::parse), default_value = "0")]
    offset: usize,

    #[structopt(
        short,
        long,
        parse(try_from_str = parse_int::parse),
        default_value = "-1",
        help = "if negative, read until EOF"
    )]
    size: isize,

    #[structopt(default_value = "-")]
    filename_in: String,
}

fn read(filename: &str) -> eyre::Result<Vec<u8>> {
    if filename == "-" {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf)?;
        Ok(buf)
    } else {
        Ok(fs::read(filename)?)
    }
}

fn cut(buf: &[u8], offset: usize, size: isize) -> eyre::Result<&[u8]> {
    // offset == buf.len() も許す (Rust のスライスの規約と同じ)
    eyre::ensure!(
        offset <= buf.len(),
        "offset out of range (input size: {})",
        buf.len()
    );

    if size < 0 {
        return Ok(&buf[offset..]);
    }

    let size = size as usize;
    eyre::ensure!(
        size <= buf.len() - offset,
        "size out of range (input size: {})",
        buf.len()
    );

    Ok(&buf[offset..offset + size])
}

fn is_safe_to_write(buf: &[u8]) -> bool {
    atty::isnt(atty::Stream::Stdout) || str::from_utf8(buf).is_ok()
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let buf = read(&opt.filename_in)?;

    let buf = cut(&buf, opt.offset, opt.size)?;

    eyre::ensure!(
        is_safe_to_write(buf),
        "writing this data will break your terminal. aborting"
    );

    io::stdout().write_all(buf)?;

    Ok(())
}
