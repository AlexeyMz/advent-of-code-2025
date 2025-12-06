use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct RangeSet<T: Copy + Ord> {
    ranges: Vec<RangeInclusive<T>>,
}

impl<T: Copy + Ord> RangeSet<T> {
    pub fn new(overlapping_ranges: &[RangeInclusive<T>]) -> RangeSet<T> {
        let mut sorted_ranges = overlapping_ranges.to_vec();
        sorted_ranges.sort_by_key(|r| *r.start());

        let mut ranges: Vec<RangeInclusive<T>> = vec![];
        for range in sorted_ranges.into_iter() {
            let last_range = ranges.last().cloned();
            if let Some(last) = last_range {
                if last.contains(range.start()) {
                    if !last.contains(range.end()) {
                        ranges.pop();
                        ranges.push(*last.start()..=*range.end());
                    }
                } else {
                    ranges.push(range);
                }
            } else {
                ranges.push(range);
            }
        }

        RangeSet { ranges }
    }

    pub fn find_range(&self, p: T) -> Option<RangeInclusive<T>> {
        self.find_range_index(p)
            .map(|index| self.get_current_range(index, p))?
    }

    /// Finds the lowest range index `i` such that `ranges[i].source <= point`.
    fn find_range_index(&self, p: T) -> Option<usize> {
        self.ranges
            .binary_search_by(|probe| probe.start().cmp(&p))
            .ok()
    }

    fn get_current_range(&self, range_index: usize, p: T) -> Option<RangeInclusive<T>> {
        let found = &self.ranges[range_index];
        if p >= *found.start() && p <= *found.end() {
            Some(found.clone())
        } else {
            None
        }
    }

    pub fn ranges(&self) -> &Vec<RangeInclusive<T>> {
        &self.ranges
    }
}
