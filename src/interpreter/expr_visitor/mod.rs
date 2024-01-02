mod binary_operations;
use crate::viskum_callable::ViskumCallable;

use crate::{
    expr::*,
    token::{ Literal, TokenType },
    error_handler::ViskumError,
    util::factorial,
    environment::environment_value::EnvironmentValue,
};

use super::Interpreter;

type Output = Literal;

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
                    Literal::Num(x) => {
                        return Ok(Literal::Num(factorial(x)));
                    }
                    lit => {
                        return Err(
                            ViskumError::new(
                                format!(
                                    "{} is not defined for {}",
                                    expr.operator.lexeme,
                                    lit.to_type_string()
                                ).as_str(),
                                expr.operator.clone(),
                                "file.vs"
                            )
                        );
                    }
                }
            }
            _ => {
                return Err(
                    ViskumError::new(
                        format!("Invalid postfix: {}", expr.operator.lexeme).as_str(),
                        expr.operator.clone(),
                        "file.vs"
                    )
                );
            }
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
        match expr.assignment_token.ttype {
            | TokenType::Equal
            | TokenType::PlusEqual
            | TokenType::MinusEqual
            | TokenType::StarEqual
            | TokenType::SlashEqual
            | TokenType::PowerEqual => {
                let left = self.environment_get(&expr.token)?;

                let right = self.evaluate(&expr.value)?;

                let new_value = match expr.assignment_token.ttype {
                    TokenType::Equal => right,
                    TokenType::PlusEqual => binary_operations::plus(&left, &right)?,
                    TokenType::MinusEqual => binary_operations::minus(&left, &right)?,
                    TokenType::StarEqual => binary_operations::multiplication(&left, &right)?,
                    TokenType::SlashEqual => binary_operations::division(&left, &right)?,
                    TokenType::PowerEqual => binary_operations::exponential(&left, &right)?,
                    _ => {
                        return Err(
                            ViskumError::new(
                                format!(
                                    "{} is not defined for {}",
                                    expr.assignment_token.lexeme,
                                    left.to_type_string()
                                ).as_str(),
                                expr.assignment_token.clone(),
                                "file.vs"
                            )
                        );
                    }
                };

                return Ok(
                    self.environment_assign(&expr.token, EnvironmentValue::new(new_value, false))?
                );
            }
            TokenType::Increment | TokenType::Decrement => {
                let adjustment = match expr.assignment_token.ttype {
                    TokenType::Increment => 1.0,
                    TokenType::Decrement => -1.0,
                    _ => 0.0,
                };

                let variable_value = self.environment_get(&expr.token)?;
                let variable_number = variable_value.to_num();

                match variable_number {
                    Ok(x) => {
                        return Ok(
                            self.environment_assign(
                                &expr.token,
                                EnvironmentValue::new(Literal::Num(x + adjustment), false)
                            )?
                        );
                    }
                    Err(_) => {
                        return Err(
                            ViskumError::new(
                                format!(
                                    "{} is not defined for {}",
                                    expr.assignment_token.lexeme,
                                    variable_value.to_type_string()
                                ).as_str(),
                                expr.assignment_token.clone(),
                                "file.vs"
                            )
                        );
                    }
                }
            }
            _ => {
                return Err(
                    ViskumError::new(
                        format!("Invalid assignment: {}", expr.assignment_token.lexeme).as_str(),
                        expr.assignment_token.clone(),
                        "file.vs"
                    )
                );
            }
        }
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Output, ViskumError> {
        let lhs_evaluated = self.evaluate(&expr.left)?;

        match expr.operator.ttype {
            TokenType::Or => {
                if self.is_truthy(&lhs_evaluated) {
                    Ok(lhs_evaluated)
                } else {
                    Ok(self.evaluate(&expr.right)?)
                }
            }
            TokenType::And => {
                if self.is_truthy(&lhs_evaluated) {
                    Ok(self.evaluate(&expr.right)?)
                } else {
                    Ok(lhs_evaluated)
                }
            }
            _ => {
                return Err(
                    ViskumError::new(
                        format!("Invalid logical operator: {}", expr.operator.lexeme).as_str(),
                        expr.operator.clone(),
                        "file.vs"
                    )
                );
            }
        }
    }

    fn visit_call_expr(&self, expr: &CallExpr) -> Result<Output, ViskumError> {
        let callee = self.evaluate(&expr.callee)?;

        let mut arguments = Vec::new();

        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        if let Literal::Func(func) = callee {
            if arguments.len() != func.arity {
                return Err(
                    ViskumError::new(
                        format!(
                            "Expected {} arguments but received {}",
                            func.arity,
                            arguments.len()
                        ).as_str(),
                        expr.paren.clone(),
                        "file.vs"
                    )
                );
            }

            return Ok(func.call(self, &arguments)?);
        } else {
            return Err(
                ViskumError::new(
                    format!("A {} is not callable", callee.to_type_string()).as_str(),
                    expr.paren.clone(),
                    "file.vs"
                )
            );
        }
    }
}
