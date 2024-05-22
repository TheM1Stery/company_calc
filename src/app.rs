use std::sync::mpsc::{self, Receiver, Sender};

use sqlx::SqlitePool;

mod exports;
mod map;
mod model;
mod operations;
mod table;

use self::{
    map::map_to_new,
    model::Company,
    operations::{add_company, delete_company, get_all_companies, Operation},
    table::CompanyTable,
};

pub struct MyApp {
    tx: Sender<Operation>,
    rx: Receiver<Operation>,
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

#[derive(Default, Debug)]
pub struct EditedCompanyRow {
    pub id: i64,
    pub name: String,
    pub remainder_begin_month_pos: String,
    pub remainder_begin_month_neg: String,
    pub debit_turnover: String,
    pub credit_turnover: String,
}

#[derive(Default, Debug)]
pub struct NewCompanyRow {
    pub name: String,
    pub remainder_begin_month_pos: String,
    pub remainder_begin_month_neg: String,
    pub debit_turnover: String,
    pub credit_turnover: String,
}

#[derive(Debug)]
pub enum Row {
    Constant(Company),
    BeingEdited(EditedCompanyRow),
    New(NewCompanyRow),
    Total,
}

impl Row {
    fn constant(&self) -> &Company {
        if let Row::Constant(company) = self {
            company
        } else {
            panic!("Not a constant row");
        }
    }
}

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    // #[serde(skip)]
    vec: Vec<Row>,

    mode: Mode,

    was_fetched_on_start: bool,

    selected_rows: std::collections::HashSet<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            vec: Default::default(),
            mode: Mode::Normal,
            was_fetched_on_start: false,
            selected_rows: Default::default(),
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
        if !self.state.was_fetched_on_start {
            fetch_all(self.db.clone(), self.tx.clone());
            self.state.was_fetched_on_start = true;
        }
        if let Ok(op) = self.rx.try_recv() {
            match op {
                Operation::Add { new_companies } => {
                    let mapped_into_constant: Vec<_> =
                        new_companies.into_iter().map(Row::Constant).collect();

                    remove_non_constant(&mut self.state.vec);

                    self.state.vec.extend(mapped_into_constant);
                }
                Operation::FetchAll { all_companies } => {
                    if let Ok(companies) = all_companies {
                        self.state.vec = companies.into_iter().map(Row::Constant).collect();
                    }
                }
                Operation::Delete { deleted_companies } => {
                    self.state.vec.retain(|x| {
                        !(matches!(x, Row::Constant(_))
                            && deleted_companies.contains(&x.constant().id))
                    });
                    self.state.selected_rows.clear();
                }
                _ => todo!(),
            }
        }

        let top_panel = egui::TopBottomPanel::top("top_panel").show_separator_line(false);

        top_panel.show(ctx, |ui| {
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
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Add row").clicked() {
                        add_row(&mut self.state);
                    }

                    let button = ui.add_enabled(
                        !self.state.selected_rows.is_empty(),
                        egui::Button::new("Delete"),
                    );
                    if button.clicked() {
                        delete_selected(self.db.clone(), &mut self.state, self.tx.clone());
                    }
                });
                use egui_extras::{Size, StripBuilder};
                StripBuilder::new(ui)
                    .size(Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            let mut table = CompanyTable::new(
                                &mut self.state.vec,
                                &mut self.state.selected_rows,
                            );
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                table.table_ui(ui);
                            });
                            ui.horizontal(|ui| {
                                if let Mode::Add = &self.state.mode {
                                    if ui.button("Save").clicked() {
                                        save_all(self.db.clone(), &mut self.state, self.tx.clone());
                                        self.state.mode = Mode::Normal;
                                    }
                                }

                                if !matches!(self.state.mode, Mode::Normal)
                                    && ui.button("Cancel").clicked()
                                {
                                    remove_non_constant(&mut self.state.vec);
                                    self.state.mode = Mode::Normal;
                                }
                            });
                        });
                    });
            });
        });
    }
}

fn add_row(state: &mut State) {
    state.mode = Mode::Add;
    state.vec.push(Row::New(NewCompanyRow::default()));
}

fn remove_non_constant(vec: &mut Vec<Row>) {
    vec.retain(|x| matches!(x, Row::Constant(_)))
}

fn delete_selected(db: SqlitePool, state: &mut State, tx: Sender<Operation>) {
    let row_ids: Vec<_> = state
        .vec
        .iter()
        .enumerate()
        .filter(|(i, row)| state.selected_rows.contains(i) && matches!(row, Row::Constant(_)))
        .map(|(_, row)| row.constant().id)
        .collect();

    tokio::spawn(async move {
        let mut ids_deleted = Vec::new();
        for id in row_ids {
            delete_company(db.clone(), id).await.unwrap();

            ids_deleted.push(id);
        }

        tx.send(Operation::Delete {
            deleted_companies: ids_deleted,
        })
    });
}

fn fetch_all(db: SqlitePool, tx: Sender<Operation>) {
    tokio::spawn(async move {
        let all_companies = get_all_companies(db.clone()).await;

        tx.send(Operation::FetchAll { all_companies })
    });
}

fn save_all(db: SqlitePool, state: &mut State, tx: Sender<Operation>) {
    let new_rows: Vec<_> = state
        .vec
        .iter()
        .filter_map(|e| match e {
            Row::New(row) => Some(row),
            _ => None,
        })
        .flat_map(map_to_new)
        .collect();

    tokio::spawn(async move {
        let mut vec = Vec::new();

        // TODO: needs to be rewritten to use JoinSet
        for row in new_rows {
            let item = add_company(db.clone(), row).await;
            vec.push(item.unwrap());
        }

        tx.send(Operation::Add { new_companies: vec })
    });
}
