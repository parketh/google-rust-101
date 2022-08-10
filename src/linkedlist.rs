#[derive(Clone)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

impl<T> List<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        List::Empty
    }

    #[allow(clippy::match_like_matches_macro)]
    pub fn is_empty(&self) -> bool {
        match self {
            List::Empty => true,
            _ => false,
        }
    }

    // ...

    pub fn push(self, element: T) -> Self {
        List::Cons(element, Box::new(self))
    }

    pub fn pop(self) -> Option<(Self, T)> {
        match self {
            List::Empty => None,
            List::Cons(element, tail) => Some((*tail, element)),
        }
    }

    pub fn head(&self) -> Option<&T> {
        match self {
            List::Empty => None,
            List::Cons(element, _) => Some(element),
        }
    }

    pub fn tail(&self) -> &Self {
        match self {
            List::Empty => self,
            List::Cons(_, tail) => tail,
        }
    }
}

#[test]
fn test_empty() {
    assert!(List::<i32>::new().is_empty());
}

#[test]
fn test_push() {
    let list = List::new().push(1).push(2).push(3);

    let (list, pop0) = list.pop().unwrap();
    let (list, pop1) = list.pop().unwrap();
    let (list, pop2) = list.pop().unwrap();

    assert_eq!(pop0, 3);
    assert_eq!(pop1, 2);
    assert_eq!(pop2, 1);
    assert!(list.is_empty());
}

#[cfg(test)]
fn takes_list(_list: List<u32>) {
    /* Useful work */
}

#[test]
#[cfg(fail)]
fn example_move() {
    let list = List::new();
    takes_list(list);
    assert!(list.is_empty());
}

#[test]
fn example_clone() {
    let list = List::new();
    takes_list(list.clone());
    assert!(list.is_empty());
}

#[test]
fn example_format() {
    println!("My new list {}!", List::new().push("foobar"))
}

impl<T: std::fmt::Display> std::fmt::Display for List<T> {
    #[allow(clippy::needless_return)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        let mut current = self;
        while let Some(element) = current.head() {
            current = current.tail();
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }
            write!(f, "{}", element)?;
        }
        return Ok(());
    }
}

#[test]
fn test_fmt() {
    let list = List::new();
    assert_eq!(format!("{}", list), "");
    let list = list.push(1);
    assert_eq!(format!("{}", list), "1");
    let list = list.push(2).push(3);
    assert_eq!(format!("{}", list), "3, 2, 1");

    let list = List::new();
    assert_eq!(format!("{}", list), "");
    let list = list.push("one");
    assert_eq!(format!("{}", list), "one");
    let list = list.push("two").push("three");
    assert_eq!(format!("{}", list), "three, two, one");
}