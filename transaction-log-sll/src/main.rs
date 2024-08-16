use std::fmt::Display;


#[derive(Debug)]
struct Node<T: Display> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T: Display + Clone> Node<T>{
    fn new(value: T) -> Box<Node<T>>{
        Box::new(Node{
            value,
            next: None
        })
    }

    fn push(&mut self, value: T){
        if self.next.is_none(){
            self.next = Some(Node::new(value));
            return;
        }

        let mut temp = self.next.take().unwrap();
        temp.push(value);
        self.next = Some(temp)
    }

    fn pop(&mut self) -> T{
        if self.next.is_none(){
            return self.value.clone()
        }

        let mut next = self.next.take().unwrap();
        if next.next.is_none(){
            return next.value
        } else {
            let ret = next.pop();
            self.next = Some(next);
            return ret
        }
    }

    fn display(&self){
        print!("{} ", self.value);
        if let Some(next) = &self.next{
            next.display()
        }
    }
}

#[derive(Debug)]
struct LL<T: Display + Clone>{
    start: Option<Box<Node<T>>>
}

impl<T: Display + Clone> LL<T>{
    fn new() -> LL<T>{
        return LL{start: None};
    }

    fn push(&mut self, value: T){
        if self.start.is_none(){
            self.start = Some(Node::new(value))
        } else {
            let mut temp = self.start.take().unwrap();
            temp.push(value);
            self.start = Some(temp)
        }
    }

    fn pop(&mut self) -> Option<T>{
        if self.start.is_none(){
            return None
        } else {
            let mut temp = self.start.take().unwrap();
            if temp.next.is_none(){
                return Some(temp.value.clone())
            } else {
                let ret = temp.pop();
                self.start = Some(temp);
                return Some(ret.clone());
            }
        }
    }

    fn push_in_front(&mut self, value: T){
        if self.start.is_none(){
            self.push(value)
        } else {
            let temp = self.start.take().unwrap();
            let mut new = Node::new(value);
            new.next = Some(temp);

            self.start = Some(new)
        }
    }

    fn pop_from_front(&mut self) -> Option<T>{
        if self.start.is_none(){
            return None 
        } else {
            let mut curr = self.start.take().unwrap();
            if curr.next.is_some(){
                let temp = curr.next.take();
                self.start = temp;
            }

            return Some(curr.value);
        }
    }

    fn display(&self){
        if self.start.is_none(){
            println!();
            return
        }

        if let Some(start) = &self.start{
            start.display()
        }
        println!();
    }
}

#[derive(Clone)]
enum TransactionKind{
    Credit,
    Debit
}

#[derive(Clone)]
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
    let mut ll = LL::new();
    ll.push(Transaction { kind: TransactionKind::Credit, amount: 500 });
    ll.push(Transaction { kind: TransactionKind::Debit, amount:40 });
    ll.push(Transaction { kind: TransactionKind::Debit, amount:80 });
    ll.pop();
    ll.push_in_front(Transaction { kind: TransactionKind::Debit, amount:80 });
    ll.push_in_front(Transaction { kind: TransactionKind::Debit, amount:90 });
    ll.pop_from_front();
    ll.display()
}
