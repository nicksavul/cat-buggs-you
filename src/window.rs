use std::mem::{size_of, zeroed};
use std::ptr::null_mut;

use winapi::ctypes::{c_int, c_void};
use winapi::Interface;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::guiddef::IID;
use winapi::shared::minwindef::{ATOM, FALSE, HINSTANCE, LPARAM, LRESULT, TRUE, UINT, WPARAM};
use winapi::shared::windef::{HBRUSH, HGDIOBJ, HWND, RECT};
use winapi::shared::windowsx::{GET_X_LPARAM, GET_Y_LPARAM};
use winapi::shared::winerror::S_OK;
use winapi::um::combaseapi::{CLSCTX_INPROC, CoCreateInstance};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::processthreadsapi::{GetCurrentThreadId, GetThreadId};
use winapi::um::wingdi::{CreateCompatibleDC, CreatePen,
                         CreateSolidBrush, GdiTransparentBlt, GetStockObject,
                         PS_SOLID, Rectangle, RGB, SelectObject};
use winapi::um::winnt::HRESULT;
use winapi::um::winuser::{BeginPaint, CallNextHookEx, COLOR_WINDOW, CREATESTRUCTW,
                          CreateWindowExW, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT,
                          DefWindowProcW, DispatchMessageW, EndPaint, FillRect, FindWindowW,
                          GetClientRect, GetDC, GetMessageW, GetWindow, GetWindowLongPtrW,
                          GetWindowThreadProcessId, GW_HWNDPREV, GWL_EXSTYLE, GWLP_USERDATA,
                          InvalidateRect, KillTimer, LWA_ALPHA, LWA_COLORKEY, MK_LBUTTON,
                          MOUSEHOOKSTRUCT, MOUSEHOOKSTRUCTEX, MSG, PAINTSTRUCT, RDW_ERASENOW,
                          RDW_INTERNALPAINT, RDW_INVALIDATE, RDW_UPDATENOW, RedrawWindow, RegisterClassExW,
                          ReleaseDC, SetLayeredWindowAttributes, SetTimer, SetWindowLongPtrW,
                          SetWindowLongW, SetWindowsHookExW, ShowWindow, SW_SHOWMAXIMIZED,
                          TranslateMessage, UnhookWindowsHookEx, UpdateWindow, WH_MOUSE, WH_MOUSE_LL,
                          WNDCLASSEXW, WNDCLASSW, WS_EX_LAYERED, WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
                          WS_EX_TRANSPARENT, WS_POPUP};
use winapi::um::winuser::{
    WM_CLOSE,
    WM_CREATE,
    WM_DESTROY,
    WM_KEYDOWN,
    WM_LBUTTONDOWN,
    WM_LBUTTONUP,
    WM_MOUSEMOVE,
    WM_NCHITTEST,
    WM_PAINT,
    WM_SIZE,
    WM_SIZING,
    WM_TIMER,
    WM_RBUTTONDOWN,
    WM_RBUTTONUP
};

use crate::app_state::{AppState};
use crate::app_state::Spritesheet::{BMP, RAW};
use crate::util::{make_bitmap, SHCreateMemStream, to_wide};
use crate::util::HIStream;

// window class name
const CLASS_NAME: &'static str = "overlay-window-class";


/// This is a window struct,
pub struct Window {
    /// Stores a window handle to be accessible outside of WinAPI callbacks
    hwnd: HWND,
    /// inner struct to keep data to be passed to WinAPI callback
    app_state: AppState,
}



impl Window {

