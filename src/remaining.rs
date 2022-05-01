#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Remaining {
    Hours(u8),
    Minutes(u8),
    Seconds(u8),
}

impl From<std::time::Duration> for Remaining {
    #[allow(clippy::cast_possible_truncation)]
    fn from(duration: std::time::Duration) -> Self {
        const NINETY_MINUTES: u64 = 60 * 90;
        const MAX_HOURS: u64 = 255 * 60 * 60;

        let total_seconds = duration.as_secs();
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
        Self::Hours(hours)
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
fn std_duration_hours() {
    let remaining: Remaining = std::time::Duration::from_secs(70 * 60 * 60).into();
    assert_eq!(remaining, Remaining::Hours(70));
}
