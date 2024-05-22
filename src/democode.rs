#[no_mangle]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// demo
// #[no_mangle]
// fn android_mainrs(app: slint::android::AndroidApp) {
//     slint::android::init(app).unwrap();

//     // ... rest of your code ...
//     slint::slint!{
//         export component MainWindow inherits Window {
//             Text { text: "Hello World"; }
//         }
//     }
//     MainWindow::new().unwrap().run().unwrap();
// }

fn mainui222() -> Result<(),slint::PlatformError> {
    let ui = AppWindow::new()?;

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 2);
    //         let cnt = ui.get_counter();
    //         let cntstr = cnt .to_string();
    //         let pfx : slint::SharedString = "nammme".into();
    //         ui.set_fdname( pfx + &cntstr);
    //     }
    // });
    // ui.on_justrunafunc({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         println!("ffffff");
    //         // ui.set_counter(ui.get_counter() + 2);
    //         // let cnt = ui.get_counter();
    //         // let cntstr = cnt .to_string();
    //         // let pfx : slint::SharedString = "nammme".into();
    //         // ui.set_fdname( pfx + &cntstr);
    //     }
    //     // println("ffff");
    // });

    let _ = ui.run();
    Ok(())
}