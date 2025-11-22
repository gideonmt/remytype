use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Menu,
    Test,
    Results,
    Stats,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestMode {
    Words,
    Time,
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub test_mode: TestMode,
    pub word_count: usize,
    pub time_limit: u64,
    pub language: String,
    pub lines_to_display: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            test_mode: TestMode::Time,
            word_count: 50,
            time_limit: 30,
            language: "english_200".to_string(),
            lines_to_display: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserStats {
    pub total_tests: usize,
    pub average_wpm: f64,
    pub best_wpm: f64,
    pub average_accuracy: f64,
    pub total_words_typed: usize,
    pub total_time_seconds: u64,
}

impl Default for UserStats {
    fn default() -> Self {
        Self {
            total_tests: 0,
            average_wpm: 0.0,
            best_wpm: 0.0,
            average_accuracy: 0.0,
            total_words_typed: 0,
            total_time_seconds: 0,
        }
    }
}

pub struct App {
    pub mode: AppMode,
    pub menu_selection: usize,
    pub settings_selection: usize,
    pub test_text: String,
    pub current_input: String,
    pub current_pos: usize,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub errors: usize,
    pub wpm: f64,
    pub accuracy: f64,
    pub settings: Settings,
    pub user_stats: UserStats,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Menu,
            menu_selection: 0,
            settings_selection: 0,
            test_text: String::from("the quick brown fox jumps over the lazy dog"),
            current_input: String::new(),
            current_pos: 0,
            start_time: None,
            end_time: None,
            errors: 0,
            wpm: 0.0,
            accuracy: 0.0,
            settings: Settings::default(),
            user_stats: UserStats::default(),
        }
    }

    pub fn menu_up(&mut self) {
        if self.menu_selection > 0 {
            self.menu_selection -= 1;
        }
    }

    pub fn menu_down(&mut self) {
        if self.menu_selection < 3 {
            self.menu_selection += 1;
        }
    }

    pub fn settings_up(&mut self) {
        if self.settings_selection > 0 {
            self.settings_selection -= 1;
        }
    }

    pub fn settings_down(&mut self) {
        if self.settings_selection < 4 {
            self.settings_selection += 1;
        }
    }

    pub fn modify_setting(&mut self, increase: bool) {
        match self.settings_selection {
            0 => {
                self.settings.test_mode = match self.settings.test_mode {
                    TestMode::Words => TestMode::Time,
                    TestMode::Time => TestMode::Words,
                };
            }
            1 => {
                if increase && self.settings.word_count < 200 {
                    self.settings.word_count += 10;
                } else if !increase && self.settings.word_count > 10 {
                    self.settings.word_count -= 10;
                }
            }
            2 => {
                if increase && self.settings.time_limit < 300 {
                    self.settings.time_limit += 15;
                } else if !increase && self.settings.time_limit > 15 {
                    self.settings.time_limit -= 15;
                }
            }
            3 => {
                let languages = ["english_200", "english_1000", "english_10000"];
                let current_idx = languages.iter().position(|&l| l == self.settings.language).unwrap_or(0);
                let next_idx = if increase {
                    (current_idx + 1) % languages.len()
                } else {
                    if current_idx == 0 { languages.len() - 1 } else { current_idx - 1 }
                };
                self.settings.language = languages[next_idx].to_string();
            }
            4 => {
                // Modify lines to display
                if increase && self.settings.lines_to_display < 10 {
                    self.settings.lines_to_display += 1;
                } else if !increase && self.settings.lines_to_display > 1 {
                    self.settings.lines_to_display -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn open_settings(&mut self) {
        self.mode = AppMode::Settings;
    }

    pub fn open_stats(&mut self) {
        self.mode = AppMode::Stats;
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

        if self.settings.test_mode == TestMode::Time {
            if let Some(start) = self.start_time {
                if start.elapsed().as_secs() >= self.settings.time_limit {
                    self.finish_test();
                    return;
                }
            }
        }

        if self.current_pos < self.test_text.len() {
            self.current_input.push(c);
            
            let expected = self.test_text.chars().nth(self.current_pos).unwrap();
            if c != expected {
                self.errors += 1;
            }
            
            self.current_pos += 1;

            if self.settings.test_mode == TestMode::Words {
                let words_typed = self.current_input.split_whitespace().count();
                if words_typed >= self.settings.word_count {
                    self.finish_test();
                    return;
                }
            }

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
            let words = self.current_input.split_whitespace().count() as f64;
            self.wpm = words / minutes;
            self.accuracy = ((self.current_input.len() - self.errors) as f64 / self.current_input.len() as f64) * 100.0;
            
            self.user_stats.total_tests += 1;
            self.user_stats.total_words_typed += words as usize;
            self.user_stats.total_time_seconds += duration.as_secs();
            
            if self.wpm > self.user_stats.best_wpm {
                self.user_stats.best_wpm = self.wpm;
            }
            
            let total_wpm = self.user_stats.average_wpm * (self.user_stats.total_tests - 1) as f64 + self.wpm;
            self.user_stats.average_wpm = total_wpm / self.user_stats.total_tests as f64;
            
            let total_acc = self.user_stats.average_accuracy * (self.user_stats.total_tests - 1) as f64 + self.accuracy;
            self.user_stats.average_accuracy = total_acc / self.user_stats.total_tests as f64;
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
