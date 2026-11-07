use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let list = List::Item(1, Box::new(List::Item(2, Box::new(List::Item(3, Box::new(List::End))))));
    println!("Sum: {}, length: {}", sum(&list), length(&list));

    let plan = Rc::new(String::from("Launch in June"));
    let team = share(&plan, 3);
    println!("{} owners share \"{}\"", Rc::strong_count(&plan), team[0]);

    let counter = new_counter();
    let also_counter = Rc::clone(&counter);
    bump(&counter);
    bump(&also_counter);
    println!("Count: {}", counter.borrow());
}

/// A list where each item holds a number and the rest of the list.
/// This doesn't build yet. Use Box to fix it.
enum List {
    Item(i32, List),
    End,
}

/// Adds up every number in the list.
fn sum(list: &List) -> i32 {
    todo!()
}

/// How many items are in the list.
fn length(list: &List) -> usize {
    todo!()
}

/// Gives back times new owners of the same value.
fn share(value: &Rc<String>, times: usize) -> Vec<Rc<String>> {
    todo!()
}

/// A counter, starting at 0, that several owners can share and change.
fn new_counter() -> Rc<RefCell<u32>> {
    todo!()
}

/// Adds 1 to the counter.
fn bump(counter: &Rc<RefCell<u32>>) {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn one_two_three() -> List {
        List::Item(1, Box::new(List::Item(2, Box::new(List::Item(3, Box::new(List::End))))))
    }

    #[test]
    fn summing() {
        assert_eq!(sum(&one_two_three()), 6);
        assert_eq!(sum(&List::End), 0);
    }

    #[test]
    fn counting_items() {
        assert_eq!(length(&one_two_three()), 3);
        assert_eq!(length(&List::End), 0);
    }

    #[test]
    fn sharing() {
        let plan = Rc::new(String::from("Launch in June"));
        let team = share(&plan, 3);
        assert_eq!(team.len(), 3);
        assert_eq!(Rc::strong_count(&plan), 4, "the original plus 3 shared owners");
        assert!(Rc::ptr_eq(&plan, &team[2]), "they all point to the same text, not copies");
        drop(team);
        assert_eq!(Rc::strong_count(&plan), 1, "when the team is dropped, only the original is left");
    }

    #[test]
    fn shared_counter() {
        let counter = new_counter();
        let other = Rc::clone(&counter);
        assert_eq!(*counter.borrow(), 0);
        bump(&counter);
        bump(&other);
        bump(&other);
        assert_eq!(*counter.borrow(), 3, "both handles change the same counter");
    }
}
