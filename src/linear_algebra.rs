pub(crate) type Matrix = Vec<Vec<f64>>;

pub(crate) trait MatrixExt {
    /// Returns the number of rows in the matrix.
    fn rows(&self) -> usize;

    /// Returns the number of columns in the matrix.
    fn columns(&self) -> usize;

    /// Computes the multiplication of the matrix by the given vector v.
    ///
    /// Let M be a n x m matrix, and let v be a vector with m columns. Then
    /// Mv gives a vector of length n.
    fn vector_product(&self, v: &[f64]) -> Vec<f64>;
}

impl MatrixExt for Matrix {
    fn rows(&self) -> usize {
        self.len()
    }

    fn columns(&self) -> usize {
        self[0].len()
    }

    fn vector_product(&self, v: &[f64]) -> Vec<f64> {
        if self.columns() != v.len() {
            panic!("number of matrix columns and number of vector rows must be equal");
        }

        let mut result = Vec::new();

        for row in self {
            let mut sum = 0.0;
            for (i, x) in row.iter().enumerate() {
                sum += v[i] * x;
            }
            result.push(sum);
        }

        result
    }
}

pub(crate) type Vector = Vec<f64>;

pub(crate) trait VectorExt {
    /// Returns the sum of the vector and the given vector y.
    fn sum(&self, y: &[f64]) -> Vec<f64>;

    /// Returns the difference between the vector and the given vector y.
    fn difference(&self, y: &[f64]) -> Vec<f64>;

    /// Returns the vector multiplied by the scalar c.
    fn scale(&self, c: f64) -> Vec<f64>;

    /// Returns the dot-product of the vector and the given vector y.
    fn dot_product(&self, y: &[f64]) -> Vec<f64>;

    /// Computes the multiplication of the vector by the given matrix m.
    ///
    /// Let v be a vector with n columns, and let M be an n x m matrix. Then
    /// vM is the transpose of the vector v multiplied by M, and it gives a
    /// vector of length m.
    fn maxtrix_product(&self, m: &Matrix) -> Vec<f64>;
}

impl VectorExt for Vector {
    fn sum(&self, y: &[f64]) -> Vec<f64> {
        if self.len() != y.len() {
            panic!("number of rows in the vectors must be equal");
        }

        let mut result = Vec::new();

        for (i, v) in self.iter().enumerate() {
            let sum = v + y.get(i).unwrap();
            result.push(sum);
        }

        result
    }

    fn difference(&self, y: &[f64]) -> Vec<f64> {
        if self.len() != y.len() {
            panic!("number of rows in the vectors must be equal");
        }

        let mut result = Vec::new();

        for (i, v) in self.iter().enumerate() {
            let diff = v - y.get(i).unwrap();
            result.push(diff);
        }

        result
    }

    fn scale(&self, c: f64) -> Vec<f64> {
        self.iter().map(|v| c * v).collect()
    }

    fn dot_product(&self, y: &[f64]) -> Vec<f64> {
        if self.len() != y.len() {
            panic!("number of rows in the vectors must be equal");
        }

        let mut result = Vec::new();

        for (i, v) in self.iter().enumerate() {
            let prod = v * y.get(i).unwrap();
            result.push(prod);
        }

        result
    }

    fn maxtrix_product(&self, m: &Matrix) -> Vec<f64> {
        if self.len() != m.rows() {
            panic!("number of vector columns and number of matrix rows must be equal");
        }

        let mut result = Vec::new();

        result
    }
}
