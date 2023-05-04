pub fn prime_factorize(input: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let mut target = input;
    let mut i = 2;
    while i * i <= target {
        if target % i != 0 {
            i += 1;
            continue;
        }
        let mut e = 0;
        while target % i == 0 {
            e += 1;
            target /= i;
        }
        res.push((i, e));

        i += 1;
    }
    if target != 1 {
        res.push((target, 1));
    }
    res
}
