use std::io;

fn input_i32() -> i32 {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Read error!");
    let num_i32 = match input.trim().parse::<i32>() {
        Ok(num) => num,
        _ => 0,
    };
    num_i32
}

fn input_str() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Read error!");
    input = String::from(input.trim());
    input
}
#[derive(Debug)]
struct Matrix {
    val: Vec<Vec<i32>>,
    name: String,
}
impl Matrix {
    fn display(&self) {
        println!("Matrix name: {}", self.name);
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", self.val[i][j]);
            }
            println!();
        }
    }
    fn init(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                self.val[i][j] = input_i32();
            }
        }
    }
    fn multiply(&self, matrix: &Matrix) -> Matrix{
        println!("Enter name of new matrix");
        let matrix_name = input_str();
        let mut new_matrix = Matrix {
            val: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            name: matrix_name,
        };
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0;
                for k in 0..3 {
                    sum += self.val[i][k] * matrix.val[k][j];
                }
                new_matrix.val[i][j] = sum;
            }
        }
        new_matrix
    }
    fn add(&self, matrix: &Matrix) -> Matrix {
        println!("Enter name of the new matrix");
        let matrix_name = input_str();
        let mut new_matrix = Matrix {
            val: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
            name: matrix_name,
        };
        for i in 0..3 {
            for j in 0..3 {
                new_matrix.val[i][j] = self.val[i][j] + matrix.val[i][j];
            }
        }
        new_matrix
    }
}

fn main() {
    let mut matrix1 = Matrix {
        val: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        name: String::from("Matrix1"),
    };
    matrix1.init();
    matrix1.display();
    let mut matrix2 = Matrix {
        val: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        name: String::from("Matrix2"),
    };
    matrix2.init();
    matrix2.display();
    let matrix3 = matrix1.multiply(&matrix2);
    matrix3.display();
    let matrix4 = matrix1.multiply(&matrix2);
    matrix4.display();
}
