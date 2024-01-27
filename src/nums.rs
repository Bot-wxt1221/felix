#[derive(Clone, Copy, Debug, Default)]
pub struct Num {
    pub index: usize,
    pub skip: u16,
}

#[derive(Debug)]
pub struct StateMemo {
    pub path: std::path::PathBuf,
    pub num: Num,
    pub cursor_pos: u16,
}

pub enum Move {
    Up,
    Down,
    Jump,
    List,
}

impl Num {
    pub fn new() -> Self {
        Num { index: 0, skip: 0 }
    }
    pub fn go_up(&mut self) {
        self.index -= 1;
    }
    pub fn go_down(&mut self) {
        self.index += 1;
    }
    pub fn go_bottom(&mut self, pos: usize) {
        self.index = pos;
    }
    pub fn reset(&mut self) {
        self.index = 0;
        self.skip = 0;
    }
    pub fn inc_skip(&mut self) {
        self.skip += 1;
    }
    pub fn dec_skip(&mut self) {
        self.skip -= 1;
    }
}
