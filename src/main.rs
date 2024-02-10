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
struct Cube([Side; 6]);

impl Cube {
    fn default() -> Self {
        let all_sides: [Side; 6] = [
            Side([[Yellow; 3]; 3]),
            Side([[Blue; 3]; 3]),
            Side([[Red; 3]; 3]),
            Side([[White; 3]; 3]),
            Side([[Orange; 3]; 3]),
            Side([[Green; 3]; 3]),
        ];
        Cube(all_sides)
    }
}

fn main() {

    // all_sides = for color in Color::iter() {
    //     side = [[color; 3]; 3];
    // }
    // RedSide = ;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cube() {
        let default_cube = Cube([
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
        ]);
        assert_eq!(default_cube, Cube::default());
    }
}
