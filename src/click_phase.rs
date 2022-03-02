use winapi::ctypes::c_int;
use std::fmt::{Display, Formatter};

pub struct Mouse {
    rmb: [bool;2],
    lmb: [bool;2],
    pos: (c_int, c_int),
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            rmb: [false;2],
            lmb: [false;2],
            pos: (0, 0)
        }
    }

    pub fn l_up(&mut self) {
        self.lmb[1] = self.lmb[0];
        self.lmb[0] = false;
    }

    pub fn l_down(&mut self) {
        self.lmb[1] = self.lmb[0];
        self.lmb[0] = true;
    }

    pub fn r_up(&mut self) {
        self.rmb[1] = self.rmb[0];
        self.rmb[0] = false;
    }

    pub fn r_down(&mut self) {
        self.rmb[1] = self.rmb[0];
        self.rmb[0] = true;
    }

    pub fn set_pos(&mut self, pos: (c_int, c_int))  {
        self.pos = pos;
    }


    pub fn get_lmb(&self) -> bool {
        self.lmb[0]
    }


    pub fn get_rmb(&self) -> bool {
        self.rmb[0]
    }

    pub fn get_pos(&self) -> (c_int, c_int) {
        self.pos.clone()
    }

    pub fn clicked_l(&mut self) -> bool {
        let ret = (self.lmb[0] && !self.lmb[1]);
        if ret {
            self.lmb = [false;2];
        }
        ret
    }

    pub fn released_l(&mut self) -> bool {
        let ret = (!self.lmb[0] && self.lmb[1]);
        if ret {
            self.lmb = [false;2];
        }
        ret
    }

    pub fn clicked_r(&self) -> bool {
        self.rmb[0] && !self.rmb[1]
    }

    pub fn released_r(&self) -> bool {
        !self.rmb[0] && self.rmb[1]
    }
}

impl Display for Mouse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{\n\tRMB: {}\n\tLMB: {}\n\tPOS: ({}, {})\n}}", self.rmb[0], self.lmb[0], self.pos.0, self.pos.1))
    }
}


