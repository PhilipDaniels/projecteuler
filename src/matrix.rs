use std::str::FromStr;
use std::ops;

/// A crude 2d matrix class.
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T> Matrix<T> {
    /// Returns the number of rows in the matrix. 0 for an empty matrix.
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix. The matrix is assumed to be a rectangle,
    /// so this is just the length of the first row.
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns true if the matrix is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.rows == 0
    }
}

// This allows us to extract a row from the Matrix as a slice, which is itself indexable, so you
// get a 2d matrix syntax of `m[row][col]`.
impl<T> ops::Index<usize> for Matrix<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &[T] {
        let start = row * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<T> FromStr for Matrix<T>
    where T: FromStr
{
    type Err = String;

    /// Constructs a matrix from a string. The string should consist of one or more lines separated
    /// by `\n` characters. Each line should consist of 1 or more values separated by spaces.
    /// Each line must have the same number of elements.
    /// The matrix is immutable once constructed. The elements cannot currently be changed, nor
    /// can rows and columns be added or deleted.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Matrix { rows: 0, cols: 0, data: Vec::new() };

        // TODO: Improve this mess. Use a custom error type? Allocate less.
        for line in s.lines() {
            let mut row = Vec::new();
            for s in line.split(' ').collect::<Vec<_>>() {
                match s.parse::<T>() {
                    Ok(v) => row.push(v),
                    Err(_) => return Err(format!("Error, cannot convert {} to a number", s))
                }
            }


            result.rows += 1;
            if result.cols == 0 {
                result.cols = row.len();
            } else if result.cols != row.len() {
                return Err(format!("Number of columns on row {} differs from {} columns on first row", row.len(), result.cols));
            }

            result.data.extend(row);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use std::str::FromStr;

    #[test]
    fn from_str_for_empty_string_returns_empty_matrix() {
        let m = Matrix::<i32>::from_str("").unwrap();
        assert!(m.is_empty());
        assert_eq!(m.rows(), 0);
        assert_eq!(m.cols(), 0);
    }

    #[test]
    fn from_str_for_one_dimensional_string_returns_correct_vector() {
        let m = Matrix::<i32>::from_str("20 30 40").unwrap();
        assert_eq!(m.rows(), 1);
        assert_eq!(m.cols(), 3);
        assert_eq!(m[0], [20, 30, 40]);
    }

    #[test]
    fn from_str_for_two_dimensional_string_returns_correct_vector() {
        let m = Matrix::<i32>::from_str("10 20 30\n40 50 60\n70 80 90").unwrap();
        assert_eq!(m.rows(), 3);
        assert_eq!(m.cols(), 3);
        assert_eq!(m[0], [10, 20, 30]);
        assert_eq!(m[1], [40, 50, 60]);
        assert_eq!(m[2], [70, 80, 90]);
    }

    #[test]
    fn from_str_for_string_containing_non_numbers_returns_err() {
        let m = Matrix::<i32>::from_str("20 hello 40");
        assert!(m.is_err());
    }

    #[test]
    fn from_str_for_string_containing_different_numbers_of_cols_returns_err() {
        let m = Matrix::<i32>::from_str("10\n20 30");
        let err_msg = m.err().unwrap();
        assert!(err_msg.contains("Number of columns on row 2 differs from 1 columns on first row"));
    }
}
