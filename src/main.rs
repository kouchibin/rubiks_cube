#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Yellow,
    Blue,
    Red,
    White,
    Orange,
    Green,
}
use Color::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Face([[Color; 3]; 3]);

impl Face {
    fn transpose(&mut self) {
       // 00->02, 01->12, 02->22;
       // 10->01, 11->11, 12->21;
       // 20->00, 21->10, 22->20
       let original = self.clone();
       for i in 0..=2 {
        for j in 0..=2 {
            self.0[j][2-i] = original.0[i][j];
        }
       }
    }
}

// TODO - Make it a hashmap instead.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cube {
    // White at the bottom. Red on the front.
    y: Face,
    b: Face,
    r: Face,
    w: Face,
    o: Face,
    g: Face,
}

impl Cube {
    fn new(y: Face, b: Face, r: Face, w: Face, o: Face, g: Face) -> Self {
        Cube { y, b, r, w, o, g }
    }

    fn default() -> Self {
        Self::new(
            Face([[Yellow; 3]; 3]),
            Face([[Blue; 3]; 3]),
            Face([[Red; 3]; 3]),
            Face([[White; 3]; 3]),
            Face([[Orange; 3]; 3]),
            Face([[Green; 3]; 3]),
        )
    }

    fn adjacent_faces(color: Color) -> [Color;4] {
        match color {
            Yellow => [Red, Blue, Orange, Green],
            Blue => [Yellow, Red, White, Orange],
            Red => [Yellow, Green, White, Blue],
            Orange => [Yellow, Blue, White, Green],
            White => [Red, Green, Orange, Blue],
            Green => [Yellow, Orange, White, Red]
        }
    }

    fn rotate_clockwise(&mut self, color: Color) {
        match color {
            Red    => self.r.transpose(),
            Blue   => self.b.transpose(),
            Yellow => self.y.transpose(),
            Orange => self.o.transpose(),
            White  => self.w.transpose(),
            Green  => self.g.transpose()
        }
        let _adjacent_faces = Cube::adjacent_faces(color);
        match color {
            Yellow => {
                let tmp = self.g.0[0];
                self.g.0[0] = self.o.0[0];
                self.o.0[0] = self.b.0[0];
                self.b.0[0] = self.r.0[0];
                self.r.0[0] = tmp;
            },
            Blue => {
                let mut tmp = [White; 3];
                for r in 0..=2 {
                    tmp[r] = self.o.0[r][2];
                    self.o.0[r][2] = self.w.0[2-r][0];

                }
                for r in 0..=2 {
                    self.w.0[r][0] = self.r.0[r][0];
                }
                for r in 0..=2 {
                    self.r.0[r][0] = self.y.0[r][0];
                }
                for r in 0..=2 {
                    self.y.0[r][0] = tmp[r];
                }
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_transpose() {
        let mut face = Face([
            [Orange, Orange, Orange],
            [Green, Green, Green],
            [Red, Red, Red],
        ]);
        face.transpose();
        assert_eq!(
            Face([
                [Red, Green, Orange],
                [Red, Green, Orange],
                [Red, Green, Orange],
            ]),
            face
        );
    }

    #[test]
    fn test_default_cube() {
        let default_cube = Cube::new(
            Face([
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
            ]),
            Face([[Blue, Blue, Blue], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Face([[Red, Red, Red], [Red, Red, Red], [Red, Red, Red]]),
            Face([
                [White, White, White],
                [White, White, White],
                [White, White, White],
            ]),
            Face([
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
            ]),
            Face([
                [Green, Green, Green],
                [Green, Green, Green],
                [Green, Green, Green],
            ]),
        );
        assert_eq!(default_cube, Cube::default());
    }

    #[test]
    fn test_rotate_clockwise_yellow() {
        let mut cube = Cube::default();
        cube.rotate_clockwise(Yellow);

        let expected_cube = Cube::new(
            Face([
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
            ]),
            Face([[Red, Red, Red], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Face([[Green, Green, Green], [Red, Red, Red], [Red, Red, Red]]),
            Face([
                [White, White, White],
                [White, White, White],
                [White, White, White],
            ]),
            Face([
                [Blue, Blue, Blue],
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
            ]),
            Face([
                [Orange, Orange, Orange],
                [Green, Green, Green],
                [Green, Green, Green],
            ]),
        );
        assert_eq!(expected_cube, cube);
    }
    #[test]
    fn test_rotate_clockwise_blue() {
        let mut cube = Cube::default();
        cube.rotate_clockwise(Blue);

        let expected_cube = Cube::new(
            Face([
                [Orange, Yellow, Yellow],
                [Orange, Yellow, Yellow],
                [Orange, Yellow, Yellow],
            ]),
            Face([[Blue, Blue, Blue], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Face([[Yellow, Red, Red], [Yellow, Red, Red], [Yellow, Red, Red]]),
            Face([
                [Red, White, White],
                [Red, White, White],
                [Red, White, White],
            ]),
            Face([
                [Orange, Orange, White],
                [Orange, Orange, White],
                [Orange, Orange, White],
            ]),
            Face([
                [Green, Green, Green],
                [Green, Green, Green],
                [Green, Green, Green],
            ]),
        );
        assert_eq!(expected_cube, cube);
    }
}
