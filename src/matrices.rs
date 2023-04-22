#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    // Create a new matrix with the specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    // Create a new matrix based on a string, with the first two characters being the rows and
    // columns respectively
    pub fn create_from_string(s : &String) -> Self {
        let mut split = s.split_whitespace();
        let row_s: usize = split.next().unwrap().parse().unwrap();
        let col_s: usize = split.next().unwrap().parse().unwrap();
        let mut mat = Matrix::new(row_s, col_s);
        let mut curvec = vec![0.0; row_s * col_s];
        let mut curpos = 0;
        for val_s in split{
            let f = val_s.parse().unwrap();
            curvec[curpos] = f; 
            curpos += 1;
        }
        mat.load_from_vector(curvec);
        mat
        
    }

    //retuns a vector of the rows
    pub fn get_rows(&self) -> Vec<Vec<f64>> {
        let mut rows = Vec::with_capacity(self.rows);
        for i in 0..self.rows {
            let row_start = i * self.cols;
            let row_end = row_start + self.cols;
            rows.push(self.data[row_start..row_end].to_vec());
        }
        rows
    }

    // Get the element at the specified row and column
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    // Set the element at the specified row and column
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.data[row * self.cols + col] = val;
    }

    // Transpose the matrix
    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }

    // Multiply the matrix by a scalar
    pub fn scalar_multiply(&self, scalar: f64) -> Self {
        let mut result = Matrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) * scalar);
            }
        }
        result
    }

    // Multiply the matrix by another matrix
    pub fn matrix_multiply(&self, other: &Matrix) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    // Swap two rows in the matrix
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        for j in 0..self.cols {
            let temp = self.get(row1, j);
            self.set(row1, j, self.get(row2, j));
            self.set(row2, j, temp);
        }
    }

    // Scale a row in the matrix by a factor
    pub fn scale_row(&mut self, row: usize, factor: f64) {
        for j in 0..self.cols {
            let val = self.get(row, j) * factor;
            self.set(row, j, val);
        }
    }

    // Add a multiple of one row to another row
    pub fn add_multiple_of_row(&mut self, src_row: usize, dest_row: usize, factor: f64) {
        for j in 0..self.cols {
            let val = self.get(src_row, j) * factor + self.get(dest_row, j);
            self.set(dest_row, j, val);
        }
    }
    pub fn load_from_vector(&mut self, vector : Vec<f64>){
        let mut currow = 0;
        let mut curcol = 0;
        for val in vector.iter() {
            self.set(currow, curcol, *val);
            curcol += 1;
            if curcol % self.cols == 0{
                curcol = 0;
                currow += 1;
            }
        }
    }
    // gets the transpose of a matrix
    pub fn transpose(&mut self) {
        for i in 0..self.rows {
            for j in i+1..self.cols {
                let index1 = i * self.cols + j;
                let index2 = j * self.cols + i;
                self.data.swap(index1, index2);
            }
        }
        std::mem::swap(&mut self.rows, &mut self.cols);
    }
    // Convert the matrix to echelon form using Gaussian elimination
    pub fn echelon_form(&mut self) {
        let mut lead = 0;
        for r in 0..self.rows {
            if lead >= self.cols {
                return;
            }
            let mut i = r;
            while self.get(i, lead) == 0.0 {
                i += 1;
                if i == self.rows {
                    i = r;
                    lead += 1;
                    if lead == self.cols {
                        return;
                    }
                }
            }
            if i != r {
                self.swap_rows(i, r);
            }
            self.scale_row(r, 1.0 / self.get(r, lead));
            for i in 0..self.rows {
                if i != r {
                    self.add_multiple_of_row(r, i, -self.get(i, lead));
                }
            }
            lead += 1;
        }
    }
    // matrix inverse
    pub fn inverse(&self) -> Option<Matrix> {
        if self.rows != self.cols {
            return None; // Non-square matrices are not invertible
        }

        let n = self.rows;
        let mut a = self.clone();
        let mut b = Matrix::identity(n);

        for i in 0..n {
            let mut j = i;
            while j < n && a[(j, i)] == 0.0 {
                j += 1;
            }
            if j == n {
                return None; // Matrix is singular
            }
            if j != i {
                a.swap_rows(i, j);
                b.swap_rows(i, j);
            }
            let pivot = a[(i, i)];
            a.scale_row(i, 1.0 / pivot);
            b.scale_row(i, 1.0 / pivot);
            for k in 0..n {
                if k != i {
                    let factor = a[(k, i)];
                    a.add_scaled_row(k, i, -factor);
                    b.add_scaled_row(k, i, -factor);
                }
            }
        }

        Some(b)
    }

    // calculates the determinant
    pub fn determinant(&self) -> Option<f64> {
        if self.rows != self.cols {
            return None; // Determinant is only defined for square matrices
        }

        let n = self.rows;
        let mut a = self.clone();
        let mut det = 1.0;

        for i in 0..n {
            let mut j = i;
            while j < n && a[(j, i)] == 0.0 {
                j += 1;
            }
            if j == n {
                return Some(0.0); // Matrix is singular
            }
            if j != i {
                a.swap_rows(i, j);
                det = -det;
            }
            let pivot = a[(i, i)];
            det *= pivot;
            a.scale_row(i, 1.0 / pivot);
            for k in i+1..n {
                let factor = a[(k, i)];
                a.add_scaled_row(k, i, -factor);
            }
        }

        Some(det)
    }
    // returns a string
    pub fn to_string(&self) -> String {
        format!("{:?}", self.data)
    }
}
