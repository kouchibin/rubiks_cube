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

    fn execute(&mut self, sequence: &str) {
        for orientation in sequence.chars() {
            self.rotate(orientation);
        }
    }

    fn rotate(&mut self, orientation: char) {
        match orientation {
            'U' => {
                self.up.transpose();
                for i in [0, 1, 2] {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.right.0[i];
                    self.right.0[i] = self.back.0[i];
                    self.back.0[i] = self.left.0[i];
                    self.left.0[i] = tmp;
                }
            }
            'D' => {
                self.down.transpose();
                for i in [6, 7, 8] {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.left.0[i];
                    self.left.0[i] = self.back.0[i];
                    self.back.0[i] = self.right.0[i];
                    self.right.0[i] = tmp;
                }
            }
            'L' => {
                self.left.transpose();
                for i in [0, 3, 6] {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.up.0[i];
                    self.up.0[i] = self.back.0[8 - i];
                    self.back.0[8 - i] = self.down.0[i];
                    self.down.0[i] = tmp;
                }
            }
            'R' => {
                self.right.transpose();
                for i in [2, 5, 8] {
                    let tmp = self.front.0[i];
                    self.front.0[i] = self.down.0[i];
                    self.down.0[i] = self.back.0[8 - i];
                    self.back.0[8 - i] = self.up.0[i];
                    self.up.0[i] = tmp;
                }
            }
            'F' => {
                self.front.transpose();
                let tmp1 = self.up.0[6];
                let tmp2 = self.up.0[7];
                let tmp3 = self.up.0[8];
                self.up.0[6] = self.left.0[8];
                self.up.0[7] = self.left.0[5];
                self.up.0[8] = self.left.0[2];
                self.left.0[2] = self.down.0[0];
                self.left.0[5] = self.down.0[1];
                self.left.0[8] = self.down.0[2];
                self.down.0[0] = self.right.0[6];
                self.down.0[1] = self.right.0[3];
                self.down.0[2] = self.right.0[0];
                self.right.0[0] = tmp1;
                self.right.0[3] = tmp2;
                self.right.0[6] = tmp3;
            }
            'B' => {
                self.back.transpose();
                let tmp1 = self.up.0[0];
                let tmp2 = self.up.0[1];
                let tmp3 = self.up.0[2];
                self.up.0[0] = self.right.0[2];
                self.up.0[1] = self.right.0[5];
                self.up.0[2] = self.right.0[8];
                self.right.0[2] = self.down.0[8];
                self.right.0[5] = self.down.0[7];
                self.right.0[8] = self.down.0[6];
                self.down.0[6] = self.left.0[0];
                self.down.0[7] = self.left.0[3];
                self.down.0[8] = self.left.0[6];
                self.left.0[0] = tmp3;
                self.left.0[3] = tmp2;
                self.left.0[6] = tmp1;
            }
            'u' | 'd' | 'l' | 'r' | 'f' | 'b' => {
                // Rotate counter clockwise, which is the same as
                // rotating clockwise 3 times.
                let orientation = orientation.to_ascii_uppercase();
                for _ in 0..3 {
                    self.rotate(orientation);
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
    use rand::random;

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
        cube.rotate('U');

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
    fn test_rotate_clockwise_down() {
        let mut cube = Cube::default();
        cube.rotate('D');

        let expected_cube = Cube::new(
            Face([Red, Red, Red, Red, Red, Red, Blue, Blue, Blue]),
            Face([
                Orange, Orange, Orange, Orange, Orange, Orange, Green, Green, Green,
            ]),
            Face([Blue, Blue, Blue, Blue, Blue, Blue, Orange, Orange, Orange]),
            Face([Green, Green, Green, Green, Green, Green, Red, Red, Red]),
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
        cube.rotate('L');

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

    #[test]
    fn test_rotate_clockwise_right() {
        let mut cube = Cube::default();
        cube.rotate('R');

        let expected_cube = Cube::new(
            Face([Red, Red, White, Red, Red, White, Red, Red, White]),
            Face([
                Yellow, Orange, Orange, Yellow, Orange, Orange, Yellow, Orange, Orange,
            ]),
            Face([Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue, Blue]),
            Face([
                Green, Green, Green, Green, Green, Green, Green, Green, Green,
            ]),
            Face([
                Yellow, Yellow, Red, Yellow, Yellow, Red, Yellow, Yellow, Red,
            ]),
            Face([
                White, White, Orange, White, White, Orange, White, White, Orange,
            ]),
        );
        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_rotate_clockwise_front() {
        let mut cube = Cube::default();
        cube.rotate('F');

        let expected_cube = Cube::new(
            Face([Red, Red, Red, Red, Red, Red, Red, Red, Red]),
            Face([
                Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange,
            ]),
            Face([Blue, Blue, White, Blue, Blue, White, Blue, Blue, White]),
            Face([
                Yellow, Green, Green, Yellow, Green, Green, Yellow, Green, Green,
            ]),
            Face([
                Yellow, Yellow, Yellow, Yellow, Yellow, Yellow, Blue, Blue, Blue,
            ]),
            Face([
                Green, Green, Green, White, White, White, White, White, White,
            ]),
        );
        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_rotate_clockwise_back() {
        let mut cube = Cube::default();
        cube.rotate('B');

        let expected_cube = Cube::new(
            Face([Red, Red, Red, Red, Red, Red, Red, Red, Red]),
            Face([
                Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange, Orange,
            ]),
            Face([Yellow, Blue, Blue, Yellow, Blue, Blue, Yellow, Blue, Blue]),
            Face([
                Green, Green, White, Green, Green, White, Green, Green, White,
            ]),
            Face([
                Green, Green, Green, Yellow, Yellow, Yellow, Yellow, Yellow, Yellow,
            ]),
            Face([White, White, White, White, White, White, Blue, Blue, Blue]),
        );
        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_rotate_counter_clockwise_up() {
        let mut cube = Cube::default();
        cube.rotate('u');

        let expected_cube = Cube::new(
            Face([Blue, Blue, Blue, Red, Red, Red, Red, Red, Red]),
            Face([
                Green, Green, Green, Orange, Orange, Orange, Orange, Orange, Orange,
            ]),
            Face([Orange, Orange, Orange, Blue, Blue, Blue, Blue, Blue, Blue]),
            Face([Red, Red, Red, Green, Green, Green, Green, Green, Green]),
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
    fn test_execute_sequence() {
        let mut cube = Cube::default();
        cube.execute("UDLRFBudlrfb");

        let expected_cube = Cube::new(
            Face([Red, Red, Red, Blue, Red, Green, Red, Red, Red]),
            Face([
                Orange, Orange, Orange, Green, Orange, Blue, Orange, Orange, Orange,
            ]),
            Face([Blue, White, Blue, Red, Blue, Orange, Blue, Yellow, Blue]),
            Face([
                Green, White, Green, Orange, Green, Red, Green, Yellow, Green,
            ]),
            Face([
                Yellow, Yellow, Yellow, Blue, Yellow, Green, Yellow, Yellow, Yellow,
            ]),
            Face([White, White, White, Blue, White, Green, White, White, White]),
        );
        assert_eq!(expected_cube, cube);
    }

    #[test]
    fn test_execute_sequence_random() {
        use rand::seq::IteratorRandom;
        let mut rng = rand::thread_rng();
        let mut random_sequence = String::new();
        for _ in 0..1000 {
            let o = "UDLRFBudlrfb".chars().choose(&mut rng).unwrap();
            random_sequence.push(o);
        }

        let mut cube = Cube::default();
        cube.execute(&random_sequence);

        let mut reversed_sequence = String::new();
        for o in random_sequence.chars().rev() {
            let reversed = if o.is_ascii_lowercase() {
                o.to_ascii_uppercase()
            } else {
                o.to_ascii_lowercase()
            };
            reversed_sequence.push(reversed);
        }
        cube.execute(&reversed_sequence);

        assert_eq!(Cube::default(), cube);
    }
}
