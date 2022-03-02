use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use winapi::um::objidlbase::IStream;
use winapi::shared::minwindef::{BYTE, UINT, LPVOID};
use winapi::um::winnt::HRESULT;
use winapi::shared::windef::{HBITMAP, HDC};
use std::ptr::null_mut;
use winapi::shared::winerror::S_OK;
use std::mem::{zeroed, size_of};
use winapi::um::combaseapi::{CoCreateInstance, CLSCTX_INPROC, CoUninitialize};
use winapi::um::wincodec::{CLSID_WICImagingFactory, IWICImagingFactory, IWICFormatConverter, IWICBitmapDecoder, WICDecodeMetadataCacheOnDemand, IWICBitmapFrameDecode, IWICBitmapSource, GUID_WICPixelFormat24bppBGR, WICBitmapDitherTypeNone, WICBitmapPaletteTypeMedianCut, WICRect, GUID_WICPixelFormat32bppBGR};
use winapi::shared::guiddef::IID;
use winapi::Interface;
use winapi::ctypes::{c_void, c_double};
use winapi::um::wingdi::{BITMAPINFOHEADER, BI_RGB, GetDeviceCaps, HORZRES, VERTRES, BITMAPINFO, RGBQUAD, CreateDIBitmap, CBM_INIT, DIB_RGB_COLORS};


pub fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}


pub type HIStream = *const IStream;


#[link(name="Shlwapi")]
extern "system" {
    pub fn SHCreateMemStream(pbInit: *const BYTE, cbInit: UINT) -> HIStream;
}

#[link(name="Ole32")]
extern "system" {
    pub fn CoInitialize(pvReserved: LPVOID) -> HRESULT;
}


pub unsafe fn make_bitmap(hdc_ctx: HDC, data: &[u8]) -> HBITMAP {
    assert_eq!(S_OK, CoInitialize(null_mut()));

    let mut pFactory: *mut c_void = zeroed();
    assert_eq!(S_OK, CoCreateInstance(&CLSID_WICImagingFactory as *const IID,
                                   null_mut(),
                                   CLSCTX_INPROC,
                                   &IWICImagingFactory::uuidof(),
                                   &mut pFactory as *mut *mut c_void));
    let pFactory: &mut IWICImagingFactory = (pFactory as *mut _ as *mut IWICImagingFactory).as_mut().unwrap();

    let mut pConverter: *mut IWICFormatConverter = unsafe {zeroed()};
    assert_eq!(S_OK,
               pFactory
                   .CreateFormatConverter(&mut pConverter as *mut *mut IWICFormatConverter));
    let pConverter: &mut IWICFormatConverter = pConverter.as_mut().unwrap();


    let hStream: HIStream = SHCreateMemStream(data.as_ptr(), data.len() as UINT);

    let mut pDecoder: *mut IWICBitmapDecoder = zeroed();
    assert_eq!(S_OK, pFactory.CreateDecoderFromStream(
        hStream,
        null_mut(),
        WICDecodeMetadataCacheOnDemand, &mut pDecoder as *mut *mut IWICBitmapDecoder,
    ));
    let pDecoder: &mut IWICBitmapDecoder = pDecoder.as_mut().unwrap();

    let mut idx: UINT = zeroed();
    assert_eq!(S_OK, pDecoder.GetFrameCount(&mut idx));
    let idx: UINT = idx - 1;

    let mut pFrame: *mut IWICBitmapFrameDecode = zeroed();
    assert_eq!(S_OK, pDecoder.GetFrame(
        idx,
        &mut pFrame as *mut *mut IWICBitmapFrameDecode
    ));
    let pFrame: &mut IWICBitmapFrameDecode = pFrame.as_mut().unwrap();

    let (mut width, mut height): (UINT, UINT) = (0, 0);
    assert_eq!(S_OK, pFrame.GetSize(&mut width, &mut height));

    dbg!((width, height));

    assert_eq!(S_OK, pConverter.Initialize(
    &**pFrame as *const IWICBitmapSource,
    &GUID_WICPixelFormat24bppBGR,
    WICBitmapDitherTypeNone,
    null_mut(),
    0.0 as c_double,
    WICBitmapPaletteTypeMedianCut)
    );

    let mut pix_buf: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 3);
    pix_buf.fill(0);


    assert_eq!(S_OK, pFrame.CopyPixels(
            null_mut(),
            width * 3,
            width * height * 3,
            pix_buf.as_mut_ptr()
        ));

    let bmp_info_header: BITMAPINFOHEADER = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32),
        biPlanes: 1,
        biBitCount: 24,
        biCompression: BI_RGB,
        biSizeImage: 0,
        biXPelsPerMeter: GetDeviceCaps(hdc_ctx, HORZRES),
        biYPelsPerMeter: GetDeviceCaps(hdc_ctx, VERTRES),
        biClrUsed: 0,
        biClrImportant: 0
    };

    let bmp_info: BITMAPINFO = BITMAPINFO {
        bmiHeader: bmp_info_header.clone(),
        bmiColors: [RGBQUAD::default();1],
    };

    let hbitmap = CreateDIBitmap(hdc_ctx,
                   &bmp_info_header,
                   CBM_INIT,
                   pix_buf.as_ptr() as *const _ as *const c_void,
                   &bmp_info,
                   DIB_RGB_COLORS);
    assert!(!hbitmap.is_null());

    CoUninitialize();
    hbitmap

}