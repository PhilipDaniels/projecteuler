use std::str::FromStr;

/// A crude 2d matrix class.
pub struct Matrix<T> {
    data: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    /// Constructs a matrix from a string. The string should consist of one or more lines separated
    /// by `\n` characters. Each line should consist of 1 or more values separated by spaces.
    /// Each line must have the same number of elements.
    pub fn make_from_string(s: &str) -> Result<Matrix<T>, String>
        where T: FromStr
    {
        let mut result = Matrix { data: Vec::new() };

        // TODO: Improve this mess.
        for line in s.lines() {
            let mut row = Vec::new();
            for s in line.split(" ").collect::<Vec<_>>() {
                match s.parse::<T>() {
                    Ok(v) => row.push(v),
                    Err(_) => return Err(format!("Error, cannot convert {} to a number", s))
                }

            }

            result.data.push(row);
        }

        Ok(result)
    }

    /// Returns true if the matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    /// Returns the number of rows in the matrix. 0 for an empty matrix.
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn row(&self, n: usize) -> &Vec<T> {
        &self.data[n]
    }

    /// Returns the number of columns in the matrix. The matrix is assumed to be a rectangle,
    /// so this is just the length of the first row.
    pub fn columns(&self) -> usize {
        if self.rows() == 0 {
            0
        } else {
            self.data[0].len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn make_from_string_for_empty_string_returns_empty_matrix() {
        let m = Matrix::<i32>::make_from_string("").unwrap();
        assert!(m.is_empty());
    }

    #[test]
    fn make_from_string_for_string_containing_non_numbers_returns_err() {
        let m = Matrix::<i32>::make_from_string("20 hello 40");
        assert!(m.is_err());
    }

    #[test]
    fn make_from_string_for_one_dimensional_string_returns_correct_vector() {
        let m = Matrix::<i32>::make_from_string("20 30 40").unwrap();
        assert_eq!(m.rows(), 1);
        assert_eq!(m.columns(), 3);
        assert_eq!(m.data[0], vec![20, 30, 40]);
    }

    #[test]
    fn make_from_for_two_dimensional_string_returns_correct_vector() {
        let m = Matrix::<i32>::make_from_string("20 30 40\n50 60 70\n80 90 100").unwrap();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.columns(), 3);
        assert_eq!(m.data[0], vec![20, 30, 40]);
        assert_eq!(m.data[1], vec![50, 60, 70]);
        assert_eq!(m.data[2], vec![80, 90, 100]);
    }
}
