pub struct Node<T>{
    pub data: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct LinkedTable<T>{
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
    
impl<T> LinkedTable<T>{
    pub fn new() -> LinkedTable<T>{
        LinkedTable{
            head: None,
            len: 0,
        }
    }

    #[allow(dead_code)]
    pub fn get_len(&self) -> usize{
        self.len
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

    #[allow(dead_code)]
    pub fn pop(&mut self)->Option<Box<Node<T>>>{
        if self.len==0{
            return None;
        }
        else if  self.len==1 {
            self.len-=1;
            let res=self.head.take();
            return res;
        }
        let mut cur=self.head.as_mut().unwrap();
        while ! (!cur.next.is_none() && cur.next.as_mut().unwrap().next.is_none() ){
            cur=cur.next.as_mut().unwrap();
        }
        let res=cur.next.take();
        self.len-=1;
        return res;
        
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

    pub fn for_all<F>(&self, mut f:F) where F: FnMut(& T){
        let mut cur=self.head.as_ref();
        while !cur.is_none() {
            f(&cur.unwrap().data);
            cur = cur.unwrap().next.as_ref();
        }
    }

}

