use std::io;
use chrono;


#[derive(Debug)]
pub struct Expense{
    pub id:u32,
    pub name:String,
    pub amount:f32,
    pub date:String,
}


fn main() {
    println!("Please enter you limit");
let mut expense_limit = String::new();
io::stdin().read_line(&mut expense_limit).expect("Failed to read new line");
let expense_limit = expense_limit.trim().parse::<f32>().unwrap();


let mut expenses:Vec<Expense> = Vec::new();
    loop{

        println!("What do you want to do ?");
        println!("1. Add expense.");
        println!("2. Edit expense.");
        println!("3. Remove expense");
        println!("4. Display expense");
        println!("5. Display total expense");
        println!("6. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim().parse::<u32>().unwrap();

        

        match choice{
            1 =>{
                println!("Enter the expense item");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                println!("Enter the amount");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount = amount.trim().parse::<u32>().expect("Invalid input");

                

                let date = chrono::Local::now();
                println!("{}",date.format("%d %b %Y   %H:%M:%S").to_string());
                

                let id = expenses.len() as u32 + 1;

                let item = Expense{
                    id,
                    name,
                    amount: amount as f32,
                    date: date.to_string(),
                };
                
                expenses.push(item);
                
            },
            2 =>{
                println!("Enter the id of expense to update");

                let mut id =String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<usize>().unwrap();
                if id > expenses.len(){ 
                    println!("Invalid input");
                }else{
                    println!("Enter the new expense name");

                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name).expect("Failed to read line");
                let new_name = new_name.trim().to_string();

                

                println!("Enter the new amount");

                let mut new_amount = String::new();
                io::stdin().read_line(&mut new_amount).expect("Failed to read line");
                let new_amount = new_amount.trim().parse::<u32>().unwrap();

                edit_expense( &mut expenses,id,new_name, new_amount);
                println!("Expense with id {} updated successfully",&id);
                }

                
            },
            3 =>{
                remove_expense(&mut expenses);
            },
            4 =>{
                
                display_expenses(&expenses);
            },
            5 =>{
                total_expense(&expenses,expense_limit);
            }

            9 =>{
                println!("Exited successfully");
                return;
            }
            _ => {
                println!("Invalid choice");
            }
        }



    }
    
}



fn edit_expense(expenses:&mut Vec<Expense>, id:usize,new_name: String,new_amount:u32){
    if let Some(expense) = expenses.iter_mut().find(|exp| exp.id == id as u32){
        expense.name = new_name;
        expense.amount = new_amount as f32;
        
    }else{
        println!("Expense with the id could not be found");
    }
}

fn display_expenses(expense:&Vec<Expense>){
    for exp in expense{
        println!("{} - {} {} {}",exp.id,exp.name,exp.amount,exp.date);
    }
}

fn remove_expense(expense:&mut Vec<Expense>){
println!("Please enter the id of expense to remove");

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");
let input = input.trim().parse::<usize>().unwrap();

if input >= expense.len(){
    println!("Invalid input");
}else{
    expense.remove(input);
    println!("expense deleted successfully");
}


}

fn total_expense(expenses:&Vec<Expense>,limit:f32,){
    let total:f32= expenses.iter().map(|exp| exp.amount).sum();
    if total > limit{
        println!("You have exceeded your limit");
        
    }else{
          println!("Your total expense is {}",total)
    }
    
    
    
}

