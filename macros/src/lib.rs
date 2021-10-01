#[macro_export]
macro_rules! hashmap {
    ($k: expr => $v: expr $(,$fk: expr => $fv: expr)*) => {{
        let mut myhashmap = ::std::collections::HashMap::new();
        myhashmap.insert($k, $v);
        $(myhashmap.insert($fk, $fv);)*
        myhashmap
    }};
    ($($fk: expr => $fv: expr,)+) => {{
        let mut myhashmap = ::std::collections::HashMap::new();
        $(myhashmap.insert($fk, $fv);)+
        myhashmap
    }};

    ()=>{{
        ::std::collections::HashMap::new()
    }};
}
