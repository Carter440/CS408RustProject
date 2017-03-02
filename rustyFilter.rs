
fn iterate<F>(vect: &mut [i32], f: F) -> Vec<i32> where F: Fn(i32) -> bool{
	let mut v: Vec<i32> = vec![];
	for n in 0..10{
		if f(vect[n]){
			v.push(vect[n]);
		}
	}
	return v;
}

fn main() {
	let mut nums = vec![0,1,2,3,4,5,6,7,8,9];
	let isEven = |x| x % 2 == 0;
	let isOdd = |x| x % 2 == 1;
	let evens = iterate(nums.as_mut_slice(),isEven);
	let odds = iterate(nums.as_mut_slice(), isOdd);
	println!("{:?} \nEvens: {:?} \nOdds: {:?}\n",nums,evens,odds);
}