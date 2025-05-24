use crate::Message;

pub enum Printer {
    Pretty,
    Oneline,
    JSON,
}

impl Printer {
    pub fn print_message(&self, message: &Message) -> () {
        match *self {
            Self::Pretty => print_message_pretty(message),
            Self::Oneline => print_message_oneline(message),
            Self::JSON => print_message_json(message),
        }
    }
}

fn print_message_pretty(m: &Message) {
    let s_line = anstyle::Style::new().bold().fg_color(Some(anstyle::AnsiColor::BrightBlack.into()));
    let s_link = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::BrightBlack.into()));
    let s_sym = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Green.into()));
    let s_name = anstyle::Style::new().bold().fg_color(Some(anstyle::AnsiColor::BrightGreen.into()));
    let s_time = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));

    anstream::println!("{s_line}--------------------------------------------------------------------------------{s_line:#}");
    anstream::println!("{s_time}{}{s_time:#} {s_link}/topics/{}/posts/{}{s_link:#}",
        m.created_at().naive_local(),
        m.topic_id(),
        m.id(),
    );
    anstream::println!(
        "{s_sym}[{s_sym:#}{s_name}{} ({}){s_name:#}{s_sym}]{s_sym:#}",
        m.account_fullname(),
        m.account_name(),
    );
    anstream::println!("{}", m.message())
}

fn print_message_oneline(m: &Message) {
    let s_link = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::BrightBlack.into()));
    let s_sym = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Green.into()));
    let s_name = anstyle::Style::new().bold().fg_color(Some(anstyle::AnsiColor::BrightGreen.into()));
    let s_time = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Blue.into()));

    anstream::println!("{s_sym}[{s_sym:#}{s_name}{} ({}){s_name:#}{s_sym}]{s_sym:#} {s_time}{}{s_time:#} {s_link}/topics/{}/posts/{}{s_link:#} {}",
    m.account_fullname(),
    m.account_name(),
    m.created_at().naive_local(),
    m.topic_id(),
    m.id(),
    m.message().replace("\n", " "),
    );
}

fn print_message_json(m: &Message) {
    let _ = serde_json::to_writer(std::io::stdout(), m);
    println!("");
}
