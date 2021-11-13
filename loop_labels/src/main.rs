fn main() {
    let mut ct: i32 = 0;

    'counting_up: loop {
        println!("count = {}", ct);
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break; //breaks the inner loop
            }

            if ct == 2 {
                break 'counting_up; //breaks the outermost loop
            }

            remaining -= 1;
        }

        ct += 1;
    }

    println!("End count = {}", ct);
}
