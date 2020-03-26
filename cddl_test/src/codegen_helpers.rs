
pub (crate) trait CodeBlock {
    fn line<T: ToString>(&mut self, line: T) -> &mut Self;

    fn push_block(&mut self, block: codegen::Block) -> &mut Self;
}

impl CodeBlock for codegen::Function {
    fn line<T: ToString>(&mut self, line: T) -> &mut Self {
        self.line(line)
    }

    fn push_block(&mut self, block: codegen::Block) -> &mut Self {
        self.push_block(block)
    }
}

impl CodeBlock for codegen::Block {
    fn line<T: ToString>(&mut self, line: T) -> &mut Self {
        self.line(line)
    }
    
    fn push_block(&mut self, block: codegen::Block) -> &mut Self {
        self.push_block(block)
    }
}