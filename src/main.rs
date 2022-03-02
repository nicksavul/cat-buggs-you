#![feature(with_options)]
#![feature(stmt_expr_attributes)]

use winapi::um::libloaderapi::GetModuleHandleA;
use std::ptr::null_mut;
use std::ffi::c_void;
use crate::app_state::{AppState};
use crate::action::Action;

mod window;
mod util;
mod app_state;
mod click_phase;
mod cat;
mod action;


fn initial_state() -> AppState {
    app_state::AppState::initial()
        .with_spritesheet(include_bytes!("../res/src-24.bmp"))

        .with_action( // right

            Action::new(3, 0)
                .with_frame((0, 2, 192, 192))
                .with_frame((1, 2, 192, 192))
                .with_frame((2, 2, 192, 192))

        )


        .with_action( // left

                      Action::new(-3, 0)
                          .with_frame((0, 1, 192, 192))
                          .with_frame((1, 1, 192, 192))
                          .with_frame((2, 1, 192, 192))

        )


        .with_action( // forward

                      Action::new(0, -3)
                          .with_frame((0, 3, 192, 192))
                          .with_frame((1, 3, 192, 192))
                          .with_frame((2, 3, 192, 192))

        )


        .with_action( // backward

                      Action::new(0, 3)
                          .with_frame((0, 0, 192, 192))
                          .with_frame((1, 0, 192, 192))
                          .with_frame((2, 0, 192, 192))

        )


        .with_action( // right-run

                      Action::new(5, 0)
                          .with_frame((6, 2, 192, 192))
                          .with_frame((7, 2, 192, 192))
                          .with_frame((8, 2, 192, 192))

        )


        .with_action( // left-run

                      Action::new(-5, 0)
                          .with_frame((6, 1, 192, 192))
                          .with_frame((7, 1, 192, 192))
                          .with_frame((8, 1, 192, 192))

        )


        .with_action( // forward-run

                      Action::new(0, -5)
                          .with_frame((6, 3, 192, 192))
                          .with_frame((7, 3, 192, 192))
                          .with_frame((8, 3, 192, 192))

        )


        .with_action( // backward-run

                      Action::new(0, 5)
                          .with_frame((6, 0, 192, 192))
                          .with_frame((7, 0, 192, 192))
                          .with_frame((8, 0, 192, 192))

        )








        .with_action( // top-right

                      Action::new(3, -2)
                          .with_frame((3, 3, 192, 192))
                          .with_frame((4, 3, 192, 192))
                          .with_frame((5, 3, 192, 192))

        )


        .with_action( // top-left

                      Action::new(-3, -2)
                          .with_frame((3, 1, 192, 192))
                          .with_frame((4, 1, 192, 192))
                          .with_frame((5, 1, 192, 192))

        )


        .with_action( // bottom-left

                      Action::new(-3, 2)
                          .with_frame((3, 0, 192, 192))
                          .with_frame((4, 0, 192, 192))
                          .with_frame((5, 0, 192, 192))

        )


        .with_action( // bottom-right

                      Action::new(3, 2)
                          .with_frame((3, 2, 192, 192))
                          .with_frame((4, 2, 192, 192))
                          .with_frame((5, 2, 192, 192))

        )


        .with_action( // top-left-run

                      Action::new(-5, -3)
                          .with_frame((9, 1, 192, 192))
                          .with_frame((10, 1, 192, 192))
                          .with_frame((11, 1, 192, 192))

        )


        .with_action( // top-right-run

                      Action::new(5, -3)
                          .with_frame((9, 3, 192, 192))
                          .with_frame((10, 3, 192, 192))
                          .with_frame((11, 3, 192, 192))

        )


        .with_action( // bottom-right-run

                      Action::new(5, 3)
                          .with_frame((9, 2, 192, 192))
                          .with_frame((10, 2, 192, 192))
                          .with_frame((11, 2, 192, 192))

        )


        .with_action( // bottom-left-run

                      Action::new(-5, 3)
                          .with_frame((9, 0, 192, 192))
                          .with_frame((10, 0, 192, 192))
                          .with_frame((11, 0, 192, 192))

        )






        .with_action( // sleep-A

                      Action::new(0, 0)
                          .with_frame((0, 4, 192, 192))
                          .with_frame((1, 4, 192, 192))
                          .with_frame((2, 4, 192, 192))

        )


        .with_action( // sleep-B

                      Action::new(0, 0)
                          .with_frame((0, 5, 192, 192))
                          .with_frame((1, 5, 192, 192))
                          .with_frame((2, 5, 192, 192))

        )


        .with_action( // sleep-C

                      Action::new(0, 0)
                          .with_frame((0, 6, 192, 192))
                          .with_frame((1, 6, 192, 192))
                          .with_frame((2, 6, 192, 192))

        )


        .with_action( // sleep-D

                      Action::new(0, 0)
                          .with_frame((0, 7, 192, 192))
                          .with_frame((1, 7, 192, 192))
                          .with_frame((2, 7, 192, 192))

        )


        .with_action( // sleep-E

                      Action::new(0, 0)
                          .with_frame((6, 4, 192, 192))
                          .with_frame((7, 4, 192, 192))
                          .with_frame((8, 4, 192, 192))

        )



        .with_action( // sleep-F

                      Action::new(0, 0)
                          .with_frame((6, 5, 192, 192))
                          .with_frame((7, 5, 192, 192))
                          .with_frame((8, 5, 192, 192))

        )





        .with_action( // emergency-food

                      Action::new(0, 0)
                          .with_frame((4, 7, 192, 192))
                          .label(1)

        )

        .with_cat(0, 0, 0.5)
}

fn main() {

    window::Window::new("sbcat-2", initial_state())
        .run();
}
