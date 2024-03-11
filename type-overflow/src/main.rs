fn main() {
    checked();
    wrapping();
    saturating();
    overflowing();
}

fn panic_in_debug_build() {
    // 다음 코드는 디버그 빌드에서 패닉에 빠진다.
    let mut i = 1;
    loop {
        i *= 10;
    }
    // 릴리스 빌드에서는 이 곱셈의 결과가 음수로 순환되기 때문에 루프가 무한정 실행된다.
}

fn panic_in_every_build() {
    // 다음 코드는 어떤 빌드에서든 패닉에 빠진다.
    let mut i: i32 = 1;
    loop {
        // 패닉: 오버플로를 일으키는 곱셈(모든 빌드에서 발생함)
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}

/// 정수 산술 메서드의 종류
/// 1. 점검 (checked)
/// 2. 순환 (wrapping)
/// 3. 포화 (saturating)
/// 4. 넘침 (overflowing)

fn checked() {
    // 점검(checked) 연산은 결과를 Option에 담아 반환한다.
    // 수학적으로 옳은 결과를 주어진 타입의 값으로 표현할 수 있으면 Some(v)가 되고, 그렇지 않으면 None이 된다.

    // 10과 20의 합은 u8로 표현할 수 있다.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // 안타깝게도 100과 200의 합은 그럴 수 없다.
    assert_eq!(100_u8.checked_add(200), None);

    // 덧셈을 하는데 오버플로가 발생하면 패닉에 빠진다.
    // let sum: u8 = 100_u8.checked_add(200_u8).unwrap();

    // 이상하지만 부호 있는 나눗셈도 오버플로를 일으키는 경우가 있다.
    // 부호 있는 n비트의 타입은 -2^(n-1)은 표현 할 수 있지만 2^(n-1)은 표현 할 수 없다.
    assert_eq!((-128_i8).checked_div(-1), None);
}

fn wrapping() {
    // 순환(wrapping) 연산은 수학적으로 옳은 결과를 주어진 값으 범위로 나눈 나머지에 해당하는 값을 반환한다.

    // 첫 번째 곲은 u16으로 표현할 수 있다.
    // 두 번째 곱은 그럴 수 없으므로 250000을 2^16으로 나눈 나머지가 산출된다.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // 부호 있는 타입을 대산으로 하는 연산은 음숫값으로 순환될 수도 있다.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // 비트별 자리 이동 연산에서는 이동 거리가 값의 크기 안에 들어가도록 순환된다.
    // 따라서 16비트 타입을 대상으로 하는 17비트 자리 이동은 1비트 자리 이동과 같다.
    assert_eq!(5_i16.wrapping_shl(17), 10);
}

fn saturating() {
    // 포화(saturating) 연산은 표현 할 수 있는 값 중에서 수학적으로 옳은 결과에 가장 가까운 값을 반환한다.
    // 다시 말해서 결과가 타입이 표현할 수 있는 최댓값과 최솟값 사이에 오도록 '고정'된다.

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // 포화 연산을 수행하는 나눗셈, 나머지, 비트별 자리 이동 메서드는 없다.
}

fn overflowing() {
    // 넘침(overflowing) 연산은 (result, overflowed) 튜플을 반환하는데,
    // 여기서 result는 함수의 순환 버전이 반환하는 값이고 overflowed는 오버플로 발생 여부를 나타내는 bool이다.

    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // 단, overflowing_shl과 overflowing_shr은 overflowed의 의미가 약간 다른데,
    // 이들은 자리 이동 거리가 타입 자체의 비트 크기와 같거나 클 때만 참을 반환한다.
    // 이때 실제로 적용되는 자리 이동 거리는 요청된 자리 이동 거리를 타입의 비트 크기로 나눈 나머지다.

    // 'u16'을 대상으로 하는 17비트 자리 이동은 거리가 타입 자체의 비트 크기를 넘어선다.
    // 따라서 17을 16으로 나눈 나머지인 1이 실제로 적용되는 자리 이동 거리다.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}
