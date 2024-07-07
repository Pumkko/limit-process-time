#![warn(clippy::all, clippy::precedence)]

mod process;

fn main() {
    let process_names = process::get_all_running_proces().unwrap();
    println!("{process_names:#?}");
}
