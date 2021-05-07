use short_url_api::base62;
use std::collections::HashMap;

#[test]
fn test_encode() {
    let mut cases = HashMap::new();

    cases.insert(0, "0");
    cases.insert(22, "M");
    cases.insert(61, "z");
    cases.insert(3843, "zz");
    cases.insert(3718, "xy");

    for (input, want) in cases {
        let got = base62::encode(input as u64);
        if got.as_str() != want {
            panic!(
                "unexpected result with input({}): want {} but got {}",
                input, want, got
            )
        }
    }
}

#[test]
fn test_decode() {
    let mut cases: HashMap<&str, u64> = HashMap::new();

    cases.insert("0", 0);
    cases.insert("z", 61);
    cases.insert("zz", 3843);
    cases.insert("zz0", 238266);
    cases.insert("xy", 3718);

    for (input, want) in cases {
        let got = base62::decode(input).unwrap();
        if got != want {
            panic!(
                "unexpected result with input({}): want {} but got {}",
                input, want, got
            )
        }
    }
}
