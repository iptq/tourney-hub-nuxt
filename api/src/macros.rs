macro_rules! Resp {
    () => { impl warp::Filter<
        Extract = (impl warp::Reply,),
        Error = warp::Rejection,
    > + Clone }
}

#[macro_export]
macro_rules! route_any {
    ($hm:ident $hp:tt => $h:expr $(, $tm:ident $tp:tt => $t:expr)* $(,)*) => {{
        use warp::Filter;
        route_any!(@internal @path $hm $hp).and($h)
            $(.or(route_any!(@internal @path $tm $tp).and($t)))*
    }};

    (@internal @path GET ()) => {{ warp::get() }};
    (@internal @path POST ()) => {{ warp::post() }};
    (@internal @path $m:ident $p:tt) => {{
        use warp::path;
        path! $p.and(route_any!(@internal @path $m ()))
    }};
}
