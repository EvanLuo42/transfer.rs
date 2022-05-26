use clap::{Arg, Command};

mod api;
mod error;

fn command() -> Command<'static> {
    Command::new("transfer")
}

#[tokio::main]
async fn main() {
    let matches = command()
        .version("0.1.0")
        .author("Evan Luo <ziyun.luo@icloud.com>")
        .about("A Rust implement of transfer.sh")
        .arg(Arg::new("file_path")
            .short('f')
            .value_name("FILE_PATH")
            .help("The path of where the file locate.")
            .required(true)
        )
        .arg(Arg::new("file_name")
            .short('n')
            .value_name("FILE_NAME")
            .help("File name")
            .required(true)
        )
        .try_get_matches_from(std::env::args_os())
        .unwrap_or_else(|e| e.exit());

    let file_path = matches.value_of("file_path").unwrap();
    let file_name = matches.value_of("file_name").unwrap();

    let response = api::put_file(file_path, file_name).await;

    println!("Link:\n{}", response.text().await.unwrap())
}