const DEFAULT_PREFIX: &str = "!";
const EMPTY_STR: &str = ""; // This is more convenient for lifetime issues

#[derive(PartialEq)]
pub enum Case {
    Sensitive,
    Insensitive,
}

#[derive(PartialEq)]
pub enum Prefix {
    Yes,
    No,
}

/// Parsed the message content as a command, returning the args if the requested command was found
pub fn parse<'a>(
    message: &'a serenity::all::Message,
    base_command: &str,
    case: Case,
    prefix: Prefix,
) -> Option<Vec<&'a str>> {
    let prefix = match prefix {
        Prefix::Yes => DEFAULT_PREFIX,
        Prefix::No => EMPTY_STR,
    };

    let mut cmd = message.content.split_whitespace();

    let first = cmd.next()?;

    if !first.starts_with(prefix) {
        // No prefix, no command
        return None;
    }

    match case {
        Case::Sensitive => {
            if first != format!("{prefix}{base_command}") {
                return None;
            }
        }
        Case::Insensitive => {
            if first.to_lowercase() != format!("{prefix}{}", base_command.to_lowercase()) {
                return None;
            }
        }
    }

    Some(cmd.collect())
}

// pub fn is_command(message: &serenity::all::Message, command: &str, case: Case) -> bool {
//     message
//         .content
//         .split(' ')
//         .next()
//         .map(ToString::to_string)
//         .map(|c| {
//             if case == Case::Insensitive {
//                 c.to_lowercase()
//             } else {
//                 c
//             }
//         })
//         == if case == Case::Insensitive {
//             Some(command.to_lowercase().to_string())
//         } else {
//             Some(command.to_string())
//         }
// }
