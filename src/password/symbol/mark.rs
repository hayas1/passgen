use once_cell::sync::Lazy;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MarkSet {
    mark: HashSet<char>,
}
impl Default for MarkSet {
    fn default() -> Self {
        Self { mark: Self::DEFAULT_MARK.chars().collect() }
    }
}
impl MarkSet {
    pub const DEFAULT_MARK: &'static str = "^!@#&";
    pub const DEFAULT_MARK_SET: Lazy<HashSet<char>> =
        Lazy::new(|| Self::DEFAULT_MARK.chars().collect());
    pub const CANDIDATE_MARK: &'static str = ".,_-+=/\\^!?@#&\"'$%:;><()[]{}";
    pub const CANDIDATE_MARK_SET: Lazy<HashSet<char>> =
        Lazy::new(|| Self::CANDIDATE_MARK.chars().collect());

    /// return new empty MarkSet
    pub fn new() -> Self {
        Self { mark: HashSet::new() }
    }

    /// return true if this set contains the value
    pub fn contains(&self, mark: &char) -> bool {
        self.mark.contains(mark)
    }

    /// return true if this set contains nothing
    pub fn is_empty(&self) -> bool {
        self.mark.is_empty()
    }

    /// return iterator candidate mark and is it contains this set
    pub fn get_marks(&self) -> impl Iterator<Item = (char, bool)> + '_ {
        Self::CANDIDATE_MARK.chars().map(move |c| (c, self.contains(&c)))
    }

    /// get iterator of this set
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, char> {
        self.mark.iter()
    }

    /// if mark is contained remove it, not contained and contained in candidate insert it
    pub fn toggle(&mut self, mark: char) -> bool {
        match self.contains(&mark) {
            true => self.remove(&mark),
            false => self.insert(mark),
        }
    }

    /// if mark is contained in candidate mark, insert it
    pub fn insert(&mut self, mark: char) -> bool {
        if Self::CANDIDATE_MARK_SET.contains(&mark) {
            self.mark.insert(mark)
        } else {
            false
        }
    }

    /// all mark contains in this set
    pub fn insert_all(&mut self) {
        self.mark.extend(Self::CANDIDATE_MARK_SET.clone())
    }

    /// if mark is contained, remove it
    pub fn remove(&mut self, mark: &char) -> bool {
        self.mark.remove(mark)
    }

    /// clear contains elements
    pub fn clear(&mut self) {
        self.mark.clear()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn all_mark_is_different_test() {
        assert_eq!(
            MarkSet::DEFAULT_MARK.chars().collect::<Vec<_>>().len(),
            MarkSet::DEFAULT_MARK.chars().collect::<HashSet<_>>().len()
        );
        assert_eq!(
            MarkSet::CANDIDATE_MARK.chars().collect::<HashSet<_>>().len(),
            MarkSet::CANDIDATE_MARK_SET.len(),
        )
    }

    #[test]
    fn candidate_mark_contain_all_default_mark_test() {
        let mut default = HashSet::new();
        for c in MarkSet::CANDIDATE_MARK.chars() {
            if MarkSet::DEFAULT_MARK_SET.contains(&c) {
                default.insert(c);
            }
        }
        assert_eq!(default, MarkSet::DEFAULT_MARK_SET.clone());
    }

    #[test]
    fn contains_test() {
        let default = MarkSet::default();
        assert!(default.contains(&'!'));
        assert!(default.contains(&'#'));
        assert!(!default.contains(&'a'));
        assert!(!default.contains(&'A'));
        assert!(!default.contains(&'0'));
        assert!(!default.contains(&'\\'));
    }

    #[test]
    fn remove_insert_test() {
        let mut ms = MarkSet::default();
        {
            assert!(!ms.insert('a')); // 'a' is not contains in candidate mark, so returned false
            assert!(!ms.contains(&'a'));
        }
        {
            assert!(!ms.contains(&'?'));
            assert!(!ms.remove(&'?'));
            assert!(!ms.contains(&'?'));
            assert!(ms.insert('?'));
            assert!(ms.contains(&'?'));
            assert!(ms.remove(&'?'));
            assert!(!ms.contains(&'?'));
        }
    }

    #[test]
    fn toggle_test() {
        let mut ms = MarkSet::default();
        assert!(!ms.contains(&'/'));
        assert!(ms.toggle('/'));
        assert!(ms.contains(&'/'));
        assert!(ms.toggle('/'));
        assert!(!ms.contains(&'/'));
    }

    #[test]
    fn toggle_return_false_test() {
        let mut ms = MarkSet::default();
        assert!(!ms.toggle('a'));
        assert!(!ms.toggle('A'));
        assert!(!ms.toggle('0'));
    }
}
