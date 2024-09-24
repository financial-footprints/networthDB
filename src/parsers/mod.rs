use crate::reader::File;

pub struct Statement {
    pub date: String,
    pub description: String,
    pub withdrawal: f64,
    pub deposit: f64,
    pub balance: f64,
}

pub struct Parser {
    pub id: String,
    pub identify: fn(&File) -> bool,
    pub parse: fn(&File) -> Vec<Statement>,
}

impl Parser {
    pub fn identify(&self, file: &File) -> bool {
        (self.identify)(file)
    }

    pub fn parse(&self, file: &File) -> Vec<Statement> {
        (self.parse)(file)
    }
}

pub fn get_parser(file: &File) -> Parser {
    let parsers = get_all_parsers();

    for parser in parsers {
        if parser.identify(file) {
            return parser;
        }
    }

    panic!("No matching parser found for the given file data");
}

mod hdfcind;
mod icicind;

fn get_all_parsers() -> Vec<Parser> {
    let mut parsers = Vec::new();

    parsers.push(hdfcind::get_parser());
    parsers.push(icicind::get_parser());

    return parsers;
}
