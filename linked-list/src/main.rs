use std::{fmt::Display};


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

fn main() {
    let mut a = LL::new();
    a.push(5);
    println!("{:#?}", a);
    a.push(20);
    println!("{:#?}", a);
    a.push(47);
    println!("{:#?}", a);
    a.pop();
    println!("{:#?}", a);
    a.push_in_front(7);
    println!("{:#?}", a);
    a.pop_from_front();
    println!("{:#?}", a);
    a.display();
    a.pop_from_front();
    // println!("{:#?}", a);
    // a.pop_from_front();
    // println!("{:#?}", a);
    a.display();
}
