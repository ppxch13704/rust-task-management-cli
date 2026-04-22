use std::io::{self, Write};

#[derive(Clone)]
#[derive(Debug)]
struct Task{
    title: String,
    done: bool,
}



fn print_task(task: &Vec<Task>){
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
                io::stdin().read_line(&mut new_task);
                task_vec.push(Task {
                    title: String::from(new_task.trim().to_string()),
                    done: false
                });
                println!("Task Added !");
            }
            "2" => {
                print_task(&task_vec);
            }
            "3" => {
                print_task(&task_vec);
                print!("Choose : ");
                io::stdout().flush();
                io::stdin().read_line(&mut select_task);
                let idx: usize = select_task.trim().parse().expect("Number");
                task_vec[idx-1].done = true;
                println!("Task Completed !");

            }
            "4" =>{
                print_task(&task_vec);
                print!("Choose to delete : ");
                io::stdout().flush();
                io::stdin().read_line(&mut select_task);
                let idx: usize = select_task.trim().parse().expect("Number");
                task_vec.remove(idx-1);
                println!("Task Deleted !");

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
