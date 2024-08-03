/// LogSumExp関数
/// fns: ログスケールで評価される関数のリスト。漸近する関数のリストを渡す。
/// x: 引数
/// 戻り値: 与えた引数を適用した結果のLogSumExp (log(exp(f1(x)) + exp(f2(x)) + ...))
pub fn log_sum_exp(fns: &[impl Fn(f64) -> f64], x: f64) -> f64 {
    let mut max = f64::NEG_INFINITY;
    for f in fns {
        max = max.max(f(x));
    }
    let mut sum = 0.0;
    for f in fns {
        sum += (f(x) - max).exp();
    }
    max + sum.ln()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, threshold: f64) -> bool {
        (a - b).abs() < threshold
    }

    #[test]
    fn test_log_sum_exp() {
        let fns = vec![|_| 3.0, |x| x];
        assert!(close(log_sum_exp(&fns, -1000.0), 3.0, 1e-10));
        assert!(close(log_sum_exp(&fns, 1000.0), 1000.0, 1e-10));
    }
}
