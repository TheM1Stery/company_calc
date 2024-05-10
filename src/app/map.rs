use super::{model::NewCompany, NewCompanyRow};

pub fn map_to_new(
    NewCompanyRow {
        name,
        remainder_begin_month_pos,
        remainder_begin_month_neg,
        debit_turnover,
        credit_turnover,
    }: &NewCompanyRow,
) -> Result<NewCompany, Box<dyn std::error::Error>> {
    // need to get rid of allocation, this is bad
    let remainder = match (
        remainder_begin_month_pos.as_str(),
        remainder_begin_month_neg.as_str(),
    ) {
        (first, "") => first.to_string(),
        ("", second) => format!("-{second}"),
        _ => String::new(),
    };
    let parsed_remainder: f64 = remainder.parse()?;

    println!("{parsed_remainder}");

    let parsed_debit: f64 = debit_turnover.parse()?;

    let parsed_credit: f64 = credit_turnover.parse()?;

    Ok(NewCompany {
        name: name.to_string(),
        remainder_begin_month: parsed_remainder,
        debit_turnover: parsed_debit,
        credit_turnover: parsed_credit,
    })
}
