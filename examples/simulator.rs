use embedded_graphics_in_memory::InMemoryDisplay;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let size = Size::new(128, 64);
    let mut display = SimulatorDisplay::<BinaryColor>::new(size);
    let mut in_memory = InMemoryDisplay::<BinaryColor>::new(size);

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut in_memory)?;

    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(&mut in_memory)?;

    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(&mut in_memory)?;

    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(line_style)
        .draw(&mut in_memory)?;

    Text::new("Hello World!", Point::new(5, 5), text_style).draw(&mut in_memory)?;

    // in-memory to real display
    in_memory.update(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
