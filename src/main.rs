use std::env;

mod parsers;
mod reader;

fn load_env_variables() -> String {
    dotenv::dotenv().ok();
    let env_file_path = env::var("ENV_FILE_PATH").ok();

    if let Some(path) = env_file_path {
        dotenv::from_path(path).ok();
    }

    let file_path = env::var("FILE_PATH").expect("E0001: FILE_PATH not provided");

    return file_path;
}

fn main() {
    let file_path = load_env_variables();

    let file_data = reader::read_file(&file_path);
    let parser = parsers::get_parser(&file_data);

    println!("File Type: {}", file_data.file_type);
    println!("Parser: {}", parser.id);
}
