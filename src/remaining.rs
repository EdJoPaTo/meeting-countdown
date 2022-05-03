#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Remaining {
    ManyHours(u8),
    /// Hours, Minutes
    SingleDigitHours(u8, u8),
    Minutes(u8),
    Seconds(u8),
}

impl Remaining {
    /// Returns the update interval in seconds
    pub const fn update_interval(self) -> u32 {
        match self {
            Remaining::Seconds(s) if s <= 20 => 1,
            Remaining::Seconds(_) => 5,
            Remaining::Minutes(m) if m <= 2 => 5,
            _ => 30,
        }
    }
}

impl From<std::time::Duration> for Remaining {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn from(duration: std::time::Duration) -> Self {
        let total_seconds = duration.as_secs_f32().round() as u64;
        if total_seconds <= 90 {
            return Self::Seconds(total_seconds as u8);
        }

        let total_minutes = total_seconds / 60;
        if total_minutes <= 90 {
            return Self::Minutes(total_minutes as u8);
        }

        let total_hours = total_minutes / 60;
        let hours = total_hours
            .try_into()
            .expect("More than 2^8 hours aren't possible with this tool");
        if hours <= 9 {
            let minutes = total_minutes % 60;
            Self::SingleDigitHours(hours, minutes as u8)
        } else {
            Self::ManyHours(hours)
        }
    }
}

#[test]
fn std_duration_seconds() {
    let remaining: Remaining = std::time::Duration::from_secs(70).into();
    assert_eq!(remaining, Remaining::Seconds(70));
}

#[test]
fn std_duration_minutes() {
    let remaining: Remaining = std::time::Duration::from_secs(70 * 60).into();
    assert_eq!(remaining, Remaining::Minutes(70));
}

#[test]
fn std_duration_single_digit_hours() {
    let remaining: Remaining = std::time::Duration::from_secs(150 * 60).into();
    assert_eq!(remaining, Remaining::SingleDigitHours(2, 30));
}

#[test]
fn std_duration_many_hours() {
    let remaining: Remaining = std::time::Duration::from_secs(70 * 60 * 60).into();
    assert_eq!(remaining, Remaining::ManyHours(70));
}
