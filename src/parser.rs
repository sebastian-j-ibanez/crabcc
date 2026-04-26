// Copyright (c) 2026 Sebastian Ibanez

#[derive(Debug, Clone)]
pub enum AstNode {
    Program,
    Function(Function),
    Return(Box<AstNode>),
    Constant(Constant),
}

// Function nodes: [NAME, BODY]
type Function = (String, Box<AstNode>);

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    Integer(i32),
    Float(f32),
    Bool(bool),
}

/*
int main() {
    return 2;
}
*/
