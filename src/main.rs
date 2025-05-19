mod linear_algebra;

use std::vec;
use crate::linear_algebra::MatrixExt;
use crate::linear_algebra::VectorExt;

fn main() {
    #[allow(non_snake_case)]
    let M = vec![
        vec![2.0, 4.0, -3.0],
        vec![21.0, -6.0, -1.0],
    ];

    println!("the shape of m is {} X {}", M.rows(), M.columns());
    println!("M is {:?}", M);

    let u = vec![21.0, -6.0];
    let v = vec![2.0, 4.0, -3.0];
    let w = vec![21.0, -6.0, -1.0];

    let c = 10.0;

    println!("v is {:?}", v);
    println!("w is {:?}", w);
    println!("v + w is {:?}", v.sum(&w));
    println!("v - w is {:?}", v.difference(&w));
    println!("c is {c} and cx is {:?}", v.scale(c));
    println!("v • w is {:?}", v.dot_product(&w));
    println!("Mv is {:?}", M.vector_product(&v));
    println!("uM is {:?}", u.maxtrix_product(&M));
}
