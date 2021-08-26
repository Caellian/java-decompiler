use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    Primitive,
    Class,
    Generic,
}

#[derive(Debug)]
pub enum ControlFlow {
    If {
        condition: Expression,
        content: ExpressionBlock,
        else_ifs: Vec<ControlFlow>,
        else_block: Option<ExpressionBlock>,
    },
    While {
        condition: Expression,
        content: ExpressionBlock,
    },
    DoWhile {
        condition: Expression,
        content: ExpressionBlock,
    },
    For {
        setup: Expression,
        condition: Expression,
        content: ExpressionBlock,
        increment: Expression,
    },
    Try {
        setup: Option<Expression>,
        content: ExpressionBlock,
        catch: HashMap<Type, ExpressionBlock>,
        finally: Option<ExpressionBlock>,
    },
}

#[derive(Debug)]
pub struct Declaration {
    of: String,
    t: Type,
    value: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Assignment,
    Literal { value: u32 },
    FlowControl(Box<ControlFlow>),
}

#[derive(Debug)]
pub struct ExpressionBlock {}
