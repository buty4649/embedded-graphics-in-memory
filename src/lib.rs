use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, OriginDimensions, Pixel, PixelColor, Point, Size},
};

#[derive(Debug)]
pub struct InMemoryDisplay<C> {
    size: Size,
    pixels: Box<[C]>,
}

impl<C: PixelColor + From<BinaryColor>> InMemoryDisplay<C> {
    pub fn new(size: Size) -> Self {
        let pixel_count = size.width as usize * size.height as usize;
        let default_color = C::from(BinaryColor::Off);
        let pixels = vec![default_color; pixel_count].into_boxed_slice();
        Self { size, pixels }
    }
}

impl<C: PixelColor> InMemoryDisplay<C> {
    pub fn get_pixel(&self, point: Point) -> C {
        self.point_to_index(point)
            .and_then(|index| self.pixels.get(index).copied())
            .expect("can't get point outside of display")
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let Ok((x, y)) = <(u32, u32)>::try_from(point) {
            if x < self.size.width && y < self.size.height {
                return Some((x + y * self.size.width) as usize);
            }
        }

        None
    }

    pub fn update<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<(), D::Error> {
        let pixel_count = self.size.width as usize * self.size.height as usize;
        let pixels = (0..pixel_count).map(|c| {
            let height = c / self.size.width as usize;
            let width = c % self.size.width as usize;
            Pixel(Point::new(width as i32, height as i32), self.pixels[c])
        });
        display.draw_iter(pixels)
    }
}

impl<C: PixelColor> DrawTarget for InMemoryDisplay<C> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                self.pixels[index] = color;
            }
        }

        Ok(())
    }
}

impl<C> OriginDimensions for InMemoryDisplay<C> {
    fn size(&self) -> Size {
        self.size
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
