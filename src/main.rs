pub mod section;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return ExitCode::FAILURE
    }

    let name = &args[1];

    let file = match std::fs::read(name) {
        Ok(file) => file,
        Err(_) => return ExitCode::FAILURE,
    };


    if file.len() < 8 {
        return ExitCode::FAILURE
    }

    let magic_number = &file[0..4];
    let version = &file[4..4];

    if magic_number != &[0x0, 'a' as u8, 's' as u8, 'm' as u8] {
        return ExitCode::FAILURE
    }

    




    ExitCode::SUCCESS
}