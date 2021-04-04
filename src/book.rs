use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    OneSamuel,
    TwoSamuel,
    OneKings,
    TwoKings,
    OneChronicles,
    TwoChronicles,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalm,
    Proverbs,
    Ecclesiastes,
    SongofSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    OneCorinthians,
    TwoCorinthians,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    OneThessalonians,
    TwoThessalonians,
    OneTimothy,
    TwoTimothy,
    Titus,
    Philemon,
    Hebrews,
    James,
    OnePeter,
    TwoPeter,
    OneJohn,
    TwoJohn,
    ThreeJohn,
    Jude,
    Revelation,
}

impl FromStr for Book {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Genesis" => Ok(Self::Genesis),
            "Exodus" => Ok(Self::Exodus),
            "Leviticus" => Ok(Self::Leviticus),
            "Numbers" => Ok(Self::Numbers),
            "Deuteronomy" => Ok(Self::Deuteronomy),
            "Joshua" => Ok(Self::Joshua),
            "Judges" => Ok(Self::Judges),
            "Ruth" => Ok(Self::Ruth),
            "1 Samuel" => Ok(Self::OneSamuel),
            "2 Samuel" => Ok(Self::TwoSamuel),
            "1 Kings" => Ok(Self::OneKings),
            "2 Kings" => Ok(Self::TwoKings),
            "1 Chronicles" => Ok(Self::OneChronicles),
            "2 Chronicles" => Ok(Self::TwoChronicles),
            "Ezra" => Ok(Self::Ezra),
            "Nehemiah" => Ok(Self::Nehemiah),
            "Esther" => Ok(Self::Esther),
            "Job" => Ok(Self::Job),
            "Psalm" => Ok(Self::Psalm),
            "Proverbs" => Ok(Self::Proverbs),
            "Ecclesiastes" => Ok(Self::Ecclesiastes),
            "SongofSolomon" => Ok(Self::SongofSolomon),
            "Isaiah" => Ok(Self::Isaiah),
            "Jeremiah" => Ok(Self::Jeremiah),
            "Lamentations" => Ok(Self::Lamentations),
            "Ezekiel" => Ok(Self::Ezekiel),
            "Daniel" => Ok(Self::Daniel),
            "Hosea" => Ok(Self::Hosea),
            "Joel" => Ok(Self::Joel),
            "Amos" => Ok(Self::Amos),
            "Obadiah" => Ok(Self::Obadiah),
            "Jonah" => Ok(Self::Jonah),
            "Micah" => Ok(Self::Micah),
            "Nahum" => Ok(Self::Nahum),
            "Habakkuk" => Ok(Self::Habakkuk),
            "Zephaniah" => Ok(Self::Zephaniah),
            "Haggai" => Ok(Self::Haggai),
            "Zechariah" => Ok(Self::Zechariah),
            "Malachi" => Ok(Self::Malachi),
            "Matthew" => Ok(Self::Matthew),
            "Mark" => Ok(Self::Mark),
            "Luke" => Ok(Self::Luke),
            "John" => Ok(Self::John),
            "Acts" => Ok(Self::Acts),
            "Romans" => Ok(Self::Romans),
            "1 Corinthians" => Ok(Self::OneCorinthians),
            "2 Corinthians" => Ok(Self::TwoCorinthians),
            "Galatians" => Ok(Self::Galatians),
            "Ephesians" => Ok(Self::Ephesians),
            "Philippians" => Ok(Self::Philippians),
            "Colossians" => Ok(Self::Colossians),
            "1 Thessalonians" => Ok(Self::OneThessalonians),
            "2 Thessalonians" => Ok(Self::TwoThessalonians),
            "1 Timothy" => Ok(Self::OneTimothy),
            "2 Timothy" => Ok(Self::TwoTimothy),
            "Titus" => Ok(Self::Titus),
            "Philemon" => Ok(Self::Philemon),
            "Hebrews" => Ok(Self::Hebrews),
            "James" => Ok(Self::James),
            "1 Peter" => Ok(Self::OnePeter),
            "2 Peter" => Ok(Self::TwoPeter),
            "1 John" => Ok(Self::OneJohn),
            "2 John" => Ok(Self::TwoJohn),
            "3 John" => Ok(Self::ThreeJohn),
            "Jude" => Ok(Self::Jude),
            "Revelation" => Ok(Self::Revelation),
            _ => Err("Unknown book of the Bible"),
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s: String = String::from(match self {
            Self::Genesis => "Genesis",
            Self::Exodus => "Exodus",
            Self::Leviticus => "Leviticus",
            Self::Numbers => "Numbers",
            Self::Deuteronomy => "Deuteronomy",
            Self::Joshua => "Joshua",
            Self::Judges => "Judges",
            Self::Ruth => "Ruth",
            Self::OneSamuel => "1 Samuel",
            Self::TwoSamuel => "2 Samuel",
            Self::OneKings => "1 Kings",
            Self::TwoKings => "2 Kings",
            Self::OneChronicles => "1 Chronicles",
            Self::TwoChronicles => "2 Chronicles",
            Self::Ezra => "Ezra",
            Self::Nehemiah => "Nehemiah",
            Self::Esther => "Esther",
            Self::Job => "Job",
            Self::Psalm => "Psalm",
            Self::Proverbs => "Proverbs",
            Self::Ecclesiastes => "Ecclesiastes",
            Self::SongofSolomon => "Song of Solomon",
            Self::Isaiah => "Isaiah",
            Self::Jeremiah => "Jeremiah",
            Self::Lamentations => "Lamentations",
            Self::Ezekiel => "Ezekiel",
            Self::Daniel => "Daniel",
            Self::Hosea => "Hosea",
            Self::Joel => "Joel",
            Self::Amos => "Amos",
            Self::Obadiah => "Obadiah",
            Self::Jonah => "Jonah",
            Self::Micah => "Micha",
            Self::Nahum => "Nahum",
            Self::Habakkuk => "Habakkuk",
            Self::Zephaniah => "Zephaniah",
            Self::Haggai => "Haggai",
            Self::Zechariah => "Zechariah",
            Self::Malachi => "Malachi",
            Self::Matthew => "Matthew",
            Self::Mark => "Mark",
            Self::Luke => "Luke",
            Self::John => "John",
            Self::Acts => "Acts",
            Self::Romans => "Romans",
            Self::OneCorinthians => "1 Corinthians",
            Self::TwoCorinthians => "2 Corinthians",
            Self::Galatians => "Galatians",
            Self::Ephesians => "Ephesians",
            Self::Philippians => "Philippians",
            Self::Colossians => "Colossians",
            Self::OneThessalonians => "1 Thessalonians",
            Self::TwoThessalonians => "2 Thessalonians",
            Self::OneTimothy => "1 Timothy",
            Self::TwoTimothy => "2 Timothy",
            Self::Titus => "Titus",
            Self::Philemon => "Philemon",
            Self::Hebrews => "Hebrews",
            Self::James => "James",
            Self::OnePeter => "1 Peter",
            Self::TwoPeter => "2 Peter",
            Self::OneJohn => "1 John",
            Self::TwoJohn => "2 John",
            Self::ThreeJohn => "3 John",
            Self::Jude => "Jude",
            Self::Revelation => "Revelation",
        });

        write!(f, "{}", s)
    }
}
