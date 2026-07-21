use image::Rgb32FImage;

pub struct RtwImage {
    bytes_per_pixel: i32,
    fdata: Vec<f32>,   // Linear floating point pixel data
    bdata: Vec<u8>,    // Linear 8-bit pixel data
    image_width: i32,  // Loaded image width
    image_height: i32, // Loaded image height
    bytes_per_scanline: i32,
}

#[allow(dead_code)]
impl RtwImage {
    pub fn default() -> Self {
        RtwImage {
            bytes_per_pixel: 3,
            fdata: Vec::new(),
            bdata: Vec::new(),
            image_width: 0,
            image_height: 0,
            bytes_per_scanline: 0,
        }
    }

    pub fn new(image_filename: &str) -> Self {
        // Loads image data from the specified file inside the "images/" directory.
        // If the image was not loaded successfully, width() and height() will return 0.

        let mut img = RtwImage::default();
        let path = format!("images/{}", image_filename);
        if !img.load(&path) {
            eprintln!("ERROR: Could not load image file '{}'.\n", image_filename);
        }
        img
    }

    pub fn load(&mut self, filename: &str) -> bool {
        // Loads the linear (gamma=1) image data from the given file name. Returns true if the
        // load succeeded. The resulting data buffer contains the three [0.0, 1.0]
        // floating-point values for the first pixel (red, then green, then blue). Pixels are
        // contiguous, going left to right for the width of the image, followed by the next row
        // below, for the full height of the image.
        // RGBA images are composited against a white background so transparent areas don't
        // become black.

        let dyn_img = match image::ImageReader::open(filename) {
            Ok(reader) => match reader.decode() {
                Ok(img) => img,
                Err(_) => return false,
            },
            Err(_) => return false,
        };

        let rgb32f: Rgb32FImage = if dyn_img.color().has_alpha() {
            // Composite RGBA against white: result = src_rgb * alpha + white * (1 - alpha)
            let rgba = dyn_img.to_rgba32f();
            let (w, h) = rgba.dimensions();
            let mut rgb = Rgb32FImage::new(w, h);
            for y in 0..h {
                for x in 0..w {
                    let px = rgba.get_pixel(x, y);
                    let a = px.0[3];
                    rgb.put_pixel(
                        x,
                        y,
                        image::Rgb([
                            px.0[0] * a + 1.0 - a,
                            px.0[1] * a + 1.0 - a,
                            px.0[2] * a + 1.0 - a,
                        ]),
                    );
                }
            }
            rgb
        } else {
            dyn_img.to_rgb32f()
        };

        let (width, height) = rgb32f.dimensions();
        self.image_width = width as i32;
        self.image_height = height as i32;
        self.bytes_per_scanline = (width as i32) * self.bytes_per_pixel;

        // Get the internal f32 pixel data (row-major, R, G, B contiguous)
        self.fdata = rgb32f.into_raw();

        // Also generate the 8‑bit pixel data as a backup
        self.convert_to_bytes();
        true
    }

    pub fn width(&self) -> i32 {
        if self.fdata.is_empty() {
            0
        } else {
            self.image_width
        }
    }

    pub fn height(&self) -> i32 {
        if self.fdata.is_empty() {
            0
        } else {
            self.image_height
        }
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> &[u8] {
        const MAGENTA: &[u8] = &[255, 0, 255];
        if self.bdata.is_empty() {
            return MAGENTA;
        }

        let x = Self::clamp(x, 0, self.image_width);
        let y = Self::clamp(y, 0, self.image_height);
        let idx = (y * self.bytes_per_scanline + x * self.bytes_per_pixel) as usize;
        &self.bdata[idx..idx + 3]
    }

    fn clamp(x: i32, low: i32, high: i32) -> i32 {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }

    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (256.0 * value) as u8
        }
    }

    fn convert_to_bytes(&mut self) {
        // Convert the linear floating point pixel data to bytes, storing the resulting byte
        // data in the `bdata` member.

        let total_bytes = (self.image_width * self.image_height * self.bytes_per_pixel) as usize;
        let mut bdata = vec![0u8; total_bytes];

        // Iterate through all pixel components, converting from [0.0, 1.0] float values to
        // unsigned [0, 255] byte values.
        let fdata = &self.fdata;

        for i in 0..total_bytes {
            bdata[i] = Self::float_to_byte(fdata[i]);
        }
        self.bdata = bdata;
    }
}
