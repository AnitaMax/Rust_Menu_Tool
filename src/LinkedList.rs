pub struct Node<T>{
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T>{
    pub head: Option<Box<Node<T>>>,
    pub len: usize,
}

impl<T> Node<T>{
    fn new(data: T) -> Node<T>{
        Node{
            data: data,
            next: None,
        }
    }

    pub fn get_data(&self) -> &T{
        &self.data
    }
}
    
impl<T> LinkedList<T>{
    pub fn new() -> LinkedList<T>{
        LinkedList{
            head: None,
            len: 0,
        }
    }
    pub fn push(&mut self, data: T){
        let new_node = Box::new(Node::new(data));
        if self.len == 0 {
            self.head = Some(new_node);
            self.len +=1;
        }else{
            let mut cur=self.head.as_mut().unwrap();
            while !cur.next .is_none(){
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(new_node);
            self.len+=1;
        }
    }

    pub fn find<F>(&self, judge:F) -> Option<&Node<T>> where F: Fn(&T) -> bool{
        let mut cur=self.head.as_ref();
        while !cur .is_none() {
            if judge(&cur.unwrap().data){
                return Some(cur.unwrap().as_ref());
            }
            cur = cur.unwrap().next.as_ref();
        }
        None
    }

    pub fn for_all<F>(&self, f:F) where F: Fn(&T){
        let mut cur=self.head.as_ref();
        while !cur.is_none() {
            f(&cur.unwrap().data);
            cur = cur.unwrap().next.as_ref();
        }
    }

}
    