use super::super::ast;
use super::ir_tree as ir;
use super::super::error::Error;
use super::super::types::{Type, FuncType};
use std::collections::HashMap;

type ResultIr<T> = Result<T, Error>;

impl ast::ProgramAST {
    pub fn to_ir(self, ex_func_list: HashMap<String, ir::DecFuncIr>) -> ResultIr<(ir::ProgramIr)> {
        let mut func_list: HashMap<String, ir::FuncIr> = HashMap::new();
        for stmt in self.stmt_list {
            stmt.to_ir(&mut func_list)?;
        }
        Result::Ok(ir::ProgramIr {
            func_list,
            ex_func_list,
        })
    }
}

struct VariableTable(Vec<String>);

impl VariableTable {
    fn find_variable_id(&self, name: &String) -> Option<usize> {
        self.0.iter().rev().enumerate().find(|(_, name2)| {
            *name2 == name
        }).map(|(id, _)| id)
    }
}

impl ast::StmtAST {
    fn to_ir(self, func_list: &mut HashMap<String, ir::FuncIr>) -> ResultIr<Option<ir::FuncIr>> {
        let option = match self {
            ast::StmtAST::DefFuncAST(def_func_ast) => {
                let func_type = FuncType {
                    param_types: (0..def_func_ast.params.len()).map(|_| Type::Unknown).collect(),
                    ret_type: Type::Unknown,
                };
                let var_table = VariableTable(def_func_ast.params.into_iter().map(|x| x.id).collect());
                let body_ir = def_func_ast.body.to_ir(&var_table);
                let func_ir = ir::FuncIr {
                    name: def_func_ast.func_name,
                    body: body_ir?,
                    ty: func_type,
                    pos: def_func_ast.pos,
                };
                func_list.insert(func_ir.name.clone(), func_ir);
                Option::None
            }
            _ => Option::None
        };
        Result::Ok(option)
    }
}

impl ast::ExprAST {
    fn to_ir(self, var_table: &VariableTable) -> ResultIr<ir::ExprIr> {
        let ir =
            match self {
                ast::ExprAST::NumAST(x) => ir::ExprIr::create_numir(x.num),
                ast::ExprAST::OpAST(x) => {
                    let x = *x;
                    ir::ExprIr::create_opir(x.op, x.l_expr.to_ir(var_table)?, x.r_expr.to_ir(var_table)?)
                }
                ast::ExprAST::VariableAST(x) =>
                    match var_table.find_variable_id(&x.id) {
                        Some(id) => ir::ExprIr::create_variableir(id, x.pos),
                        _ => ir::ExprIr::create_global_variableir(x.id, x.pos)
                    },
                ast::ExprAST::ParenAST(x) => x.expr.to_ir(var_table)?,
                ast::ExprAST::FuncCallAST(x) => {
                    let x = *x;
                    let func = x.func.to_ir(var_table)?;
                    if x.params.len() == 0 {
                        func
                    } else {
                        let params: ResultIr<Vec<ir::ExprIr>> =
                            x.params.into_iter()
                                .map(|x| x.to_ir(var_table)).collect();
                        ir::ExprIr::create_callir(func, params?)
                    }
                }
            };
        Result::Ok(ir)
    }
}


#[test]
fn ast_to_ir_test() {
    use combine::stream::state::SourcePosition;
    let ast = ast::ProgramAST {
        stmt_list: vec![
            ast::StmtAST::DefFuncAST(
                ast::DefFuncAST {
                    func_name: "hoge".to_string(),
                    params: vec![
                        ast::VariableAST::new("a".to_string(), SourcePosition { column: 0, line: 0 }),
                        ast::VariableAST::new("b".to_string(), SourcePosition { column: 0, line: 0 })
                    ],
                    body: ast::ExprAST::create_variable_ast("b".to_string(), SourcePosition { column: 0, line: 0 }),
                    pos: SourcePosition { column: 0, line: 0 },
                }
            )
        ]
    };
    let mut func_list = HashMap::new();
    func_list.insert(
        "hoge".to_string(),
        ir::FuncIr {
            name: "hoge".to_string(),
            body: ir::ExprIr::create_variableir(0, SourcePosition { line: 0, column: 0 }),
            ty: FuncType { ret_type: Type::Unknown, param_types: vec![Type::Unknown, Type::Unknown] },
            pos: SourcePosition { column: 0, line: 0 },
        },
    );
    let ir = ir::ProgramIr { func_list, ex_func_list: HashMap::new() };
    assert_eq!(ast.to_ir(HashMap::new()).unwrap(), ir);
}