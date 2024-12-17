trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: &impl Licensed, software2: &impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    let soft1 = SomeSoftware;
    let soft2 = OtherSoftware;

    if compare_license_types(&soft1, &soft2) {
        println!("SomeSoftware and OtherSoftware are have the same license ({:?})", soft1.licensing_info());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(&SomeSoftware, &OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(&OtherSoftware, &SomeSoftware));
    }
}
