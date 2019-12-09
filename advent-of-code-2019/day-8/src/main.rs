use std::io;

struct Image {
    width: u32,
    height: u32,
    frames: Vec<Vec<u8>>,
}

impl Image {
    fn read_image(width: u32, height: u32) -> Self {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let image_size = (width * height) as usize;
        let frames = input
            .trim()
            .chars()
            .map(|ch| ((ch as i32) - ('0' as i32)) as u8)
            .collect::<Vec<_>>()
            .chunks(image_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        Image {
            width,
            height,
            frames,
        }
    }

    fn combine_frames(&self) -> Vec<u8> {
        let mut combined_frame = self.frames[0].to_vec();
        for i in 0..combined_frame.len() {
            combined_frame[i] = self
                .frames
                .iter()
                .map(|frame| frame[i])
                .find(|px| *px != 2)
                .unwrap_or(2);
        }
        combined_frame
    }

    fn print_frame(frame: &Vec<u8>, width: u32, height: u32) {
        let width = width as usize;
        assert_eq!(width * height as usize, frame.len());
        for (i, px) in frame.iter().enumerate() {
            if i != 0 && i % width == 0 {
                println!();
            }
            let px = match px {
                0 => '.',
                1 => '#',
                2 => ' ',
                _ => panic!("unknown pixel value"),
            };
            print!("{}", px);
        }
    }
}

fn count(frame: &Vec<u8>, value: u8) -> usize {
    frame.iter().filter(|&x| *x == value).count()
}

fn solve1(image: &Image) {
    let res = image
        .frames
        .iter()
        .min_by_key(|frame| count(frame, 0))
        .map(|frame| count(frame, 1) * count(frame, 2));
    println!("Result 1: {:?}", res);
}

fn solve2(image: &Image) {
    Image::print_frame(&image.combine_frames(), image.width, image.height);
}

fn main() {
    let image = Image::read_image(25, 6);
    solve1(&image);
    solve2(&image);
}
