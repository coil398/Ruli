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
    
    #[test]
    fn test_eval_addition() {
        assert_eq!(
            eval(&Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::Number(1),
                Expr::Number(2),
            ])),
            Ok(Expr::Float(3.0))
        );
    }
    
    #[test]
    fn test_eval_multiplication() {
        assert_eq!(
            eval(&Expr::List(vec![
                Expr::Symbol("*".to_string()),
                Expr::Number(3),
                Expr::Number(4),
            ])),
            Ok(Expr::Float(12.0))
        );
    }
    
    #[test]
    fn test_eval_subtraction() {
        assert_eq!(
            eval(&Expr::List(vec![
                Expr::Symbol("-".to_string()),
                Expr::Number(10),
                Expr::Number(5),
                Expr::Number(2),
            ])),
            Ok(Expr::Float(3.0))
        );
    }
    
    #[test]
    fn test_eval_division() {
        assert_eq!(
            eval(&Expr::List(vec![
                Expr::Symbol("/".to_string()),
                Expr::Number(10),
                Expr::Number(2),
            ])),
            Ok(Expr::Float(5.0))
        );
    }
    
    #[test]
    fn test_eval_division_by_zero() {
        assert!(
            eval(&Expr::List(vec![
                Expr::Symbol("/".to_string()),
                Expr::Number(10),
                Expr::Number(0),
            ])).is_err()
        );
    }
    
    #[test]
    fn test_eval_empty_list() {
        assert!(eval(&Expr::List(vec![])).is_err());
    }
    
    #[test]
    fn test_eval_unknown_function() {
        assert!(
            eval(&Expr::List(vec![
                Expr::Symbol("unknown".to_string()),
                Expr::Number(1),
                Expr::Number(2),
            ])).is_err()
        );
    }
    
    #[test]
    fn test_eval_nested_expressions() {
        assert_eq!(
            eval(&Expr::List(vec![
                Expr::Symbol("+".to_string()),
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Number(2),
                    Expr::Number(3),
                ]),
                Expr::Number(4),
            ])),
            Ok(Expr::Float(10.0))
        );
    }
}
