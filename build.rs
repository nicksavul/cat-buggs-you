fn main() {

}

// use cat_2_packing::frame_packing::{Content, Anim, pack};
//
// fn main() {
//
//     let mut content: Content = Vec::new();
//
//     content.push(
//         Anim::new().named("forward")
//             .with_displacement([0, 3])
//             .with_frame("res/forward (1).bmp", 0)
//             .unwrap()
//             .with_frame("res/forward (2).bmp", 1)
//             .unwrap()
//             .with_frame("res/forward (3).bmp", 2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("backward")
//             .with_displacement([0, -3])
//             .with_frame("res/backward (1).bmp",0)
//             .unwrap()
//             .with_frame("res/backward (2).bmp",1)
//             .unwrap()
//             .with_frame("res/backward (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("left")
//             .with_displacement([3, 0])
//             .with_frame("res/left (1).bmp",0)
//             .unwrap()
//             .with_frame("res/left (2).bmp",1)
//             .unwrap()
//             .with_frame("res/left (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("right")
//             .with_displacement([-3, 0])
//             .with_frame("res/right (1).bmp",0)
//             .unwrap()
//             .with_frame("res/right (2).bmp",1)
//             .unwrap()
//             .with_frame("res/right (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("D1")
//             .with_displacement([3, 1])
//             .with_frame("res/D1 (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D1 (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D1 (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D2")
//             .with_displacement([-3, 1])
//             .with_frame("res/D2 (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D2 (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D2 (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D3")
//             .with_displacement([-3, -1])
//             .with_frame("res/D3 (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D3 (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D3 (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D4")
//             .with_displacement([3, -1])
//             .with_frame("res/D4 (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D4 (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D4 (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("forward run")
//             .with_displacement([0, 5])
//             .with_frame("res/forward run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/forward run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/forward run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("backward run")
//             .with_displacement([0, -5])
//             .with_frame("res/backward run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/backward run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/backward run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("left run")
//             .with_displacement([5, 0])
//             .with_frame("res/left run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/left run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/left run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("right run")
//             .with_displacement([-5, 0])
//             .with_frame("res/right run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/right run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/right run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D1 run")
//             .with_displacement([5, 3])
//             .with_frame("res/D1 run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D1 run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D1 run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D2 run")
//             .with_displacement([-5, 3])
//             .with_frame("res/D2 run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D2 run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D2 run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D3 run")
//             .with_displacement([-5, -3])
//             .with_frame("res/D3 run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D3 run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D3 run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("D4 run")
//             .with_displacement([5, -3])
//             .with_frame("res/D4 run (1).bmp",0)
//             .unwrap()
//             .with_frame("res/D4 run (2).bmp",1)
//             .unwrap()
//             .with_frame("res/D4 run (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("sleepA")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepA (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepA (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepA (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("sleepB")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepB (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepB (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepB (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("sleepC")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepC (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepC (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepC (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("sleepD")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepD (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepD (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepD (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//     content.push(
//         Anim::new().named("sleepE")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepE (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepE (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepE (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//     content.push(
//         Anim::new().named("sleepF")
//             .with_displacement([0, 0])
//             .with_frame("res/sleepF (1).bmp",0)
//             .unwrap()
//             .with_frame("res/sleepF (2).bmp",1)
//             .unwrap()
//             .with_frame("res/sleepF (3).bmp",2)
//             .unwrap()
//             .allow_random_select()
//     );
//
//
//
//
//     pack(&content, ".//data")
//
// }