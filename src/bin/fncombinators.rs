//function combinators example
fn main() {
 let vector = vec!(1,3,4,5,3);
  
 //map values
 let mapped_vector = vector.iter().map(|&x| x +1).collect::<Vec<i32>>();
 println!("{:?}",mapped_vector);

 //filter values
 let filtered_values = vector.iter().filter(|&x| x%2 ==0).collect::<Vec<&i32>>();
 println!("{:?}",filtered_values);

 //count number of items
 let vec_count = vector.iter().count();
 println!("vector count  is {}",vec_count);

 //zip with index
 let index_vec = 0..vec_count;
 let index_zipped_vector = vector.iter().zip(index_vec).collect::<Vec<(&i32,usize)>>(); 
 println!("zipped vector is {:?}",index_zipped_vector);

 //fold to find the sum
 
 let sum = vector.iter().fold(0,(|sum,value| sum+value));
 println!("sum of the vector is {}",sum);

 //find max 
 let max = vector.iter().max().unwrap();
 println!("max is {}",max);

 //forall 
 let greater_than_zero = vector.iter().all(|&x| x > 0 ) ;
 println!("greater than zero {}",greater_than_zero);

 //list of words using flat map
 let lines_vec = vec!("hello,how","are,you");
 let words_vec = lines_vec.iter().flat_map(|&x| x.split(",")).collect::<Vec<&str>>();
 println!("{:?}", words_vec);
}
