fn f(x:f64) -> f64{
	return (x*x) - 4.0;
}
fn f_poch(x:f64) -> f64{
	return 2.0*x;
}
/*
fn met_newt(a:f64, eps:f64, N: u128) -> f64{
	let mut x = a;
	loop{
		let fx = f(x);
		let poch = f_poch(x);
		let x_next = x - (fx/poch);
        	if (x_next - x).abs() < eps {
        		 return x_next;
        }
        x = x_next;
	}
}
*/
fn met_newt(a:f64, eps:f64, N: u128) -> f64{
	let mut x = a;
	for z in 0..N{
	let fx = f(x);
        let poch = f_poch(x);
        let x_next = x - (fx / poch);
        if (x_next - x).abs() < eps {
            return x_next;
        }
        x = x_next;
    }
    return x;
	
}
fn main() {
	let mut n = 1.0;
	let mut eps = 0.001;
	let mut N = 10000; 
	let result = met_newt(n, eps, N);
	println!("Results: {}", result);
}
