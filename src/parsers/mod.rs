mod hdfc;
mod icici;

use crate::reader::FileData;

pub struct Parser {
    pub id: String,
    pub identify: fn(file_data: &FileData) -> bool,
}

fn get_all_parsers() -> Vec<Parser> {
    let mut parsers = Vec::new();

    parsers.push(hdfc::get_parser());
    parsers.push(icici::get_parser());

    return parsers;
}

pub fn get_parser(file_data: &FileData) -> Parser {
    let parsers = get_all_parsers();

    for parser in parsers {
        if (parser.identify)(file_data) {
            return parser;
        }
    }

    panic!("No matching parser found for the given file data");
}
