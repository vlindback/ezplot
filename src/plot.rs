
use num::Num;
use num::cast::AsPrimitive;
use std::fmt::Display;

pub fn horizontal_bar_chart<T>(chart_width: f64, data: &[T]) 
        
    where T: Num + AsPrimitive<f64> + PartialOrd + Display {
    if let Some(max_value) = data.iter().cloned().max_by(|a, b| a.partial_cmp(b).unwrap()) {

        let max_bar_length = (max_value.as_() / max_value.as_() * chart_width).round() as usize;

        for &value in data {
            // Scale the number of `#` characters to represent the value
            let bar_length: usize = (value.as_() / max_value.as_() * chart_width).round() as usize; // 50 is the chart width
            let bar: String = "#".repeat(bar_length);
            println!("{:>5} | {}{}", value, bar, " ".repeat(max_bar_length - bar_length) + "|");
        }
     } else {
        println!("No data available to plot!");
    }
}