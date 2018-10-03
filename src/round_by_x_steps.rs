//! a solution to problem in codewars: https://www.codewars.com/kata/round-by-0-dot-5-steps/train/rust

fn solution(n: f64) -> f64 {
    let m1 = (n - n.floor()).abs();
    let m2 = (n - n.ceil()).abs();
    let m3 = (n - n.floor() - 0.5).abs();
//    if m1 < m2 && m1 < m3 {
//        return n.floor();
//    }
    if m1 < m2 {
        if m1 < m3 {
            return n.floor();
        } else if m2 < m3 {
            return n.ceil();
        } else {return n.floor() + 0.5};
    } else if m2 < m3 {
        return n.ceil();
    } else { return n.floor() + 0.5; }

}

fn best_practise(n:f64)->f64{
    (2.0*n).round()/2.0
}

#[test]
fn sample_tests() {
    assert_eq!(solution(4.2), 4.0);
    assert_eq!(solution(4.4), 4.5);
    assert_eq!(solution(4.6), 4.5);
    assert_eq!(solution(4.8), 5.0);
}

#[test]
fn best_practise_tests() {
    assert_eq!(best_practise(4.2), 4.0);
    assert_eq!(best_practise(4.4), 4.5);
    assert_eq!(best_practise(4.6), 4.5);
    assert_eq!(best_practise(4.8), 5.0);
}