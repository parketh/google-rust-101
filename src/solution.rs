#![allow(dead_code)]
// Rust 101 Codelab.
//
// In the slides you watched me create a very functional style linked list.
// This codelab will work through making a more imperative-friendly and overall
// better designed version.
//
// In addition to the list itself we will make it more usable by making it
// implement the Iterator trait. This will make our list very ergonomic and give
// us access to a lot of pre-written functionality.
//
// Lastly we will use our list type to write some simple functions to complete
// the codelab.
//
// To do the codelab simply read the comments and replace the unimplemented!()
// calls with your own code. While you could search for all instances of unimplemented!()
// I recommend that you read all the code in order. Note that you are not
// required to understand the #[test] functions, but I still recommend giving
// them a quick read. If you have any questions I'll be glad to help.
//
// You can run the tests with `cargo test codelab`.

/// Linked List.
///
/// This is the linked list structure that we will be using throughout the
/// codelab. Unlike the slides it is broken into two parts List and Node.
///
/// List can be thought of as a nullable pointer. It may point to a Node, or it
/// may not. A "null" head "pointer" will indicate the empty list.
///
/// Node is a value in the list, as well as the next "pointer". This is like the
/// cons cell in lisp.
///
/// Take a moment to be sure you understand the type definitions, they will be
/// crucial to implementing the functions.
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: List<T>,
}

impl<T> List<T> {
    /// Creates an empty list.
    ///
    /// This "static" function constructs a new list with no elements.
    pub fn empty() -> Self {
        // unimplemented!("codelab::List::empty()");
        List { head: None }
    }

    /// Returns true iff the list is empty.
    pub fn is_empty(&self) -> bool {
        // unimplemented!("codelab::List::is_empty(&self)");
        self.head.is_none()
    }

    /// Adds a new element to the front of the list.
    pub fn push(&mut self, element: T) {
        // Tip: Rust doesn't allow invalid values, so it might be difficult to move
        // the tail from the current head to the newly added head element and Node.
        // However the `Option<T>::take(&mut self) -> Option<T>` function allows you
        // to take the Option and leave None in it's place. This allows you to use it
        // elsewhere, you can then replace the temporary None with the updated value.
        //
        // Hint: This will require allocating a new Node in the list.
        // unimplemented!("codelab::List::push(&self, T)");

        let previous_head = self.head.take();

        let new_head = Node {
            val: element,
            next: List {
                head: previous_head,
            },
        };

        self.head = Some(Box::new(new_head));
    }

    /// Removes the first element from the list and return it.
    pub fn pop(&mut self) -> Option<T> {
        // Tip: This function returns None if the list is empty, so you will need to
        // handle the two cases.
        //
        // This is a hard one, Feel free to ask for help.
        // I recommend remembering the following things that are required.
        // - Take the previous head and handle the Some and None cases.
        // - Update self.head by taking it from the tail.
        // - Return the value that was in the first node.
        // unimplemented!("codelab::List::pop(&mut self)");
        let previous_head = self.head.take();
        match previous_head {
            Some(box_tail) => {
                let tail = *box_tail;
                self.head = tail.next.head;
                Some(tail.val)
            }
            None => None,
        }
    }
}

#[test]
fn test_01_empty() {
    // Just having a constructor isn't very useful so we will just ensure it
    // doesn't panic.
    List::<i32>::empty();
    List::<String>::empty();
}

#[test]
fn test_02_is_empty() {
    // Note: We don't have push yet, so we will implement it ourselves.

    let list = List::empty();
    assert!(list.is_empty());

    let list = List {
        head: Some(Box::new(Node { val: 1, next: list })),
    };
    assert!(!list.is_empty());

    let list = List {
        head: Some(Box::new(Node { val: 2, next: list })),
    };
    assert!(!list.is_empty());
}

#[test]
fn test_03_push() {
    // Again, having a constructor and push doesn't give us much. So just ensure
    // it doesn't panic.
    let mut list = List::empty();
    assert!(list.is_empty());

    list.push(1);
    assert!(!list.is_empty());
    list.push(2);

    assert!(!list.is_empty());
    list.push(3);

    let mut list = List::empty();
    assert!(list.is_empty());

    list.push("foo");
    assert!(!list.is_empty());

    list.push("bar");
    assert!(!list.is_empty());

    list.push("baz");
    assert!(!list.is_empty());
}

#[test]
fn test_04_pop() {
    let mut list = List::empty();
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));

    let mut list = List::empty();
    list.push("one");
    list.push("two");
    list.push("three");

    assert_eq!(list.pop(), Some("three"));
    assert_eq!(list.pop(), Some("two"));
    assert_eq!(list.pop(), Some("one"));
}

