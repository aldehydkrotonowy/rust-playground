pub fn run() {
	// Array is primitive value in rust;
	let arr1 = [1, 2, 3];
	let arr2 = arr1;

	println!("arrays: {:?}", (arr1, arr2));

	// now the same with non-primiteve vector

	// With non-primitives, if you assign another variable to a piece of data,
	// the fiest variable wll no longer hold that value. You'll need to use a reference (&)
	// to point to the resource.

	let vect1 = vec![1, 2, 3];
	let vect2 = &vect1;

	println!("vectors: {:?}", (&vect1, vect2));
}
