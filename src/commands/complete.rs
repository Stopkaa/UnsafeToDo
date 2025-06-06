use crate::{commands::Command, todo::TodoList};

#[derive(Debug)]
pub struct CompleteCommand;

impl Command for CompleteCommand {
    fn execute(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            return Err("No Index proposed".into());
        }

        let id = args[0].parse::<usize>()?;
        let mut todo_list = TodoList::load().unwrap();
        if let Some(todo ) = todo_list.get_todo_mut(id) {
            todo.complete(true);
            todo_list.save()?;
            println!("Todo with ID: {} finished", id);
        }
        else {
            return Err(format!("Todo with ID: {} not found", id).into());
        }
        Ok(())
    }
}
