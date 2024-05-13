use sqlx::SqlitePool;

use super::model::{Company, NewCompany};

pub enum Operation {
    Add { new_companies: Vec<Company> },
    Edit { edited_company: Company },
    FetchAll {all_companies: Result<Vec<Company>, sqlx::Error>}
}

enum RemainderType {
    Debit,
    Credit,
}

pub async fn add_company(
    db: SqlitePool,
    NewCompany {
        name,
        remainder_begin_month,
        debit_turnover,
        credit_turnover,
    }: NewCompany,
) -> Result<Company, sqlx::Error> {
    let remainder_type = match remainder_begin_month {
        v if v > 0. => RemainderType::Debit,
        v if v < 0. => RemainderType::Credit,
        _ => RemainderType::Debit,
    };

    let remainder = match remainder_type {
        RemainderType::Debit => remainder_begin_month - credit_turnover,
        RemainderType::Credit => remainder_begin_month + debit_turnover,
    };

    let result = sqlx::query_as!(Company, r#"INSERT INTO company (name, remainder_begin_month, debit_turnover, credit_turnover, remainder_end_month)
                    VALUES (?, ?, ?, ?, ?)
                    RETURNING id, name, remainder_begin_month, debit_turnover, credit_turnover, remainder_end_month"#,
                    name, remainder_begin_month, debit_turnover, credit_turnover, remainder)
        .fetch_one(&db)
        .await?;

    Ok(result)
}


pub async fn get_all_companies(db: SqlitePool) -> Result<Vec<Company>, sqlx::Error>{
    let result = sqlx::query_as!(Company,"SELECT * FROM company")
        .fetch_all(&db)
        .await?;

    Ok(result)
}
