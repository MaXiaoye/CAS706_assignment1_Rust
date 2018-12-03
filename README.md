# CAS706_assignment1_Rust


Assignment 1
Generic Quicksort. Write 3 different versions of quicksort (in each language), which explicitly:

Sort a list of integers in ascending order
Sorts a list of ``anything'' via an externally provided comparison function.
Sorts a list of ``anything'' via an externally provided comparison function, and an externally provided partition function (which itself uses the comparison function).

Please compile and run src/main.rs

Key functions:

partitionOne:  Old function for int, no external comparision function
partitionTwo:  Uses external comparision function

I learn partitionTwo and reference this partition function from 
https://rosettacode.org/wiki/Sorting_algorithms/Quicksort#Rust

partitionThree: Modify my own function to partitionThree that uses external comparision function

quick_sortOne(list, left, right): No external comparision function. Where left and right are boundaries of array (0 and length-1 by default)
quick_sortTwo(list, comparision): Uses external comparision function. Where comparision is external function. Example: &|x,y| x.len() > y.len() means length descending order.
quick_sortThree(list, comparision, partition): Uses external comparision and partition functions. Where partition is external partition function. Example: &partitionThree means to use partitionThree as partition function.

e.g. 
quick_sortThree(&mut list3, &|x,y| x.len() > y.len(), &partitionThree);



Test cases:
list1 is
 [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
Sorted list1 in ascending order
 [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

list2 is
 ["bcdef", "hkl", "airs", "catyxc", "ho", "a"]
Sorted list2 in alphabetical order
 ["a", "airs", "bcdef", "catyxc", "hkl", "ho"]
Sorted list2 in length descending order
 ["bcdef", "hkl", "airs", "catyxc", "ho", "a"]
