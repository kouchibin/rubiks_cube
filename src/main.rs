use std::collections::HashMap;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
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
struct Face([Color; 9]);

impl Face {
    fn transpose(&mut self) {
        let original = self.clone();
        let ring_index = [0, 1, 2, 5, 8, 7, 6, 3];
        for i in 0..ring_index.len() {
            let from = ring_index[i];
            let to = ring_index[(i + 2) % ring_index.len()];
            self.0[to] = original.0[from]
        }
    }
}

// White as bottom. Red as front.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    front: Face,
    back: Face,
    left: Face,
    right: Face,
    up: Face,
    down: Face,
}

impl Cube {
    fn new(front: Face, back: Face, left: Face, right: Face, up: Face, down: Face) -> Self {
        Cube {
            front,
            back,
            left,
            right,
            up,
            down,
        }
    }

    fn default() -> Self {
        Self::new(
            Face([Red; 9]),
            Face([Orange; 9]),
            Face([Blue; 9]),
            Face([Green; 9]),
            Face([Yellow; 9]),
            Face([White; 9]),
        )
    }

    fn adjacent_faces(color: Color) -> [Color; 4] {
        match color {
            Yellow => [Red, Blue, Orange, Green],
            Blue => [Yellow, Red, White, Orange],
            Red => [Yellow, Green, White, Blue],
            Orange => [Yellow, Blue, White, Green],
            White => [Red, Green, Orange, Blue],
            Green => [Yellow, Orange, White, Red],
        }
    }

    fn execute(&mut self, sequence: &str) {
        match sequence {
            "U" => {
                self.up.transpose();
                for i in 0..=2 {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.right.0[i];
                    self.right.0[i] = self.back.0[i];
                    self.back.0[i] = self.left.0[i];
                    self.left.0[i] = tmp;
                }
            }
            "L" => {
                self.left.transpose();
                for i in [0, 3, 6] {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.up.0[i];
                    self.up.0[i] = self.back.0[i+2];
                    self.back.0[i+2] = self.down.0[i];
                    self.down.0[i] = tmp;
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
        let mut face = Face([Orange, Orange, Orange, Green, Green, Green, Red, Red, Red]);
        face.transpose();
        assert_eq!(
            Face([Red, Green, Orange, Red, Green, Orange, Red, Green, Orange,]),
            face
        );
    }

    #[test]
    fn test_default_cube() {
        let default_cube = Cube::new(
            Face([Red, Red, Red, Red, Red, Red, Red, Red, Red]),
            Face([
                Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange,
            ]),
            Face([Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue]),
            Face([
                Green, Green, Green, Green, Green, Green, Green, Green, Green,
            ]),
            Face([
                Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
            ]),
            Face([
                White, White, White, White, White, White, White, White, White,
            ]),
        );
        assert_eq!(default_cube, Cube::default());
    }

    #[test]
    fn test_rotate_clockwise_up() {
        let mut cube = Cube::default();
        cube.execute("U");

        let expected_cube = Cube::new(
            Face([Green, Green, Green, Red, Red, Red, Red, Red, Red]),
            Face([
                Blue, Blue, Blue, Orange, Orange, Orange, Orange, Orange, Orange,
            ]),
            Face([Red, Red, Red, Blue, Blue, Blue, Blue, Blue, Blue]),
            Face([
                Orange, Orange, Orange, Green, Green, Green, Green, Green, Green,
            ]),
            Face([
                Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
            ]),
            Face([
                White, White, White, White, White, White, White, White, White,
            ]),
        );
        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_rotate_clockwise_left() {
        let mut cube = Cube::default();
        cube.execute("L");

        let expected_cube = Cube::new(
            Face([Yellow, Red, Red, Yellow, Red, Red, Yellow, Red, Red]),
            Face([
                Orange, Orange, White, Orange, Orange, White, Orange, Orange, White,
            ]),
            Face([Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue]),
            Face([
                Green, Green, Green, Green, Green, Green, Green, Green, Green,
            ]),
            Face([
                Orange, Yellow, Yellow, Orange, Yellow, Yellow, Orange, Yellow, Yellow,
            ]),
            Face([Red, White, White, Red, White, White, Red, White, White]),
        );
        assert_eq!(expected_cube, cube);
    }
}
