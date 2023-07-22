pub struct Item {
    pub character: char,
    pub priority: usize,
}

impl Item {
    pub fn from_char(c: char) -> Self {
        Item {
            character: c,
            priority: Self::priority(c),
        }
    }

    fn priority(c: char) -> usize {
        if !c.is_alphabetic() {
            panic!("invalid input");
        }

        if c.is_uppercase() {
            return c as usize - 38;
        } else {
            return c as usize - 96;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Item;

    #[test]
    fn test() {
        assert_eq!(
            (1..=26).collect::<Vec<usize>>(),
            ('a'..='z')
                .map(|c| Item::from_char(c).priority)
                .collect::<Vec<usize>>()
        );

        assert_eq!(
            (27..=52).collect::<Vec<usize>>(),
            ('A'..='Z')
                .map(|c| Item::from_char(c).priority)
                .collect::<Vec<usize>>()
        );
    }
}
