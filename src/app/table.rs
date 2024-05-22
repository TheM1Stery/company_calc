use egui::Ui;
use egui_extras::{Column, TableBuilder, TableRow};

use super::{model::Company, Row};

pub struct CompanyTable<'a> {
    rows: &'a mut Vec<Row>,
    selected_rows: &'a mut std::collections::HashSet<usize>,
}

impl<'a> CompanyTable<'a> {
    pub fn new(
        rows: &'a mut Vec<Row>,
        selected_rows: &'a mut std::collections::HashSet<usize>,
    ) -> CompanyTable<'a> {
        Self {
            rows,
            selected_rows,
        }
    }

    pub fn table_ui(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();

        let builder = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .sense(egui::Sense::click())
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::initial(25.0).at_least(25.0).at_most(30.0))
            .column(Column::auto())
            .columns(Column::initial(100.0).at_least(100.0).at_most(250.0), 3)
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        builder
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Код");
                        ui.separator();
                    });
                });

                header.col(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.strong("Наименование");
                        ui.separator();
                    });
                });
                let cols = ["Дебет", "Кредит"];
                header.col(|ui| multi_header(ui, "Остаток на начало месяца", Some(cols)));
                header.col(|ui| multi_header(ui, "Оборот за месяц", Some(cols)));
                header.col(|ui| multi_header(ui, "Остаток на конец", Some(cols)));
            })
            .body(|body| {
                let row_height = 18.0;
                body.rows(row_height, self.rows.len(), |mut row| {
                    let index = row.index();
                    match &mut self.rows[index] {
                        Row::Constant(company) => {
                            row.set_selected(self.selected_rows.contains(&index));
                            row_constant(&mut row, company);
                            self.toggle_selection(index, &row.response());
                        }
                        Row::BeingEdited(_) => todo!(),
                        Row::New(new_company) => {
                            row.col(|ui| {
                                ui.label("");
                            });

                            row.col(|ui| {
                                ui.text_edit_singleline(&mut new_company.name);
                            });

                            row.col(|ui| {
                                ui.columns(3, |columns| {
                                    if columns[0]
                                        .text_edit_singleline(
                                            &mut new_company.remainder_begin_month_pos,
                                        )
                                        .changed()
                                    {
                                        new_company.remainder_begin_month_neg.clear();
                                    }

                                    columns[1].add(egui::Separator::default().vertical());
                                    if columns[2]
                                        .text_edit_singleline(
                                            &mut new_company.remainder_begin_month_neg,
                                        )
                                        .changed()
                                    {
                                        new_company.remainder_begin_month_pos.clear();
                                    }
                                });
                            });

                            row.col(|ui| {
                                ui.columns(3, |columns| {
                                    columns[0]
                                        .text_edit_singleline(&mut new_company.debit_turnover);
                                    columns[1].add(egui::Separator::default().vertical());
                                    columns[2]
                                        .text_edit_singleline(&mut new_company.credit_turnover);
                                });
                            });
                        }
                        Row::Total => todo!(),
                    }
                });
            });
    }

    fn toggle_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selected_rows.contains(&row_index) {
                self.selected_rows.remove(&row_index);
                return;
            }

            self.selected_rows.insert(row_index);
        }
    }
}

fn row_constant(row: &mut TableRow, company: &Company) {
    row.col(|ui| {
        ui.label(format!("{}", company.id));
    });
    row.col(|ui| {
        ui.label(&company.name);
    });
    row.col(|ui| {
        ui.columns(2, |columns| {
            let mut remainder = company.remainder_begin_month;
            if remainder >= 0. {
                columns[0].vertical_centered(|ui| {
                    ui.label(format!("{remainder}"));
                });
            }
            // columns[1].add(egui::Separator::default().vertical());
            if remainder < 0. {
                remainder *= -1.;
                columns[1].vertical_centered(|ui| {
                    ui.label(format!("{remainder}"));
                });
            }
        });
    });
    row.col(|ui| {
        ui.columns(2, |columns| {
            columns[0].vertical_centered(|ui| ui.label(format!("{}", company.debit_turnover)));
            columns[1].vertical_centered(|ui| ui.label(format!("{}", company.credit_turnover)));
        });
    });
    row.col(|ui| {
        ui.columns(2, |columns| {
            let mut remainder = company.remainder_end_month;
            if remainder >= 0. {
                columns[0].vertical_centered(|ui| ui.label(format!("{remainder}")));
            }
            if remainder < 0. {
                remainder *= -1.;
                columns[1].vertical_centered(|ui| ui.label(format!("{remainder}")));
            }
        })
    });
}

fn row_editable() {}

fn multi_header(ui: &mut Ui, title: &str, cols: Option<[&str; 2]>) {
    ui.vertical_centered(|ui| {
        ui.strong(title);
        ui.separator();
        if let Some(cols) = cols {
            ui.columns(2, |columns| {
                columns[0].vertical_centered(|ui| {
                    ui.strong(cols[0]);
                });
                columns[1].vertical_centered(|ui| {
                    ui.strong(cols[1]);
                });
            });
        }
    });
}
