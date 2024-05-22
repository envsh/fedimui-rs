fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
    // https://docs.rs/slint-build/latest/slint_build/fn.compile_with_config.html

    // macos cupertino, android material, other fluent
    // cosmic 还可以，看看不能用在android上。
    // slint的 material 是设计的最差的。
    // cupertino 在 android上按钮太长。cosmic 在android上滚动失效
    // 在 android 上只有 material 能滚动，也就是 android 只能用 material。

    // let config =
    //     slint_build::CompilerConfiguration::new()
    //         .with_style("material-dark".into());
    // slint_build::compile_with_config("ui/appwindow.slint", config).unwrap();
}
