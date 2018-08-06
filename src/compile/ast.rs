#[derive(Debug, Clone, PartialEq)]
pub struct ProgramAST {
    pub stmt_list: Vec<StmtAST>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtAST {
    ExprAST(ExprAST),
    InfixAST(InfixAST),
}

impl StmtAST {
    pub fn create_infixl_ast(op: String, priority: i8) -> StmtAST {
        StmtAST::InfixAST(InfixAST {
            op: op,
            priority: priority,
            ty: InfixType::Left,
        })
    }
    pub fn create_infixr_ast(op: String, priority: i8) -> StmtAST {
        StmtAST::InfixAST(InfixAST {
            op: op,
            priority: priority,
            ty: InfixType::Right,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprAST {
    OpTokenListAST(OpTokenListAST),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InfixAST {
    pub ty: InfixType,
    pub op: String,
    pub priority: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InfixType {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpTokenAST {
    NumAST(NumAST),
    Op(String),
}

impl OpTokenAST {
    pub fn create_num_ast(num: String) -> OpTokenAST {
        OpTokenAST::NumAST(NumAST {
            num: num.parse::<i32>().unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpTokenListAST {
    pub op_token_list: Vec<OpTokenAST>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumAST {
    pub num: i32,
}