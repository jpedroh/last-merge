use model::CSTNode;

pub type ParsingHandler = fn(root: CSTNode) -> CSTNode;

pub struct ParsingHandlers {
    handlers: Vec<ParsingHandler>,
}

impl ParsingHandlers {
    pub fn new(handlers: Vec<ParsingHandler>) -> Self {
        Self { handlers }
    }

    pub fn run<'a>(&'a self, root: CSTNode<'a>) -> CSTNode<'a> {
        self.handlers.iter().fold(root, |acc, handler| handler(acc))
    }
}
