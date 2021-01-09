use std::fmt::Write;

use crate::domain::*;

/// It can't have the state with same name.
pub type States = Vec<State>;

/// The structure for defined state in meta
#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Ord, PartialOrd)]
pub enum State {
    Mutex(Vec<State>),
    Normal(String),
    Parent(String, Vec<State>),
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            State::Normal(s) => s.to_string(),
            State::Parent(name, list) =>
                name.to_owned() + "[" + Self::states_to_string(list, ",").as_str() + "]",
            State::Mutex(list) =>
                Self::states_to_string(list, "|"),
        }
    }
}

impl State {
    pub fn states_to_string(states: &States, separator: &str) -> String {
        if states.len() < 1 {
            return "".to_string();
        }
        let mut rtn = states[0].to_string();
        for x in 1..states.len() {
            let _ = write!(&mut rtn, "{}{}", separator, states[x].to_string());
        }
        rtn
    }

    pub fn string_to_states(states: &str) -> Result<(States, usize)> {
        // check length
        if states.len() < 1 {
            return Err(NatureError::VerifyError("states string should not be empty".to_string()));
        }
        // store temp result
        let mut rtn: States = vec![];
        let mut normal = String::new();
        let mut mutex: States = vec![];
        let mut is_mutex = false;
        let mut parent: Option<State> = None;
        let mut x = 0;
        // main progress
        while x < states.len() {
            let c = &states[x..x + 1];
            match c {
                "," => {    // separator
                    if is_mutex {
                        if normal.len() > 0 {
                            mutex.push(State::Normal(normal));
                            normal = String::new();
                        } else if parent.is_some() {
                            mutex.push(parent.unwrap());
                            parent = None;
                        }
                        let mut nm: States = vec![];
                        nm.append(&mut mutex);
                        rtn.push(State::Mutex(nm));
                        is_mutex = false;
                    } else {
                        if normal.len() > 0 {  // the ']' logic will make `normal` be empty.
                            rtn.push(State::Normal(normal));
                            normal = String::new();
                        } else if parent.is_some() {
                            rtn.push(parent.unwrap());
                            parent = None;
                        }
                    }
                }
                "|" => {    // mutex
                    is_mutex = true;
                    if normal.len() > 0 {  // the ']' logic will make `normal` be empty.
                        mutex.push(State::Normal(normal));
                        normal = String::new();
                    } else if parent.is_some() {
                        mutex.push(parent.unwrap());
                        parent = None;
                    }
                }
                "[" => {    // parent begin
                    let r = Self::string_to_states(&states[x + 1..])?;
                    x = x + r.1;
                    parent = Some(State::Parent(normal, r.0));
                    normal = String::new();
                }
                "]" => {    // parent end
                    match is_mutex {
                        false => {
                            if normal.len() > 0 {
                                rtn.push(State::Normal(normal));
                            } else if parent.is_some() {
                                rtn.push(parent.unwrap());
                            }
                        }
                        true => {
                            if normal.len() > 0 {
                                mutex.push(State::Normal(normal))
                            } else if parent.is_some() {
                                mutex.push(parent.unwrap());
                            }
                            rtn.push(State::Mutex(mutex));
                        }
                    }
                    return Ok((rtn, x + 1));
                }
                _ => {      // literal
                    let w = write!(&mut normal, "{}", c);
                    if w.is_err() {
                        return Err(NatureError::SystemError(w.err().unwrap().to_string()));
                    }
                }
            }
            x = x + 1;
        }
        // the last normal unhandled by loop
        if is_mutex {
            if normal.len() > 0 {
                mutex.push(State::Normal(normal));
            } else if parent.is_some() {
                mutex.push(parent.unwrap());
            }
            rtn.push(State::Mutex(mutex));
        } else {
            if normal.len() > 0 {
                rtn.push(State::Normal(normal));
            } else if parent.is_some() {
                rtn.push(parent.unwrap());
            }
        }
        Ok((rtn, states.len()))
    }

    pub fn include(&self, another: &State) -> bool {
        if self.eq(another) {
            return true;
        }
        match self {
            State::Mutex(x) => x.iter().find(|a| { a.include(another) }).is_some(),
            State::Parent(_, x) => x.iter().find(|a| { a.include(another) }).is_some(),
            State::Normal(_) => false
        }
    }

