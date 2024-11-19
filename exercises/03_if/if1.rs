use std::cmp::Ordering;

fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    /* if a > b {
        a
    }
    else if a < b {
        b
    }
    else {
        a
    } */
   // Officially recommended solution
   let c = a.abs();
   match a.cmp(&b) {
       Ordering::Greater => a,
       Ordering::Less => b,
       Ordering::Equal => c
   }
}

fn main() {
    // You can optionally experiment here.
    let num_a = 1;
    let num_b = 1;
    println!("From {num_a} and {num_b}, {} is bigger", bigger(num_a, num_b));
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
