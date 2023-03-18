struct Observations {
    all_whitespace: bool,
    last_non_whitespace_is_question_mark: bool,
    has_lowercase: bool,
    has_uppercase: bool,
}

impl Default for Observations {
    fn default() -> Self {
        Self {
            all_whitespace: true,
            last_non_whitespace_is_question_mark: false,
            has_lowercase: false,
            has_uppercase: false,
        }
    }
}

enum Message {
    Speech { question: bool, yelled: bool },
    Silence,
}

impl Message {
    fn response(&self) -> &'static str {
        use Message::*;

        match self {
            Speech {
                question: false,
                yelled: false,
            } => "Whatever.",
            Speech {
                question: false,
                yelled: true,
            } => "Whoa, chill out!",
            Speech {
                question: true,
                yelled: false,
            } => "Sure.",
            Speech {
                question: true,
                yelled: true,
            } => "Calm down, I know what I'm doing!",
            Silence => "Fine. Be that way!",
        }
    }
}

impl From<&str> for Observations {
    fn from(s: &str) -> Self {
        s.chars()
            .fold(Self::default(), |mut observations, character| {
                if character.is_whitespace() {
                    return observations;
                }

                observations.all_whitespace = false;
                observations.last_non_whitespace_is_question_mark = false;

                if character == '?' {
                    observations.last_non_whitespace_is_question_mark = true;
                } else if character.is_lowercase() {
                    observations.has_lowercase = true;
                } else if character.is_uppercase() {
                    observations.has_uppercase = true;
                }

                observations
            })
    }
}

impl From<Observations> for Message {
    fn from(observations: Observations) -> Self {
        match observations {
            Observations {
                all_whitespace: false,
                last_non_whitespace_is_question_mark,
                has_lowercase,
                has_uppercase,
            } => Self::Speech {
                question: last_non_whitespace_is_question_mark,
                yelled: has_uppercase && !has_lowercase,
            },
            Observations {
                all_whitespace: true,
                ..
            } => Self::Silence,
        }
    }
}

pub fn reply(message: &str) -> &str {
    let observations: Observations = message.into();
    let message: Message = observations.into();
    message.response()
}
