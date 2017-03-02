use std::ops::Add;

struct Complex {
	real: i32,
	unreal: i32,
}

impl Add for Complex{
	type Output = Complex;

	fn add(self, other: Complex) -> Complex{
		Complex{
			real: self.real + other.real,
			unreal: self.unreal + other.unreal,
		}
	}
}

fn main() {
	let complexSum = Complex{ real: 1, unreal: 0} + Complex{ real: 0, unreal: 3};
	println!("{} + {}i", complexSum.real, complexSum.unreal);
}