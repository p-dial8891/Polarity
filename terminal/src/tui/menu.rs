use crossterm::{
    event::{KeyCode, Event}
};
use std::{slice, iter};
use crate::tui::input::{Input};
use crate::tui::app::Keys::{*};

#[derive(Clone, PartialEq, Debug)]
pub enum MenuLevel {
    Level1(&'static str),
    Level2(&'static str, KeyCode, KeyCode)
}

#[derive(Clone)]
pub struct MenuLevels<'a> {
    pub c : iter::Cycle<slice::Iter<'a, MenuLevel>>,
    pub size : usize,
    pub input_set : &'a [KeyCode]
}

impl MenuLevel {

    pub fn visit(&self, ml : &mut MenuLevels, ip : &mut Input) -> Self {

        if let Some(Event::Key(e)) = ip.ev {
            // TODO: Move this search logic to the input module.
            if ml.input_set.iter().find(|x| { **x == e.code }).is_some() &&
                e.is_press() 
            {
                eprintln!("Menu Key Captured {:?}", e.code);
                ip.ev = None;
                let k = e.code;
                match self {

                    MenuLevel::Level1(s) => {

                        ml.visited_by_level1(s, k)

                    },

                    MenuLevel::Level2(s, _, _) => {

                        ml.visited_by_level2(s, k)

                    }

                }
            } else {
                self.clone()
            }
        } else {
            self.clone()
        }
    }
}

impl MenuLevels<'_> {

    fn visited_by_level1(&mut self, s : &'static str, k : KeyCode) -> MenuLevel {

        let mut count = 0;
        let mut iter = self.c.clone().take(self.size);

        for i in iter {
            match i {

                MenuLevel::Level1(t) => {
                    eprintln!("Loop menu is {:?}", i);
                    if ( s != *t ) && ( k == KeyCode::Tab ) {
                        eprintln!("Count is {:?}", count);
                        self.c.nth(( if count < 1 {self.size-1} else {count - 1}));
                        return MenuLevel::Level1(t);
                    }
                },

                MenuLevel::Level2(t, entry, exit) => {
                    eprintln!("Loop menu is {:?}", i);
                    if s == *t {
                        if k == *entry {
                            self.c.nth(( if count < 1 {self.size-1} else {count - 1}));
                            return MenuLevel::Level2(s, *entry, *exit);
                        }
                    }
                }
            }
            count += 1;
        }
        return self.c.clone().next().unwrap().clone();
    }

    fn visited_by_level2(&mut self, s : &'static str, k : KeyCode) -> MenuLevel {

        let mut count = 0;
        let mut iter = self.c.clone().take(self.size);

        for i in iter {
            match i {

                MenuLevel::Level2(t, entry, exit) => {
                    if k == *exit {
                        let n = self.c.clone()
                            .position(|x| { *x == MenuLevel::Level1(t) })
                            .expect("Backtracking to Level 1 menu error");
                        self.c.nth(( if n < 1 {self.size-1} else {n - 1}));
                        return MenuLevel::Level1(t);
                    } else {
                        return MenuLevel::Level2(t, *entry, *exit);
                    }
                },
                MenuLevel::Level1(t) => { return MenuLevel::Level1(t); }
            }
            count += 1;
        }
        MenuLevel::Level1(s)
    }
}