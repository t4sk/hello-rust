#![allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

fn f1_f2_match() -> Result<u32, String> {
    // Panic on error
    // f1().unwrap();
    // f2().unwrap();

    let res_1 = f1();
    let out_1 = match res_1 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f1".to_string());
        }
    };

    let res_2 = f2();
    let out_2 = match res_2 {
        Ok(num) => num,
        Err(_) => {
            return Err("error from f2".to_string());
        }
    };

    Ok(out_1 + out_2)
}

// Return type must be same (or can be casted) for f1 and f2
fn f1_f2_question() -> Result<u32, String> {
    let out_1 = f1()?;
    let out_2 = f2()?;
    Ok(out_1 + out_2)
}

fn main() {
    // Match
    let res = f1_f2_match();
    println!("{:?}", res);
    // Question operator
    let res = f1_f2_question();
    println!("{:?}", res);
}
