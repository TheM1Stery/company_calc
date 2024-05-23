use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};
use serde::{Deserialize, Serialize};

use super::model::Company;
pub fn export_to_pdf() {
    todo!()
}

#[derive(Deserialize, Serialize)]
pub struct CompanyExcel {
    #[serde(rename = "Код")]
    id: i64,

    #[serde(rename = "Наименование")]
    name: String,

    #[serde(rename = "Начало-Дебет")]
    remainder_begin_month_debit: Option<f64>,

    #[serde(rename = "Начало-Кредит")]
    remainder_begin_month_credit: Option<f64>,

    #[serde(rename = "Оборот-Дебет")]
    debit_turnover: f64,

    #[serde(rename = "Оборот-Кредит")]
    credit_turnover: f64,

    #[serde(rename = "Конец-Дебет")]
    remainder_end_month_debit: Option<f64>,

    #[serde(rename = "Конец-Кредит")]
    remainder_end_month_credit: Option<f64>,
}

pub fn map_to_excel(company: &Company) -> CompanyExcel {
    CompanyExcel {
        id: company.id,
        name: company.name.to_owned(),
        remainder_begin_month_debit: if company.remainder_begin_month >= 0. {
            Some(company.remainder_begin_month)
        } else {
            None
        },
        remainder_begin_month_credit: if company.remainder_begin_month < 0. {
            Some(company.remainder_end_month)
        } else {
            None
        },
        debit_turnover: company.debit_turnover,
        credit_turnover: company.credit_turnover,
        remainder_end_month_debit: if company.remainder_end_month >= 0. {
            Some(company.remainder_end_month)
        } else {
            None
        },
        remainder_end_month_credit: if company.remainder_end_month < 0. {
            Some(company.remainder_end_month)
        } else {
            None
        },
    }
}

pub fn export_to_excel(rows: &[CompanyExcel]) -> Result<Vec<u8>, XlsxError> {
    let mut workbook = Workbook::new();
    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Add some formats to use with the serialization data.
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6E0B4");

    worksheet.deserialize_headers_with_format::<CompanyExcel>(0, 0, &header_format)?;

    worksheet.serialize(&rows)?;

    workbook.save_to_buffer()
}
