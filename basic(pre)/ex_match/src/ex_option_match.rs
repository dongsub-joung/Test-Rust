fn main() 
{
	fn plus_one(x: Option<i32>) -> Option<i32> 
	{
		 match x 
		{
			 None => None,
			Some(i) => Some(i + 1),
		}
	}
	let five= Some(5);
	let _six= plus_one(five);
	let _none= plus_one(None);

	println!("some is {}, none is {}", _six, _none);
}
