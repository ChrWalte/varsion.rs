use crate::{data, varsion::Varsion};
use std::{env, fs, path::Path};

const SEMVER_SPEC_URL: &str = "https://raw.githubusercontent.com/semver/semver/master/semver.md";

#[derive(Debug)]
struct Command {
    name: String,
    shorts: Vec<String>,
    args: Vec<Command>,
    help: String,
    usage: String,
    // TODO: run: fn()
}

// TODO: instead of showing help on nothing given, prompt the user?
pub fn parse_command(args: Vec<String>) {
    let given_command = args.get(0).unwrap_or(&"".to_string()).clone();
    let is_args = args.len() > 1;
    // TODO: instead of matching here, loop through each command and check if it matches, call run() function after
    match given_command.to_ascii_lowercase().as_str() {
        "help" | "h" => {
            if is_args {
                show_help(Some(args[1].clone()));
            } else {
                show_help(None)
            }
        }
        "version" | "v" => {
            println!("{}", show_version());
        }
        "init" | "ini" => {
            init_varsion(args[1..].to_vec());
        }
        "delete" | "del" => {
            if is_args {
                delete_varsion(Some(args[1].clone()));
            } else {
                delete_varsion(None);
            }
        }
        "increment" | "inc" | "+" => {
            update_varsion(args[1..].to_vec(), 1);
        }
        "decrement" | "dec" | "-" => {
            update_varsion(args[1..].to_vec(), -1);
        }
        "spec" | "s" => {
            show_semver_spec();
        }
        _ => {
            println!("Unknown command");
            show_help(None);
        }
    }
}

// TODO: rename args->subcommands
// TODO: Install, Update, and Uninstall commands?
fn get_commands() -> Vec<Command> {
    vec![
        // HELP COMMAND
        Command {
            name: "help".to_string(),
            shorts: vec!["h".to_string()],
            args: vec![Command {
                name: "<command>".to_string(),
                shorts: vec![],
                args: vec![],
                help: "show usage of command".to_string(),
                usage: "help <command>".to_string(),
            }],
            help: "show available commands".to_string(),
            usage: "help".to_string(),
        },
        // VERSION COMMAND
        Command {
            name: "version".to_string(),
            shorts: vec!["v".to_string()],
            args: vec![],
            help: "show installed version".to_string(),
            usage: "version".to_string(),
        },
        // INIT COMMAND
        Command {
            name: "init".to_string(),
            shorts: vec!["ini".to_string()],
            args: vec![
                Command {
                    name: "<starting-version>".to_string(),
                    shorts: vec![],
                    args: vec![],
                    help: "provide an initial starting version in proper semver format".to_string(),
                    usage: "init 0.0.0-who+knows".to_string(),
                },
                Command {
                    name: "<directory>".to_string(),
                    shorts: vec![],
                    args: vec![],
                    help: "provide a root directory for the VERSION file ".to_string(),
                    usage: "init ./repos/who-knows".to_string(),
                },
            ],
            help: "initialize a VERSION file with a semver version".to_string(),
            usage: "init".to_string(),
        },
        // DELETE COMMAND
        Command {
            name: "delete".to_string(),
            shorts: vec!["del".to_string()],
            args: vec![Command {
                name: "<directory>".to_string(),
                shorts: vec![],
                args: vec![],
                help: "provide a root directory to delete the VERSION file from".to_string(),
                usage: "delete ./repos/who-knows".to_string(),
            }],
            help: "delete a VERSION file from a directory".to_string(),
            usage: "delete".to_string(),
        },
        // INCREMENT COMMAND
        Command {
            name: "increment".to_string(),
            shorts: vec!["inc".to_string(), "+".to_string()],
            args: vec![Command {
                name: "<major|ma|minor|mi|patch|pa>".to_string(),
                shorts: vec![],
                args: vec![],
                help: "provide a section of the version to increment".to_string(),
                usage: "increment patch".to_string(),
            }],
            help: "increment a section of the version (default: patch)".to_string(),
            usage: "increment".to_string(),
        },
        // DECREMENT COMMAND
        Command {
            name: "decrement".to_string(),
            shorts: vec!["dec".to_string(), "-".to_string()],
            args: vec![
                Command {
                    name: "<major|ma|minor|mi|patch|pa>".to_string(),
                    shorts: vec![],
                    args: vec![],
                    help: "provide a section of the version to decrement".to_string(),
                    usage: "decrement patch".to_string(),
                },
                Command {
                    name: "<directory>".to_string(),
                    shorts: vec![],
                    args: vec![],
                    help: "provide a root directory to decrement from the VERSION file".to_string(),
                    usage: "decrement major ./repos/who-knows".to_string(),
                },
            ],
            help: "decrement a section of the version (default: patch)".to_string(),
            usage: "decrement".to_string(),
        },
        // SPEC COMMAND
        Command {
            name: "spec".to_string(),
            shorts: vec!["s".to_string()],
            args: vec![],
            help: "get the semantic versioning specification from semver.org".to_string(),
            usage: "spec".to_string(),
        },
    ]
}

