pub mod cmd;
pub mod data;
pub mod varsion;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cmd::parse_command(args[1..].to_vec());
}
