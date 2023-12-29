mod binary_operations;

use crate::{
    expr::*,
    token::{ Literal, TokenType },
    error_handler::ViskumError,
    util::factorial,
    environment::environment_value::EnvironmentValue,
};

use super::Interpreter;

type Output = Literal;

impl<'a> Interpreter<'a> {
    // 0, null and false are false. Everything else is true
    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Bool(false) | Literal::Null => false,
            Literal::Num(x) => {
                if *x == 0.0 { false } else { true }
            }
            _ => true,
        }
    }
}

impl<'a> ExprVisitor<Output> for Interpreter<'a> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Output, ViskumError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::EqualEqual => {
                Ok(Literal::Bool(binary_operations::is_equal(&left, &right)))
            }
            TokenType::BangEqual => {
                Ok(Literal::Bool(!binary_operations::is_equal(&left, &right)))
            }
            TokenType::Greater => { Ok(binary_operations::greater(&left, &right)?) }
            TokenType::GreaterEqual => { Ok(binary_operations::greater_equal(&left, &right)?) }
            TokenType::Less => { Ok(binary_operations::less(&left, &right)?) }
            TokenType::LessEqual => { Ok(binary_operations::less_equal(&left, &right)?) }
            TokenType::Minus => { Ok(binary_operations::minus(&left, &right)?) }
            TokenType::Plus => { Ok(binary_operations::plus(&left, &right)?) }
            TokenType::Slash => { Ok(binary_operations::division(&left, &right)?) }
            TokenType::Star => { Ok(binary_operations::multiplication(&left, &right)?) }
            TokenType::Power => { Ok(binary_operations::exponential(&left, &right)?) }
            _ => { Ok(Literal::Null) }
        }
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Output, ViskumError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Output, ViskumError> {
        if let Some(literal) = &expr.value { Ok(literal.clone()) } else { Ok(Literal::Null) }
    }

    fn visit_prefix_expr(&self, expr: &PrefixExpr) -> Result<Output, ViskumError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus =>
                match right {
                    Literal::Num(x) => { Ok(Literal::Num(-x)) }
                    _ => {
                        Err(
                            ViskumError::new(
                                format!(
                                    "'{}' cannot negate a {}",
                                    expr.operator.lexeme,
                                    right.to_type_string()
                                ).as_str(),
                                expr.operator.clone(),
                                "file.vs"
                            )
                        )
                    }
                }
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right))),

            _ =>
                Err(
                    ViskumError::new(
                        format!("Invalid prefix: {}", expr.operator.lexeme).as_str(),
                        expr.operator.clone(),
                        "file.vs"
                    )
                ),
        }
    }

    fn visit_postfix_expr(&self, expr: &PostfixExpr) -> Result<Output, ViskumError> {
        let left = self.evaluate(&expr.left)?;

        match expr.operator.ttype {
            TokenType::Factorial => {
                match left {
                    Literal::Num(x) => Ok(Literal::Num(factorial(x))),
                    lit =>
                        Err(
                            ViskumError::new(
                                format!(
                                    "{} is not defined for {}",
                                    expr.operator.lexeme,
                                    lit.to_type_string()
                                ).as_str(),
                                expr.operator.clone(),
                                "file.vs"
                            )
                        ),
                }
            }
            _ =>
                Err(
                    ViskumError::new(
                        format!("Invalid postfix: {}", expr.operator.lexeme).as_str(),
                        expr.operator.clone(),
                        "file.vs"
                    )
                ),
        }
    }

    fn visit_ternary_expr(&self, expr: &TernaryExpr) -> Result<Output, ViskumError> {
        let condition = self.evaluate(&expr.condition)?;

        let is_condition_true = self.is_truthy(&condition);

        if is_condition_true {
            Ok(self.evaluate(&expr.true_expr)?)
        } else {
            Ok(self.evaluate(&expr.false_expr)?)
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Output, ViskumError> {
        Ok(self.environment_get(&expr.token)?)
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Output, ViskumError> {
        Ok(
            self.environment_assign(
                &expr.token,
                EnvironmentValue::new(self.evaluate(&expr.value)?, false)
            )?
        )
    }
}
