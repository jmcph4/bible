use clap::{crate_version, Clap};

use crate::bible_version::BibleVersion;

pub mod bible_version;

#[derive(Clap)]
#[clap(version = crate_version!())]
#[allow(dead_code)] /* TODO: remove! */
pub struct Opts {
    verse: String,
    bible: Option<BibleVersion>,
}

fn main() {
    let _opts: Opts = Opts::parse();

    println!("Hello, world!");
}
