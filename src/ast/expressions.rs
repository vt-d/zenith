use super::{Identifier, Span, Type};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Call(Box<CallExpr>),
    Member(Box<MemberExpr>),
    Index(Box<IndexExpr>),
    Cast(Box<CastExpr>),
    Block(Box<BlockExpr>),
    If(Box<IfExpr>),
    Match(Box<MatchExpr>),
    Loop(Box<LoopExpr>),
    While(Box<WhileExpr>),
    For(Box<ForExpr>),
    Range(Box<RangeExpr>),
    MacroInvocation(Box<MacroInvocation>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i128, Option<Type>),
    Float(f64, Option<Type>),
    String(String),
    Character(char),
    Boolean(bool),
    Array(Vec<Expression>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(n, ty) => {
                if let Some(t) = ty {
                    write!(f, "{}{:?}", n, t)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::Float(n, ty) => {
                if let Some(t) = ty {
                    write!(f, "{}{:?}", n, t)
                } else {
                    write!(f, "{}", n)
                }
            }
            Literal::String(s) => write!(f, "\"{}\"", s),
            Literal::Character(c) => write!(f, "'{}'", c),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Array(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", elem)?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    ShlAssign,
    ShrAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Neg,
    Not,
    BitNot,
    Deref,
    Ref,
    RefMut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpr {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpr {
    pub object: Expression,
    pub member: Identifier,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpr {
    pub array: Expression,
    pub index: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CastExpr {
    pub expr: Expression,
    pub target_type: Type,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpr {
    pub statements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpr {
    pub condition: Expression,
    pub then_branch: Expression,
    pub else_branch: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpr {
    pub value: Expression,
    pub arms: Vec<MatchArm>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Literal(Literal),
    Identifier(Identifier),
    Tuple(Vec<Pattern>),
    Struct(Identifier, Vec<(Identifier, Pattern)>),
    Or(Vec<Pattern>),
    Range(Box<Pattern>, Box<Pattern>),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpr {
    pub body: Expression,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileExpr {
    pub condition: Expression,
    pub body: Expression,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForExpr {
    pub pattern: Pattern,
    pub iterator: Expression,
    pub body: Expression,
    pub label: Option<Identifier>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeExpr {
    pub start: Option<Expression>,
    pub end: Option<Expression>,
    pub inclusive: bool,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroInvocation {
    pub name: Identifier,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use std::f64::consts;

    use super::*;
    use crate::ast::Span;

    #[test]
    fn test_literal_expressions() {
        let integer = Expression::Literal(Literal::Integer(42, None));
        let float = Expression::Literal(Literal::Float(consts::PI, None));
        let string = Expression::Literal(Literal::String("hello".to_string()));
        let character = Expression::Literal(Literal::Character('a'));
        let boolean = Expression::Literal(Literal::Boolean(true));

        assert!(matches!(
            integer,
            Expression::Literal(Literal::Integer(42, None))
        ));
        assert!(matches!(
            float,
            Expression::Literal(Literal::Float(consts::PI, None))
        ));
        assert!(matches!(string, Expression::Literal(Literal::String(_))));
        assert!(matches!(
            character,
            Expression::Literal(Literal::Character('a'))
        ));
        assert!(matches!(
            boolean,
            Expression::Literal(Literal::Boolean(true))
        ));
    }

    #[test]
    fn test_binary_expression() {
        let span = Span::dummy();
        let left = Expression::Literal(Literal::Integer(1, None));
        let right = Expression::Literal(Literal::Integer(2, None));

        let binary = Expression::Binary(Box::new(BinaryExpr {
            left,
            operator: BinaryOperator::Add,
            right,
            span,
        }));

        if let Expression::Binary(expr) = binary {
            assert!(matches!(expr.operator, BinaryOperator::Add));
            assert!(matches!(
                expr.left,
                Expression::Literal(Literal::Integer(1, None))
            ));
            assert!(matches!(
                expr.right,
                Expression::Literal(Literal::Integer(2, None))
            ));
        } else {
            panic!("Expected binary expression");
        }
    }

    #[test]
    fn test_call_expression() {
        let span = Span::dummy();
        let callee = Expression::Identifier(Identifier::new("foo".to_string(), span.clone()));
        let arg = Expression::Literal(Literal::Integer(42, None));

        let call = Expression::Call(Box::new(CallExpr {
            callee,
            arguments: vec![arg],
            span,
        }));

        if let Expression::Call(expr) = call {
            assert!(matches!(expr.callee, Expression::Identifier(_)));
            assert_eq!(expr.arguments.len(), 1);
        } else {
            panic!("Expected call expression");
        }
    }

    #[test]
    fn test_if_expression() {
        let span = Span::dummy();
        let condition = Expression::Literal(Literal::Boolean(true));
        let then_branch = Expression::Literal(Literal::Integer(1, None));
        let else_branch = Some(Expression::Literal(Literal::Integer(2, None)));

        let if_expr = Expression::If(Box::new(IfExpr {
            condition,
            then_branch,
            else_branch,
            span,
        }));

        if let Expression::If(expr) = if_expr {
            assert!(matches!(
                expr.condition,
                Expression::Literal(Literal::Boolean(true))
            ));
            assert!(matches!(
                expr.then_branch,
                Expression::Literal(Literal::Integer(1, None))
            ));
            assert!(matches!(
                expr.else_branch,
                Some(Expression::Literal(Literal::Integer(2, None)))
            ));
        } else {
            panic!("Expected if expression");
        }
    }

    #[test]
    fn test_match_expression() {
        let span = Span::dummy();
        let value = Expression::Identifier(Identifier::new("x".to_string(), span.clone()));
        let pattern = Pattern::Literal(Literal::Integer(1, None));
        let body = Expression::Literal(Literal::String("one".to_string()));

        let match_expr = Expression::Match(Box::new(MatchExpr {
            value,
            arms: vec![MatchArm {
                pattern,
                guard: None,
                body,
                span: span.clone(),
            }],
            span,
        }));

        if let Expression::Match(expr) = match_expr {
            assert!(matches!(expr.value, Expression::Identifier(_)));
            assert_eq!(expr.arms.len(), 1);
            assert!(matches!(
                expr.arms[0].pattern,
                Pattern::Literal(Literal::Integer(1, None))
            ));
        } else {
            panic!("Expected match expression");
        }
    }
}
