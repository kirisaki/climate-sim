use h3ron::H3Cell;
use h3ron::ToCoordinate;
use image::GrayImage;

pub trait GrayImageExt {
    fn luminance_at_latlng(&self, lat: f64, lng: f64) -> f64;
    fn elevation_at_latlng(&self, lat: f64, lng: f64) -> f64;
}

impl GrayImageExt for GrayImage {
    fn luminance_at_latlng(&self, lat: f64, lng: f64) -> f64 {
        let (width, height) = self.dimensions();

        let fx = ((lng + 180.0) / 360.0) * (width as f64 - 1.0);
        let fy = ((90.0 - lat) / 180.0) * (height as f64 - 1.0);

        let x0 = fx.floor() as u32;
        let y0 = fy.floor() as u32;
        let x1 = (x0 as u32 + 1).min(width - 1);
        let y1 = (y0 as u32 + 1).min(height - 1);

        let dx = fx - x0 as f64;
        let dy = fy - y0 as f64;

        let get = |x: u32, y: u32| -> f64 { self.get_pixel(x, y)[0] as f64 };

        let top = get(x0, y0) * (1.0 - dx) + get(x1, y0) * dx;
        let bottom = get(x0, y1) * (1.0 - dx) + get(x1, y1) * dx;

        top * (1.0 - dy) + bottom * dy
    }

    fn elevation_at_latlng(&self, lat: f64, lng: f64) -> f64 {
        let lum = self.luminance_at_latlng(lat, lng);
        -1000.0 + (lum as f64 / 255.0) * 9000.0
    }
}

/// generate the elevation map from H3 cells
pub fn h3_cells_to_elevation_map(
    image: &GrayImage,
    h3_cells: Vec<H3Cell>,
) -> anyhow::Result<Vec<(H3Cell, f64)>> {
    let mut result = Vec::with_capacity(h3_cells.capacity());

    for cell in h3_cells.iter() {
        let coord = cell.to_coordinate().unwrap();
        let elev = image.elevation_at_latlng(coord.y, coord.x);
        result.push((cell.clone(), elev));
    }

    Ok(result)
}