pub fn show_help(for_command: Option<String>) {
    // TODO: update to get VERSION from somewhere. Embedded in the binary? or env var?
    println!(
        "varsion - {} - https://chrwalte.com/varsion.rs ",
        show_version()
    );
    println!("using semantic versioning specification - https://semver.org/");

    let commands = get_commands();
    if for_command.is_some() {
        // help for a specific command
        let given_command = for_command.unwrap().to_ascii_lowercase();
        println!("showing [{}] command usage:", given_command);
        for command in commands {
            if command.name == given_command || command.shorts.contains(&given_command) {
                println!(" help: \t{}", command.help);
                println!(" usage:\t{}", command.usage);
                if !command.shorts.is_empty() {}
                println!(" alias:\t {}", command.shorts.join(", "));
                if !command.args.is_empty() {
                    println!("\n subcommands:");
                    for subcommand in command.args {
                        println!("   - {:<20}{}", subcommand.name + ":", subcommand.help);
                        println!("   - {:<20}{}", "usage:", subcommand.usage);
                    }
                }
                return;
            }
        }
        println!("command not found: [{}]", given_command);
        show_help(None);
        return;
    }
    println!("showing available commands:");
    let commands = get_commands();
    for command in commands {
        if !command.shorts.is_empty() {
            let shorts = command.shorts.join(", ");
            let command_and_shorts = format!("{:<10}({})", command.name, shorts);
            println!(" - {:<20}{}", command_and_shorts, command.help);
        } else {
            println!(" - {:<20}{}", command.name, command.help);
        }
    }
}

pub fn show_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

pub fn init_varsion(args: Vec<String>) {
    let mut version = Varsion::init(None, None);
    let mut use_dir = env::current_dir().unwrap();
    if !args.is_empty() {
        for arg in args {
            let path = Path::new(&arg);
            if path.is_dir() {
                // check if arg is a directory
                use_dir = path.to_path_buf();
            } else if Varsion::valid_version(arg.clone()) {
                // check if arg is a version
                version = Varsion::from_string(arg);
            } else {
                println!("invalid argument: [{}]", arg);
                return;
            }
        }
    }
    // TODO: check if VERSION file already exists
    // TODO: check if path has VERSION already appended
    let version_as_string = version.to_string();
    let version_file_path = use_dir.join("VERSION");
    let version_file_path_as_str = version_file_path.to_str().unwrap();
    println!(
        "initializing VERSION file at [{}] with [{}]",
        version_file_path_as_str, version_as_string
    );
    data::write_str_to_disk(version_file_path_as_str, version_as_string.as_str());
    println!("[{}] initialized", version_file_path_as_str);
}

pub fn delete_varsion(from_directory: Option<String>) {
    let mut use_dir = env::current_dir().unwrap();
    if from_directory.is_some() {
        use_dir = Path::new(&from_directory.unwrap()).to_path_buf();
    }
    if !use_dir.is_dir() {
        panic!("[{}] is not a directory", use_dir.to_str().unwrap());
    }
    let version_file_path = use_dir.join("VERSION");
    if version_file_path.exists() {
        match fs::remove_file(&version_file_path) {
            Ok(_) => println!("deleted [{}]", version_file_path.to_str().unwrap()),
            Err(why) => println!(
                "error deleting [{}]: [{}]",
                version_file_path.to_str().unwrap(),
                why
            ),
        }
        return;
    }
    panic!(
        "VERSION file not found in directory: [{}]",
        use_dir.to_str().unwrap()
    );
}

pub fn update_varsion(args: Vec<String>, by_amount: i32) {
    // TODO: add support for by_amount pass in BY USER
    let mut use_dir = env::current_dir().unwrap();
    let mut use_section = "patch".to_string();
    if !args.is_empty() {
        for arg in args {
            match arg.to_ascii_lowercase().as_str() {
                "major" => {
                    use_section = "major".to_string();
                }
                "ma" => {
                    use_section = "major".to_string();
                }
                "minor" => {
                    use_section = "minor".to_string();
                }
                "mi" => {
                    use_section = "minor".to_string();
                }
                "patch" => {
                    use_section = "patch".to_string();
                }
                "p" => {
                    use_section = "patch".to_string();
                }
                _ => {
                    // is it a path?
                    let path = Path::new(&arg);
                    if path.is_dir() {
                        use_dir = path.to_path_buf();
                    } else {
                        panic!("unknown argument: [{}]", &arg);
                    }
                }
            }
        }
    }
    // TODO: check if VERSION file already exists
    // TODO: check if path has VERSION already appended
    let version_file_path = use_dir.join("VERSION");
    let read_version = data::read_string_from_disk(version_file_path.to_str().unwrap());
    let mut version = Varsion::from_string(read_version.clone());
    version = match use_section.as_str() {
        // TODO: add support for by_amount pass in BY USER
        "major" => version.update_major(by_amount),
        "minor" => version.update_minor(by_amount),
        "patch" => version.update_patch(by_amount),
        _ => {
            panic!("invalid section: [{}]", use_section);
        }
    };
    let version_file_path_as_str = version_file_path.to_str().unwrap();
    let new_version = version.to_string();
    data::write_str_to_disk(version_file_path_as_str, new_version.as_str());
    println!("[{}] -> [{}]", read_version, new_version);
}

pub fn show_semver_spec() {
    println!("semver spec: https://semver.org/");
    let spec = match reqwest::blocking::get(SEMVER_SPEC_URL) {
        Ok(response) => match response.text() {
            Ok(spec) => spec,
            Err(why) => {
                panic!("error getting semver spec: [{}]", why);
            }
        },
        Err(why) => {
            panic!("error getting semver spec: [{}]", why);
        }
    };
    println!("{}", spec);
}
