pub trait Queue<T> {
    fn add(&mut self, x: T);
    fn remove(&mut self) -> T;
}

pub trait Deque<T> {
    fn add_first(&mut self, x: T);
    fn remove_first(&mut self) -> T;
    fn add_last(&mut self, x: T);
    fn remove_last(&mut self) -> T;
}

pub trait List<T> {
    fn size(&self) -> usize;
    fn get(&self, i: usize) -> &T;
    fn set(&mut self, i: usize, x: T);
    fn add(&mut self, i: usize, x: T);
    fn remove(&mut self, i: usize) -> T;
}

// Unordered Set
pub trait USet<T> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    fn find(&self, x: T) -> Option<&T>;
}

pub type Pair<K, V> = (K, V);

// compare(x, y)
// <0 if x < y
// =0 if x = y
// >0 if x > y

// Sorted Set
pub trait SSet<T: Ord> {
    fn size(&self) -> usize;
    fn add(&mut self, x: T) -> bool;
    fn remove(&mut self, x: T) -> Option<T>;
    // y >= x を満たす最小のyを返す
    fn find(&self, x: T) -> Option<&T>;
}
