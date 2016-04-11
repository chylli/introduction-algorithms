//! Algorithms of chapter2
///# Example
///```
///use algorithm::chapter2::Chapter2;
///let mut list = vec![5,2,1,3,4];
///list.insertion_sort();
///assert_eq!(list, vec![1,2,3,4,5]);
///```

pub trait Chapter2<T: Copy + PartialOrd> {
    fn insertion_sort(&mut self);
}

impl<T: Copy + PartialOrd> Chapter2<T> for Vec<T> {
    fn insertion_sort(&mut self){
        for j in 1..self.len(){
            let key = self[j];
            let mut i = (j - 1) as isize;
            
            while i >= 0 && self[i as usize] > key {
                self[(i+1) as usize] = self[i as usize];
                i = i - 1
            }
            self[(i + 1) as usize] = key
        }
    }
}