    pub fn has_name(&self, name: &str) -> bool {
        match self {
            State::Mutex(x) => x.iter().find(|a| { a.has_name(name) }).is_some(),
            State::Parent(n, x) => {
                if n == name { true } else {
                    x.iter().find(|a| { a.has_name(name) }).is_some()
                }
            }
            State::Normal(x) => x == name
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            State::Normal(s) => s.clone(),
            State::Mutex(_) => "".to_string(),
            State::Parent(s, _) => s.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct StatePath {
    pub is_mutex: bool,
    pub desc_seq: Vec<CheckType>,
}

/// type and it's position where it is in the path
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum CheckType {
    Normal(u16),
    Parent(u16),
    Mutex(u16),
}

impl Default for CheckType {
    fn default() -> Self {
        CheckType::Parent(0)
    }
}

#[cfg(test)]
mod string_to_states_virtual_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : test under here
    #[test]
    fn for_parent_and_no_mutex() {
        let rtn = State::string_to_states("p[b]|a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            State::Normal("a".to_string()),
        ]));
    }

    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states("a|p[b]|a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            State::Normal("a".to_string()),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_square_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    #[test]
    fn for_normal_and_mutex() {
        let rtn = State::string_to_states("p[b|a]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("b".to_string()),
                State::Normal("a".to_string()),
            ]),
        ]));
    }

    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : test under here
    #[test]
    fn for_parent_and_no_mutex() {
        let rtn = State::string_to_states("p[p[b]]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
        ]));
    }

    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states("p[a|p[b]]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("a".to_string()),
                State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
            ]),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_comma_end {
    use super::*;

    // normal, no parent, no mutex : already test in other place
    // normal, no parent, mutex : already test in other place
    // normal, parent, no mutex : does not exist this case
    // normal, parent, mutex : does not exist this case
    // no normal, parent, not mutex : already test in other place
    // no normal, parent, mutex : test under here
    #[test]
    fn for_parent_and_mutex() {
        let rtn = State::string_to_states("a|p[b],a").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Parent("p".to_string(), vec![State::Normal("b".to_string())]),
        ]));
        assert_eq!(rtn.0[1], State::Normal("a".to_string()));
    }
}

#[cfg(test)]
mod string_to_states_for_mixed {
    use super::*;

    #[test]
    fn normal_parent() {
        let rtn = State::string_to_states("a,p[c]").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Parent("p".to_string(), vec![State::Normal("c".to_string())]));
    }

    #[test]
    fn normal_mutex() {
        let rtn = State::string_to_states("a,c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }

    #[test]
    fn parent_mutex() {
        let rtn = State::string_to_states("p[a],c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }


    #[test]
    fn complex() {
        let rtn = State::string_to_states("a,p[a],m|n,p[m|m,p[a,b]|p[c,c]]").unwrap();
        assert_eq!(rtn.0.len(), 4);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[2], State::Mutex(vec![
            State::Normal("m".to_string()),
            State::Normal("n".to_string()),
        ]));
//        发现的问题： [ 逻辑不能立即放到 range 中去， 要看后面是，才可以，如果是 | 则需要放到 mutex 中去。
        assert_eq!(rtn.0[3], State::Parent("p".to_string(), vec![
            State::Mutex(vec![
                State::Normal("m".to_string()),
                State::Normal("m".to_string()),
            ]),
            State::Mutex(vec![
                State::Parent("p".to_string(), vec![
                    State::Normal("a".to_string()),
                    State::Normal("b".to_string()),
                ]),
                State::Parent("p".to_string(), vec![
                    State::Normal("c".to_string()),
                    State::Normal("c".to_string()),
                ]),
            ])]));
    }
}

#[cfg(test)]
mod string_to_states_for_mutex {
    use super::*;

    #[test]
    fn single() {
        let rtn = State::string_to_states("a|b").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
        ]));

        let rtn = State::string_to_states("a|b|c").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
            State::Normal("c".to_string()),
        ]));
    }

    #[test]
    fn multi() {
        let rtn = State::string_to_states("a|b,c|d").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
        ]));
        assert_eq!(rtn.0[1], State::Mutex(vec![
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]));
    }
}

#[cfg(test)]
mod string_to_states_for_parent {
    use super::*;

    #[test]
    fn one_child() {
        let rtn = State::string_to_states("p[a]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));
    }

    #[test]
    fn three_children() {
        let rtn = State::string_to_states("p[a,b,c]").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string()),
            State::Normal("c".to_string())]));
    }

    #[test]
    fn three_parent() {
        let rtn = State::string_to_states("p1[a],p2[b],p3[c]").unwrap();
        assert_eq!(rtn.0.len(), 3);
        assert_eq!(rtn.0[0], State::Parent("p1".to_string(), vec![State::Normal("a".to_string())]));
        assert_eq!(rtn.0[1], State::Parent("p2".to_string(), vec![State::Normal("b".to_string())]));
        assert_eq!(rtn.0[2], State::Parent("p3".to_string(), vec![State::Normal("c".to_string())]));
    }

    #[test]
    fn comma_end() {
        let rtn = State::string_to_states("p[a,").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));

        let rtn = State::string_to_states("p[a,b],").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())]));
    }

    #[test]
    fn right_square_missed() {
        let rtn = State::string_to_states("p[a").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![State::Normal("a".to_string())]));

        let rtn = State::string_to_states("p[a,b").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())]));
    }
}

