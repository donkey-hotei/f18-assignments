use std::collections::{HashMap, HashSet};

struct Event {
  pub name: String,
  pub month: u8,
  pub day: u8,
  pub year: u32
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

/* You need to complete two functions in this implementation
 * has_conflict() and update_event(). Note that the argument(s) and
 * return values for these two functions are missing below.
 * You can refer to tests for more information. */
impl Event {
  pub fn new(name: String, month: u8, day: u8, year: u32) -> Event {
    Event { name, month, day, year }
  }

  /* This function checks if two events are one the same date */
  pub fn has_conflict(&self, other: &Event) -> bool {
    self == other
  }

  /* This function shifts the date of an event by one day.
   * You can assume that the date is not on the last day
   * of a month */
  pub fn update_event(&mut self) {
      self.day += 1
  }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Trie {
  chr: char,
  has: bool,
  children: Vec<Trie>,
}

/* ["a", "cc", "ab"] =>
   {'\0', false, [
     {'a', true, [{'b', true, []}]},
     {'c', false, [{'c', true, []}]}
   ]}
*/

impl Trie {
  pub fn new(strs: &Vec<&str>) -> Trie {
    Trie::build(strs, '\0')
  }

  fn build(strs: &Vec<&str>, chr: char) -> Trie {
    // find all _unique_ first characters
    let ss = strs.iter()
        .filter_map(|s| s.chars().next()).collect::<HashSet<_>>();
    Trie {
        chr: chr,
        has: strs.iter().any(|s| s.len() == 0),
        children:
            ss.into_iter()
              .map(|c| { // each unique character has a node at this depth
                Trie::build(
                    // find all those words whose next letter is this trie's
                    // letter, and use their remainders as it's children...
                    &strs.iter().filter(|s| s.chars().next().unwrap() == c)
                                .map(|s|
                                     if s.len() > 1 { &s[1..] }
                                     else { "" })
                                .collect::<Vec<_>>(), c)
              }).collect::<Vec<_>>(),
    }
  }

  pub fn contains(&self, s: &str) -> bool {
    if s.len() == 0 { return self.has }
    else {
        let chr = s.chars().next().unwrap();
        for child in self.children.iter() {
            if child.chr == chr {
                return child.contains(&s[1..]);
            }
        }
    }

    false
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_event() {
    let event1 = Event::new("Pac-12 Championship".into(), 12, 1, 2017);
    let mut event2 = Event::new("Group Project Meeting".into(), 12, 1, 2017);
    assert!(event1.has_conflict(&event2));

    event2.update_event();
    assert_eq!(event2.day, 2);
  }

  #[test]
  fn test_trie() {
    let trie = Trie::new(&vec!["b", "ab"]);
    assert_eq!(trie.contains("ab"), true);
    assert_eq!(trie.contains("ac"), false);
    assert_eq!(trie.contains("a"), false);
    assert_eq!(trie.contains("b"), true);
  }
}
