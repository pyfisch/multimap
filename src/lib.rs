use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::hash_map::Keys;
use std::iter::Iterator;
use std::hash::Hash;
use std::ops::Index;

pub use std::collections::hash_map::Iter as IterAll;
pub use std::collections::hash_map::IterMut as IterAllMut;

/// A MultiMap implementation which is just a wrapper around std::collections::HashMap.
/// See HashMap's documentation for more details.
///
/// Some of the methods are just thin wrappers, some methods does change a little semantics
/// and some methods are new (doesn't have an equivalent in HashMap.)
///
/// The MultiMap is generic for the key (K) and the value (V). Internally the values are
/// stored in a generic Vector.
///
/// # Examples
///
/// ```
/// use multimap::MultiMap;
///
/// // create a new MultiMap. An explicit type signature can be omitted because of the
/// // type inference.
/// let mut queries = MultiMap::new();
///
/// // insert some queries.
/// queries.insert("urls", "http://rust-lang.org");
/// queries.insert("urls", "http://mozilla.org");
/// queries.insert("urls", "http://wikipedia.org");
/// queries.insert("id", "42");
/// queries.insert("name", "roger");
///
/// // check if there's any urls.
/// println!("Are there any urls in the multimap? {:?}.",
///     if queries.contains_key("urls") {"Yes"} else {"No"} );
///
/// // get the first item in a key's vector.
/// assert_eq!(queries.get("urls"), Some(&"http://rust-lang.org"));
///
/// // get all the urls.
/// assert_eq!(queries.get_vec("urls"),
///     Some(&vec!["http://rust-lang.org", "http://mozilla.org", "http://wikipedia.org"]));
///
/// // iterate over all keys and the first value in the key's vector.
/// for (key, value) in queries.iter() {
///     println!("key: {:?}, val: {:?}", key, value);
/// }
///
/// // iterate over all keys and the key's vector.
/// for (key, values) in queries.iter_all() {
///     println!("key: {:?}, values: {:?}", key, values);
/// }
///
/// // the different methods for getting value(s) from the multimap.
/// let mut map = MultiMap::new();
///
/// map.insert("key1", 42);
/// map.insert("key1", 1337);
///
/// assert_eq!(map["key1"], 42);
/// assert_eq!(map.get("key1"), Some(&42));
/// assert_eq!(map.get_vec("key1"), Some(&vec![42, 1337]));
/// ```
pub struct MultiMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K, V> MultiMap<K, V> where K: Eq + Hash {

    /// Creates an empty MultiMap
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::new();
    /// ```
    pub fn new() -> MultiMap<K,V> {
        MultiMap { inner: HashMap::new() }
    }

    /// Creates an empty multimap with the given initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> MultiMap<K,V> {
        MultiMap { inner: HashMap::with_capacity(capacity) }
    }

    /// Inserts a key-value pair into the multimap. If the key does exists in
    /// the map then the key is pushed to that key's vector. If the key doesn't
    /// exists in the map a new vector with the given value is inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert("key", 42);
    /// ```
    pub fn insert(&mut self, k: K, v: V) {
        if self.inner.contains_key(&k) {
            let mut values = self.inner.get_mut(&k).unwrap();
            values.push(v);
        }
        else {
            let mut values = Vec::new();
            values.push(v);
            self.inner.insert(k,values);
        }
    }

    /// Returns true if the map contains a value for the specified key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.contains_key(k)
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(2, 1337);
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a key from the map, returning the vector of values at
    /// the key if the key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.remove(&1), Some(vec![42, 1337]));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.remove(k)
    }

    /// Returns a reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get(&1), Some(&42));
    /// ```
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get(k).map(|v| &v[0])
    }

    /// Returns a mutable reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_mut(&1) {
    ///     *v = 99;
    /// }
    /// assert_eq!(map[&1], 99);
    /// ```
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get_mut(k).map(|mut v| v.get_mut(0).unwrap())
    }

    /// Returns a reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get_vec(&1), Some(&vec![42, 1337]));
    /// ```
    pub fn get_vec<Q: ?Sized>(&self, k: &Q) -> Option<&Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get(k)
    }

    /// Returns a mutable reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_vec_mut(&1) {
    ///     (*v)[0] = 1991;
    ///     (*v)[1] = 2332;
    /// }
    /// assert_eq!(map.get_vec(&1), Some(&vec![1991, 2332]));
    /// ```
    pub fn get_vec_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut Vec<V>>
        where K: Borrow<Q>,
              Q: Eq + Hash
    {
        self.inner.get_mut(k)
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let map: MultiMap<usize, usize> = MultiMap::new();
    /// assert!(map.capacity() >= 0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// assert!(map.is_empty());
    /// map.insert(1,42);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the map, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is &'a K.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(2,1337);
    /// map.insert(4,1991);
    ///
    /// for key in map.keys() {
    ///     println!("{:?}", key);
    /// }
    /// ```
    pub fn keys<'a>(&'a self) -> Keys<'a, K, Vec<V>> {
        self.inner.keys()
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the first element in the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {:?}, val: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter { inner: self.inner.iter() }
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and a mutable reference to the first element in the
    /// corresponding key's vector. Iterator element type is (&'a K, &'a mut V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value *= *value;
    /// }
    ///
    /// for (key, value) in map.iter() {
    ///     println!("key: {:?}, val: {:?}", key, value);
    /// }
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut { inner: self.inner.iter_mut() }
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, values) in map.iter_all() {
    ///     println!("key: {:?}, values: {:?}", key, values);
    /// }
    /// ```
    pub fn iter_all(&self) -> IterAll<K, Vec<V>> {
        self.inner.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, values) in map.iter_all_mut() {
    ///     for value in values.iter_mut() {
    ///         *value = 99;
    ///     }
    /// }
    ///
    /// for (key, values) in map.iter_all() {
    ///     println!("key: {:?}, values: {:?}", key, values);
    /// }
    /// ```
    pub fn iter_all_mut(&mut self) -> IterAllMut<K, Vec<V>> {
        self.inner.iter_mut()
    }
}

impl<'a, K, V, Q: ?Sized> Index<&'a Q> for MultiMap<K, V>
    where K: Eq + Hash + Borrow<Q>,
          Q: Eq + Hash
{
    type Output = V;

    fn index(&self, index: &Q) -> &V {
        self.inner.get(index)
            .map(|v| &v[0])
            .expect("no entry found for key")
    }
}

#[derive(Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    inner: IterAll<'a,K, Vec<V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        self.inner.next().map(|(k,v)| (k, &v[0]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {
    fn len(&self) -> usize { self.inner.len() }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    inner: IterAllMut<'a,K, Vec<V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        self.inner.next().map(|(k,v)| (k, &mut v[0]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) { self.inner.size_hint() }
}

