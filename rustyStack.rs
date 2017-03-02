use Stack::*;

enum Stack{
	Cons(u32, Box<Stack>),
	Nil,
}

impl Stack{
	
	fn new() -> Stack {
		Nil
	}

	fn push(self, elem: u32) -> Stack{
		Cons(elem, Box::new(self))
	}

	fn peek(&self) -> u32 {
		match *self{
			Cons(head,_) => {
				head
			},
			Nil => 0u32,
		}
	}

	fn pop(self) -> Stack{
		match self{
			Cons(_, tail) =>{
				*tail
			},
			Nil => Nil,
		}
	}
	
	fn len(&self) -> u32 {
		match *self{
		Cons(_,ref tail) => 1 + tail.len(),
		Nil => 0
		}
	}

	fn stringify(&self) -> String{
		match *self{
			Cons(head, ref tail) => {
				format!("{}, {}", head, tail.stringify())
			},
			Nil=>{
				format!("Nil")
			},
		}
	}
}



fn main(){
	let mut stack = Stack::new();

	stack = stack.push(1);
	stack = stack.push(2);
	stack = stack.push(3);

	println!("stack of length: {}", stack.len());
	println!("{}", stack.stringify());
	println!("");

	stack = stack.pop();
	stack = stack.pop();

	println!("stack of length: {}", stack.len());
	println!("{}", stack.peek());
	println!("");
}