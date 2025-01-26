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

    let splitted = message.content.split(' ').collect::<Vec<&str>>();

    if splitted.is_empty() {
        // How ?
        return None;
    }

    // Unwrap is fine, we checked above if the vec is empty
    let first = splitted.first().unwrap();
    
    if !first.starts_with(prefix) {
        // No prefix, no command
        return None;
    }

    match case {
        Case::Sensitive => {
            if first != &format!("{prefix}{base_command}") {
                return None;
            }
        }
        Case::Insensitive => {
            if first.to_lowercase() != format!("{prefix}{}", base_command.to_lowercase()) {
                return None;
            }
        }
    }

    Some(splitted)
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
