struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    println!("{}", greetings("Jane"));

    let r = Rectangle { height: 3, width: 4 };
    println!("Is it a square? {}", r.is_square());
}

fn greetings(name: &str) -> String {
    format!("Hello, {}!", &name)
}

#[cfg(test)]
mod basic_tests {
    #[test]
    fn test_one_to_be_one() {
        assert!(1 == 1);
    }

    #[test]
    #[should_panic]
    fn expect_panic() {
        panic!("Oh no!");
    }

    #[test]
    fn expect_1_plus_3_to_eq_4() {
        assert_eq!(1 + 3, 4);
    }

    #[test]
    fn expect_1_plus_1_not_to_eq_10() {
        // in Dec, haha!
        assert_ne!(1 + 1, 10);
    }
}

#[cfg(test)]
mod test_fuctions {
    #[test]
    fn test_greetings() {
        // use `super` to access the outsider world
        // as we are in a module
        assert_eq!(super::greetings("Hegwin"), "Hello, Hegwin!");
    }
}

#[cfg(test)]
mod test_structs {
    #[test]
    fn test_is_square() {
        let r = super::Rectangle { height: 10, width: 10 };

        assert!(r.is_square());
    }

    #[test]
    #[should_panic]
    fn test_is_not_square() {
        let r = super::Rectangle { height: 3, width: 4 };
        assert!(r.is_square());
    }
}
