// FIXME: Make me pass! Diff budget: 30 lines.

// I AM NOT DONE

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string(&mut self, s: &str) -> &mut Builder {
        self.string = Some(s);
        return self;
    }

    fn number(&mut self, n: usize) -> &mut Builder {
       self.number = Some(n);
       return self;
    }
}

impl ToString for Builder {
    // Implement the trait
    fn to_string(&self) -> String {
        let string = match self.string {
            Some(s) => s,
            None => String::from("")
        };
        let number = match self.number {
            Some(n) => n.to_string(),
            None => String::from("")
        };

        string + " " + &number

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
