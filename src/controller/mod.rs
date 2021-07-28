use std::collections::{HashMap, VecDeque, HashSet};

pub mod binding_args;

#[derive(Copy, Clone, PartialEq)]
pub enum KeyEventType {
    PRESSED,
    RELEASED,
}

#[derive(Copy, Clone, PartialEq)]
pub struct KeyEvent<T> {
    pub key: T,
    pub etype: KeyEventType,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyPressMode {
    PRESSED,
    RELEASED,
    HELD,
    NONE,
}

// #[derive(Copy, Clone)]
pub struct ControllerAction<TCONTRLLLED, VARGSEXTERNAL, TBINDARG: Copy + Clone> {
    pub func: fn(& mut TCONTRLLLED, &VARGSEXTERNAL, KeyPressMode, TBINDARG),
    pub bind_arg: TBINDARG,
}

impl<TCONTRLLLED, VARGSEXTERNAL, TBINDARG: Copy + Clone> ControllerAction<TCONTRLLLED, VARGSEXTERNAL, TBINDARG> {
    pub fn perform(&self, controlled: &mut TCONTRLLLED, args: &VARGSEXTERNAL, key_mode: KeyPressMode) {
        (self.func)(controlled, args, key_mode, self.bind_arg);
    }
}

pub struct Controller<TCONTRLLLED, VARGSEXTERNAL, TINPUT: std::cmp::Eq + std::hash::Hash, TBINDARG: Copy + Clone> {
    bindings: HashMap<TINPUT, ControllerAction< TCONTRLLLED, VARGSEXTERNAL, TBINDARG>>,
    events: VecDeque<KeyEvent<TINPUT>>,
    keys_press_mode: HashMap<TINPUT, KeyPressMode>,
    active_keys: HashSet<TINPUT>
}

impl<TCONTRLLLED, VARGSEXTERNAL, TINPUT: std::cmp::Eq + std::hash::Hash + Copy + Clone, TBINDARG: Copy + Clone> Controller<TCONTRLLLED, VARGSEXTERNAL, TINPUT, TBINDARG> {
    pub fn new() -> Self {
        Self {
            bindings:        HashMap::new(),
            events:          VecDeque::new(), 
            keys_press_mode: HashMap::new(),
            active_keys:     HashSet::new(),
        }
    }

    pub fn bind_key(&mut self, key: TINPUT, action: ControllerAction<TCONTRLLLED, VARGSEXTERNAL, TBINDARG>) -> &mut Self {
        self.bindings.insert(key, action);
        
        self
    }

    pub fn key_event(&mut self, event: KeyEvent<TINPUT>) {
        self.events.push_back(event);
    }

    pub fn get_key_mode(&self, key: TINPUT) -> KeyPressMode {
        match self.keys_press_mode.get(&key) {
            Some(&mode) => mode,
            None => KeyPressMode::NONE,
        }
    }

    fn process_pressed_and_released(&mut self) {
        let mut keys_to_remove: Vec<TINPUT> = Vec::new();

        for &key in self.active_keys.iter() {
            match self.get_key_mode(key) {
                KeyPressMode::PRESSED => {
                    self.keys_press_mode.insert(key, KeyPressMode::HELD);
                }

                KeyPressMode::RELEASED => {
                    self.keys_press_mode.insert(key, KeyPressMode::NONE);
                    keys_to_remove.push(key);
                }

                _ => {}
            }
        }

        for key in keys_to_remove {
            self.active_keys.remove(&key);
        }
    }

    pub fn controller_tick(&mut self) {
        self.process_pressed_and_released();

        // process new events
        for event in self.events.iter() {
            match event.etype {
                KeyEventType::PRESSED  => {
                    if !self.active_keys.contains(&event.key) {
                        self.active_keys.insert(event.key);
                        self.keys_press_mode.insert(event.key, KeyPressMode::PRESSED);
                    }
                }
                KeyEventType::RELEASED => {
                    self.keys_press_mode.insert(event.key, KeyPressMode::RELEASED);
                }
            }
        }

        self.events.clear();
    }

    pub fn bindings_tick(&mut self, controlled: &mut TCONTRLLLED, args: &VARGSEXTERNAL) {
        for &key in self.active_keys.iter() {
            match self.bindings.get(&key) {
                Some(action) => action.perform(controlled, args, self.get_key_mode(key)),
                None         => {}
            }
        }
    }
}
