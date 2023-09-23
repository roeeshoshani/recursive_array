use recursive_array::RecursiveArray;

fn main() {
    let array = recursive_array::recursive_array![1, 2, 3];
    let array2 = array.push_back(4);
    println!("{:?}", array2.as_slice())
}
