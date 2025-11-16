use crate::print;
use axum::{Router, routing::get};

pub async fn run_webserver() {
    // build our application with a single route
    let app = Router::new().route("/", get(print_me_some));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn print_me_some() {
    let printer_data = print::PrintData {
        print_meta_data: print::PrintMetaData {
            global_message_count: 10,
            message_number: 1,
            global_print_count: 5,
            message_print_count: 1,
            written_at: "16.11.2025 23:55".to_string(),
        },
        title: Some("Test Titel Nr. 1 Wird sehr sehr lang, was passiert dann wohl!?".to_string()),
        message: "Diese Nachricht ist \n sehr wichtig!\n\n Jajaja12324566788\n!@#$%^&*(){}-=\\|"
            .to_string(),
        author: Some("Marlon".to_string()),
    };

    print::print(printer_data).expect("Printer fucked");
}
