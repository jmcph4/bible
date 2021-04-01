use std::str::FromStr;

/* TODO: expand this list!
 * 
 * A likely candidate for a canonical source would be the source of all human
 * knowledge, Wikipedia: https://en.wikipedia.org/wiki/List_of_English_Bible_
 * translations#Complete_Bibles
 */
#[derive(Debug)]
pub enum BibleVersion {
    EnglishStandardVersion,
    KingJames,
    NewInternationalVersion,
    NewRevisedStandardVersion,
}

impl FromStr for BibleVersion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "esv" | "ESV" => Ok(Self::EnglishStandardVersion),
            "kjv" | "KJV" => Ok(Self::KingJames),
            "niv" | "NIV" => Ok(Self::NewInternationalVersion),
            "nsrv" | "NSRV" => Ok(Self::NewRevisedStandardVersion),
            _ => Err("Unknown version of the Bible"),
        }
    }
}

