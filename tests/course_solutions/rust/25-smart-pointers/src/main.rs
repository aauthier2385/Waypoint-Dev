use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

enum List {
    Item(i32, Box<List>),
    End,
}

fn sum(list: &List) -> i32 {
    match list {
        List::Item(value, rest) => value + sum(rest),
        List::End => 0,
    }
}

fn length(list: &List) -> usize {
    match list {
        List::Item(_, rest) => 1 + length(rest),
        List::End => 0,
    }
}

fn share(value: &Rc<String>, times: usize) -> Vec<Rc<String>> {
    (0..times).map(|_| Rc::clone(value)).collect()
}

fn new_counter() -> Rc<RefCell<u32>> {
    Rc::new(RefCell::new(0))
}

fn bump(counter: &Rc<RefCell<u32>>) {
    *counter.borrow_mut() += 1;
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
