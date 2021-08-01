use crate::topic::{get_verb, Topic};

pub fn publish(topic: &Topic, value: &str) {
    let verb = get_verb(topic);
    println!("{} {}", verb, value);
}
