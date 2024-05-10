use std::time::Duration;

use app::MyApp;
use eframe::NativeOptions;
use tokio::runtime::Builder;

mod app;
pub mod database;

#[derive(Debug)]
pub enum AppError{
    StdError{error: Box<dyn std::error::Error>},
    EframeError{error: eframe::Error}
}

pub fn run(options: NativeOptions) -> Result<(), AppError> {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let _enter = rt.enter();

    let db = rt.block_on(database::get_pooled_connection("test.db"));

    if let Err(err) = db {
        return Err(AppError::StdError { error: Box::new(err)})
    }

    let copied_db = db.as_ref().unwrap().clone();

    rt.block_on(async {
        sqlx::migrate!().run(&copied_db).await
    }).expect("Couldn't migrate");


    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });



    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::new(MyApp::new(cc, db.unwrap()))
        }),
    ).map_err(|error| AppError::EframeError { error })
}
