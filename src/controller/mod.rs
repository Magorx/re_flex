use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Copy, Clone, PartialEq)]
enum KeyEventType {
    PRESSED,
    RELEASED,
}

#[derive(Copy, Clone, PartialEq)]
struct KeyEvent {
    key: char,
    etype: KeyEventType,
}

#[derive(Copy, Clone, PartialEq)]
enum KeyPressMode {
    PRESSED,
    RELEASED,
    HELD,
    NONE,
}

#[derive(Copy, Clone, PartialEq)]
pub struct ControllerActionArgs {
    id: i32,
}

#[derive(Copy, Clone, PartialEq)]
pub struct ControllerAction {
    func: fn(ControllerActionArgs, KeyPressMode),
    args: ControllerActionArgs,
}

impl ControllerAction {
    pub fn perform(&self, key_mode: KeyPressMode) {
        (self.func)(self.args, key_mode);
    }
}

pub struct Controller {
    bindings: HashMap<char, ControllerAction>,
    events: VecDeque<KeyEvent>,
    keys_press_mode: Vec<KeyPressMode>,
    active_keys: HashSet<char>
}

impl Controller {
    pub fn new() -> Self {
        Self {
            bindings:        HashMap::new(),
            events:          VecDeque::new(), 
            keys_press_mode: Vec::new(),
            active_keys:     HashSet::new(),
        }
    }

    pub fn bind_key(&mut self, key: char, action: ControllerAction) {
        self.bindings.insert(key, action);
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        self.events.push_back(event);
    }

    fn process_pressed_and_released(&mut self) {
        let mut keys_to_remove: Vec<char> = Vec::new();

        for &key in self.active_keys.iter() {
            match self.keys_press_mode[key as usize] {
                KeyPressMode::PRESSED => {
                    self.keys_press_mode[key as usize] = KeyPressMode::HELD;
                }

                KeyPressMode::RELEASED => {
                    self.keys_press_mode[key as usize] = KeyPressMode::NONE;
                    keys_to_remove.push(key);
                }

                _ => {}
            }
        }

        for key in keys_to_remove {
            self.active_keys.remove(&key);
        }
    }

    pub fn tick(&mut self) {
        self.process_pressed_and_released();

        // process new events
        for event in self.events.iter() {
            match event.etype {
                KeyEventType::PRESSED  => {
                    self.active_keys.insert(event.key);
                    self.keys_press_mode[event.key as usize] = KeyPressMode::PRESSED;
                }
                KeyEventType::RELEASED => {
                    self.keys_press_mode[event.key as usize] = KeyPressMode::RELEASED;
                }
            }
        }

        self.events.clear();
        self.bindings_tick();
    }

    pub fn bindings_tick(&mut self) {
        for &key in self.active_keys.iter() {
            match self.bindings.get(&key) {
                Some(action) => action.perform(self.keys_press_mode[key as usize]),
                None         => {}
            }
        }
    }
}
