mod plot;

fn main() {


    let data: [f64; 25] = [ 3.4 ,4.8, 4.4, 3.6, 3.7, 4.2, 4.0, 3.5, 3.7, 3.8,
                        3.6, 3.6, 3.8, 3.3, 3.5, 3.4, 3.3 ,3.4, 3.7, 3.7, 
                        3.7, 4.0, 4.0, 4.0, 4.1];
    
    let args: std::env::Args = std::env::args();

    if args.len() < 2 {
        println!("Usage: program <name>");
        return
    } else {
        plot::horizontal_bar_chart(50.0, &data);
    }
}