/// Draining iterator for owned list.
///
/// This will allow someone to take the `T`s out of the Iterator even if `T` can't be
/// copied.
///
/// This could be implemented directly on `List` since it is a recursive structure
/// but I think it is cleaner to keep it as a separate structure.
pub struct DrainingIterator<T>(List<T>);

impl<T> Iterator for DrainingIterator<T> {
    /// The type of element we will yield.
    type Item = T;

    /// Returns the next item from the iterator.
    ///
    /// This function is the only method of iterator and returns `Some(Item)`
    /// each time it is called until the iterator is finished. Then it returns
    /// `None`.
    fn next(&mut self) -> Option<Self::Item> {
        // Hint: This iterator will remove items as it iterates.
        // Hint: You have already written a method that matches this requirement.
        // unimplemented!("codelab::DrainingIterator::next(&mut self)");
        self.0.pop()
    }
}

// Unless the signatures are changed our list must work with non-copyable data,
// but just to be sure I made this part of the tests :)
#[derive(Debug, PartialEq)]
struct NonCopyable(u32);

#[test]
fn test_05_draining_iter_next() {
    let mut list = List::empty();
    list.push(NonCopyable(1));
    list.push(NonCopyable(2));
    list.push(NonCopyable(3));

    let mut iter = DrainingIterator(list);
    assert_eq!(iter.next(), Some(NonCopyable(3)));
    assert_eq!(iter.next(), Some(NonCopyable(2)));
    assert_eq!(iter.next(), Some(NonCopyable(1)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

/// Allow our list to be automatically converted to an Iterator.
///
/// This makes our type much more ergonomic to use, as well as allows better
/// compatibility with other code which uses this feature.
impl<T> IntoIterator for List<T> {
    /// The type of the Iterator we will return.
    type IntoIter = DrainingIterator<T>;
    /// The type of element that our Iterator will yield.
    type Item = T;

    /// Returns a DrainingIterator for this list.
    //
    // I have implemented this method for you. There will be a second impl of
    // IntoIterator that you will implement on your own.
    fn into_iter(self) -> Self::IntoIter {
        DrainingIterator(self)
    }
}

#[test]
fn test_06_into_draining_iter() {
    let mut list = List::empty();
    list.push(NonCopyable(1));
    list.push(NonCopyable(2));
    list.push(NonCopyable(3));

    // We can now use a for loop on our type!
    let mut items = Vec::new();
    for item in list {
        items.push(item);
    }
    assert_eq!(items, vec![NonCopyable(3), NonCopyable(2), NonCopyable(1)]);
}

/// Reference iterator for reference to list.
///
/// This will allow someone to iterate over their List and get a reference to
/// each element. This is useful because it allows you to iterate over a list
/// multiple times! What a novel feature!
//
// This type is slightly more complicated because we can't modify the original
// list. Instead we need to keep a reference to our current location in the
// list. To do this we also need to ensure that the Iterator doesn't live
// longer then the list we are iterating over, otherwise we would have dangling
// references.
pub struct ReferenceIter<'a, T: 'a> {
    current: &'a List<T>,
}

/// Reference iterator for reference to list list.
///
/// Note: The 'a parameter is to indicate that the reference we return can't be
// used for longer then the Iterator exists.
impl<'a, T> Iterator for ReferenceIter<'a, T> {
    /// The type of element we will yield.
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // unimplemented!("Iterator::<&List<T>>::next(&mut self)");
        match &self.current.head {
            Some(tail) => {
                self.current = &tail.next;
                Some(&tail.val)
            }
            None => None,
        }
    }
}

#[test]
fn test_07_reference_iter_next() {
    let mut list = List::empty();
    list.push(NonCopyable(1));
    list.push(NonCopyable(2));
    list.push(NonCopyable(3));

    let mut iter = ReferenceIter { current: &list };
    assert_eq!(iter.next(), Some(&NonCopyable(3)));
    assert_eq!(iter.next(), Some(&NonCopyable(2)));
    assert_eq!(iter.next(), Some(&NonCopyable(1)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

/// Into ReferenceIter for a reference to a List.
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ReferenceIter<'a, T>;

    /// Returns a ReferenceIter for this list.
    fn into_iter(self) -> Self::IntoIter {
        // Hint: Look at the ReferenceIter definition to see what is required to
        // construct one.
        // Hint: Remember that `self: &List<T>`
        // Hint: Look at the previous IntoIterator impl that I provided for you.
        // unimplemented!("IntoIterator::<&List<T>>::into_iter(self)");
        ReferenceIter { current: self }
    }
}

#[test]
fn test_08_into_reference_iter() {
    let mut list = List::empty();
    list.push(3);
    list.push(2);
    list.push(1);

    let mut results = Vec::new();
    for n in &list {
        results.push(*n);
    }
    for n in &list {
        results.push(*n * 2);
    }
    assert_eq!(results, vec![1, 2, 3, 2, 4, 6]);
}

#[derive(Debug, PartialEq)]
struct Score {
    home_points: u32,
    away_points: u32,
}

#[derive(Debug, PartialEq)]
enum Team {
    Draw,
    Home,
    Away,
}

/// Returns the total score for the home and away teams across games.
///
/// Given a List<Score> representing the scores for a sequence of games, produce
/// a Score representing the total of those scores.
fn total_score(results: &List<Score>) -> Score {
    // Hint: You can use a for loop to iterate over the results.
    // https://doc.rust-lang.org/1.1.0/book/for-loops.html
    //
    // Hint: Or you could call `results.into_iter()` and use any of the methods
    // provided by the iterator trait.
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html
    // unimplemented!("codelab::total_score()");
    results.into_iter().fold(
        Score {
            home_points: 0,
            away_points: 0,
        },
        |Score {
             home_points,
             away_points,
         },
         result| Score {
            home_points: home_points + result.home_points,
            away_points: away_points + result.away_points,
        },
    )
}

#[test]
fn test_09_total_score() {
    let mut scores = List::empty();
    assert_eq!(
        total_score(&scores),
        Score {
            home_points: 0,
            away_points: 0
        }
    );

    scores.push(Score {
        home_points: 0,
        away_points: 0,
    });
    assert_eq!(
        total_score(&scores),
        Score {
            home_points: 0,
            away_points: 0
        }
    );

    scores.push(Score {
        home_points: 3,
        away_points: 0,
    });
    assert_eq!(
        total_score(&scores),
        Score {
            home_points: 3,
            away_points: 0
        }
    );

    scores.push(Score {
        home_points: 0,
        away_points: 3,
    });
    assert_eq!(
        total_score(&scores),
        Score {
            home_points: 3,
            away_points: 3
        }
    );

    scores.push(Score {
        home_points: 2,
        away_points: 1,
    });
    assert_eq!(
        total_score(&scores),
        Score {
            home_points: 5,
            away_points: 4
        }
    );
}

/// Returns the Team with the highest score over all the games in the list.
fn highest_total_score(results: &List<Score>) -> Team {
    // Hint: total_score() might be useful :)
    // unimplemented!("codelab::highest_total_score()");
    let Score {
        home_points,
        away_points,
    } = total_score(results);

    match home_points.cmp(&away_points) {
        std::cmp::Ordering::Greater => Team::Home,
        std::cmp::Ordering::Equal => Team::Draw,
        std::cmp::Ordering::Less => Team::Away,
    }
}

#[test]
fn test_10_highest_total_score() {
    let mut scores = List::empty();
    assert_eq!(highest_total_score(&scores), Team::Draw);

    scores.push(Score {
        home_points: 0,
        away_points: 0,
    });
    assert_eq!(highest_total_score(&scores), Team::Draw);

    scores.push(Score {
        home_points: 3,
        away_points: 0,
    });
    assert_eq!(highest_total_score(&scores), Team::Home);

    scores.push(Score {
        home_points: 0,
        away_points: 3,
    });
    assert_eq!(highest_total_score(&scores), Team::Draw);

    scores.push(Score {
        home_points: 2,
        away_points: 1,
    });
    assert_eq!(highest_total_score(&scores), Team::Home);
}

/// Returns the games where the home team wins!
///
/// Given a List<Score> returns one with only the winning games.
fn games_worth_watching(results: List<Score>) -> List<Score> {
    // unimplemented!("codelab::games_worth_watching()");
    let mut good = List::empty();
    for r in results {
        if r.home_points > r.away_points {
            good.push(r);
        }
    }
    good
}

#[test]
fn test_11_games_worth_watching() {
    let mut scores = List::empty();
    scores.push(Score {
        home_points: 0,
        away_points: 0,
    });
    scores.push(Score {
        home_points: 0,
        away_points: 3,
    });
    let mut good = games_worth_watching(scores);
    assert_eq!(good.pop(), None);

    let mut scores = List::empty();
    scores.push(Score {
        home_points: 0,
        away_points: 0,
    });
    scores.push(Score {
        home_points: 5,
        away_points: 3,
    });
    scores.push(Score {
        home_points: 0,
        away_points: 3,
    });
    scores.push(Score {
        home_points: 1,
        away_points: 0,
    });
    let mut good: Vec<_> = games_worth_watching(scores).into_iter().collect();
    good.sort_by(|a, b| a.home_points.cmp(&b.home_points));
    assert_eq!(
        good,
        vec![
            Score {
                home_points: 1,
                away_points: 0
            },
            Score {
                home_points: 5,
                away_points: 3
            },
        ]
    );
}

// Congratulations! You have made it to the end of the codelab, I hope you enjoyed Rust 101.
// Remember to mark you attendance in Grow: http://go/iamhere