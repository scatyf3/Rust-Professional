/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + Clone + std::fmt::Debug> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + Clone + std::fmt::Debug> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd+ Clone + std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    // 感觉没有正确get到它给的接口的设计意图TT
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    
    // 不该这样，一个列表弄完之后，另外一个列表慢慢在loop中挪过来就行
    pub fn remove_front(&mut self) {
        unsafe {
            // 确保 start 是 Some，并且可以安全地获取下一个节点
            if let Some(start_ptr) = self.start {
                let next_ptr = (*start_ptr.as_ptr()).next; // 获取下一个节点的指针
                if let Some(next_node) = next_ptr {
                    // 更新 start 为下一个节点的指针
                    // new_unchecked创建一个not null
                    self.start = Some(NonNull::new_unchecked(next_node.as_ptr()));
                } else {
                    // 如果没有下一个节点，设置 start 为 None
                    self.start = None;
                }
            }
        }
    }


	pub fn merge (mut list_a: LinkedList<T>,mut list_b: LinkedList<T>) -> Self
	{
		let mut res = Self {
            length: 0,
            start: None,
            end: None,
        };

        loop {
            match (list_a.get(0), list_b.get(0)) {
                (Some(value_a), Some(value_b)) => {
                    if value_a < value_b {
                        println!("a{:?}", value_a);
                        res.add((*value_a).clone());
                        list_a.remove_front();
                    } else {
                        println!("b {:?}", value_b);
                        res.add((*value_b).clone());
                        list_b.remove_front();
                    }
                }
                (Some(value_a), None) => {
                    println!("case a");
                    // res.add((*value_a).clone());
                    // 将 list_a 剩余部分连接到 res
                    if let Some(end_ptr) = res.end {
                        unsafe {
                            // TODO 从当前的节点连接
                            (*end_ptr.as_ptr()).next = list_a.start;
                        }
                    } 
                    break;
                }
                (None, Some(value_b)) => {
                    println!("case b");
                    // res.add((*value_b).clone());
                    // 将 list_b 剩余部分连接到 res
                    if let Some(end_ptr) = res.end {
                        unsafe {
                            (*end_ptr.as_ptr()).next = list_b.start; // 连接剩余的节点
                        }
                    } 
                    break;
                }
                (None, None) => break, // 都为空，退出循环
            }
        }
        res
        
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}