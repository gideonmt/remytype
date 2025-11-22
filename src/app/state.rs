use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Menu,
    Test,
    Results,
}

pub struct App {
    pub mode: AppMode,
    pub menu_selection: usize,
    pub test_text: String,
    pub current_input: String,
    pub current_pos: usize,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub errors: usize,
    pub wpm: f64,
    pub accuracy: f64,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            menu_selection: 0,
            test_text: String::from("the quick brown fox jumps over the lazy dog"),
            current_input: String::new(),
            current_pos: 0,
            start_time: None,
            end_time: None,
            errors: 0,
            wpm: 0.0,
            accuracy: 0.0,
        }
    }

    pub fn menu_up(&mut self) {
        if self.menu_selection > 0 {
            self.menu_selection -= 1;
        }
    }

    pub fn menu_down(&mut self) {
        if self.menu_selection < 2 {
            self.menu_selection += 1;
        }
    }

    pub fn start_test(&mut self) {
        self.mode = AppMode::Test;
        self.current_input.clear();
        self.current_pos = 0;
        self.start_time = Some(Instant::now());
        self.end_time = None;
        self.errors = 0;
    }

    pub fn type_char(&mut self, c: char) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        if self.current_pos < self.test_text.len() {
            self.current_input.push(c);
            
            let expected = self.test_text.chars().nth(self.current_pos).unwrap();
            if c != expected {
                self.errors += 1;
            }
            
            self.current_pos += 1;

            if self.current_pos >= self.test_text.len() {
                self.finish_test();
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.current_pos > 0 {
            self.current_input.pop();
            self.current_pos -= 1;
        }
    }

    fn finish_test(&mut self) {
        self.end_time = Some(Instant::now());
        if let Some(start) = self.start_time {
            let duration = self.end_time.unwrap().duration_since(start);
            let minutes = duration.as_secs_f64() / 60.0;
            let words = self.test_text.split_whitespace().count() as f64;
            self.wpm = words / minutes;
            self.accuracy = ((self.test_text.len() - self.errors) as f64 / self.test_text.len() as f64) * 100.0;
        }
        self.mode = AppMode::Results;
    }

    pub fn return_to_menu(&mut self) {
        self.mode = AppMode::Menu;
        self.current_input.clear();
        self.current_pos = 0;
        self.start_time = None;
        self.end_time = None;
    }
}
