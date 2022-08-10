#![allow(dead_code, path_statements)]

#[cfg(fail)]
fn upper(string: &str) -> &str {
    let upper = string.to_uppercase();
    return &upper;
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn test_min_good() {
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let m = min(a + "ccc", b);
    assert_eq!(m, "aaaccc");
}

#[test]
#[cfg(fail)]
fn test_min_bad() {
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let m = min(&(a + "ccc"), &b);
    assert_eq!(m, "aaaccc");
}

#[test]
fn test_min_fixed() {
    let a = "aaa".to_string();
    let b = "bbb".to_string();
    let a_plus_ccc = a + "ccc";
    let m = min(&a_plus_ccc, &b);
    assert_eq!(m, "aaaccc");
}

struct Character {
    name: String,
    age: u32,
}

struct Thing;

struct Id(u64);

struct Point(f64, f64);

#[test]
fn test_construct_structs() {
    let c = Character {
        name: "Rusty".to_string(),
        age: 8,
    };

    let t = Thing;

    let i = Id(1234);

    let p = Point(1.5, 5.25);

    let _ = c.name;
    let _ = c.age;
    let _ = t;
    let _ = i.0;
    let _ = p.0 + p.1;
}

enum Decision {
    Undecided,
    Approved,
    Rejected,
}

enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

#[test]
fn test_construct_enums() {
    WebEvent::PageLoad;
    WebEvent::KeyPress('j');
    WebEvent::Click { x: 150, y: 230 };
}

fn hypotenuse(a: f64, b: f64) -> f64 {
    let a_squared = a * a;
    let b_squared = b.powi(2);
    let sum = a_squared + b_squared;
    sum.sqrt()
}

#[test]
fn test_hypotenuse() {
    assert_eq!(hypotenuse(3.0, 4.0), 5.0);
    assert_eq!(hypotenuse(0.0, 0.0), 0.0);
}

#[allow(clippy::needless_return)]
fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for current in 2..=n {
        result *= current;
    }
    return result;
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(4), 24);
}

#[test]
#[should_panic(expected = "overflow")]
fn test_factorial_error() {
    factorial(3928);
}

#[cfg(highlight)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug, PartialEq)]
enum Gift {
    Phone,
    Watch,
    Cash(u64),
}

fn holiday_gift(year: i32) -> Option<Gift> {
    match year {
        1998..=2007 => Some(Gift::Cash(1000)),
        2008..=2013 => Some(Gift::Phone),
        2014 => Some(Gift::Watch),
        2015 => Some(Gift::Phone),
        2016..=2019 => None,
        _ => None,
    }
}

#[test]
fn test_holiday_gift() {
    assert_eq!(holiday_gift(1998), Some(Gift::Cash(1000)));
    assert_eq!(holiday_gift(2007), Some(Gift::Cash(1000)));
    assert_eq!(holiday_gift(2008), Some(Gift::Phone));
    assert_eq!(holiday_gift(2013), Some(Gift::Phone));
    assert_eq!(holiday_gift(2014), Some(Gift::Watch));
    assert_eq!(holiday_gift(2015), Some(Gift::Phone));
    assert_eq!(holiday_gift(2016), None);
    assert_eq!(holiday_gift(2017), None);
    assert_eq!(holiday_gift(2100), None);
}

fn regift(gift: Gift, who: &str) {
    println!("Look {}! I got you a {:?}", who, gift);
}

struct Memegen;

impl Memegen {
    fn rant() {
        /* Insert dank memes here. */
    }
}

#[test]
#[cfg(fail)]
fn use_holiday_gift_2010() {
    let gift = holiday_gift(2010);
    regift(gift, "Uncle");
}

#[test]
fn use_holiday_gift_2012() {
    regift(holiday_gift(2012).unwrap(), "Ma");
}

#[test]
#[should_panic(expected = "Option::unwrap")]
fn use_holiday_gift_2016() {
    regift(holiday_gift(2016).unwrap(), "Dad");
}

#[test]
#[should_panic(expected = "awesome gifts")]
fn use_holiday_gift_2017() {
    regift(
        holiday_gift(2017).expect("Google gives awesome gifts!"),
        "Dad",
    );
}

#[test]
fn use_holiday_gift_2018() {
    match holiday_gift(2018) {
        Some(gift) => regift(gift, "Sis"),
        None => Memegen::rant(),
    }
}

fn factorial2(n: u64) -> Option<u64> {
    match n {
        0 => Some(1),
        _ => match factorial2(n - 1) {
            Some(prev) => prev.checked_mul(n),
            None => None,
        },
    }
}

fn factorial2_2(n: u64) -> Option<u64> {
    match n {
        0 => Some(1),
        _ => factorial2_2(n - 1)?.checked_mul(n),
    }
}

fn factorial3(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    } else {
        factorial3(n - 1).and_then(|prev| prev.checked_mul(n))
    }
}

#[test]
fn test_factorial2() {
    assert_eq!(factorial2(0), Some(1));
    assert_eq!(factorial2(1), Some(1));
    assert_eq!(factorial2(4), Some(24));
    assert_eq!(factorial2(3928), None);
}

#[test]
fn test_factorial2_2() {
    assert_eq!(factorial2_2(0), Some(1));
    assert_eq!(factorial2_2(1), Some(1));
    assert_eq!(factorial2_2(4), Some(24));
    assert_eq!(factorial2_2(3928), None);
}

#[test]
fn test_factorial3() {
    assert_eq!(factorial3(0), Some(1));
    assert_eq!(factorial3(1), Some(1));
    assert_eq!(factorial3(4), Some(24));
    assert_eq!(factorial3(3928), None);
}

impl std::fmt::Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Character({} is {})", self.name, self.age)
    }
}

#[test]
fn format_person() {
    let person = Character {
        name: "Pikachu".to_string(),
        age: 6,
    };
    assert_eq!(format!("{:?}", person), "Character(Pikachu is 6)");
}

#[cfg(fail)]
fn f1(_t: Thing) -> &str {
    unreachable!()
}

fn f2(_t: Thing) -> &'static str {
    unreachable!()
}

fn f3(_t: &Thing) -> &str {
    unreachable!()
}

#[cfg(fail)]
fn f4(_thing1: &Thing, _thing2: &Thing) -> &str {
    unreachable!()
}

fn f5<'a>(_thing1: &Thing, _thing2: &'a Thing) -> &'a str {
    unreachable!()
}

fn f6<'a>(_thing1: &'a Thing, _thing2: &'a Thing) -> &'a str {
    unreachable!()
}