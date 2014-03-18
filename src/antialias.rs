//! Describe type of antialiasing when rendering text or shapes

///  Specifies the type of antialiasing to do when rendering text or shapes.
///
/// As it is not necessarily clear from the above what advantages a particular antialias method provides, since 1.12, there is also a set of hints: CAIRO_ANTIALIAS_FAST: Allow the backend to degrade raster quality for speed CAIRO_ANTIALIAS_GOOD: A balance between speed and quality CAIRO_ANTIALIAS_BEST: A high-fidelity, but potentially slow, raster mode
///
/// These make no guarantee on how the backend will perform its rasterisation (if it even rasterises!), nor that they have any differing effect other than to enable some form of antialiasing. In the case of glyph rendering, CAIRO_ANTIALIAS_FAST and CAIRO_ANTIALIAS_GOOD will be mapped to CAIRO_ANTIALIAS_GRAY, with CAIRO_ANTALIAS_BEST being equivalent to CAIRO_ANTIALIAS_SUBPIXEL.
///
/// The interpretation of CAIRO_ANTIALIAS_DEFAULT is left entirely up to the backend, typically this will be similar to CAIRO_ANTIALIAS_GOOD.
///
/// Since 1.0
#[repr(i32)]
pub enum Antialias {
  /// Use the default antialiasing for the subsystem and target device, since 1.0
  Default = 0,
  /// Use a bilevel alpha mask, since 1.0
  None = 1,
  /// Perform single-color antialiasing (using shades of gray for black text on a white background, for example), since 1.0
  Gray = 2,
  /// Perform antialiasing by taking advantage of the order of subpixel elements on devices such as LCD panels, since 1.0
  Subpixel = 3,
  /// Hint that the backend should perform some antialiasing but prefer speed over quality, since 1.12
  Fast = 4,
  /// The backend should balance quality against performance, since 1.12
  Good = 5,
  ///Hint that the backend should render at the highest quality, sacrificing speed if necessary, since 1.12
  Best = 6
}

