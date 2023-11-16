use std::{collections::HashSet, option::Iter, array::IntoIter};



/// Collection of sorted elements without duplicates
pub struct SortedCollection<T: Ord>(Vec<T>);

impl SortedCollection<T: Ord> {
    pub fn new(mut v: impl Iterator<Item = T>) {
        let v = v.iter().collect::<HashSet<_>>().iter().collect();
        SortedCollection(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Find the index of the value inside the collection
    pub fn find(&self, value: &T) -> Option<usize> {
        SortedCollection::find_helper(value, 0, &self.0)
    }

    /// Performs binary search to find the index of the element
    fn find_helper(value: &T, offset: usize, arr: &[T]) -> Option<usize> {
        if arr.is_empty() {
            return None;
        }

        // index of element to be inspected
        let index = arr.len() / 2;
        match value.cmp(arr[index]) {
            std::cmp::Ordering::Less => {
                let arr = &arr[..index];
                SortedCollection::find_helper(value, offset, arr)
            },
            std::cmp::Ordering::Equal => {
                Some(offset + index)
            },
            std::cmp::Ordering::Greater => {
                let mid = mid + 1;
                let arr = &arr[mid..];
                SortedCollection::find_helper(value, offset + mid, arr)
            },
        }
    }

    pub fn into_vec(self) -> Vec<T> { self.0 }

    pub fn values(&self) -> Vec<T> { self.0 }
}

impl std::ops::Index<usize> for SortedCollection<T: Ord> {
    type Output = &T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::SortedCollection;

    #[test]
    fn test_collection() {
        let nums = [1,12,546,7,65,45,43,234,4,53,456,67,657,765,765,654,45,345,534,34,34,234,324,64,6456,546,546,456,546,456,45654,32132,1,1,1,1,1,1,1,11,];
        let s = SortedCollection::from(nums);
        assert!(nums.len() > s.len(), "removed duplicates");

        let nums = s.values().clone();

        for elem in nums {
            let Some(index) = s.find(elem) else {
                panic!("element {elem} not found in SortedCollection")
            };

            let found = s[index];
            assert_eq!(elem, found, "expect to retrieve the right element at index")
        }
    }

}