
// https://play.integer32.com/?gist=4d52c229918537b01dcdd84509c02b63&version=undefined
#[macro_export]
macro_rules! dispatch_to {
    ($val:expr,$args:expr => {$($func:ident),*,}) => {
            match $val {
                $(
                    stringify!($func) => $func($args),
                )*
                _ => Ok(()),
            }
    }
}
