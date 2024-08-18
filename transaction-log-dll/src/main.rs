use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

pub struct List<T: Display> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T: Display> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}


impl<T: Display> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }

    fn display(&self){
        println!("{}", &self.elem);
        if let Some(node) = &self.next{
            let node_ref = node.borrow();
            node_ref.display()
        }
    }
}

impl<T: Display> List<T> {
    fn new() -> Self {
        List { head: None, tail: None }
    }

    fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn display(&self){
        match &self.head{
            Some(node) => {
                let node_ref = node.borrow();
                node_ref.display()
            },

            None => {
                println!("No transactions")
            }
        }
    }
}

enum TransactionKind{
    Credit,
    Debit
}

struct Transaction{
    kind: TransactionKind,
    amount: u64,
}

impl Display for Transaction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind{
            TransactionKind::Credit => write!(f, "Credited {}", self.amount),
            TransactionKind::Debit => write!(f, "Debited {}", self.amount)
        }
    }
}

fn main() {
    let mut ll = List::new();
    ll.push_back(Transaction { kind: TransactionKind::Credit, amount: 500 });
    ll.push_back(Transaction { kind: TransactionKind::Debit, amount:40 });
    ll.push_back(Transaction { kind: TransactionKind::Debit, amount:80 });
    ll.pop_back();
    ll.push_front(Transaction { kind: TransactionKind::Debit, amount:80 });
    ll.push_front(Transaction { kind: TransactionKind::Debit, amount:90 });
    ll.pop_front();
    ll.display()
}
