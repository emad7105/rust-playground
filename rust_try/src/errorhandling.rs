use anyhow::{anyhow, Result};

pub fn run() {
    let sum = add_values(true);
    match sum {
        Ok(sum) => {
            println!("sum = {:?}", sum)
        }
        Err(e) => {
            println!("error = {:?}", e)
        }
    }
}

fn add_values(do_fail: bool) -> Result<i32> {
    if do_fail {
        Err(anyhow!("Error!!!! {:?}", "Yes".to_string()))
    } else {
        Ok(2 + 2)
    }
}