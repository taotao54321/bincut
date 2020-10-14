use std::io::{self, Read, Write};
use std::str;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Extract byte-range from input.")]
struct Opt {
    #[structopt(short, long, default_value = "0")]
    offset: usize,

    #[structopt(
        short,
        long,
        default_value = "-1",
        help = "if negative, read until EOF"
    )]
    size: isize,
}

fn read_and_cut(offset: usize, size: isize) -> eyre::Result<&'static [u8]> {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    // offset == buf.len() も許す (Rust のスライスの規約と同じ)
    eyre::ensure!(
        offset <= buf.len(),
        "offset out of range (input size: {})",
        buf.len()
    );

    if size < 0 {
        return Ok(&buf.leak()[offset..]);
    }

    let size = size as usize;
    eyre::ensure!(
        size <= buf.len() - offset,
        "size out of range (input size: {})",
        buf.len()
    );

    Ok(&buf.leak()[offset..offset + size])
}

fn is_safe_to_write(buf: &[u8]) -> bool {
    atty::isnt(atty::Stream::Stdout) || str::from_utf8(buf).is_ok()
}

fn main() -> eyre::Result<()> {
    let opt = Opt::from_args();

    let buf = read_and_cut(opt.offset, opt.size)?;

    eyre::ensure!(
        is_safe_to_write(buf),
        "writing this data will break your terminal. aborting"
    );

    io::stdout().write_all(buf)?;

    Ok(())
}
