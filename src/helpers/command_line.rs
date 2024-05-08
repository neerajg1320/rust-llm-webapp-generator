use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the print color
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statemnt
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{} ", agent_statement);
        stdout.execute(ResetColor).unwrap();
    }
}

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!();
    println!("{}", question);

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    // Read the user input
    let mut user_response = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");

    // Trim whitespace and return response
    return user_response.trim().to_string();
}

// Get user response that code is safe to execute
pub fn confirm_safe_code() -> bool {
    let mut stdout: std::io::Stdout = stdout();
    loop {
        // Print the warning in specified color
        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();

        println!("");
        println!("WARNING: You are about to run code written entirely by AI.");
        println!("Review your code and confirm if you wish to continue");

        // Reset the color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] All good");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop the project");

        stdout.execute(ResetColor).unwrap();
        let mut human_response = String::new();
        stdin()
            .read_line(&mut human_response)
            .expect("Failed to read response");

        // Trim whitespace and convert to lowercase
        let human_response = human_response.trim().to_lowercase();
        match human_response.as_str() {
            "1" | "ok" | "y" => return true,
            "2" | "no" | "n" => return false,
            _ => {
                println!("Invalid input. Please select 1 or 2");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrintCommand;

    #[test]
    pub fn tests_prints_agent_messages() {
        PrintCommand::AICall.print_agent_message(
            "Managing Agent",
            "Managing Agent in action as it is invoked",
        )
    }
}
