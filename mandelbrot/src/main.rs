use num::Complex;

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}
/// `c`가 망ㅔㄹ브로 집합에 속하는지 아닌지를 판단하며, 결론 내리는데 필요한 반복 횟수는 최대
/// `limit`회로 제한한다.
///
/// `c`가 망델브로 집합에 속하지 않으면 `Some(i)`를 반환하는데, 여기서 `i`는 `c`가 원점을
/// 중심으로 반경이 2인 원을 벗어나는 데 걸린 반복 횟수다. `c`가 망델브로 집합에 속하는 것
/// 같으면(좀 더 정확히 말해서 반복 횟수가 `limit`이 될 때까지도 `c`가 망델브로 집합에
/// 속하지 않는다는 걸 입증하지 못하면) `None`을 반환한다.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}
