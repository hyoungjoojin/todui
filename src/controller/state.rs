pub enum State {
    Continue,
    Reload,
    Break,
    Error,
    PostTask,

    DeleteTask(String),
}
