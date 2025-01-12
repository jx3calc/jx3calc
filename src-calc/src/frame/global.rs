mod buff;

use crate::config;

static ARRAY_SIZE: usize = 1024;

pub trait SubTrait<V> {
    fn struct_name() -> &'static str;
    fn tab_init();
    fn tab_get(key: &V) -> Self;
}

struct ManagerItem<K, V>
where
    K: Eq + std::hash::Hash,
    V: 'static,
{
    map: std::collections::HashMap<K, &'static V>,
    data: Vec<*mut V>,
}

unsafe impl<K: Eq + std::hash::Hash, V> Send for ManagerItem<K, V> {}
unsafe impl<K: Eq + std::hash::Hash, V> Sync for ManagerItem<K, V> {}

pub struct Manager<K, V>
where
    K: Eq + std::hash::Hash + std::fmt::Debug + Clone,
    V: 'static + SubTrait<K>,
{
    v: std::sync::RwLock<ManagerItem<K, V>>,
}

impl<K, V> Manager<K, V>
where
    K: Eq + std::hash::Hash + std::fmt::Debug + Clone,
    V: SubTrait<K>,
{
    fn new() -> Self {
        let _ = config::Config::try_init(); // Ensure config is initialized
        V::tab_init();
        Manager {
            v: std::sync::RwLock::new(ManagerItem {
                map: std::collections::HashMap::new(),
                data: Vec::new(),
            }),
        }
    }

    fn add(&self, key: K, value: V) {
        let mut v = self.v.write().unwrap();
        if v.map.contains_key(&key) {
            return;
        }
        let i = v.map.len() % ARRAY_SIZE;
        if i == 0 {
            let array: Box<[V; ARRAY_SIZE]> = Box::new(unsafe { std::mem::zeroed() });
            v.data.push(Box::leak(array).as_mut_ptr());
        }
        unsafe {
            let ptr = v.data.last_mut().unwrap().add(i);
            std::ptr::write(ptr, value);
            v.map.insert(key, &*ptr);
        }
    }

    fn get(&self, key: &K) -> &'static V {
        let v = self.v.read().unwrap();
        let res = v.map.get(key);
        if res.is_some() {
            return res.unwrap();
        }
        drop(v);
        let value = V::tab_get(key);
        self.add((*key).clone(), value);
        self.v.read().unwrap().map.get(key).expect(&format!(
            "[global] Key `{:?}` not found in struct {}",
            key,
            V::struct_name()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct TestKey(i32);

    struct TestValue(i32);

    impl SubTrait<TestKey> for TestValue {
        fn struct_name() -> &'static str {
            "TestValue"
        }

        fn tab_init() {
            // Initialization logic for TestValue
        }

        fn tab_get(key: &TestKey) -> Self {
            TestValue(key.0 * 2)
        }
    }

    #[test]
    fn test_manager_add_and_get() {
        let manager = std::sync::Arc::new(Manager::<TestKey, TestValue>::new());

        let key1 = TestKey(1);
        let value1 = TestValue(10);
        manager.add(key1.clone(), value1);

        let retrieved_value = manager.get(&key1);
        assert_eq!(retrieved_value.0, 10);

        let key2 = TestKey(2);
        let retrieved_value2 = manager.get(&key2);
        assert_eq!(retrieved_value2.0, 4); // tab_get logic doubles the key value
    }

    #[test]
    fn test_manager_concurrent_access() {
        let manager = std::sync::Arc::new(Manager::<TestKey, TestValue>::new());

        let key1 = TestKey(1);
        let value1 = TestValue(10);
        manager.add(key1.clone(), value1);

        let manager_clone = std::sync::Arc::clone(&manager);
        let handle = std::thread::spawn(move || {
            let retrieved_value = manager_clone.get(&key1);
            assert_eq!(retrieved_value.0, 10);
        });

        handle.join().unwrap();
    }

    #[test]
    fn test_manager_memory_leak() {
        let manager = std::sync::Arc::new(Manager::<TestKey, TestValue>::new());

        assert_eq!(manager.v.read().unwrap().map.len(), 0);
        assert_eq!(manager.v.read().unwrap().data.len(), 0);

        let key0 = TestKey(0);
        let value0 = TestValue(10);
        manager.add(key0.clone(), value0);
        assert_eq!(manager.v.read().unwrap().map.len(), 1);
        assert_eq!(manager.v.read().unwrap().data.len(), 1);

        let key1 = TestKey(1);
        let _ = manager.get(&key1);
        assert_eq!(manager.v.read().unwrap().map.len(), 2);
        assert_eq!(manager.v.read().unwrap().data.len(), 1);

        for i in 2..ARRAY_SIZE {
            let key = TestKey(i as i32);
            let _ = manager.get(&key);
            assert_eq!(manager.v.read().unwrap().map.len(), i + 1);
            assert_eq!(manager.v.read().unwrap().data.len(), 1);
        }

        let key = TestKey((ARRAY_SIZE - 1) as i32);
        let _ = manager.get(&key); // duplicate key
        assert_eq!(manager.v.read().unwrap().map.len(), ARRAY_SIZE);
        assert_eq!(manager.v.read().unwrap().data.len(), 1);

        let key = TestKey(ARRAY_SIZE as i32);
        let _ = manager.get(&key);
        assert_eq!(manager.v.read().unwrap().map.len(), ARRAY_SIZE + 1);
        assert_eq!(manager.v.read().unwrap().data.len(), 2);
    }
}
