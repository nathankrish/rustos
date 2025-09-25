// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.string = Some(value.into());
        self
    }

    fn number(&mut self, number: usize) -> &mut Self {
        self.number = Some(number);
        self
    }
}

impl ToString for Builder {
    // Implement the trait
    fn to_string(&self) -> String {
        let mut res = "".to_owned();
        match &self.string {
            None => {},
            Some(s) => { res.push_str(&s); }
        }
        if self.string.is_some() && self.number.is_some() {
            res.push_str(&" ".to_owned())
        }
        match self.number {
            None => {},
            Some(n) => { res.push_str(&n.to_string()); }
        }; 
        res

    }
}

// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default().string("heap!".to_owned()).to_string();

    assert_eq!(c, "heap!");
}
