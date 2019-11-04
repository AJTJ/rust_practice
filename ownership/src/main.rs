fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!!");
    println!("s1: {}", s1);

    let mut another_s = "Here we go!";

    println!("the other {}", another_s);

    another_s = "brood";

    println!("the other {}", another_s);

    // s2 is a copy of s1, BUT it uses the heap data that the pointer of s1 points to.
    let s2 = s1;
    println!("moving s2 to s2: {}", s2);

    //ALSO s1 no longer exists, it has been moved to s2
    // the following would produce an error
    // println!("s1 {}", s1)

    //s3 is a CLONE, and therefore a FULL COPY
    let s3 = s2.clone();

    println!("s3, a full copy: {}", s3);

    // integers are COPY, therefore they do not get ownership taken away from them.
    let n1 = 1;
    let mut n2 = n1;
    n2 = n2 + 1;
    // n1 still exists
    println!("n1 {}", n1);
    println!("n2 {}", n2);

    println!("return num {}", return_num(n2));
    println!("n2 still avail {}", n2);

    //the following function takes the s3 value
    return_str(s3);
    // the following is an error since s3's value was moved
    // by the function call.
    // println!("s3 not in scope {}", s3);
}

fn return_num(x: i32) -> i32 {
    x
}

fn return_str(x: String) {
    println!("the s3 string moved into this func!! {}", x);
}
