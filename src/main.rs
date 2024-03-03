mod math;

fn main() {
    let vec = vec![2, 5, 1, 8, 9, 13];
    let sum: i32 = vec.iter().sum();
    let mut prod = 1;
    for ele in &vec {
        prod *= ele;
    }
    let mut new_vec = Vec::new();
    for ele in &vec {
        if *ele > 3 {
            new_vec.push(ele.pow(2));
        }
    }
    println!("My first vector S is: {:?}", vec);
    println!("The sum of my vector is {sum}");
    println!("The product of my vector is {prod}");
    println!("Elements of S' <- {{ x² | x ∈ S, x > 3 }}: {:?}", new_vec);
}
