use crate::lib::calendar;

mod lib;

fn main() {
    let year: calendar::year::Year = calendar::year::year_2023();

    println!("Year: {}", year.id);
    println!("Gross Income: {}", year.gross_income());
    println!("Gross Expense: {}", year.gross_expense());
    println!("Net Income: {}", year.net_income());
    for month in year.month_array().iter() {
        println!(
            "=== Month: {} === {}/{} ===",
            month.display_name(),
            month.display_number(),
            year.id
        );
        println!("Starting Savings: {}", month.savings_at_start);
        println!("Budget: {}", month.budget);
        println!("Gross Income: {}", month.gross_income());
        println!("Gross Expense: {}", month.gross_expense());
        println!("Net Income: {}", month.net_income());
    }
}
