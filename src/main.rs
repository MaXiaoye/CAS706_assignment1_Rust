fn partition(list: &mut [u32], left: isize, right: isize) -> isize {

	if left >= right {return 0};
	let mut list = list;
	let mut left_1 = left;
	//println!("left is: {}", left_1);
	
	let mut right_1 = right - 1 ;
	//let mut right_1 = right;
	//println!("right is: {}", right_1);
	
	let pivot = list[right as usize];
	
	while left_1 < right_1 {
	
		while list[left_1 as usize] < pivot && left_1 < right {left_1 +=1;}
		while list[right_1 as usize] >= pivot && right_1 >0 {right_1 -=1;}
	
		if left_1 < right_1 {list.swap(left_1 as usize, right_1 as usize)};
	
		//println!("right is: {}, left is: {}", left_1, right_1)
		//println!("{:?}", list);
	
	}
	
	list.swap(left_1 as usize, right as usize);
	println!("{:?}", list);
	
	left_1
}

fn quick_sort(list: &mut [u32], left: isize, right: isize) {
	
	if left < right {
	
		let index = partition(list, left, right);
		quick_sort(list, left, index-1);
		quick_sort(list, index+1, right);
	
	}
}

fn main() {
    //println!("Hello, world!");
    let mut a = [1,4,3,2,2,3,6,4,2,1];
    let list_size = a.len() - 1;
    //println!("{}", list_size);
    //let index = partition(&mut a, 0, 5);
    //println!("index is {}", index);
    //quick_sort(&mut a, 0, list_size as isize);
    quick_sort(&mut a, 0, 9);
    println!("{:?}", a);
}
