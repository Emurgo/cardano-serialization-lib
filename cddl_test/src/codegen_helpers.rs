
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

pub (crate) trait DataType {
    fn derive(&mut self, derive: &str) -> &mut Self;
}

impl DataType for codegen::Struct {
    fn derive(&mut self, derive: &str) -> &mut Self {
        self.derive(derive)
    }
}

impl DataType for codegen::Enum {
    fn derive(&mut self, derive: &str) -> &mut Self {
        self.derive(derive)
    }
}