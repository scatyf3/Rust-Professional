/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T : std::fmt::Display>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: std::fmt::Display> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            // items : Vec::new(), 不可以，否则左右关系错误...
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        //append value to the bottom of array(heap)
        self.count+=1;
        let mut cur_idx = self.count;
        self.items.push(value);
        //向上移动
        println!("len: {}", self.items.len());
        println!("cur idx: {}", cur_idx);
        while (self.comparator)(&self.items[cur_idx], &self.items[self.parent_idx(cur_idx)]){
            println!("add swap betwen {} and {}",self.items.len(),cur_idx);
            //println!("{}", );
            let parent_index = self.parent_idx(cur_idx); 
            if parent_index==0{
                break;
            }
            self.items.swap(cur_idx, parent_index); 
            cur_idx = parent_index;
        }

        //for elem in &self.items{
        //    println!("{}", elem);
        //}
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        //功能是找到更该被迭代器找到的下一个index
        println!("{}",self.count);
        if self.left_child_idx(idx)<=self.count && self.right_child_idx(idx)<=self.count{
            println!("both in bound");
            if (self.comparator) (&self.items[self.left_child_idx(idx)], &self.items[self.right_child_idx(idx)]){
                println!("{}, {}",self.items[self.left_child_idx(idx)],self.items[self.right_child_idx(idx)]);
                return self.left_child_idx(idx);
            }else {
                println!("{}, {}",self.items[self.left_child_idx(idx)],self.items[self.right_child_idx(idx)]);
                //println!("{}",self.right_child_idx(idx));
                return self.right_child_idx(idx);
            }
        }else if self.left_child_idx(idx)>self.count && self.right_child_idx(idx)<=self.count{
            println!("left not in bound");
            return self.right_child_idx(idx);
        }else if self.left_child_idx(idx)<=self.count && self.right_child_idx(idx)>self.count{
            println!("right not in bound");
            return self.left_child_idx(idx);
        }else{
            return idx;
        }
    }

    fn heapify_down(&mut self){
        // 拿走根之后，重新排布...
        
        // 第一步居然是把最后一个元素拿到根
        println!("prev vector");
        for elem in &self.items{
            println!("{}", elem);
        }
        println!("count before heapify {}",self.count);
        self.count-=1;
        println!("count after heapify {}",self.count);
        self.items.swap(self.count, 1);
        println!("vector after swap");
        for elem in &self.items{
            println!("{}", elem);
        }
        let mut cur_idx = 1;
        println!("next index to swap {}", self.smallest_child_idx(cur_idx));
        
        // 根元素向下
        while self.smallest_child_idx(cur_idx)!=cur_idx{
            let children_index = self.smallest_child_idx(cur_idx);
            println!("swap between {} and {}",cur_idx,children_index);
            self.items.swap(cur_idx, children_index); 
            cur_idx = children_index;
            println!("vector after swap");
            for elem in &self.items{
                println!("{}", elem);
            }
        }        
    }
}

impl<T:std::fmt::Display> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T:std::fmt::Display> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // TODO 应该remove吗🤔
        if self.count==0{
            return None;
        }
        let res = self.items.remove(1);
        println!("next {}", res);
        self.heapify_down();
        return Some(res);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::fmt::Display>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::fmt::Display>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}