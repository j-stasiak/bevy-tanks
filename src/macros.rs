#[macro_export]
macro_rules! get_single_or_return {
    ($query:ident) => {
        match $query.get_single() {
            Ok(result) => result,
            Err(_) => return,
        }
    };
}

#[macro_export]
macro_rules! get_single_mut_or_return {
    ($query:ident) => {
        match $query.get_single_mut() {
            Ok(result) => result,
            Err(_) => return,
        }
    };
}
