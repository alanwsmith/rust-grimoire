use osascript::JavaScript;
use serde::Deserialize;

#[derive(Deserialize)]
struct AlertResult {}

fn main() {
    println!("moving");

    let script = JavaScript::new(
        r#"
        Application("Google Chrome").windows[0].bounds = { "x": 20, "y": 20, "width": 400, "height": 400 };
        "#,
    );

    let result: Result<
        AlertResult,
        osascript::Error,
    > = script.execute();

    match result {
        Ok(_) => {
            println!("{}", "it worked")
        }
        Err(e) => {
            println!("{}", e);
            println!("{}", "it failed")
        }
    }

    // let script = JavaScript::new(
    //     "
    //     var App =
    // Application('Finder');         App.
    // includeStandardAdditions = true;
    //     return
    // App.displayAlert($params.title, {
    //         message:
    // $params.message,
    // 'as': $params.alert_type,
    //         buttons:
    // $params.buttons,         });
    // ",
    // );

    // let rv: AlertResult = script
    //     .execute_with_params(
    //         AlertParams {
    //             title: "Shit is on
    // fire!"
    //                 .into(),
    //             message: "What is
    // happening"
    //                 .into(),
    //             alert_type: "critical"
    //                 .into(),
    //             buttons: vec![
    //                 "Show
    // details"
    //                     .into(),
    //                 "Ignore".into(),
    //             ],
    //         },
    //     )
    //     .unwrap();

    // println!(
    //     "You clicked '{}'",
    //     rv.button
    // );
}
// fn main() {
//     println!("HERE")
// }
