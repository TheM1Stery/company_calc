use egui::Ui;
use egui_extras::{Column, TableBuilder, TableRow};

use super::{model::Company, Row};

pub struct CompanyTable<'a> {
    rows: &'a mut Vec<Row>,
}

impl<'a> CompanyTable<'a> {
    pub fn new(rows: &'a mut Vec<Row>) -> CompanyTable<'a> {
        Self { rows }
    }

    pub fn table_ui(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height();

        let builder = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);

        builder
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Код");
                });

                header.col(|ui| {
                    ui.strong("Наименование");
                });
                header.col(|ui| multi_header(ui, "Остаток на начало месяца", ["Дебет", "Кредит"]));
                header.col(|ui| multi_header(ui, "Оборот за месяц", ["Дебет", "Кредит"]));
                header.col(|ui| multi_header(ui, "Остаток на конец", ["Дебет", "Кредит"]));
            })
            .body(|body| {
                let row_height = 18.0;
                body.rows(row_height, self.rows.len(), |mut row| {
                    let index = row.index();
                    match &mut self.rows[index] {
                        Row::Constant(company) => {
                            row_constant(&mut row, company);
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

fn multi_header(ui: &mut Ui, title: &str, cols: [&str; 2]) {
    ui.vertical_centered(|ui| {
        ui.strong(title);
        ui.add(egui::Separator::default().horizontal());
        ui.columns(2, |columns| {
            columns[0].vertical_centered(|ui| {
                ui.strong(cols[0]);
            });
            columns[1].vertical_centered(|ui| {
                ui.strong(cols[1]);
            });
        });
    });
}
