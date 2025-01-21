use crate::expr::Expr;

fn eval(expr: &Expr) -> Result<Expr, String> {
    match expr {
        Expr::Number(_) | Expr::Float(_) => Ok(expr.clone()),

        Expr::List(list) => {
            if list.is_empty() {
                return Err("Empty list".to_string());
            }

            let first = &list[0];

            match first {
                Expr::Symbol(op) if op == "+" || op == "-" || op == "*" || op == "/" => {
                    let mut iter = list.iter().skip(1).map(eval);
                    let first = iter.next().ok_or("Expected at least one argument")??;

                    iter.try_fold(first, |acc, x| {
                        let acc = match acc {
                            Expr::Number(n) => n as f64,
                            Expr::Float(f) => f,
                            _ => return Err("Expected a number".to_string()),
                        };

                        let x = x?;
                        let x = match x {
                            Expr::Number(n) => n as f64,
                            Expr::Float(f) => f,
                            _ => return Err("Expected a number".to_string()),
                        };

                        Ok(match op.as_str() {
                            "+" => Expr::Float(acc + x),
                            "-" => Expr::Float(acc - x),
                            "*" => Expr::Float(acc * x),
                            "/" => {
                                if x == 0.0 {
                                    return Err("Division by zero".to_string());
                                }
                                Expr::Float(acc / x)
                            }
                            _ => unreachable!(),
                        })
                    })
                }
                _ => Err(format!("Unknown function: {:?}", first)),
            }
        }

        Expr::Symbol(s) => Err(format!("Undefined symbol: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        assert_eq!(eval(&Expr::Number(42)), Ok(Expr::Number(42)));
        assert_eq!(eval(&Expr::Float(3.14)), Ok(Expr::Float(3.14)));
        assert_eq!(eval(&Expr::Number(-42)), Ok(Expr::Number(-42)));
    }
}
