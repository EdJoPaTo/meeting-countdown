#[derive(Debug, PartialEq)]
pub enum Topic {
    Hue,
    Sat,
    Text,
}

pub fn get_verb(topic: &Topic) -> &'static str {
    match topic {
        Topic::Hue => "hue",
        Topic::Sat => "sat",
        Topic::Text => "text",
    }
}
