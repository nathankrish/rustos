// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16),
}

// What traits does `Duration` need to implement?
impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        let self_ms : u64 = match self {
            Duration::MilliSeconds(ms) => { *ms },
            Duration::Seconds(sec) => { (*sec * 1000).into() },
            Duration::Minutes(mins) => { (*mins * 1000 * 60).into() }
        };

        let other_ms: u64 = match self {
            Duration::MilliSeconds(ms) => { *ms },
            Duration::Seconds(sec) => { (*sec * 1000).into() },
            Duration::Minutes(mins) => { (*mins * 1000 * 60).into() }
        };

        self_ms == other_ms 
    }
}

#[test]
fn traits() {
    assert_eq!(Duration::Seconds(120), Duration::Minutes(2));
    assert_eq!(Duration::Seconds(420), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(420000), Duration::Minutes(7));
    assert_eq!(Duration::MilliSeconds(43000), Duration::Seconds(43));
}
