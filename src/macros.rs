#[macro_export]
macro_rules! ok_or_continue {
    ( $x: expr) => {
        match $x {
            Ok(val) => val,
            Err(_) => continue,
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ( $x: expr) => {
        match $x {
            Ok(val) => val,
            Err(_) => return,
        }
    };
}
