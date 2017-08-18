use nalgebra::{Matrix4, Vector3, Unit, U4, MatrixArray, Matrix};
use nom::{space, line_ending};

use types::float;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Transformation {
    Translation(f32, f32, f32),
    Rotation(f32, f32, f32, f32),
    Scale(f32, f32, f32),
}

impl From<Transformation> for Matrix<f32, U4, U4, MatrixArray<f32, U4, U4>> {
    fn from(transformation: Transformation) -> Matrix4<f32> {
        match transformation {
            Transformation::Translation(x, y, z) => {
                Matrix4::new_translation(&Vector3::new(x, y, z))
            },
            Transformation::Rotation(x, y, z, theta) => {
                Matrix4::from_axis_angle(&Unit::new_normalize(Vector3::new(x, y, z)), theta)
            },
            Transformation::Scale(x, y, z) => {
                Matrix4::new_nonuniform_scaling(&Vector3::new(x, y, z))
            },
        }
    }
}

named!(pub transformation<&[u8], Transformation>,
    alt!(
        do_parse!(
            tag!("t")   >>
            space       >>
            x: float    >>
            space       >>
            y: float    >>
            space       >>
            z: float    >>
            line_ending >>
            (Transformation::Translation(x, y, z))
        ) |
        do_parse!(
            tag!("r")        >>
            space           >>
            x: float        >>
            space           >>
            y: float        >>
            space           >>
            z: float        >>
            space           >>
            theta: float    >>
            line_ending     >>
            (Transformation::Rotation(x, y, z, theta))
        ) |
        do_parse!(
            tag!("s")    >>
            space       >>
            x: float    >>
            space       >>
            y: float    >>
            space       >>
            z: float    >>
            line_ending >>
            (Transformation::Scale(x, y, z))
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;
    #[test]
    fn parse_transformation() {
        assert_eq!(transformation(b"t 1 2 3\n"), IResult::Done(&b""[..], Transformation::Translation(1.0, 2.0, 3.0)));
        assert_eq!(transformation(b"r 1 2 3 1\n"), IResult::Done(&b""[..], Transformation::Rotation(1.0, 2.0, 3.0, 1.0)));
        assert_eq!(transformation(b"s 1 2 3\n"), IResult::Done(&b""[..], Transformation::Scale(1.0, 2.0, 3.0)));
    }
}
