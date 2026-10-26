//! The app's CPU-heavy logic, in plain Rust. It knows nothing about WebAssembly or the browser, so it is tested with
//! `cargo test` like any other crate, and the same code could run on a server or a desktop.

/// The region of the complex plane to render, and how hard to look.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub center_x: f64,
    pub center_y: f64,
    /// Width of the view in the complex plane; height follows from the aspect ratio.
    pub span: f64,
    pub max_iter: u32,
}

/// Iterations until z = z² + c escapes |z| > 2, or `max_iter` if it never does (the point is in the set).
#[inline]
pub fn escape_time(cx: f64, cy: f64, max_iter: u32) -> u32 {
    let (mut x, mut y, mut i) = (0.0f64, 0.0f64, 0);
    while i < max_iter && x * x + y * y <= 4.0 {
        let xt = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = xt;
        i += 1;
    }
    i
}

/// Render `view` into `out`, RGBA bytes, `width * height * 4` long. Points in the set are dark; the rest are shaded
/// by how quickly they escape, in the site's teal.
pub fn render(view: View, width: usize, height: usize, out: &mut [u8]) {
    assert_eq!(out.len(), width * height * 4, "the buffer must hold width * height RGBA pixels");
    render_rows(view, width, height, 0, out);
}

/// Render the rows from `row_start` on, as many as `out` holds (`rows * width * 4` bytes), of a `width * height` image.
/// Rendering in bands lets the caller keep each call short.
pub fn render_rows(view: View, width: usize, height: usize, row_start: usize, out: &mut [u8]) {
    assert_eq!(out.len() % (width * 4), 0, "the buffer must hold whole rows");
    let scale = view.span / width as f64;
    let (x0, y0) = (view.center_x - view.span / 2.0, view.center_y - scale * height as f64 / 2.0);
    for (r, row) in out.chunks_exact_mut(width * 4).enumerate() {
        let cy = y0 + (row_start + r) as f64 * scale;
        for (px, pixel) in row.as_chunks_mut::<4>().0.iter_mut().enumerate() {
            let i = escape_time(x0 + px as f64 * scale, cy, view.max_iter);
            let [r, g, b] = shade(i, view.max_iter);
            *pixel = [r, g, b, 255];
        }
    }
}

fn shade(i: u32, max_iter: u32) -> [u8; 3] {
    if i >= max_iter {
        return [11, 20, 18];
    }
    let t = (i as f64 / max_iter as f64).sqrt();
    [(114.0 * t) as u8, (40.0 + 175.0 * t) as u8, (50.0 + 170.0 * t) as u8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin_is_in_the_set_and_two_is_not() {
        assert_eq!(escape_time(0.0, 0.0, 500), 500);
        assert!(escape_time(2.0, 2.0, 500) < 2);
    }

    #[test]
    fn render_fills_every_pixel_opaque() {
        let view = View { center_x: -0.5, center_y: 0.0, span: 3.0, max_iter: 64 };
        let mut out = vec![0u8; 8 * 6 * 4];
        render(view, 8, 6, &mut out);
        assert!(out.as_chunks::<4>().0.iter().all(|p| p[3] == 255));
    }

    #[test]
    fn bands_add_up_to_the_whole_image() {
        let view = View { center_x: -0.5, center_y: 0.0, span: 3.0, max_iter: 64 };
        let (w, h) = (10, 8);
        let mut whole = vec![0u8; w * h * 4];
        render(view, w, h, &mut whole);
        let mut banded = vec![0u8; w * h * 4];
        for (i, band) in banded.chunks_mut(w * 3 * 4).enumerate() {
            render_rows(view, w, h, i * 3, band);
        }
        assert_eq!(whole, banded);
    }

    #[test]
    #[should_panic(expected = "width * height")]
    fn render_rejects_a_wrong_buffer() {
        let view = View { center_x: 0.0, center_y: 0.0, span: 1.0, max_iter: 8 };
        render(view, 4, 4, &mut [0u8; 3]);
    }
}
