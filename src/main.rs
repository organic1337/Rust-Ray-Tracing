use std::fs::File;
use std::io::Write;

fn write_matrix_to_ppm(matrix: Vec<Vec<(i32, i32, i32)>>, file_path: &str) {
    let mut file = File::create(file_path).unwrap();
    file.write(b"P3").unwrap();

    write!(file, "{} {}\n", matrix[0].len(), matrix.len()).unwrap();

    for line in matrix {
        for element in line {
            let (red, green, blue) = element;
            write!(file, "{} {} {} ", red, green, blue).unwrap();
        }

        write!(file, "\n").unwrap();
    }
}

fn main() {
    let matrix = vec![vec![(35, 0, 20), (35, 0, 20), (35, 0, 20)], vec![(255, 0, 255), (0, 0, 0), (35, 0, 255)], vec![(255, 255, 255), (255, 255, 255), (0, 0, 0)]];
    write_matrix_to_ppm(matrix, "output.ppm");

}
