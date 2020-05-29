use crate::mutex::Mutex;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;

#[async_trait]
pub trait BatchFn<K, V> {
    async fn load(&self, keys: &[K]) -> HashMap<K, V>
    where
        K: 'async_trait,
        V: 'async_trait;
}

struct State<K, V> {
    completed: HashMap<K, V>,
    pending: HashSet<K>,
}

impl<K: Eq + Hash, V> State<K, V> {
    fn with_cache(cache: HashMap<K, V>) -> Self {
        State {
            completed: cache,
            pending: HashSet::new(),
        }
    }
}

pub struct Loader<K, V, F>
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: BatchFn<K, V>,
{
    state: Arc<Mutex<State<K, V>>>,
    load_fn: Arc<Mutex<F>>,
}

impl<K, V, F> Clone for Loader<K, V, F>
where
    K: Eq + Hash + Clone,
    V: Clone,
    F: BatchFn<K, V>,
{
    fn clone(&self) -> Self {
        Loader {
            state: self.state.clone(),
            load_fn: self.load_fn.clone(),
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl<K, V, F> Loader<K, V, F>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone,
    F: BatchFn<K, V>,
{
    pub fn new(load_fn: F) -> Loader<K, V, F> {
        Loader::with_cache(load_fn, HashMap::new())
    }
}

impl<K, V, F> Loader<K, V, F>
where
    K: Eq + Hash + Clone + Debug,
    V: Clone,
    F: BatchFn<K, V>,
{
    pub fn with_cache(load_fn: F, cache: HashMap<K, V>) -> Loader<K, V, F> {
        Loader {
            state: Arc::new(Mutex::new(State::with_cache(cache))),
            load_fn: Arc::new(Mutex::new(load_fn)),
        }
    }

    pub async fn load(&self, key: K) -> V {
        let mut state = self.state.lock().await;
        if let Some(v) = state.completed.get(&key) {
            return (*v).clone();
        }

        state.pending.insert(key.clone());
        let keys = state.pending.drain().collect::<Vec<K>>();
        let load_fn = self.load_fn.lock().await;
        let load_ret = load_fn.load(keys.as_ref()).await;
        drop(load_fn);
        for (k, v) in load_ret.into_iter() {
            state.completed.insert(k, v);
        }
        return state
            .completed
            .get(&key)
            .cloned()
            .unwrap_or_else(|| panic!("found key {:?} in load result", key));
    }
}
