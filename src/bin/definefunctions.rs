//An example to show defining and passing around functions

fn simple_function() {
    println!("function called");
}

fn add(a:i32, b:i32) -> i32  {
    a + b
}
fn higer_order_fn<F>(value:i32, step: F)  -> i32
where F: Fn(i32) -> i32 {
    step(value)
}

fn higer_order_fn_return<'a>(step_value:& 'a i32) ->   Box<Fn(i32) -> i32 + 'a > {
       Box::new(move |x:i32| x+step_value)
   }
fn main() {

    simple_function();

    //call add function normally
    println!("{}", add(10,20));

    //save function in a variable

    let fn_variable = add;

    println!("calling using function variable {}",fn_variable(10,20));

    fn add_one(x:i32)->i32 { x+1}
    let result = higer_order_fn(20, add_one);
    println!("result after using higher order fn {}", result);
    // higher order function with function literal
    let result = higer_order_fn(20, |x:i32| x +1 );
    println!("result after using higher order fn {}", result);
    //capture return function
    let step_value = &10;
    let step_function = higer_order_fn_return(step_value);
    println!("the stepped value is{}", step_function(50)); 
}
