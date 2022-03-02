use std::ops::Range;
use std::ptr::null_mut;
use std::time::SystemTime;

use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use winapi::ctypes::c_int;
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::WORD;
use winapi::shared::windef::{HBITMAP, HDC, HHOOK, RECT};

use crate::app_state::Spritesheet::{NONE, RAW};
use crate::cat::{Cat, cat_current_frame_dimensions, inside, cat_next_frame_dimensions, action, motivate, cat_next_frame_idx, is_motivated};
use crate::click_phase::{Mouse};
use crate::action::Action;


pub enum Spritesheet {
    NONE,
    RAW(Vec<u8>),
    BMP(HDC),
}



pub struct AppState {
    pub client_rect: RECT,
    pub render_timer_id: UINT_PTR,
    pub bitmap_spritesheet: Spritesheet,
    pub hhook: HHOOK,

    cats: Vec<Cat>,
    actions: Vec<Action>,

    shall_update: SystemTime,
    shall_update_once_per: u128,

    pub mouse: Mouse,

    rng: ThreadRng,

}



impl AppState {
    pub fn initial() -> Self {
        Self {
            client_rect: RECT {left: 0, right: 0, top: 0, bottom: 0},
            render_timer_id: 0,
            bitmap_spritesheet: NONE,
            hhook: null_mut(),

            cats: vec![],
            actions: vec![],

            shall_update: SystemTime::now(),
            shall_update_once_per: 250,

            mouse: Mouse::new(),

            rng: thread_rng(),
        }
    }

    pub fn with_spritesheet(mut self, data: &[u8]) -> Self {
        let mut ret: Self = Self::initial();
        ret.bitmap_spritesheet = RAW(data.to_vec());
        ret

    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn with_cat(mut self, x: c_int, y: c_int, scale: f32) -> Self {
        self.cats.push(
            Cat::new(x, y, scale)
        );
        self
    }


    pub fn update(&mut self) {



        if SystemTime::now().duration_since(self.shall_update).unwrap().as_millis() > self.shall_update_once_per {
            for cat in self.cats.iter_mut() {
                let (cat_w, cat_h) = cat_next_frame_dimensions(cat, &self.actions);

                if cat.is_locked {

                    let (mx, my) = self.mouse.get_pos();
                    let (future_x, future_y) = (mx - cat.grasp_x, my - cat.grasp_y);


                    if inside(&RECT{left: future_x, top: future_y, right: future_x + cat_w, bottom: future_y + cat_h},
                              &self.client_rect) {
                        cat.x = future_x;
                        cat.y = future_y;
                    }

                    cat.current_frame = cat_next_frame_idx(cat, &self.actions);
                }

                else {

                    let (mut dx, mut dy) = self.actions[cat.current_action].delta.clone();
                    let (mut future_x, mut future_y) = (cat.x + dx, cat.y + dy);

                    if !is_motivated(cat) || !inside(&RECT { left: future_x, top: future_y, right: future_x + cat_w, bottom: future_y + cat_h },
                                                     &self.client_rect) {

                        while !is_motivated(cat) || !inside(&RECT { left: future_x, top: future_y, right: future_x + cat_w, bottom: future_y + cat_h },
                                                            &self.client_rect) {

                            action(None, cat, &self.actions, &mut self.rng);
                            motivate(cat, &mut self.rng);

                            let (ndx, ndy) = self.actions[cat.current_action].delta.clone();
                            dx = ndx;
                            dy = ndy;
                            future_x = cat.x + dx;
                            future_y = cat.y + dy;

                        }

                    }

                    else {

                        cat.current_frame = cat_next_frame_idx(cat, &self.actions);
                        cat.x = future_x;
                        cat.y = future_y;
                    }
                }



            }
        }
    }

    pub fn render(&self) -> Vec<(RECT, RECT)> {
        let mut ret: Vec<(RECT, RECT)> = Vec::new();

        for cat in  self.cats.iter() {
            let (cat_w, cat_h) = cat_current_frame_dimensions(cat, &self.actions);
            let frame = &self.actions[cat.current_action].frames[cat.current_frame];

            ret.push((
                RECT{left: cat.x, top: cat.y, right: cat_w, bottom: cat_h},
                RECT{left: frame.0 * 192, top: frame.1 * 192, right: frame.2, bottom: frame.3}
            ));
        }

        ret
    }

}



