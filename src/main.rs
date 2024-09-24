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

    let file = reader::read_file(&file_path);
    let parser = parsers::get_parser(&file);

    let parsed_data = parser.parse(&file);

    for statement in parsed_data {
        println!(
            "Date: {}, Description: {}, Withdrawal: {}, Deposit: {}, Balance: {}",
            statement.date,
            statement.description,
            statement.withdrawal,
            statement.deposit,
            statement.balance
        );
    }

    println!("File Type: {}", file.file_type);
    // println!("File Data: {:?}", file.data);
    println!("Parser: {}", parser.id);
}
