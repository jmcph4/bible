use std::process::exit;

use clap::{crate_version, Clap};

use crate::fetch::fetch;
use crate::reference::Reference;
use crate::version::BibleVersion;

pub mod book;
pub mod fetch;
pub mod reference;
pub mod version;

#[derive(Clap)]
#[clap(version = crate_version!())]
pub struct Opts {
    reference: Reference,
    #[clap(short, long)]
    bible: Option<BibleVersion>,
}

fn main() {
    let opts: Opts = Opts::parse();

    let bible_reference: Reference = opts.reference;
    let bible_version: BibleVersion = match opts.bible {
        Some(t) => t,
        None => BibleVersion::default(),
    };

    /* retrieve specified Bible verse */
    let text: String = match fetch(bible_reference, bible_version) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("bible: {}", e);
            exit(1);
        }
    };

    println!("{}", text);
}
