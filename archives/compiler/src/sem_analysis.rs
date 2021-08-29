use super::ast::{ExprAst, StmtAst};
use super::ir::{Ir, IrInst};
use super::location::Locatable;
use super::term_color::red_bold;
use super::token::Identifier;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// セマンティックエラーのメッセージのプレフィックス
static SEMANTIC_ERROR: Lazy<String> = Lazy::new(|| red_bold("Semantic error:"));

/// 中間表現を生成する際に扱う状態
#[derive(Clone, Default)]
struct Context {
    var_mappings: HashMap<String, i32>,
}

/// 抽象構文木を意味解析して、中間表現を生成する
pub fn sem_analysis(stmts: &[StmtAst]) -> Result<Ir, String> {
    let mut context = Context::default();
    let mut ir = Ir::default();

    for stmt in stmts.iter() {
        match stmt {
            StmtAst::ProcCall(proc, args) => {
                analyze_proc_call(proc, args, &mut ir, &mut context)?;
            }
            StmtAst::VarDecl(var_ident, init_expr) => {
                analyze_expr(init_expr, &mut ir, &mut context)?;
                let var_index = ir.num_globals;
                context
                    .var_mappings
                    .insert(var_ident.name.clone(), var_index);
                ir.insts.push(IrInst::SetGlobal(var_index));
                ir.num_globals += 1;
            }
            StmtAst::VarAssign(var_ident, expr) => {
                let current_context = context.clone();
                if let Some(var_index) = current_context.var_mappings.get(&var_ident.name) {
                    analyze_expr(expr, &mut ir, &mut context)?;
                    ir.insts.push(IrInst::SetGlobal(*var_index));
                } else {
                    return Err(format!(
                        "{} ({}) `{}` is not declared",
                        SEMANTIC_ERROR.as_str(),
                        var_ident.locate(),
                        var_ident.name
                    ));
                }
            }
        }
    }

    Ok(ir)
}

fn analyze_proc_call(
    proc: &Identifier,
    args: &[ExprAst],
    ir: &mut Ir,
    context: &mut Context,
) -> Result<(), String> {
    for arg in args {
        analyze_expr(arg, ir, context)?;
    }

    if proc.name == "PRINT" {
        let num_args = args.len();

        if num_args > 1 {
            return Err(format!(
                "{} ({}-{}) Expected 1 argument, found {}",
                SEMANTIC_ERROR.as_str(),
                args[0].locate().start,
                args[num_args - 1].locate().end,
                num_args
            ));
        } else if num_args == 0 {
            return Err(format!(
                "{} ({}) Expected 1 argument, found 0",
                SEMANTIC_ERROR.as_str(),
                proc.locate()
            ));
        }

        ir.insts.push(IrInst::Print);

        Ok(())
    } else {
        Err(format!(
            "{} ({}) `{}` is not defined",
            SEMANTIC_ERROR.as_str(),
            proc.locate(),
            proc.name
        ))
    }
}

fn analyze_expr(expr_ast: &ExprAst, ir: &mut Ir, context: &mut Context) -> Result<(), String> {
    match expr_ast {
        ExprAst::Ident(ident) => {
            if let Some(var_index) = context.var_mappings.get(&ident.name) {
                ir.insts.push(IrInst::GetGlobal(*var_index));
            } else {
                return Err(format!(
                    "{} ({}) `{}` is not defined",
                    SEMANTIC_ERROR.as_str(),
                    ident.locate(),
                    ident.name
                ));
            }
        }
        ExprAst::StrLit(str_lit) => {
            ir.string_pool.push(str_lit.value.clone());
            ir.insts
                .push(IrInst::GetStaticStr(ir.string_pool.len() as i32 - 1));
        }
    }

    Ok(())
}
