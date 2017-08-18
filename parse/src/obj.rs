use nom::{space, line_ending};

use types::{float, unsigned_integer};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex(pub f32, pub f32, pub f32);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Face(pub u32, pub u32, pub u32);

#[derive(Clone, PartialEq, Debug)]
pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

named!(vertex<&[u8], Vertex>,
    do_parse!(
        tag!("v")   >>
        space       >>
        x1: float   >>
        space       >>
        x2: float   >>
        space       >>
        x3: float   >>
        line_ending >>
        (Vertex(x1, x2, x3))
    )
);

named!(face<&[u8], Face>,
    do_parse!(
        tag!("f")           >>
        space               >>
        f1: unsigned_integer >>
        space               >>
        f2: unsigned_integer >>
        space               >>
        f3: unsigned_integer >>
        line_ending         >>
        (Face(f1, f2, f3))
    )
);

named!(pub obj<&[u8], Obj>,
    do_parse!(
        vertices: many0!(vertex)    >>
        faces: many0!(face)         >>
        eof!()                      >>
        ( Obj { vertices, faces } )
    )
);

#[cfg(test)]
mod types {
    use super::*;
    use nom::IResult;

    #[test]
    fn parse_vertex() {
        assert_eq!(vertex(b"v 1.0 2.0 -3\n"),   IResult::Done(&b""[..], Vertex(1.0, 2.0, -3.0)));
    }

    #[test]
    fn parse_face() {
        assert_eq!(face(b"f 1 2 3\n"), IResult::Done(&b""[..], Face(1, 2, 3)));
    }

    #[test]
    fn parse_obj() {
        assert_eq!(obj(b"v 1.0 2.0 3.0\nf 1 1 1\n"),
            IResult::Done(&b""[..],
                Obj {
                    vertices: vec![Vertex(1.0, 2.0, 3.0)],
                    faces: vec![Face(1, 1, 1)]
                }
            )
        );
    }
}
