use std::ops::Mul;

use super::Parser;
use crate::lexer::SyntaxKind;

pub(super) fn expr(p: &mut Parser) {
    expr_with_binding_power(p, 0);
}

fn expr_with_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Identifier) => p.bump(),
        _ => {}
    }

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => Op::Add,
            Some(SyntaxKind::Minus) => Op::Sub,
            Some(SyntaxKind::Asterisk) => Op::Mul,
            Some(SyntaxKind::Slash) => Op::Div,
            _ => return, // we’ll handle errors later.
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < min_binding_power {
            return;
        }

        p.bump();

        p.start_node_at(checkpoint, SyntaxKind::BinaryOp);
        expr_with_binding_power(p, right_binding_power);
        p.finish_node();
    }
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Identifier@0..7 "counter""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  BinaryOp@0..3
    Number@0..1 "1"
    Plus@1..2 "+"
    Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_binary_expression() {
        check(
            "1+2+3+4",
            expect![[r#"
Root@0..7
  BinaryOp@0..7
    BinaryOp@0..5
      BinaryOp@0..3
        Number@0..1 "1"
        Plus@1..2 "+"
        Number@2..3 "2"
      Plus@3..4 "+"
      Number@4..5 "3"
    Plus@5..6 "+"
    Number@6..7 "4""#]],
        );
    }

    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  BinaryOp@0..7
    BinaryOp@0..5
      Number@0..1 "1"
      Plus@1..2 "+"
      BinaryOp@2..5
        Number@2..3 "2"
        Asterisk@3..4 "*"
        Number@4..5 "3"
    Minus@5..6 "-"
    Number@6..7 "4""#]],
        );
    }
}