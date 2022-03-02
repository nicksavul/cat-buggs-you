use winapi::shared::windef::RECT;
use winapi::ctypes::c_int;
use std::time::SystemTime;
use rand::rngs::ThreadRng;
use std::ops::Range;
use rand::thread_rng;
use rand::seq::IteratorRandom;
use std::cell::Cell;
use crate::action::Action;

pub type CatDecision = (c_int, c_int, f32, c_int, c_int, c_int, c_int);


pub struct Cat {

    pub x: i32,
    pub y: i32,
    pub scale: f32,

    pub current_action: usize,
    pub current_frame: usize,

    pub determination_timestamp: SystemTime,
    pub determination: u128,

    pub is_locked: bool,

    pub grasp_x: i32,
    pub grasp_y: i32,

}

impl Cat {

    const DETERMINATION_RANGE: Range<i32> = (7000..27000);

    pub fn new(x: i32, y: i32,
               scale: f32) -> Self {

        Self {
            x,
            y,
            scale,

            current_action: 0,
            current_frame: 0,

            determination_timestamp: SystemTime::now(),
            determination: 1,


            is_locked: false,

            grasp_x: 0,
            grasp_y: 0,

        }
    }


}


pub fn cat_next_frame_idx(cat: &Cat, catalogue: &[Action]) -> usize {
    let cat_action_len = catalogue[cat.current_action].frames.len();
    if cat.current_frame + 1 < cat_action_len {cat.current_frame + 1} else {0}

}


pub fn cat_next_frame_dimensions(cat: &Cat, catalogue: &[Action]) -> (i32, i32) {
    let frame_idx = cat_next_frame_idx(cat, catalogue);
    let (_, _, w, h) = &catalogue[cat.current_action].frames[frame_idx];
    (
        (*w as f32  * cat.scale) as i32,
        (*h as f32  * cat.scale) as i32
    )

}

pub fn cat_current_frame_dimensions(cat: &Cat, catalogue: &[Action]) -> (i32, i32) {
    let (_, _, w, h) = &catalogue[cat.current_action].frames[cat.current_frame];
    (
        (*w as f32  * cat.scale) as i32,
        (*h as f32  * cat.scale) as i32
    )
}

/// 1 in 2 => true
pub fn inside(bbox1: &RECT, bbox2: &RECT) -> bool {
    (bbox1.left > bbox2.left && bbox1.right < bbox2.right && bbox1.bottom < bbox2.bottom && bbox1.top > bbox2.top)
}

pub fn is_motivated(cat: &Cat) -> bool {
    SystemTime::now().duration_since(cat.determination_timestamp).unwrap().as_millis() < cat.determination
}


pub fn motivate(cat: &mut Cat, brains: &mut ThreadRng) {
    cat.determination_timestamp = SystemTime::now();
    cat.determination = Cat::DETERMINATION_RANGE.choose(brains).unwrap() as u128;
}

pub fn action(select: Option<u32>, cat: &mut Cat, catalogue: &[Action], brains: &mut ThreadRng) {
    if select.is_none() {
        cat.current_action = catalogue.iter().enumerate().filter_map(|(idx, act)| -> Option<usize> {
            if act.select.is_none() {
                return Some(idx);
            }
            None
        }).choose(brains).unwrap();
    } else {
        for (idx, act) in catalogue.iter().enumerate() {
            if act.select == select {
                cat.current_action = idx;
            }
        }
    }

}

