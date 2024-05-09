use std::{
    ops::Add,
    sync::mpsc::{self, Receiver, Sender},
};

use egui::ScrollArea;
use sqlx::SqlitePool;

mod model;
mod operations;
mod table;
mod map;

use self::{map::map_to_new, model::Company, table::CompanyTable};

pub struct MyApp {
    tx: Sender<State>,
    rx: Receiver<State>,
    db: SqlitePool,
    state: State,
}

#[derive(strum::Display)]
enum Mode {
    Normal,
    Add,
    Edit,
    Delete,
}

#[derive(Default)]
pub struct EditedCompanyRow {
    pub id: i64,
    pub name: String,
    pub remainder_begin_month: String,
    pub debit_turnover: String,
    pub credit_turnover: String,
}

#[derive(Default)]
pub struct NewCompanyRow {
    pub name: String,
    pub remainder_begin_month_pos: String,
    pub remainder_begin_month_neg: String,
    pub debit_turnover: String,
    pub credit_turnover: String,
}

pub enum Row {
    Constant(Company),
    BeingEdited(EditedCompanyRow),
    New(NewCompanyRow),
    Total,
}

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    // #[serde(skip)]
    vec: Vec<Row>,

    mode: Mode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            mode: Mode::Normal,
        }
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>, db: SqlitePool) -> Self {
        let (tx, rx) = mpsc::channel();
        // if let Some(storage) = cc.storage {
        //     let state = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //     return Self { tx, rx, db, state };
        // }

        Self {
            tx,
            rx,
            db,
            state: State::default(),
        }
    }
}

impl eframe::App for MyApp {
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, &self.state);
    // }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let state = &mut self.state;
        if let Ok(new_state) = self.rx.try_recv() {
            self.state = new_state;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut table = CompanyTable::new(&mut self.state.vec);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.columns(2, |columns| {
                        columns[0].vertical(|ui| ui.heading("Application"));
                        columns[1].vertical_centered_justified(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                                ui.heading(format!("{} mode", &self.state.mode))
                            });
                        });
                    });
                });
                ScrollArea::horizontal().show(ui, |ui| {
                    table.table_ui(ui);
                })
            });

            ui.horizontal(|ui| {
                if let Mode::Add = &self.state.mode {
                    if ui.button("Save").clicked() {
                        save_all(&mut self.state, self.tx.clone());
                    }
                }

                if ui.button("Add row").clicked() {
                    add_row(&mut self.state);
                }
            });
        });
    }
}

fn add_row(state: &mut State) {
    state.mode = Mode::Add;
    state.vec.push(Row::New(NewCompanyRow::default()));
}

fn save_all(state: &mut State, tx: Sender<State>){
    let new_rows: Vec<_> = state.vec.iter().filter_map(|e| match e {
        Row::New(row) => Some(row),
        _ => None
    })
    .map(map_to_new)
    .collect();

    tokio::spawn(async move {
    });
}