    /// WinAPI callback, is fired on system message received by window
    unsafe extern "system" fn WNDPROC(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {

        /// matching message code
        match msg {

            /// On window created
            WM_CREATE => {

                /// unpack and relocate the AppState reference
                let create_struct: &mut CREATESTRUCTW =  &mut *(lparam as usize as *mut CREATESTRUCTW) ;
                let app_state: &mut AppState = &mut *(create_struct.lpCreateParams as usize as *mut AppState);
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, app_state as *mut _ as LONG_PTR);


                /// If frames were not decoded, decode and store them
                if let RAW(buf) = &app_state.bitmap_spritesheet {
                    let thisDC = GetDC(hwnd);

                    let memDC = CreateCompatibleDC(thisDC);
                    assert!(!memDC.is_null());

                    SelectObject(memDC,
                    make_bitmap(
                        thisDC,
                        buf
                    ) as HGDIOBJ);

                    app_state.bitmap_spritesheet = BMP(
                        memDC
                    );

                    ReleaseDC(hwnd, thisDC);
                }

                /// start the timer for render event
                app_state.render_timer_id = SetTimer(hwnd, 1, 25, None);

                0
            },

            /// on paint
            WM_PAINT => {
                let app_state: &mut AppState =  &mut *(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as usize as *mut AppState);

                let mut paint_struct: PAINTSTRUCT = zeroed();
                let dc = BeginPaint(hwnd, &mut paint_struct);

                /// if bitmap frames are available
                if let BMP(memDC) = app_state.bitmap_spritesheet {
                    app_state.update();

                    for (RECT{left: screen_x, top: screen_y, right: screen_w, bottom: screen_h},
                         RECT{left: tilemap_x, top: tilemap_y, right: tilemap_w, bottom: tilemap_h}) in app_state.render() {
                        assert_eq!(TRUE,
                            GdiTransparentBlt(
                                dc, screen_x, screen_y, screen_w, screen_h,
                                memDC, tilemap_x, tilemap_y, tilemap_w, tilemap_h, RGB(255, 255, 255)
                            )
                        );

                    }

                }




                EndPaint(hwnd, &paint_struct);


                DefWindowProcW(hwnd, msg, wparam, lparam)

            },
            /// on window resized
            WM_SIZE => {
                let app_state: &mut AppState = &mut *(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as usize as *mut AppState);
                GetClientRect(hwnd, &mut app_state.client_rect);
                DefWindowProcW(hwnd, msg, wparam, lparam)
            },
            /// on window closed
            WM_DESTROY | WM_CLOSE => {
                let app_state: &mut AppState = &mut *(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as usize as *mut AppState);
                KillTimer(hwnd, app_state.render_timer_id);
                UnhookWindowsHookEx(app_state.hhook);
                DefWindowProcW(hwnd, msg, wparam, lparam)
            },
            /// on timer
            WM_TIMER => {
                let app_state: &mut AppState = &mut *(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as usize as *mut AppState);
                InvalidateRect(hwnd, &app_state.client_rect, TRUE);
                UpdateWindow(hwnd);
                0
            },


            WM_MOUSEMOVE => {
                let app_state: &mut AppState = &mut *(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as usize as *mut AppState);

                if wparam == MK_LBUTTON {
                    app_state.mouse.l_down();

                } else {
                    app_state.mouse.l_up();

                }

                app_state.mouse.set_pos(
                    (GET_X_LPARAM(lparam), GET_Y_LPARAM(lparam))
                );

                0
            },



            msg => DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }

    pub fn new(window_name: &str, mut app_state: AppState) -> Self {
        
        let window_name: Vec<u16> = to_wide(window_name);
        let class_name: Vec<u16> = to_wide(CLASS_NAME);

        let hinstance: HINSTANCE = unsafe { GetModuleHandleW(null_mut()) };

        let background_brush: HBRUSH = unsafe { CreateSolidBrush(RGB(255, 255, 255)) };
        
        let window_class: WNDCLASSEXW = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as UINT,
            style: CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(Self::WNDPROC),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: background_brush,
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: null_mut()
        };


        let _atom: ATOM = unsafe { RegisterClassExW(&window_class) };

        let hwnd: HWND = unsafe {
            CreateWindowExW(WS_EX_TOOLWINDOW | WS_EX_TOPMOST | WS_EX_LAYERED, // | WS_EX_TRANSPARENT | WS_EX_LAYERED,
                            class_name.as_ptr(),
                            window_name.as_ptr(),
                            WS_POPUP,
                            CW_USEDEFAULT,
                            CW_USEDEFAULT,
                            CW_USEDEFAULT,
                            CW_USEDEFAULT,
                            null_mut(),
                            null_mut(),
                            hinstance,
                            &mut app_state as *mut _ as _)
        };



        unsafe { ShowWindow(hwnd, SW_SHOWMAXIMIZED); }
        unsafe { SetLayeredWindowAttributes(hwnd, RGB(255, 255, 255), 0, LWA_COLORKEY); }


        Self {
            hwnd,
            app_state
        }
    }


    pub fn run(&mut self) {

        let mut msg: MSG = unsafe { zeroed() };
        while unsafe { GetMessageW(&mut msg, self.hwnd, 0, 0) }.is_positive() {
            unsafe { TranslateMessage(&msg); }
            unsafe { DispatchMessageW(&msg); }

        }

    }


}