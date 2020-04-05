mod common;
mod error;
mod horizontal;
mod point;
mod vertical;

pub(crate) use crate::common::*;

fn main() {
    // Generate 5 horizontal lines
    let x = (120.0, 1500.0);
    let y = (10.0, 360.0);
    Horizontal::new()
        .x_range(x.0, x.1)
        .y_range(y.0, y.1)
        .lines(5)
        .tags()
        .write(None)
        .unwrap();

    // Generate 5 vertical lines
    let points = (Point::new(120.0, 10.0), Point::new(1500.0, 360.0));
    let mut vertical = Vertical::new(points, 5);
    vertical.get_tags().unwrap();
    write(&vertical.tags, None).unwrap();
}

fn write(document: &SvgDocument, out: Option<&Path>) -> Result<()> {
    if let Some(path) = out {
        let buffer = File::create(path).map_err(|io_error| Error::IoError {
            io_error,
            path: path.into(),
        })?;
        svg::write(buffer, document).map_err(|io_error| Error::IoError {
            io_error,
            path: path.into(),
        })?;
    } else {
        writeln!(io::stdout(), "{}", &document.to_string()).unwrap();
    }

    Ok(())
}
