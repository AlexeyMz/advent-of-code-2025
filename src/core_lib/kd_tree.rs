pub trait KDSpace {
    type Key: Eq + Clone + PartialOrd;
    type Item: Ord + Clone;
    type Distance: PartialOrd + Copy;
    fn items(self) -> impl Iterator<Item = Self::Item>;
    fn get_key(item: &Self::Item, depth: usize) -> Self::Key;
    fn key_distance(from: &Self::Key, to: &Self::Key) -> Self::Distance;
    fn item_distance(from: &Self::Item, to: &Self::Item) -> Self::Distance;
}

pub struct KDTree<S: KDSpace> {
    root: Option<KDNode<S::Item>>,
}

struct KDNode<T> {
    item: T,
    left: Option<Box<KDNode<T>>>,
    right: Option<Box<KDNode<T>>>,
}

impl<S: KDSpace> KDTree<S> {
    pub fn construct(space: S) -> KDTree<S> {
        let root = Self::construct_node(space.items().collect(), 0);
        return KDTree { root };
    }

    fn construct_node(items: Vec<S::Item>, depth: usize) -> Option<KDNode<S::Item>> {
        if items.len() == 0 {
            return None;
        }
        let middle = items.len() / 2;
        let middle_key = S::get_key(&items[middle], depth);
        let (left_items, mut right_items): (Vec<_>, Vec<_>) = items.into_iter()
            .partition(|item| S::get_key(item, depth) < middle_key);

        let i = right_items.iter()
            .position(|item| S::get_key(item, depth) == middle_key)
            .unwrap_or(0);
        let last_index = right_items.len() - 1;
        right_items.swap(i, last_index);
        let item = right_items.pop().unwrap();

        return Some(KDNode {
            item,
            left: Self::construct_node(left_items, depth + 1)
                .map(|n| Box::new(n)),
            right: Self::construct_node(right_items, depth + 1)
                .map(|n| Box::new(n)),
        });
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (S::Item, S::Item)> {
        return KDEdgeIterator::<S> {
            stack: self.root.iter().collect(),
            edges: vec![],
        };
    }

    pub fn find_nearest(&self, to: &S::Item, mut predicate: impl FnMut(&S::Item) -> bool) -> Option<S::Item> {
        match &self.root {
            Some(node) =>
                Self::find_nearest_with_depth(&node, to, 0, &mut predicate)
                    .map(|found| found.0),
            _ => None,
        }
    }

    fn find_nearest_with_depth(
        parent: &KDNode<S::Item>,
        to: &S::Item,
        depth: usize,
        predicate: &mut impl FnMut(&S::Item) -> bool
    ) -> Option<(S::Item, S::Distance)> {
        // let prefix = "  ".repeat(depth);
        // eprintln!("{}find[{}]: {:?} {:?} ", prefix, depth % 3, to, parent.item);
        let parent_distance = S::item_distance(&parent.item, to);
        let mut best: Option<(S::Item, S::Distance)> = if predicate(&parent.item) {
            Some((parent.item.clone(), parent_distance))
        } else {
            None
        };

        let mut branches = [&parent.left, &parent.right];
        let parent_key = S::get_key(&parent.item, depth);
        let to_key = S::get_key(to, depth);
        let plane_distance = S::key_distance(&to_key, &parent_key);
        if to_key > parent_key {
            // eprintln!("{}swap: {:?} {:?}", prefix, to_key, parent_key);
            branches.swap(0, 1);
        }

        // let mut i = 0;
        for branch in branches {
            if let Some(node) = branch {
                // eprintln!("{}branch {i}", prefix);
                // i += 1;
                let found = Self::find_nearest_with_depth(node, to, depth + 1, predicate);
                if let Some((item, distance)) = found &&
                    best.as_ref().map(|previous| distance < previous.1).unwrap_or(true) &&
                    predicate(&item)
                {
                    best = Some((item, distance));
                    if distance < plane_distance {
                        break;
                    }
                }
            }
        }

        return best;
    }
}

struct KDEdgeIterator<'a, S: KDSpace> {
    stack: Vec<&'a KDNode<S::Item>>,
    edges: Vec<(S::Item, S::Item)>,
}

impl<'a, S: KDSpace> Iterator for KDEdgeIterator<'a, S> {
    type Item = (S::Item, S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(edge) = self.edges.pop() {
            return Some(edge);
        } else if let Some(node) = self.stack.pop() {
            if let Some(left) = &node.left {
                self.stack.push(left);
                self.edges.push((node.item.clone(), left.item.clone()));
            }
            if let Some(right) = &node.right {
                self.stack.push(right);
                self.edges.push((node.item.clone(), right.item.clone()));
            }
            return self.next();
        } else {
            return None;
        }
    }
}
