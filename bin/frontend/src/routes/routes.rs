macro_rules! routes {
    {
        $ (
            $module: ident,
            $route: ident
        ); *
        $ (;) ?
    } => {
        $ (
            mod $module;
            pub use $module::$route;
        ) *
    };
}
