const INPUT_WIDTH: u32 = 25;
const INPUT_HEIGTH: u32 = 6;

fn print_image(image: &[u32], width: u32) -> String {
    let mut img = String::from("");

    for chunk in image.chunks(width as usize) {
        for pixel in chunk.iter() {
            img.push(match pixel {
                0 => '⬛',
                1 => '⬜',
                2 => ' ',
                _ => '_',
            });
        }
        img.push('\n');
    }

    img
}

fn combine_layers(layers: Vec<&[u32]>) -> Vec<u32> {
    let len = layers[0].len();
    let mut img: Vec<u32> = (0..len).map(|_| 2).collect();

    for layer in layers {
        for (i, pixel) in layer.iter().enumerate() {
            if img[i] == 2 {
                img[i] = *pixel;
            }
        }
    }

    img
}

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("NaN!"))
        .collect()
}

#[aoc(day8, part1)]
fn part1(layers: &[u32]) -> usize {
    let layers: Vec<&[u32]> = layers
        .chunks((INPUT_WIDTH * INPUT_HEIGTH) as usize)
        .collect();

    let layer = layers
        .iter()
        .min_by_key(|layer| layer.iter().filter(|x| **x == 0).count())
        .unwrap();

    let one_digits = layer.iter().filter(|x| **x == 1).count();
    let two_digits = layer.iter().filter(|x| **x == 2).count();

    one_digits * two_digits
}

#[aoc(day8, part2)]
fn part2(layers: &[u32]) -> String {
    let layers: Vec<&[u32]> = layers
        .chunks((INPUT_WIDTH * INPUT_HEIGTH) as usize)
        .collect();

    let img = combine_layers(layers);
    let mut img = print_image(&img, INPUT_WIDTH);
    img.insert(0, '\n');
    img
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_image(data: &[u32], width: u32, heigth: u32) -> Vec<&[u32]> {
        data.chunks((width * heigth) as usize).collect()
    }

    #[test]
    fn test_parse_image() {
        let width = 3;
        let heigth = 2;

        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
        let result = parse_image(&input, width, heigth);
        assert_eq!(result[0], [1, 2, 3, 4, 5, 6]);
        assert_eq!(result[1], [7, 8, 9, 0, 1, 2]);

        println!("{}", print_image(&result[0], width));
    }

    #[test]
    fn test_parse_image_part2() {
        let width = 2;
        let heigth = 2;

        let input = vec![0, 2, 2, 2, 1, 1, 2, 2, 2, 2, 1, 2, 0, 0, 0, 0];
        let result = parse_image(&input, width, heigth);

        assert_eq!(result[0], [0, 2, 2, 2]);
        assert_eq!(result[1], [1, 1, 2, 2]);
        assert_eq!(result[2], [2, 2, 1, 2]);
        assert_eq!(result[3], [0, 0, 0, 0]);

        let img = combine_layers(result);

        assert_eq!(print_image(&img, width), "⬛⬜\n⬜⬛\n");
    }
}
