fn main() {
    let x = five();
    println!("The value of x is {x}");
    let x = plus_one(x);
    println!("The x plus one is {x}");

    let x: (i32, i32, i32) = plus_y(1, 2);
    let a = x.0;
    let b = x.1;
    let c = x.2;
    println!("The {a} plus {b} is {c}");

    let y = {
        let x = 3;
        //注意不带分号(semicolon)
        x + 1
    };
    println!("The value of y is {y}");

    let mut arr = [9, 8, 7, 6, 5, 4, 3, 3, 2, 2];
    bubble_sort_for(&mut arr);
    let mut arr = [9, 8, 7, 6, 5, 4, 3, 3, 2, 2];
    bubble_sort_loop(&mut arr);
}

fn five() -> i32 {
    return 5;
}
fn plus_one(x: i32) -> i32 {
    //注意不带分号(semicolon)
    x + 1
}

fn plus_y(x: i32, y: i32) -> (i32, i32, i32) {
    return (x, y, x + y);
}

fn compare(x: i32, y: i32) -> bool {
    x > y
}

fn show(slice: &[i32]) {
    for &item in slice.iter() {
        print!("{},", item);
    }
    println!();
}

fn bubble_sort_for(arr: &mut [i32]) {
    show(arr);
    for i in 0..arr.len() - 1 {
        for j in i + 1..arr.len() {
            if compare(arr[i], arr[j]) {
                arr.swap(i, j);
            }
            show(arr);
        }
    }
}
fn bubble_sort_loop(arr: &mut [i32]) {
    show(arr);
    let mut i = 0;
    loop {
        if i < arr.len() - 1 {
            let mut j = i + 1;
            loop {
                if j < arr.len() {
                    if compare(arr[i], arr[j]) {
                        arr.swap(i, j);
                    }
                } else {
                    break;
                }
                j += 1;
                show(arr);
            }
        } else {
            break;
        }
        i += 1;
    }
}
