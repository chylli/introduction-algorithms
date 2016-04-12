//! Algorithms of chapter2
///# Example
///```
///use algorithm::chapter2::Chapter2;
///let mut list = vec![5,2,1,3,4];
///list.insertion_sort();
///assert_eq!(list, vec![1,2,3,4,5]);
///let mut list = vec![5,2,1,4,3];
///list.insertion_sort();
///assert_eq!(list, vec![1,2,3,4,5]);
///```
#[allow(dead_code)]
pub trait Chapter2<T: Copy + PartialOrd> {
    fn insertion_sort(&mut self);
    fn merge_sort(&mut self);
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

    fn merge_sort(&mut self){
        //rust compiler has problem here, so we must get length before call merge_sort_helper
        let len = self.len();
        merge_sort_helper(self, 0, len);
    }
}

fn merge_sort_helper<T: Copy + PartialOrd>(va: &mut Vec<T>, p: usize, r: usize) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort_helper(va, p, q);
        merge_sort_helper(va, q+1, r);
        merge(va, p, q, r);
    }

}

#[test]
fn test_merge(){
    let mut a = vec![1,3,2,4];
    merge(&mut a, 0,1,3);
    assert_eq!(a,vec![1,2,3,4]);
    a = vec![1,3,5,7,2,4];
    merge(&mut a, 0,3,5);
    assert_eq!(a,vec![1,2,3,4,5,7]);
    a = vec![2,4,1,3,5,7];
    merge(&mut a, 0,1,5);
    assert_eq!(a,vec![1,2,3,4,5,7]);
}

fn merge<T: Copy + PartialOrd>(va: &mut Vec<T>, p: usize, q: usize, r: usize){
    let n1 = q - p + 1;
    let n2 = r - q;
    let mut vl: Vec<T> = Vec::with_capacity(n1);
    let mut vr: Vec<T> = Vec::with_capacity(n2);
    for i in 0..n1 {
        vl.push(va[p + i]);
    }
    for i in 0..n2 {
        vr.push(va[q + i + 1]);
    }
    let mut i: usize = 0;
    let mut j: usize = 0;
    for k in p..r + 1 {
        if i >= n1 {
            va[k] = vr[j];
            j = j + 1;
        }

        else if j >= n2 {
            va[k] = vl[i];
            i = i + 1;
        }

        else if vl[i] <= vr[j] {
            va[k] = vl[i];
            i = i + 1;
        }
        else {
            va[k] = vr[j];
            j = j + 1;
        }
    }
}
