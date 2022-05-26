use crate::command;

pub fn file_not_found_error_exit() {
    let file_not_found_error = command().error(
        clap::error::ErrorKind::Io,
        "File not found."
    );
    file_not_found_error.exit()
}

pub fn request_failed_error_exit() {
    let request_failed_error = command().error(
        clap::error::ErrorKind::Io,
        "Request failed."
    );
    request_failed_error.exit()
}