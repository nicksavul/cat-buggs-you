use winapi::ctypes::c_int;

pub struct Action {
    pub delta: (c_int, c_int),
    pub frames: Vec<(c_int, c_int, c_int, c_int)>,
    pub select: Option<u32>,
}

impl Action {
    pub fn new(dx: c_int, dy: c_int) -> Self {
        Self {
            delta: (dx, dy),
            frames: vec![],
            select: None
        }
    }

    pub fn with_frame(mut self, f: (c_int, c_int, c_int, c_int)) -> Self {
        self.frames.push(f);
        self
    }

    pub fn label(mut self, select: u32) -> Self {
        self.select = Some(select);
        self
    }
}