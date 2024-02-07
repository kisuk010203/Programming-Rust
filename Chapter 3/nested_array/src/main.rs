fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }
    result
}
/// Calculate the magnitude of a vector by summing the squares of the elements and taking the square root.
/// ## Examples
/// ```
/// let v = [3, 4];
/// assert_eq!(math_ex::magnitude(v), 5.0);
/// ```
fn magnitude(v : &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for coord in v {
        sum += coord * coord;
    }
    sum.sqrt()
}
/// Normalize a vector by dividing each element by the magnitude.
/// ## Examples
/// ```
/// let v = [3.0, 4.0];
/// assert_eq!(math_ex::normalize(v), [0.6, 0.8]);
/// ```
fn normalize(v: &Vec<f64>) -> Vec<f64> {
    let mag = magnitude(v);
    let mut result = Vec::new();
    for coord in v {
        result.push(coord / mag);
    }
    result
}   
fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    let v = vec![5.0, 12.0];
    println!("Original Matrix: {:?}", matrix);
    println!("Transposed Matrix: {:?}", transpose(matrix));
    println!("Magnitude of v : {}"  , magnitude(&v));
    println!("Normalized v : {:?}", normalize(&v));
}
