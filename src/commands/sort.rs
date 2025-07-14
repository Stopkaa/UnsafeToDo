use crate::argument::ArgumentMeta;
use crate::commands::Command;
use crate::parser::ParsedCommand;
use crate::sort_order::SortOrder;
use crate::todo::TodoList;
use crate::config;

pub struct SortCommand;

impl Command for SortCommand {
    fn execute(&self, parsed: &ParsedCommand) -> Result<(), Box<dyn std::error::Error>> {
        let mut todo_list = TodoList::load()?;
        
        // If no positional argument provided, show current sort order and available options
        if parsed.positional.is_none() {
            let current_sort = config::get_sort_order()?;
            println!("🔄 Current sort order: {}", current_sort);
            println!("\n📋 Available sort orders:");
            
            for sort_order in SortOrder::all_single() {
                let current_primary = current_sort.without_chain();
                let indicator = if sort_order == current_primary { "→" } else { " " };
                println!("   {} {}", indicator, sort_order.current_description());
            }
            
            println!("\n💡 Single criteria usage: sort <order>");
            println!("   Examples:");
            println!("   - sort priority");
            println!("   - sort due-date");
            println!("   - sort title");
            
            println!("\n🔄 Multi-criteria usage: sort <order1>,<order2>,<order3>");
            println!("   Examples:");
            println!("   - sort due-date,priority        # First by due date, then priority");
            println!("   - sort status,due-date,priority # First status, then due date, then priority");
            println!("   - sort priority,title          # First priority, then alphabetical");
            
            println!("\n🏗️  Fluent API examples (for code):");
            println!("   - SortOrder::due_date().then(SortOrder::priority())");
            println!("   - SortOrder::status().then(SortOrder::due_date()).then(SortOrder::priority())");
            
            println!("\n📝 Available criteria:");
            println!("   priority, due-date, title, status, created");
            println!("   Add '-reverse' or '-rev' for reverse order (e.g., priority-reverse)");
            
            return Ok(());
        }

        // Get the sort order argument (could be comma-separated)
        let sort_arg = parsed.positional.as_ref().unwrap();
        
        // Parse sort order from string (supports multi-criteria)
        let sort_order = SortOrder::from_string(sort_arg)
            .ok_or_else(|| {
                let available_single = SortOrder::all_single()
                    .iter()
                    .map(|s| format!("'{}'", s.short_description().to_lowercase().replace('↑', "").replace('↓', "").replace(" first", "")))
                    .collect::<Vec<_>>()
                    .join(", ");
                
                format!(
                    "Unknown sort order: '{}'\n\n💡 Single criteria: {}\n🔄 Multi-criteria: Combine with commas (e.g., 'due-date,priority')\n\nUse 'sort' without arguments to see all options.", 
                    sort_arg,
                    available_single
                )
            })?;
        todo_list.sort_by_order(&sort_order.clone());
        config::set_sort_order(sort_order.clone())?;
        todo_list.save()?;
        
        Ok(())
    }

    fn arguments(&self) -> Vec<ArgumentMeta> {
        vec![
            ArgumentMeta {
                name: "order".to_string(),
                prefix: String::new(), // Kein Prefix für positional argument
                help: "Sort order (single: 'priority', 'due-date', 'title', etc. | multi: 'due-date,priority', 'status,due-date,priority')".to_string(),
            }
        ]
    }

    fn description(&self) -> &'static str {
        "Sort todos by single or multiple criteria (e.g., 'due-date,priority')"
    }
}