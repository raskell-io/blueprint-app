//! The WebAssembly boundary: a C ABI over `app-core`, nothing else. JavaScript asks for a buffer, the module renders
//! into its own linear memory, and JavaScript reads the pixels straight out of `memory.buffer`, with no copying
//! through glue code and no wasm-bindgen.

use app_core::{View, render_rows};

static mut BUFFER: Vec<u8> = Vec::new();

/// Make room for `width * height` RGBA pixels and return where they start in linear memory.
#[unsafe(no_mangle)]
pub extern "C" fn buffer(width: u32, height: u32) -> *mut u8 {
    let len = width as usize * height as usize * 4;
    // SAFETY: the module is single-threaded and this is the only place BUFFER is resized.
    #[allow(static_mut_refs)]
    unsafe {
        BUFFER.resize(len, 0);
        BUFFER.as_mut_ptr()
    }
}

/// Render rows `row_start .. row_start + row_count` of the view into the buffer from the last `buffer` call (same
/// width and height). JavaScript calls this band by band: V8 runs a WebAssembly function in its baseline tier until
/// it has been called often enough, and a single long call never moves up, so short calls are faster overall.
#[unsafe(no_mangle)]
pub extern "C" fn render_view(
    center_x: f64,
    center_y: f64,
    span: f64,
    max_iter: u32,
    width: u32,
    height: u32,
    row_start: u32,
    row_count: u32,
) {
    let view = View { center_x, center_y, span, max_iter };
    let (w, start, count) = (width as usize, row_start as usize, row_count as usize);
    // SAFETY: as above; `buffer(width, height)` sized it for the whole image, and the band lies inside it.
    #[allow(static_mut_refs)]
    unsafe {
        let band = &mut BUFFER[start * w * 4..(start + count) * w * 4];
        render_rows(view, w, height as usize, start, band);
    }
}
