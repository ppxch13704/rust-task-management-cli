use std::io::{self, Write};
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;


#[derive(Debug)]
struct Task{
    title: String,
    done: bool,
}



/*fn print_task(task: &Vec<Task>){
        println!("Task : ");
    for count in 0..task.len(){
        if task[count].done == true{ 
            print!("[✓]");
        }
        else {
            print!("[ ]");
        }
        println!("{}", task[count].title);
        } 
    }
*/
fn write_file(task: &Vec<Task>){
    
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("task.txt")
        .expect("Failed to open file");

    //writeln!(file,"{:?}",task);
    for count in 0..task.len(){
        if task[count].done == true{
            write!(file,"[✓]");
        }else {
            write!(file,"[ ]");
        }
        writeln!(file,"{}",task[count].title);
    }
}
fn read_file()->Result<(), Box<dyn Error>>{
    print!("Task : \n");
    let contents: String = fs::read_to_string("task.txt")?;
    println!("{}", contents);
    Ok(())
}


fn main(){

    

    let mut task_vec: Vec<Task> = Vec::new();
    
    loop{
        let mut select_task = String::new();
        io::stdout().flush().unwrap();
        println!("Features\n1. Add Task\n2. Show Tasks\n3. Complete Task\n4. Delete Task\n5. Exit\n\nEnter number to select: \n\n");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Please enter only a number !");
        match choice.trim(){
            "1" => {
                print!("Name a new Task : ");
                io::stdout().flush();
                let mut new_task = String::new();
                io::stdin().read_line(&mut new_task).expect("Number only!");
                task_vec.push(Task {
                    title: String::from(new_task.trim().to_string()),
                    done: false
                });
                println!("Task Added !");
                write_file(&task_vec);
            }
            "2" => {
                
                read_file();
            }
            "3" => {
                read_file();
                print!("Choose : ");
                io::stdout().flush();
                io::stdin().read_line(&mut select_task).expect("Number only !");
                let idx: usize = select_task.trim().parse().expect("Number");
                task_vec[idx-1].done = true;
                println!("Task Completed !");
                write_file(&task_vec);
            }
            "4" =>{
                read_file();
                print!("Choose to delete : ");
                io::stdout().flush();
                io::stdin().read_line(&mut select_task).expect("Number only !");
                let idx: usize = select_task.trim().parse().expect("Number only !");
                task_vec.remove(idx-1);
                println!("Task Deleted !");
                write_file(&task_vec);
                
            }
            "5" =>{
                println!("Exitting Program...");break;
            }
            _ => {
                println!("Enter Number only !");
            }
        }
    }
    
}
