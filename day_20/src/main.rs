use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use ndarray::{Array1, Array2, s};


#[derive(Debug, Clone)]
struct Image {
    pixels: Array2<bool>,
}

impl Image {
    fn new(pixels: Vec<Vec<bool>>) -> Image {
        let nrows = pixels.len();
        let ncols = pixels[0].len();
        let mut final_pixels: Array2<bool> = Array2::from_shape_simple_fn((nrows + 4, ncols + 4), || false);
        let mut index = 0;
        for mut row in final_pixels.rows_mut() {
            if index < 2 || index >= (2 + nrows) {
                index += 1;
                continue;
            }
            row.slice_mut(s![2..ncols + 2]).assign(&Array1::from_vec(pixels[index - 2].clone()));
            index += 1;
        }
        
        Image {
            pixels: final_pixels,
        }
    }
    fn transform_pixel(&self, row: usize, col: usize, encoder: &Vec<bool>) -> bool {
        let (nrows, ncols) = self.pixels.dim();
        let number;
        if row == 0 || row == nrows - 1 || col == 0 || col == ncols - 1 {
            if self.pixels[(0, 0)] {
                number = (2 as usize).pow(9) - 1;
            } else {
                number = 0;
            }
        } else {
            number = vec_to_num(self.get_neighbors(row, col));
        }
        encoder[number]
    }
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<bool> {
        let mut neighbors = Vec::new();
        let row = row - 1;
        let col = col - 1;
        for i in 0..3 {
            for j in 0..3 {
                neighbors.push(self.pixels[(row + i, col + j)]);
            }
        }
        neighbors
    }
    fn decode(&self, encoder: &Vec<bool>) -> Image {
        let (nrows, ncols) = self.pixels.dim();
        let mut decoded_pixels: Array2<bool> = Array2::from_shape_simple_fn((nrows + 2, ncols + 2), || self.transform_pixel(0, 0, encoder));
        for irow in 1..nrows + 1 {
            for icol in 1..ncols + 1 {
                decoded_pixels[(irow, icol)] = self.transform_pixel(irow-1, icol-1, encoder);
            }
        }
        Image {pixels: decoded_pixels}
    }
    fn total_lights(&self) -> usize {
        let mut total = 0;
        for row in self.pixels.rows() {
            for pixel in row {
                if *pixel {
                    total += 1;
                }
            }
        }
        total
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.pixels.rows().into_iter().map(|row| {
            row.iter().map(|pixel| if *pixel { '#' } else { '.' }).collect::<String>()
        }).collect::<Vec<String>>().join("\n");
        write!(f, "{}", string)
    }
    
}

fn vec_to_num(vec: Vec<bool>) -> usize {
    let mut number = 0;
    let len = vec.len() - 1;
    for (index, element) in vec.iter().enumerate() {
        if *element {
            number += (2 as usize).pow((len - index) as u32);
        }
    }
    number
}


fn parse_file(filename: &str) -> (Vec<bool>, Image) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();
    let first_line = lines_iter.next().unwrap().unwrap();
    let encoder: Vec<bool> = first_line
        .chars()
        .map(|c| c == '#')
        .collect();
    let mut image: Vec<Vec<bool>> = Vec::new();
    for line in lines_iter {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let aux: Vec<bool> = line
        .chars()
        .map(|c| c == '#')
        .collect();
        image.push(aux);
    }
    (encoder, Image::new(image))
}


fn main() {
    let filename = env::args().nth(1).expect("Please supply a filename");
    let (encoder, mut image) = parse_file(&filename);
    for _ in 0..50 {
        image = image.decode(&encoder);
        // println!("{}", image);
    }
    println!("{}", image);
    println!("{}", image.total_lights());
}
