 use cli_prompts_rs::{CliPrompt, LogType, PromptSelectOption};
 use std::process::exit;

 fn main() {
     let mut cli_prompt = CliPrompt::new();
     cli_prompt.intro("example app").unwrap();

     cli_prompt.prompt_text("Enter your name").unwrap();

     let answer = cli_prompt.prompt_confirm("Are you sure?").unwrap();

     if !answer {
         cli_prompt.cancel("Operation cancelled").unwrap();
         exit(0);
     }

     let options = vec![
         PromptSelectOption::new("option1", "Pikachu"),
         PromptSelectOption::new("option2", "Charmander"),
         PromptSelectOption::new("option3", "Squirtle"),
     ];
     let selected_option = cli_prompt
         .prompt_select("Which one do you prefer?", options)
         .unwrap();

     cli_prompt
         .log(&format!("{}", selected_option), LogType::Info)
         .unwrap();
     cli_prompt.outro("Good Bye").unwrap();
 }