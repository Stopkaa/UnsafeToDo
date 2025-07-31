#[derive(Debug)]
pub struct ArgumentMeta {
    pub name: String,
    pub prefix: String,
    pub help: String,
}

pub trait Argument {
    fn meta(&self) -> &ArgumentMeta;
}

pub fn priority_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "priority".to_string(),
        prefix: "p".to_string(),
        help: "Optional priority, e.g. -p high".to_string(),
    }
}

pub fn due_date_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "due_date".to_string(),
        prefix: "d".to_string(),
        help: "Optional due date, e.g. -d DD.MM.YYYY".to_string(),
    }
}

pub fn description_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "description".to_string(),
        prefix: "m".to_string(),
        help: "m for memo, optional description, e.g. -m \"Task details\"".to_string(),
    }
}

pub fn title_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "title".to_string(),
        prefix: "t".to_string(),
        help: "change title e.g. -t \"New Title\"".to_string(),
    }
}

pub fn finished_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "finished".to_string(),
        prefix: "f".to_string(),
        help: "change finish state, false: not finished, true: finished, e.g. -f true".to_string(),
    }
}

pub fn config_git_remote_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "git_remote_path".to_string(),
        prefix: "r".to_string(),
        help: "change git_remote_path, e.g. -r git@github-haw:torbenrathje/unsafeTodoData.git".to_string(),
    }
}

pub fn config_auto_sync_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "auto_sync".to_string(),
        prefix: "a".to_string(),
        help: "change auto_sync state, true: auto sync enabled, false: auto_sync disabled, e.g. -a true".to_string(),
    }
}

pub fn config_show_argument() -> ArgumentMeta {
    ArgumentMeta {
        name: "config_show".to_string(),
        prefix: "s".to_string(),
        help: "shows current configuration".to_string(),
    }
}