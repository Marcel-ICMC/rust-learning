fn fibo(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if n == 1 {
        return first
    }
    if n == 2 {
        return second
    }

    let mut ans = first + second;
    for _ in 2..n {
        ans = first + second;
        first = second;
        second = ans;
    }

    ans
}

fn main() {
    let n = 10;
    for i in 1..n {
        println!("fibo({}) = {}", i, fibo(i));
    }
}