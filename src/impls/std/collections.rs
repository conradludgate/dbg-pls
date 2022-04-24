use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use crate::{DebugPls, Formatter};

impl<K: DebugPls, V: DebugPls, S: ::std::hash::BuildHasher> DebugPls for HashMap<K, V, S> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_map().entries(self).finish();
    }
}

impl<K: DebugPls, V: DebugPls> DebugPls for BTreeMap<K, V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_map().entries(self).finish();
    }
}

impl<V: DebugPls, S: ::std::hash::BuildHasher> DebugPls for HashSet<V, S> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_set().entries(self).finish();
    }
}

impl<V: DebugPls> DebugPls for BTreeSet<V> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_set().entries(self).finish();
    }
}

impl<D: DebugPls> DebugPls for Vec<D> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}

impl<D: DebugPls> DebugPls for VecDeque<D> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}

impl<D: DebugPls> DebugPls for LinkedList<D> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}

impl<D: DebugPls> DebugPls for BinaryHeap<D> {
    fn fmt(&self, f: Formatter<'_>) {
        f.debug_list().entries(self).finish();
    }
}
