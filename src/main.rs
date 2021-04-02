use clap::{crate_version, Clap};

use crate::reference::Reference;
use crate::version::BibleVersion;

pub mod book;
pub mod reference;
pub mod version;

#[derive(Clap)]
#[clap(version = crate_version!())]
#[allow(dead_code)] /* TODO: remove! */
pub struct Opts {
    reference: Reference,
    #[clap(short, long)]
    bible: Option<BibleVersion>,
}

fn main() {
    let _opts: Opts = Opts::parse();

    dbg!(_opts.reference);

    // let reference: Reference = match Reference::from_str(opts.reference {};

    println!("Hello, world!");
}
