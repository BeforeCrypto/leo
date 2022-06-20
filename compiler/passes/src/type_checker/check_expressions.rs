// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::*;
use leo_errors::TypeCheckerError;

use crate::TypeChecker;

use super::director::Director;

impl<'a> ExpressionVisitor<'a> for TypeChecker<'a> {}

fn return_incorrect_type(t1: Option<Type>, t2: Option<Type>, expected: &Option<Type>) -> Option<Type> {
    match (t1, t2) {
        (Some(t1), Some(t2)) if t1 == t2 => Some(t1),
        (Some(t1), Some(t2)) => {
            if let Some(expected) = expected {
                if &t1 != expected {
                    Some(t1)
                } else {
                    Some(t2)
                }
            } else {
                Some(t1)
            }
        }
        (None, Some(_)) | (Some(_), None) | (None, None) => None,
    }
}

impl<'a> ExpressionVisitorDirector<'a> for Director<'a> {
    type AdditionalInput = Option<Type>;
    type Output = Type;

    fn visit_expression(&mut self, input: &'a Expression, expected: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_expression(input) {
            return match input {
                Expression::Identifier(expr) => self.visit_identifier(expr, expected),
                Expression::Value(expr) => self.visit_value(expr, expected),
                Expression::Binary(expr) => self.visit_binary(expr, expected),
                Expression::Unary(expr) => self.visit_unary(expr, expected),
                Expression::Ternary(expr) => self.visit_ternary(expr, expected),
                Expression::Call(expr) => self.visit_call(expr, expected),
                Expression::Err(expr) => self.visit_err(expr, expected),
            };
        }

        None
    }

    fn visit_identifier(&mut self, input: &'a Identifier, expected: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_identifier(input) {
            return if let Some(var) = self.visitor.symbol_table.clone().lookup_variable(&input.name) {
                Some(self.visitor.assert_expected_option(*var.type_, expected, input.span))
            } else {
                self.visitor
                    .handler
                    .emit_err(TypeCheckerError::unknown_sym("variable", input.name, input.span()).into());
                None
            };
        }

        None
    }

    fn visit_value(&mut self, input: &'a ValueExpression, expected: &Self::AdditionalInput) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_value(input) {
            return Some(match input {
                ValueExpression::Address(_, _) => {
                    self.visitor
                        .assert_expected_option(Type::Address, expected, input.span())
                }
                ValueExpression::Boolean(_, _) => {
                    self.visitor
                        .assert_expected_option(Type::Boolean, expected, input.span())
                }
                ValueExpression::Field(_, _) => {
                    self.visitor.assert_expected_option(Type::Field, expected, input.span())
                }
                ValueExpression::Integer(type_, str_content, _) => {
                    match type_ {
                        IntegerType::I8 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i8>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i8", input.span()).into());
                            }
                        }
                        IntegerType::I16 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i16>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i16", input.span()).into());
                            }
                        }
                        IntegerType::I32 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i32>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i32", input.span()).into());
                            }
                        }
                        IntegerType::I64 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i64>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i64", input.span()).into());
                            }
                        }
                        IntegerType::I128 => {
                            let int = if self.visitor.negate {
                                format!("-{str_content}")
                            } else {
                                str_content.clone()
                            };

                            if int.parse::<i128>().is_err() {
                                self.visitor
                                    .handler
                                    .emit_err(TypeCheckerError::invalid_int_value(int, "i128", input.span()).into());
                            }
                        }
                        IntegerType::U8 if str_content.parse::<u8>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u8", input.span()).into()),
                        IntegerType::U16 if str_content.parse::<u16>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u16", input.span()).into()),
                        IntegerType::U32 if str_content.parse::<u32>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u32", input.span()).into()),
                        IntegerType::U64 if str_content.parse::<u64>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u64", input.span()).into()),
                        IntegerType::U128 if str_content.parse::<u128>().is_err() => self
                            .visitor
                            .handler
                            .emit_err(TypeCheckerError::invalid_int_value(str_content, "u128", input.span()).into()),
                        _ => {}
                    }
                    self.visitor
                        .assert_expected_option(Type::IntegerType(*type_), expected, input.span())
                }
                ValueExpression::Group(_) => self.visitor.assert_expected_option(Type::Group, expected, input.span()),
                ValueExpression::Scalar(_, _) => {
                    self.visitor
                        .assert_expected_option(Type::Scalar, expected, input.span())
                }
                ValueExpression::String(_, _) => {
                    self.visitor
                        .assert_expected_option(Type::String, expected, input.span())
                }
            });
        }

        None
    }

    fn visit_binary(
        &mut self,
        input: &'a BinaryExpression,
        destination: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_binary(input) {
            return match input.op {
                BinaryOperation::And | BinaryOperation::Or | BinaryOperation::Nand | BinaryOperation::Nor => {
                    // Assert equal boolean types.
                    self.visitor
                        .assert_expected_option(Type::Boolean, destination, input.span());
                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::BitwiseAnd | BinaryOperation::BitwiseOr | BinaryOperation::Xor => {
                    // Assert equal boolean or integer types.
                    self.visitor.assert_bool_int_type(destination, input.span());
                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::Add => {
                    // Assert equal field, group, scalar, or integer types.
                    self.visitor
                        .assert_field_group_scalar_int_type(destination, input.span());
                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::Sub => {
                    // Assert equal field, group, or integer types.
                    self.visitor.assert_field_group_int_type(destination, input.span());
                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::Mul => {
                    // Assert field, group or integer types.
                    self.visitor.assert_field_group_int_type(destination, input.span());

                    let t1 = self.visit_expression(&input.left, &None);
                    let t2 = self.visit_expression(&input.right, &None);

                    // Allow `group` * `scalar` multiplication.
                    match (t1, t2) {
                        (Some(Type::Group), other) => {
                            self.visitor
                                .assert_expected_type(&other, Type::Scalar, input.right.span());
                            Some(
                                self.visitor
                                    .assert_expected_type(destination, Type::Group, input.span()),
                            )
                        }
                        (other, Some(Type::Group)) => {
                            self.visitor
                                .assert_expected_type(&other, Type::Scalar, input.left.span());
                            Some(
                                self.visitor
                                    .assert_expected_type(destination, Type::Group, input.span()),
                            )
                        }
                        (t1, t2) => {
                            // Assert equal field or integer types.
                            self.visitor.assert_field_int_type(destination, input.span());

                            return_incorrect_type(t1, t2, destination)
                        }
                    }
                }
                BinaryOperation::Div => {
                    // Assert equal field or integer types.
                    self.visitor.assert_field_int_type(destination, input.span());

                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::Pow => {
                    // Assert field or integer types.
                    self.visitor.assert_field_int_type(destination, input.span());

                    let t1 = self.visit_expression(&input.left, &None);
                    let t2 = self.visit_expression(&input.right, &None);

                    // Allow field * field.
                    match (t1, t2) {
                        (Some(Type::Field), type_) => {
                            self.visitor
                                .assert_expected_type(&type_, Type::Field, input.right.span());
                            Some(
                                self.visitor
                                    .assert_expected_type(destination, Type::Field, input.span()),
                            )
                        }
                        (type_, Some(Type::Field)) => {
                            self.visitor
                                .assert_expected_type(&type_, Type::Field, input.left.span());
                            Some(
                                self.visitor
                                    .assert_expected_type(destination, Type::Field, input.span()),
                            )
                        }
                        (Some(t1), t2) => {
                            // Allow integer t2 magnitude (u8, u16, u32)
                            self.visitor.assert_magnitude_type(&t2, input.right.span());
                            Some(self.visitor.assert_expected_type(destination, t1, input.span()))
                        }
                        (None, t2) => {
                            // Allow integer t2 magnitude (u8, u16, u32)
                            self.visitor.assert_magnitude_type(&t2, input.right.span());
                            *destination
                        }
                    }
                }
                BinaryOperation::Eq | BinaryOperation::Neq => {
                    // Assert first and second address, boolean, field, group, scalar, or integer types.
                    let t1 = self.visit_expression(&input.left, &None);
                    let t2 = self.visit_expression(&input.right, &None);

                    match (t1, t2) {
                        (Some(Type::IntegerType(_)), t2) => {
                            // Assert rhs is integer.
                            self.visitor.assert_int_type(&t2, input.left.span());
                        }
                        (t1, Some(Type::IntegerType(_))) => {
                            // Assert lhs is integer.
                            self.visitor.assert_int_type(&t1, input.right.span());
                        }
                        (t1, t2) => {
                            self.visitor.assert_eq_types(t1, t2, input.span());
                        }
                    }

                    // Assert destination is boolean.
                    Some(
                        self.visitor
                            .assert_expected_type(destination, Type::Boolean, input.span()),
                    )
                }
                BinaryOperation::Lt | BinaryOperation::Gt | BinaryOperation::Le | BinaryOperation::Ge => {
                    // Assert left and right are equal field, scalar, or integer types.
                    let t1 = self.visit_expression(&input.left, &None);
                    let t2 = self.visit_expression(&input.right, &None);

                    match (t1, t2) {
                        (Some(Type::Field), t2) => {
                            // Assert rhs is field.
                            self.visitor.assert_expected_type(&t2, Type::Field, input.left.span());
                        }
                        (t1, Some(Type::Field)) => {
                            // Assert lhs is field.
                            self.visitor.assert_expected_type(&t1, Type::Field, input.right.span());
                        }
                        (Some(Type::Scalar), t2) => {
                            // Assert rhs is scalar.
                            self.visitor.assert_expected_type(&t2, Type::Scalar, input.left.span());
                        }
                        (t1, Some(Type::Scalar)) => {
                            // Assert lhs is scalar.
                            self.visitor.assert_expected_type(&t1, Type::Scalar, input.right.span());
                        }
                        (Some(Type::IntegerType(_)), t2) => {
                            // Assert rhs is integer.
                            self.visitor.assert_int_type(&t2, input.left.span());
                        }
                        (t1, Some(Type::IntegerType(_))) => {
                            // Assert lhs is integer.
                            self.visitor.assert_int_type(&t1, input.right.span());
                        }
                        (_, _) => {
                            // Not enough info to assert type.
                        }
                    }

                    // Assert destination is boolean.
                    Some(
                        self.visitor
                            .assert_expected_type(destination, Type::Boolean, input.span()),
                    )
                }
                BinaryOperation::AddWrapped
                | BinaryOperation::SubWrapped
                | BinaryOperation::DivWrapped
                | BinaryOperation::MulWrapped => {
                    // Assert equal integer types.
                    self.visitor.assert_int_type(destination, input.span);
                    let t1 = self.visit_expression(&input.left, destination);
                    let t2 = self.visit_expression(&input.right, destination);

                    return_incorrect_type(t1, t2, destination)
                }
                BinaryOperation::Shl
                | BinaryOperation::ShlWrapped
                | BinaryOperation::Shr
                | BinaryOperation::ShrWrapped
                | BinaryOperation::PowWrapped => {
                    // Assert left and destination are equal integer types.
                    self.visitor.assert_int_type(destination, input.span);
                    let t1 = self.visit_expression(&input.left, destination);

                    // Assert right type is a magnitude (u8, u16, u32).
                    let t2 = self.visit_expression(&input.right, &None);
                    self.visitor.assert_magnitude_type(&t2, input.right.span());

                    return_incorrect_type(t1, t2, destination)
                }
            };
        }

        None
    }

    fn visit_unary(&mut self, input: &'a UnaryExpression, destination: &Self::AdditionalInput) -> Option<Self::Output> {
        match input.op {
            UnaryOperation::Abs => {
                // Assert integer type only.
                self.visitor.assert_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::AbsWrapped => {
                // Assert integer type only.
                self.visitor.assert_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Double => {
                // Assert field and group type only.
                self.visitor.assert_field_group_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Inverse => {
                // Assert field type only.
                self.visitor
                    .assert_expected_type(destination, Type::Field, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Negate => {
                let prior_negate_state = self.visitor.negate;
                self.visitor.negate = true;

                let type_ = self.visit_expression(&input.receiver, destination);
                self.visitor.negate = prior_negate_state;
                match type_.as_ref() {
                    Some(
                        Type::IntegerType(
                            IntegerType::I8
                            | IntegerType::I16
                            | IntegerType::I32
                            | IntegerType::I64
                            | IntegerType::I128,
                        )
                        | Type::Field
                        | Type::Group,
                    ) => {}
                    Some(t) => self
                        .visitor
                        .handler
                        .emit_err(TypeCheckerError::type_is_not_negatable(t, input.receiver.span()).into()),
                    _ => {}
                };
                type_
            }
            UnaryOperation::Not => {
                // Assert boolean, integer types only.
                self.visitor.assert_bool_int_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::Square => {
                // Assert field type only.
                self.visitor
                    .assert_expected_type(destination, Type::Field, input.span());
                self.visit_expression(&input.receiver, destination)
            }
            UnaryOperation::SquareRoot => {
                // Assert field and scalar types only.
                self.visitor.assert_field_scalar_type(destination, input.span());
                self.visit_expression(&input.receiver, destination)
            }
        }
    }

    fn visit_ternary(
        &mut self,
        input: &'a TernaryExpression,
        expected: &Self::AdditionalInput,
    ) -> Option<Self::Output> {
        if let VisitResult::VisitChildren = self.visitor.visit_ternary(input) {
            self.visit_expression(&input.condition, &Some(Type::Boolean));

            let t1 = self.visit_expression(&input.if_true, expected);
            let t2 = self.visit_expression(&input.if_false, expected);

            return return_incorrect_type(t1, t2, expected);
        }

        None
    }

    fn visit_call(&mut self, input: &'a CallExpression, expected: &Self::AdditionalInput) -> Option<Self::Output> {
        match &*input.function {
            Expression::Identifier(ident) => {
                if let Some(func) = self.visitor.symbol_table.clone().lookup_fn(&ident.name) {
                    let ret = self.visitor.assert_expected_option(func.output, expected, func.span());

                    if func.input.len() != input.arguments.len() {
                        self.visitor.handler.emit_err(
                            TypeCheckerError::incorrect_num_args_to_call(
                                func.input.len(),
                                input.arguments.len(),
                                input.span(),
                            )
                            .into(),
                        );
                    }

                    func.input
                        .iter()
                        .zip(input.arguments.iter())
                        .for_each(|(expected, argument)| {
                            self.visit_expression(argument, &Some(expected.get_variable().type_));
                        });

                    Some(ret)
                } else {
                    self.visitor
                        .handler
                        .emit_err(TypeCheckerError::unknown_sym("function", &ident.name, ident.span()).into());
                    None
                }
            }
            expr => self.visit_expression(expr, expected),
        }
    }
}