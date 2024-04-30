use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use crate::{DebugWith, Formatter};

impl<W, K: DebugWith<W>, V: DebugWith<W>, S: ::std::hash::BuildHasher> DebugWith<W>
    for HashMap<K, V, S>
{
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_map().entries_with(self, with).finish();
    }
}

impl<W, K: DebugWith<W>, V: DebugWith<W>> DebugWith<W> for BTreeMap<K, V> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_map().entries_with(self, with).finish();
    }
}

impl<W, V: DebugWith<W>, S: ::std::hash::BuildHasher> DebugWith<W> for HashSet<V, S> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_set().entries_with(self, with).finish();
    }
}

impl<W, V: DebugWith<W>> DebugWith<W> for BTreeSet<V> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_set().entries_with(self, with).finish();
    }
}

impl<W, D: DebugWith<W>> DebugWith<W> for Vec<D> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}

impl<W, D: DebugWith<W>> DebugWith<W> for VecDeque<D> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}

impl<W, D: DebugWith<W>> DebugWith<W> for LinkedList<D> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}

impl<W, D: DebugWith<W>> DebugWith<W> for BinaryHeap<D> {
    fn fmt(&self, with: &W, f: Formatter<'_>) {
        f.debug_list().entries_with(self, with).finish();
    }
}
