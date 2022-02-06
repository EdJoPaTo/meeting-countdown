#[derive(Debug, PartialEq)]
pub enum Topic {
    On,
    Hue,
    Sat,
    Text,
}

pub const fn get_verb(topic: &Topic) -> &'static str {
    match topic {
        Topic::On => "on",
        Topic::Hue => "hue",
        Topic::Sat => "sat",
        Topic::Text => "text",
    }
}
