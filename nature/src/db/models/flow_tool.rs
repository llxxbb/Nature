/// none: means can't include any one
/// all : means must include all
/// any : means must include one
/// all of above between them are `and` relation
use std::collections::{HashMap, HashSet};

pub type ContextChecker = fn(contexts: &HashMap<String, String>,
                             none: &HashSet<String>,
                             all: &HashSet<String>,
                             any: &HashSet<String>) -> bool;

pub fn context_check(contexts: &HashMap<String, String>,
                     none: &HashSet<String>,
                     all: &HashSet<String>,
                     any: &HashSet<String>) -> bool {
    for exclude in none {
        if contexts.contains_key(exclude) {
            return false;
        }
    }
    for include in all {
        if !contexts.contains_key(include) {
            return false;
        }
    }
    if any.is_empty() {
        return true;
    }
    for o in any {
        if contexts.contains_key(o) {
            return true;
        }
    }
    false
}

pub type StateChecker = fn(status: &HashSet<String>,
                           none: &HashSet<String>,
                           all: &HashSet<String>,
                           any: &HashSet<String>) -> bool;

pub fn state_check(status: &HashSet<String>,
                   none: &HashSet<String>,
                   all: &HashSet<String>,
                   any: &HashSet<String>) -> bool {
    for exclude in none {
        if status.contains(exclude) {
            return false;
        }
    }
    for include in all {
        if !status.contains(include) {
            return false;
        }
    }
    if any.is_empty() {
        return true;
    }
    for o in any {
        if status.contains(o) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod demand_test {
    use super::*;

    #[test]
    fn status_check_nothing() {
        assert_eq!(state_check(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            &Default::default(),
        ), true);
        let mut states = HashSet::<String>::new();
        states.insert("a".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &Default::default(),
            &Default::default(),
        ), true);
    }

    #[test]
    fn status_check_none() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        assert_eq!(state_check(
            &Default::default(),
            &set,
            &Default::default(),
            &Default::default(),
        ), true);
        let mut states = HashSet::<String>::new();
        states.insert("b".to_string());
        assert_eq!(state_check(
            &states,
            &set,
            &Default::default(),
            &Default::default(),
        ), false);
    }

    #[test]
    fn status_check_all() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        let mut states = HashSet::<String>::new();
        states.insert("b".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &set,
            &Default::default(),
        ), false);
        states.insert("a".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &set,
            &Default::default(),
        ), true);
    }

    #[test]
    fn status_check_any() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        let mut states = HashSet::<String>::new();
        states.insert("c".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &Default::default(),
            &set,
        ), false);
        states.insert("a".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &Default::default(),
            &set,
        ), true);
    }

    #[test]
    fn status_check_none_priority() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        let mut set2 = HashSet::<String>::new();
        set2.insert("b".to_string());
        let mut set3 = HashSet::<String>::new();
        set3.insert("c".to_string());
        let mut states = HashSet::<String>::new();
        states.insert("a".to_string());
        states.insert("b".to_string());
        states.insert("c".to_string());
        assert_eq!(state_check(
            &states,
            &set,
            &set2,
            &set3,
        ), false);
        assert_eq!(state_check(
            &states,
            &set,
            &Default::default(),
            &set3,
        ), false);
        assert_eq!(state_check(
            &states,
            &set,
            &set2,
            &Default::default(),
        ), false);
    }

    #[test]
    fn status_check_all_any() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        let mut set2 = HashSet::<String>::new();
        set2.insert("b".to_string());
        let mut states = HashSet::<String>::new();
        states.insert("a".to_string());
        states.insert("c".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), false);
        let mut states = HashSet::<String>::new();
        states.insert("c".to_string());
        states.insert("b".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), false);
        let mut states = HashSet::<String>::new();
        states.insert("a".to_string());
        states.insert("b".to_string());
        assert_eq!(state_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), true);
    }

    #[test]
    fn context_check_nothing() {
        assert_eq!(context_check(
            &Default::default(),
            &Default::default(),
            &Default::default(),
            &Default::default(),
        ), true);
        let mut states = HashMap::<String, String>::new();
        states.insert("a".to_string(), "a".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &Default::default(),
            &Default::default(),
        ), true);
    }

    #[test]
    fn context_check_none() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        assert_eq!(context_check(
            &Default::default(),
            &set,
            &Default::default(),
            &Default::default(),
        ), true);
        let mut states = HashMap::<String, String>::new();
        states.insert("b".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &set,
            &Default::default(),
            &Default::default(),
        ), false);
    }

    #[test]
    fn context_check_all() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        let mut states = HashMap::<String, String>::new();
        states.insert("b".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &set,
            &Default::default(),
        ), false);
        states.insert("a".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &set,
            &Default::default(),
        ), true);
    }

    #[test]
    fn context_check_any() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        set.insert("b".to_string());
        let mut states = HashMap::<String, String>::new();
        states.insert("c".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &Default::default(),
            &set,
        ), false);
        states.insert("a".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &Default::default(),
            &set,
        ), true);
    }

    #[test]
    fn context_check_none_priority() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        let mut set2 = HashSet::<String>::new();
        set2.insert("b".to_string());
        let mut set3 = HashSet::<String>::new();
        set3.insert("c".to_string());
        let mut states = HashMap::<String, String>::new();
        states.insert("a".to_string(), "x".to_string());
        states.insert("b".to_string(), "x".to_string());
        states.insert("c".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &set,
            &set2,
            &set3,
        ), false);
        assert_eq!(context_check(
            &states,
            &set,
            &Default::default(),
            &set3,
        ), false);
        assert_eq!(context_check(
            &states,
            &set,
            &set2,
            &Default::default(),
        ), false);
    }

    #[test]
    fn context_check_all_any() {
        let mut set = HashSet::<String>::new();
        set.insert("a".to_string());
        let mut set2 = HashSet::<String>::new();
        set2.insert("b".to_string());
        let mut states = HashMap::<String, String>::new();
        states.insert("a".to_string(), "a".to_string());
        states.insert("c".to_string(), "c".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), false);
        let mut states = HashMap::<String, String>::new();
        states.insert("c".to_string(), "x".to_string());
        states.insert("b".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), false);
        let mut states = HashMap::<String, String>::new();
        states.insert("a".to_string(), "x".to_string());
        states.insert("b".to_string(), "x".to_string());
        assert_eq!(context_check(
            &states,
            &Default::default(),
            &set,
            &set2,
        ), true);
    }
}
