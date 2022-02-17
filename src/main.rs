//crates used: whoami, colored

use colored::Colorize;

fn main() {
    println!("");
    println!("   {}    {}{}{}", "(\\ /)".bright_white(), whoami::username().bright_red(), "@".red(), whoami::hostname().bright_red());
    println!("   {}   {}{}", "( . .)".bright_white(), "OS: ".green(), whoami::distro().bright_white());
    println!("   {0}{3}{1}{3}{2}  {4}{5}", "c(".bright_white(), ")(".bright_white(), ")".bright_white(), "\"".bright_red(), "DE: ".bright_yellow(), whoami::desktop_env().to_string().bright_white());
    println!("");
    println!("   {}{}{}{}{}{}{}{}", "████".bright_black(), "████".bright_red(), "████".bright_green(), "████".bright_yellow(), "████".bright_blue(), "████".bright_magenta(), "████".bright_cyan(), "████".bright_white());
    println!("   {}{}{}{}{}{}{}{}", "████".black(), "████".red(), "████".green(), "████".yellow(), "████".blue(), "████".magenta(), "████".cyan(), "████".white());
    println!("");
}
