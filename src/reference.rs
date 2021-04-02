use std::str::FromStr;

pub type Chapter = u8;
pub type Verse = u8;

use crate::book::Book;

#[derive(Debug)]
pub struct Reference {
    book: Book,
    chapter: Option<Chapter>,
    verse: Option<Verse>,
}

impl FromStr for Reference {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<&str> = s.split(' ').collect();

        let num_elems: usize = elems.len();

        match num_elems {
            0 => Err("Empty string"),
            1 => match Book::from_str(elems[0]) {
                Ok(t) => Ok(Reference {
                    book: t,
                    chapter: None,
                    verse: None,
                }),
                Err(e) => Err(e),
            },
            n if n <= 3 => {
                let book: Book =
                    match Book::from_str(&elems[..(n - 1)].join(" ")) {
                        Ok(t) => t,
                        Err(e) => return Err(e),
                    };

                let sub_elems: Vec<&str> =
                    elems.last().unwrap().split(':').collect();

                match sub_elems.len() {
                    0 => Err("Empty string"),
                    1 => Ok(Reference {
                        book,
                        chapter: Some(sub_elems[0].parse().unwrap()),
                        verse: None,
                    }),
                    2 => Ok(Reference {
                        book,
                        chapter: Some(sub_elems[0].parse().unwrap()),
                        verse: Some(sub_elems[1].parse().unwrap()),
                    }),
                    _ => Err("Trailing characters"),
                }
            }
            _ => Err("Trailing characters"),
        }
    }
}
