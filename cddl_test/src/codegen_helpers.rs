pub (crate) trait CodeBlock {
    fn line(&mut self, line: &str) -> &mut dyn CodeBlock;

    fn push_block(&mut self, block: codegen::Block) -> &mut dyn CodeBlock;
}

impl CodeBlock for codegen::Function {
    fn line(&mut self, line: &str) -> &mut dyn CodeBlock {
        self.line(line)
    }

    fn push_block(&mut self, block: codegen::Block) -> &mut dyn CodeBlock {
        self.push_block(block)
    }
}

impl CodeBlock for codegen::Block {
    fn line(&mut self, line: &str) -> &mut dyn CodeBlock {
        self.line(line)
    }
    
    fn push_block(&mut self, block: codegen::Block) -> &mut dyn CodeBlock {
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