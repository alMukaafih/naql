pub mod manifest;
pub mod node;
pub mod traits;
pub mod zip;

#[macro_export]
macro_rules! ok {
    ($x:expr) => {
        $x.unwrap()
    };
}

#[macro_export]
macro_rules! own {
    ( $x:expr ) => {
        $x.to_owned()
    };
}

#[macro_export]
macro_rules! path {
    ( $x:expr ) => {
        ::std::path::PathBuf::from($x)
    };
}

#[macro_export]
macro_rules! join {
    ( $( $x:expr ),+ $( , )?)  => {
        {
            let mut p = ::std::path::PathBuf::new();
            $( p.push($x); )*
            p
        }
    };
}
