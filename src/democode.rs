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