impl<'a, K, V> ExactSizeIterator for IterMut<'a, K, V> {
    fn len(&self) -> usize { self.inner.len() }
}

#[test]
fn create() {
    let _: MultiMap<usize, usize> = MultiMap { inner: HashMap::new() };
}

#[test]
fn new() {
    let _: MultiMap<usize, usize> = MultiMap::new();
}

#[test]
fn with_capacity() {
    let _: MultiMap<usize, usize> = MultiMap::with_capacity(20);
}

#[test]
fn insert() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,3);
}

#[test]
fn insert_existing() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,3);
    m.insert(1,4);
}

#[test]
#[should_panic]
fn index_no_entry() {
    let m: MultiMap<usize, usize> = MultiMap::new();
    &m[&1];
}

#[test]
fn index() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    let values = m[&1];
    assert_eq!(values, 42);
}

#[test]
fn contains_key_true() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    assert!(m.contains_key(&1));
}

#[test]
fn contains_key_false() {
    let m: MultiMap<usize, usize> = MultiMap::new();
    assert_eq!(m.contains_key(&1), false);
}

#[test]
fn len() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.insert(2,1337);
    m.insert(3,99);
    assert_eq!(m.len(), 3);
}

#[test]
fn remove_not_present() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    let v = m.remove(&1);
    assert_eq!(v, None);
}

#[test]
fn remove_present() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    let v = m.remove(&1);
    assert_eq!(v, Some(vec![42]));
}

#[test]
fn get_not_present() {
    let m: MultiMap<usize, usize> = MultiMap::new();
    assert_eq!(m.get(&1), None);
}

#[test]
fn get_present() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    assert_eq!(m.get(&1), Some(&42));
}

#[test]
fn get_vec_not_present() {
    let m: MultiMap<usize, usize> = MultiMap::new();
    assert_eq!(m.get_vec(&1), None);
}

#[test]
fn get_vec_present() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.insert(1,1337);
    assert_eq!(m.get_vec(&1), Some(&vec![42, 1337]));
}

#[test]
fn capacity() {
    let m: MultiMap<usize, usize> = MultiMap::with_capacity(20);
    assert!(m.capacity() >= 20);
}

#[test]
fn is_empty_true() {
    let m: MultiMap<usize, usize> = MultiMap::new();
    assert_eq!(m.is_empty(), true);
}

#[test]
fn is_empty_false() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    assert_eq!(m.is_empty(), false);
}

#[test]
fn clear() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.clear();
    assert!(m.is_empty());
}

#[test]
fn get_mut() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    if let Some(v) = m.get_mut(&1) {
        *v = 1337;
    }
    assert_eq!(m[&1], 1337)
}

#[test]
fn get_vec_mut() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.insert(1,1337);
    if let Some(v) = m.get_vec_mut(&1) {
        (*v)[0] = 5;
        (*v)[1] = 10;
    }
    assert_eq!(m.get_vec(&1), Some(&vec![5,10]))
}

#[test]
fn keys() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.insert(2,42);
    m.insert(4,42);
    m.insert(8,42);

    let keys: Vec<_> = m.keys().cloned().collect();
    assert_eq!(keys.len(), 4);
    assert!(keys.contains(&1));
    assert!(keys.contains(&2));
    assert!(keys.contains(&4));
    assert!(keys.contains(&8));
}

#[test]
fn iter() {
    let mut m: MultiMap<usize, usize> = MultiMap::new();
    m.insert(1,42);
    m.insert(1,42);
    m.insert(4,42);
    m.insert(8,42);

    let mut iter = m.iter();

    for _ in iter.by_ref().take(2) {}

    assert_eq!(iter.len(), 1);
}

