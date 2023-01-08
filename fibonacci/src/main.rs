fn main() {
    let fib = fib_recursion(2);
    println!("fib value: {fib}");
}

fn fib_iteration(n: i32) -> i32 {
    if n < 3 {
        return 1;
    }

    let mut prev = 1;
    let mut current = 1;

    for _ in 2..n {
        let next = current + prev;
        prev = current;
        current = next;
    }

    current
}

fn fib_recursion(n: i32) -> i32 {
    if n < 3 {
        return 1;
    }

    fib_recursion(n - 2) + fib_recursion(n - 1)
}