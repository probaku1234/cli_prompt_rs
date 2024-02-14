use cli_prompts_rs::CliPrompt;
#[cfg(feature = "unstable")]
use std::{thread, time};

fn main() {
    let mut cli_prompt = CliPrompt::new();

    cli_prompt.intro("spinner example").unwrap();

    cli_prompt
        .print_note("This example shows how to use spinner feature")
        .unwrap();

    // wrap it with unstable feature to prevent build failure without unstable feature
    #[cfg(feature = "unstable")]
    {
        let task = || {
            thread::sleep(time::Duration::from_millis(5000));
        };
        cli_prompt
            .run_with_spinner("working", "Done!", 10000, task)
            .unwrap();
    }

    cli_prompt.outro("Good Bye").unwrap();
}
