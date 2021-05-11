use std::fs::File;
use std::io::Write;
use rust_ray_tracing::data_types::Color;
use std::convert::TryFrom;


fn write_matrix_to_ppm(matrix: Vec<Vec<Color>>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();

    file.write(b"P3\n").unwrap();
    write!(file, "{} {}\n", matrix[0].len(), matrix.len()).unwrap();
    file.write(b"255\n").unwrap();

    for line in matrix {
        for element in line {
            write!(file, "{} ", element).unwrap();
        }

        write!(file, "\n").unwrap();
    }
}

fn sum(array: &[i32]) -> i32 {
    let mut result = 0;
    for element in array.iter() {
        result += element;
    }

    result
}

fn main() {
    let mut matrix = vec![];

    // Just a generic matrix
    for i in 0..255usize {
        let mut line = vec![];
        for j in 0..255usize {
            let (red, green, blue) = (u8::try_from(i / 2).unwrap(), u8::try_from(j / 1).unwrap(), 50);
            line.push(Color::new(red, green, blue))
        }

        println!("{}", i);
        matrix.push(line);
    }

    matrix.push(vec![Color::new(0, 0, 0), Color::new(0, 0, 0), Color::new(0, 20, 10)]);
    matrix.push(vec![Color::new(0, 0, 0), Color::new(5, 223, 10), Color::new(255, 255, 255)]);


    write_matrix_to_ppm(matrix, "output.ppm");
}
