use std::sync::mpsc::{self, Receiver, Sender};

use egui::global_dark_light_mode_buttons;
use sqlx::SqlitePool;

mod exports;
mod map;
mod model;
mod operations;
mod table;

use self::{
    exports::{export_to_excel, map_to_excel},
    map::{map_to_edited, map_to_new},
    model::Company,
    operations::{add_company, delete_company, edit_company, get_all_companies, Operation},
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

#[derive(Default, Debug)]
pub struct TotalRow {
    pub remainder_begin_month_pos: f64,
    pub remainder_begin_month_neg: f64,
    pub debit_turnover: f64,
    pub credit_turnover: f64,
    pub remainder_end_month_pos: f64,
    pub remainder_end_month_neg: f64,
}

#[derive(Debug)]
pub enum Row {
    Constant(Company),
    BeingEdited(EditedCompanyRow),
    New(NewCompanyRow),
    Total(TotalRow),
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
    rows: Vec<Row>,

    mode: Mode,

    need_to_fetch: bool,

    need_to_calculate_total: bool,

    selected_rows: std::collections::HashSet<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rows: Default::default(),
            mode: Mode::Normal,
            need_to_fetch: true,
            selected_rows: Default::default(),
            need_to_calculate_total: true,
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
        if self.state.need_to_fetch {
            fetch_all(self.db.clone(), self.tx.clone());
            self.state.need_to_fetch = false;
        }

        if self.state.need_to_calculate_total {
            calculate_total(&self.state.rows, self.tx.clone());
            self.state.need_to_calculate_total = false;
        }
        if let Ok(op) = self.rx.try_recv() {
            match op {
                Operation::Add { new_companies } => {
                    let mapped_into_constant: Vec<_> =
                        new_companies.into_iter().map(Row::Constant).collect();

                    remove_non_constant(&mut self.state.rows, true);

                    self.state.rows.extend(mapped_into_constant);
                    self.state.need_to_calculate_total = true;
                }
                Operation::FetchAll { all_companies } => {
                    if let Ok(companies) = all_companies {
                        self.state.rows = companies.into_iter().map(Row::Constant).collect();
                    }
                    self.state.need_to_calculate_total = true;
                }
                Operation::Delete { deleted_companies } => {
                    remove_deleted(&mut self.state.rows, &deleted_companies);
                    self.state.selected_rows.clear();
                    self.state.need_to_calculate_total = true;
                }
                Operation::Edit => {
                    self.state.need_to_fetch = true;
                }
                Operation::Total { total } => {
                    remove_non_constant(&mut self.state.rows, true);
                    self.state.rows.push(Row::Total(total));
                }
            }
        }

        let top_panel = egui::TopBottomPanel::top("top_panel").show_separator_line(false);

        top_panel.show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        ui.menu_button("Export as..", |ui| {
                            if ui.button("Excel").clicked() {
                                save_to_excel(&mut self.state);
                            }
                        });

                        if ui.button("Exit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.menu_button("View", |ui| {
                        global_dark_light_mode_buttons(ui);
                    });
                });
            });
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

                    let edit_button = ui.add_enabled(
                        !self.state.selected_rows.is_empty()
                            && matches!(self.state.mode, Mode::Normal),
                        egui::Button::new("Edit"),
                    );

                    if edit_button.clicked() {
                        edit_selected_rows(&mut self.state);
                    }

                    let delete_button = ui.add_enabled(
                        !self.state.selected_rows.is_empty()
                            && matches!(self.state.mode, Mode::Normal),
                        egui::Button::new("Delete"),
                    );

                    if delete_button.clicked() {
                        delete_selected(self.db.clone(), &mut self.state, self.tx.clone());
                    }
                });
                use egui_extras::{Size, StripBuilder};
                StripBuilder::new(ui)
                    .size(Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            let mut table = CompanyTable::new(
                                &mut self.state.rows,
                                &mut self.state.selected_rows,
                            );
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                table.table_ui(ui);
                            });
                            ui.horizontal(|ui| {
                                if !matches!(self.state.mode, Mode::Normal)
                                    && ui.button("Save").clicked()
                                {
                                    match &self.state.mode {
                                        Mode::Add => {
                                            save_new_rows(
                                                self.db.clone(),
                                                &mut self.state,
                                                self.tx.clone(),
                                            );
                                        }
                                        Mode::Edit => {
                                            save_edited_rows(
                                                self.db.clone(),
                                                &mut self.state,
                                                self.tx.clone(),
                                            );
                                        }
                                        _ => (),
                                    }
                                    self.state.mode = Mode::Normal;
                                }

                                if !matches!(self.state.mode, Mode::Normal)
                                    && ui.button("Cancel").clicked()
                                {
                                    match self.state.mode {
                                        Mode::Add => {
                                            remove_non_constant(&mut self.state.rows, false);
                                        }
                                        Mode::Edit => {
                                            self.state.need_to_fetch = true;
                                        }
                                        _ => (),
                                    }
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
    state
        .rows
        .insert(state.rows.len() - 1, Row::New(NewCompanyRow::default()))
}

fn edit_selected_rows(state: &mut State) {
    state.mode = Mode::Edit;

    let rows_to_be_edited: Vec<_> = state
        .rows
        .iter_mut()
        .enumerate()
        .filter(|(i, _)| state.selected_rows.contains(i))
        .map(|(_, e)| e)
        .collect();

    rows_to_be_edited.into_iter().for_each(|x| {
        if let Row::Constant(company) = x {
            let mut remainder_begin_month_neg = String::new();
            let mut remainder_begin_month_pos = String::new();

            if company.remainder_begin_month >= 0. {
                remainder_begin_month_pos.push_str(&company.remainder_begin_month.to_string());
            } else {
                let remainder = company.remainder_begin_month * -1.;
                remainder_begin_month_neg.push_str(&remainder.to_string());
            }

            *x = Row::BeingEdited(EditedCompanyRow {
                id: company.id,
                name: company.name.to_owned(),
                remainder_begin_month_pos,
                remainder_begin_month_neg,
                debit_turnover: company.debit_turnover.to_string(),
                credit_turnover: company.credit_turnover.to_string(),
            })
        }
    })
}

fn remove_non_constant(vec: &mut Vec<Row>, remove_total: bool) {
    vec.retain(|x| match x {
        Row::Total(_) => !remove_total,
        Row::Constant(_) => true,
        _ => false,
    });
}

fn remove_deleted(vec: &mut Vec<Row>, deleted_companies: &std::collections::HashSet<i64>) {
    vec.retain(|x| !(matches!(x, Row::Constant(_)) && deleted_companies.contains(&x.constant().id)))
}

fn delete_selected(db: SqlitePool, state: &mut State, tx: Sender<Operation>) {
    let row_ids: Vec<_> = state
        .rows
        .iter()
        .enumerate()
        .filter(|(i, row)| state.selected_rows.contains(i) && matches!(row, Row::Constant(_)))
        .map(|(_, row)| row.constant().id)
        .collect();

    tokio::spawn(async move {
        let mut ids_deleted = std::collections::HashSet::new();
        for id in row_ids {
            delete_company(db.clone(), id).await.unwrap();

            ids_deleted.insert(id);
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

fn save_edited_rows(db: SqlitePool, state: &mut State, tx: Sender<Operation>) {
    let edited_rows: Vec<_> = state
        .rows
        .iter()
        .filter_map(|e| match e {
            Row::BeingEdited(row) => Some(row),
            _ => None,
        })
        .flat_map(map_to_edited)
        .collect();

    tokio::spawn(async move {
        for row in edited_rows {
            let _value = edit_company(db.clone(), row).await;
        }

        tx.send(Operation::Edit)
    });
}

fn save_new_rows(db: SqlitePool, state: &mut State, tx: Sender<Operation>) {
    let new_rows: Vec<_> = state
        .rows
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

fn save_to_excel(state: &mut State) {
    let dialog = rfd::AsyncFileDialog::new().set_file_name("company_list.xlsx");
    let save_task = dialog.save_file();
    let mapped_to_excel: Vec<_> = state
        .rows
        .iter()
        .flat_map(|r| match r {
            Row::Constant(row) => Some(map_to_excel(row)),
            _ => None,
        })
        .collect();
    tokio::spawn(async move {
        let file = save_task.await;
        if let Some(file) = file {
            let excel = export_to_excel(&mapped_to_excel).unwrap();
            _ = file.write(&excel).await;
        }
    });
}

fn calculate_total(rows: &[Row], tx: Sender<Operation>) {
    let constant_rows: Vec<_> = rows
        .iter()
        .filter_map(|x| match x {
            Row::Constant(row) => Some(row.to_owned()),
            _ => None,
        })
        .collect();

    // this is overkill to use tokio spawn for sync stuff but i don't care(i want my code to look
    // pretty)
    tokio::spawn(async move {
        let total = constant_rows
            .iter()
            .fold(TotalRow::default(), |acc, constant| {
                let positive_begin = constant.remainder_begin_month >= 0.;
                let remainder_begin = match positive_begin {
                    true => constant.remainder_begin_month,
                    false => constant.remainder_begin_month * -1.,
                };

                let positive_end = constant.remainder_end_month >= 0.;
                let remainder_end = match positive_end {
                    true => constant.remainder_end_month,
                    false => constant.remainder_end_month * -1.,
                };

                TotalRow {
                    remainder_begin_month_pos: if positive_begin {
                        acc.remainder_begin_month_pos + remainder_begin
                    } else {
                        acc.remainder_begin_month_pos
                    },
                    remainder_begin_month_neg: if !positive_begin {
                        acc.remainder_begin_month_neg + remainder_begin
                    } else {
                        acc.remainder_begin_month_neg
                    },
                    debit_turnover: acc.debit_turnover + constant.debit_turnover,
                    credit_turnover: acc.credit_turnover + constant.debit_turnover,
                    remainder_end_month_pos: if positive_end {
                        acc.remainder_begin_month_pos + remainder_end
                    } else {
                        acc.remainder_end_month_pos
                    },
                    remainder_end_month_neg: if !positive_end {
                        acc.remainder_begin_month_neg + remainder_end
                    } else {
                        acc.remainder_end_month_neg
                    },
                }
            });
        tx.send(Operation::Total { total })
    });
}
