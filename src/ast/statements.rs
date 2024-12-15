use super::expressions::{Expression, Pattern};
use super::{Block, Identifier, Span, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Empty,
    Expression(Expression),
    Let(LetStatement),
    Return(ReturnStatement),
    Break(BreakStatement),
    Continue(ContinueStatement),
    While(WhileStatement),
    For(ForStatement),
    Loop(LoopStatement),
    Block(Block),
    If(IfStatement),
    Match(MatchStatement),
    Panic(PanicStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub pattern: Pattern,
    pub type_annotation: Option<Type>,
    pub initializer: Option<Expression>,
    pub mutable: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakStatement {
    pub label: Option<Identifier>,
    pub expression: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueStatement {
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub pattern: Pattern,
    pub iterator: Expression,
    pub body: Block,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopStatement {
    pub body: Block,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Block,
    pub else_branch: Option<ElseBranch>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseBranch {
    Block(Block),
    If(Box<IfStatement>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchStatement {
    pub expression: Expression,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PanicStatement {
    pub message: Expression,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expressions::{Expression, Literal};

    fn dummy_span() -> Span {
        Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }

    #[test]
    fn test_let_statement() {
        let stmt = Statement::Let(LetStatement {
            pattern: Pattern::Identifier(Identifier::new("x".to_string(), dummy_span())),
            type_annotation: None,
            initializer: Some(Expression::Literal(Literal::Integer(42, None))),
            mutable: false,
            span: dummy_span(),
        });

        if let Statement::Let(let_stmt) = stmt {
            assert!(!let_stmt.mutable);
            assert!(matches!(let_stmt.pattern, Pattern::Identifier(_)));
            assert!(let_stmt.type_annotation.is_none());
            assert!(matches!(
                let_stmt.initializer,
                Some(Expression::Literal(Literal::Integer(42, None)))
            ));
        } else {
            panic!("Expected let statement");
        }
    }

    #[test]
    fn test_return_statement() {
        let stmt = Statement::Return(ReturnStatement {
            expression: Some(Expression::Literal(Literal::Integer(42, None))),
            span: dummy_span(),
        });

        if let Statement::Return(return_stmt) = stmt {
            assert!(matches!(
                return_stmt.expression,
                Some(Expression::Literal(Literal::Integer(42, None)))
            ));
        } else {
            panic!("Expected return statement");
        }
    }

    #[test]
    fn test_while_statement() {
        let stmt = Statement::While(WhileStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            body: Block {
                statements: vec![],
                span: dummy_span(),
            },
            label: Some(Identifier::new("loop1".to_string(), dummy_span())),
            span: dummy_span(),
        });

        if let Statement::While(while_stmt) = stmt {
            assert!(matches!(
                while_stmt.condition,
                Expression::Literal(Literal::Boolean(true))
            ));
            assert!(while_stmt.label.is_some());
            assert_eq!(while_stmt.label.unwrap().name, "loop1");
        } else {
            panic!("Expected while statement");
        }
    }

    #[test]
    fn test_if_statement() {
        let stmt = Statement::If(IfStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            then_branch: Block {
                statements: vec![],
                span: dummy_span(),
            },
            else_branch: Some(ElseBranch::Block(Block {
                statements: vec![],
                span: dummy_span(),
            })),
            span: dummy_span(),
        });

        if let Statement::If(if_stmt) = stmt {
            assert!(matches!(
                if_stmt.condition,
                Expression::Literal(Literal::Boolean(true))
            ));
            assert!(if_stmt.else_branch.is_some());
        } else {
            panic!("Expected if statement");
        }
    }

    #[test]
    fn test_match_statement() {
        let stmt = Statement::Match(MatchStatement {
            expression: Expression::Literal(Literal::Integer(1, None)),
            arms: vec![MatchArm {
                pattern: Pattern::Literal(Literal::Integer(1, None)),
                guard: None,
                body: Block {
                    statements: vec![],
                    span: dummy_span(),
                },
                span: dummy_span(),
            }],
            span: dummy_span(),
        });

        if let Statement::Match(match_stmt) = stmt {
            assert!(matches!(
                match_stmt.expression,
                Expression::Literal(Literal::Integer(1, None))
            ));
            assert_eq!(match_stmt.arms.len(), 1);
            assert!(matches!(
                match_stmt.arms[0].pattern,
                Pattern::Literal(Literal::Integer(1, None))
            ));
        } else {
            panic!("Expected match statement");
        }
    }

    #[test]
    fn test_panic_statement() {
        let stmt = Statement::Panic(PanicStatement {
            message: Expression::Literal(Literal::String("Error!".to_string())),
            span: dummy_span(),
        });

        if let Statement::Panic(panic_stmt) = stmt {
            assert!(matches!(
                panic_stmt.message,
                Expression::Literal(Literal::String(_))
            ));
        } else {
            panic!("Expected panic statement");
        }
    }
}
