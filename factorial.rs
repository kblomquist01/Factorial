use std::io;
fn main(){
    println!("what factorial number (above 1) do you want to know?");
    let mut fact_num = String::new();
    io::stdin().read_line(&mut fact_num);

    let num = fact_num.trim().parse::<usize>().expect("Invalid input");

    let mut sum = 1;

    
    sum = find_fact(1, num, sum);
    
	
    println!("the factorial of {} is {}", num, sum);

    println!("press enter to close");
    
    io::stdin().read_line(&mut fact_num);
}

fn find_fact(n1: usize, n2: usize, current: usize) -> usize{
    if(n1 <= n2){
        let sum = n1 * current;
	return find_fact(n1+1, n2, sum);
    }
    return current;
}
