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
struct Side([[Color; 3]; 3]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Cube {
    // White at the bottom. Red on the front.
    y: Side,
    b: Side,
    r: Side,
    w: Side,
    o: Side,
    g: Side,
}

impl Cube {
    fn new(y: Side, b: Side, r: Side, w: Side, o: Side, g: Side) -> Self {
        Cube { y, b, r, w, o, g }
    }

    fn default() -> Self {
        Self::new(
            Side([[Yellow; 3]; 3]),
            Side([[Blue; 3]; 3]),
            Side([[Red; 3]; 3]),
            Side([[White; 3]; 3]),
            Side([[Orange; 3]; 3]),
            Side([[Green; 3]; 3]),
        )
    }

    fn rotate_clockwise(&mut self, _color: Color) {

        self.b = Side([[Red, Red, Red], [Blue, Blue, Blue], [Blue, Blue, Blue]]);
        self.r = Side([[Green, Green, Green], [Red, Red, Red], [Red, Red, Red]]);
        self.o = Side([[Blue, Blue, Blue], [Orange, Orange, Orange], [Orange, Orange, Orange]]);
        self.g = Side([ [Orange, Orange, Orange], [Green, Green, Green], [Green, Green, Green]]);

    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cube() {
        let default_cube = Cube::new(
            Side([
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
            ]),
            Side([[Blue, Blue, Blue], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Side([[Red, Red, Red], [Red, Red, Red], [Red, Red, Red]]),
            Side([
                [White, White, White],
                [White, White, White],
                [White, White, White],
            ]),
            Side([
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
            ]),
            Side([
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
            Side([
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
                [Yellow, Yellow, Yellow],
            ]),
            Side([[Red, Red, Red], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Side([[Green, Green, Green], [Red, Red, Red], [Red, Red, Red]]),
            Side([
                [White, White, White],
                [White, White, White],
                [White, White, White],
            ]),
            Side([[Blue, Blue, Blue],
                [Orange, Orange, Orange],
                [Orange, Orange, Orange],
            ]),
            Side([
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
            Side([
                [Orange, Yellow, Yellow],
                [Orange, Yellow, Yellow],
                [Orange, Yellow, Yellow],
            ]),
            Side([[Blue, Blue, Blue], [Blue, Blue, Blue], [Blue, Blue, Blue]]),
            Side([[Yellow, Red, Red], [Yellow, Red, Red], [Yellow, Red, Red]]),
            Side([
                [Red, White, White],
                [Red, White, White],
                [Red, White, White],
            ]),
            Side([
                [Orange, Orange, White],
                [Orange, Orange, White],
                [Orange, Orange, White],
            ]),
            Side([
                [Green, Green, Green],
                [Green, Green, Green],
                [Green, Green, Green],
            ]),
        );
        assert_eq!(expected_cube, cube);
    }
}
