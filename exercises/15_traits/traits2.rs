trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement the trait `AppendBar` for a vector of strings.
// `append_bar` should push the string "Bar" into the vector.
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string());
        self
    }
}

fn main() {
    let v = vec!["Foo".to_string()];

    println!("Vec 1: {:?}", v);

    let v2: Vec<String> = v.append_bar();

    println!("Vec 2: {:?}", v2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
