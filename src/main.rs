//partition function one for int, no external comparision function
fn partitionOne(list: &mut [i32], left: isize, right: isize) -> isize {
	/*partition is done, return*/
	if left >= right {return 0};
	/*test list */
	let mut list = list;
	/*left check point starts from the first number */
	let mut left_1 = left;	
	/*right check point starts from the second last number */
	let mut right_1 = right - 1 ;
	/*select the last number as pivot*/
	let pivot = list[right as usize];
	/*partition starts */
	while left_1 < right_1 {
		/*left check point keep moving forward until it finds a number bigger than pivot */
		while list[left_1 as usize] < pivot && left_1 < right {left_1 +=1;}
		/*right check point keep moving to head until it finds a number less than pivot */
		while list[right_1 as usize] >= pivot && right_1 > left {right_1 -=1;}
		/*swap two numbers, move the number bigger than pivot to left and move the less one to right*/
		if left_1 < right_1 {list.swap(left_1 as usize, right_1 as usize)};
	}
	/*partition ends when two check points meet,then swap pivot and the left check point to make sure pivot is in the correct location*/
	list.swap(left_1 as usize, right as usize);
	/*return location of the pivot*/
	left_1
}
//partition function two, uses external comparision function. 
//I learn and reference this partition function from 
//https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust
fn partitionTwo<T,F>(list: &mut [T], comparision: &F) -> usize
	where F:Fn(&T,&T) -> bool
	{
		let length = list.len();
		let index = length / 2;
		list.swap(index,length - 1);
		let mut index_1 = 0;
		
		for i in 0..length-1 {
			if comparision(&list[i],&list[length - 1]) {
				list.swap(i,index_1);
				index_1 += 1;
			}
		}
		list.swap(index_1, length - 1);
		index_1
}

//Modify my own function to partitionThree that uses external comparision function
fn partitionThree<T,F>(list: &mut [T], comparision: &F) -> usize 
	where F:Fn(&T,&T) -> bool
	{
	/*left check point starts from the first number */
	let mut left = 0;	
	/*right check point starts from the second last number */
	let mut right = list.len() - 2;
	/*select the last number as pivot*/
	let mut pivot = list.len() - 1;
	/*partition starts */
	while left < right {
		/*left check point keep moving forward until it finds a number bigger than pivot */
		while comparision(&list[left], &list[pivot]) && left < right {left +=1;}
		/*right check point keep moving to head until it finds a number less than pivot */
		while !comparision(&list[right], &list[pivot]) && right > left {right -=1;}
		/*swap two numbers, move the number bigger than pivot to left and move the less one to right*/
		if left < right {list.swap(left, right)};
	}
	/*partition ends when two check points meet,then swap pivot and the left check point to make sure pivot is in the correct location*/
	list.swap(left, right);
	/*return location of the pivot*/
	left
}

//quicksort function one, no external comparision function
fn quick_sortOne(list: &mut [i32], left: isize, right: isize) {
	if left < right {
		/*partition firstly, index is the location of pivot*/
		let index = partitionOne(list, left, right);
		/*rec quicksort for the left and right part*/
		quick_sortOne(list, left, index-1);
		quick_sortOne(list, index+1, right);
	}
}
//quicksort function two, uses external comparision function
fn quick_sortTwo<T,F>(list: &mut [T], comparision: &F) 
	where F:Fn(&T,&T) -> bool 
	{
		let length = list.len();
		if length >= 2 {
			/*partition firstly, index is the location of pivot*/
			let index = partitionTwo(list, comparision);
			/*rec quicksort for the left and right part*/
			quick_sortTwo(&mut list[0..index], comparision);
			quick_sortTwo(&mut list[index+1..length], comparision);	
		}	
}
//quicksort function two, uses external comparision and partition functions
fn quick_sortThree<T,F,P>(list: &mut [T], comparision: &F, partition: &P) 
	where F:Fn(&T,&T) -> bool, P:Fn(&mut [T], &F) -> usize 
	{
		let length = list.len();
		if length >= 2 {
			/*partition firstly, index is the location of pivot*/
			let index = partition(list, comparision);
			/*rec quicksort for the left and right part*/
			quick_sortThree(&mut list[0..index], comparision, partition);
			quick_sortThree(&mut list[index+1..length], comparision, partition);	
		}	
}

fn main() {    
	let mut list1 = [10,9,8,7,6,5,4,3,2,1];
	let right_point1 = list1.len() - 1;
	println!("list1 is\n {:?}\n", list1);
	quick_sortOne(&mut list1, 0, right_point1 as isize);
	println!("Sorted list1 in ascending order\n {:?}\n", list1);
	
	let mut list2 = ["bcdef", "hkl", "airs", "catyxc", "ho", "a"];
	println!("list2 is\n {:?}\n", list2);
	quick_sortTwo(&mut list2, &|x,y| x < y);
	println!("Sorted list2 in alphabetical order\n {:?}\n", list2);	
	
	let mut list3 = ["bcdef", "hkl", "airs", "catyxc", "ho", "a"];
	quick_sortThree(&mut list3, &|x,y| x.len() > y.len(), &partitionThree);
	println!("Sorted list2 in length descending order\n {:?}\n", list3);
}
