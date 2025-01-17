//! Generate the contents of `docs/CommandLineHelp.md`
//!
//! See `docs/Maintenance.md` for more info.

use crate::Args;

use clap::{App, IntoApp};

pub fn print_all_help(markdown: bool) {
    if markdown {
        print_all_help_markdown();
        return;
    }

    let (main_help, subcommand_helps) = app_help();

    println!("{}", main_help);

    for (_name, message) in subcommand_helps {
        println!("{}", message);
    }
}

fn print_all_help_markdown() {
    let (main_help, subcommand_helps) = app_help();

    println!("<!-- BEGIN AUTOGENERATED CONTENT -->");

    println!(
        "
### `wolfram-app-discovery --help`

```text
{main_help}
```"
    );

    for (name, message) in subcommand_helps {
        println!(
            "
#### `wolfram-app-discovery {name} --help`

```text
{message}
```
"
        );
    }
}

//======================================
// Utilities
//======================================

fn app_help() -> (String, Vec<(String, String)>) {
    let cli_app = Args::into_app();

    let mut subcommand_helps = Vec::new();
    for subcommand in cli_app.get_subcommands().cloned() {
        let name = subcommand.get_name();
        if name == "print-all-help" || name == "wolfram-app-discovery" {
            continue;
        }

        subcommand_helps.push((
            name.to_owned(),
            help_message(cli_app.clone(), &["wolfram-app-discovery", name, "--help"]),
        ));
    }

    let main_help = help_message(cli_app.clone(), &["wolfram-app-discovery", "--help"]);

    (main_help, subcommand_helps)
}

fn help_message(app: App, args: &[&str]) -> String {
    let help = app
        .try_get_matches_from(args)
        .expect_err("expect help text error");

    format!("{}", help)
}
