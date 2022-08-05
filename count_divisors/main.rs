fn get_divisors(x: u32) -> Vec<u32> {
    let mut D: Vec<u32> = Vec::new();
    for d in 1..x+1 {
        if x % d == 0 {
            D.push(d);
        }
    }
    D
}

fn count_divisors1(x: u32) -> u32 {
    let mut count: u32 = 0;
    for d in 1..x+1 {
        if x % d == 0 {
            count += 1;
        }
    }
    count
}

fn count_divisors2(x: u32) -> u32 {
    let mut count: u32 = 0;
    let n: u32 = (x as f32).sqrt() as u32;
    for d in 1..n+1 {
        if x % d == 0 {
            if x / d == d {
                count += 1
            } else {
                count += 2;
            }
        }
    }
    count
}

fn main() {
    for x in 1..100 {
        let D = get_divisors(x);
        let c1 = count_divisors1(x);
        let c2 = count_divisors2(x);
        println!("{}: {:?} len={} count1={} count2={}", x, D, D.len(), c1, c2);
        assert!(c1 == c2);
    }
}
