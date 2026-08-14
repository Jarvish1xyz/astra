pub enum Command {
    Hello,
    Status,
    Time,
    About,
    Help,
    Exit,
    NextLine,
    Unknown(String),
}

pub fn parse(input: &str) -> Command {
    match input {
        "hello" => Command::Hello,
        "status" => Command::Status,
        "time" => Command::Time,
        "about" => Command::About,
        "help" => Command::Help,
        "exit" => Command::Exit,
        "\n" => Command::NextLine,
        other => Command::Unknown(other.to_string()),
    }
}