#[cfg(test)]
mod string_to_states_for_normal {
    use super::*;

    #[test]
    fn only_one() {
        let rtn = State::string_to_states("test").unwrap();
        assert_eq!(rtn.0.len(), 1);
        assert_eq!(rtn.0[0], State::Normal("test".to_string()));
    }

    #[test]
    fn three() {
        let rtn = State::string_to_states("a,b,c").unwrap();
        assert_eq!(rtn.0.len(), 3);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Normal("b".to_string()));
        assert_eq!(rtn.0[2], State::Normal("c".to_string()));
    }

    #[test]
    fn comma_end() {
        let rtn = State::string_to_states("a,b,").unwrap();
        assert_eq!(rtn.0.len(), 2);
        assert_eq!(rtn.0[0], State::Normal("a".to_string()));
        assert_eq!(rtn.0[1], State::Normal("b".to_string()));
    }
}

#[cfg(test)]
mod states_to_string {
    use super::*;

    #[test]
    fn to_string() {
        let string = State::Parent("a".to_string(), vec![
            State::Parent("b".to_string(), vec![
                State::Normal("e".to_string()),
                State::Mutex(vec![
                    State::Normal("f".to_string()),
                    State::Normal("g".to_string()),
                ]),
                State::Normal("i".to_string()),
            ]),
            State::Normal("c".to_string()),
            State::Normal("d".to_string()),
        ]).to_string();
        assert_eq!(string, "a[b[e,f|g,i],c,d]");
    }
}

#[cfg(test)]
mod find_and_has_name {
    use super::*;

    #[test]
    fn normal() {
        let s = State::Normal("a".to_string());
        assert_eq!(s.include(&State::Normal("b".to_string())), false);
        assert_eq!(s.include(&State::Normal("a".to_string())), true);
        assert_eq!(s.has_name("a"), true);
        assert_eq!(s.has_name("b"), false);
    }

    #[test]
    fn in_mutex() {
        let s = State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]);
        assert_eq!(s.include(&State::Normal("b".to_string())), true);
        assert_eq!(s.include(&State::Normal("a".to_string())), true);
        assert_eq!(s.include(&State::Normal("c".to_string())), false);
        assert_eq!(s.has_name("a"), true);
        assert_eq!(s.has_name("b"), true);
        assert_eq!(s.has_name("c"), false);

        let sa = State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]);
        assert_eq!(s.include(&sa), true);
        let sb = State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("c".to_string())
        ]);
        assert_eq!(s.include(&sb), false);
    }

    #[test]
    fn in_parent() {
        let s = State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]);
        assert_eq!(s.include(&State::Normal("b".to_string())), true);
        assert_eq!(s.include(&State::Normal("a".to_string())), true);
        assert_eq!(s.include(&State::Normal("c".to_string())), false);
        assert_eq!(s.has_name("p"), true);
        assert_eq!(s.has_name("a"), true);
        assert_eq!(s.has_name("b"), true);
        assert_eq!(s.has_name("c"), false);

        let sa = State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]);
        assert_eq!(s.include(&sa), true);
        let sa = State::Parent("pp".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]);
        assert_eq!(s.include(&sa), false);
        let sb = State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("c".to_string())
        ]);
        assert_eq!(s.include(&sb), false);
    }

    #[test]
    fn in_complex_parent() {
        let (s, _) = State::string_to_states("a,k[b],c|d,l[e|f,m[g,h]|n[i,j]]").unwrap();
        let s = State::Parent("pa".to_string(), s);
        assert_eq!(s.has_name("a"), true);
        assert_eq!(s.has_name("b"), true);
        assert_eq!(s.has_name("c"), true);
        assert_eq!(s.has_name("d"), true);
        assert_eq!(s.has_name("e"), true);
        assert_eq!(s.has_name("f"), true);
        assert_eq!(s.has_name("g"), true);
        assert_eq!(s.has_name("h"), true);
        assert_eq!(s.has_name("i"), true);
        assert_eq!(s.has_name("j"), true);
        assert_eq!(s.has_name("k"), true);
        assert_eq!(s.has_name("l"), true);
        assert_eq!(s.has_name("m"), true);
        assert_eq!(s.has_name("n"), true);
        assert_eq!(s.has_name("o"), false);
    }
}

#[cfg(test)]
mod normal_test {
    use super::*;

    #[test]
    fn name_test() {
        assert_eq!(State::Normal("b".to_string()).get_name(), "b".to_string());
        assert_eq!(State::Mutex(vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]).get_name(), "".to_string());
        assert_eq!(State::Parent("p".to_string(), vec![
            State::Normal("a".to_string()),
            State::Normal("b".to_string())
        ]).get_name(), "p".to_string());
    }
}