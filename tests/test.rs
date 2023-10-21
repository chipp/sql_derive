use sql_derive::FromSql;
use sql_derive::ToSql;

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(FromSql, ToSql, Debug, PartialEq)]
    enum Level {
        Low,
        Medium,
        High,
    }

    #[test]
    fn test_from_sql() {
        use rusqlite::types::{FromSql, ValueRef};

        assert_eq!(Level::column_result(ValueRef::Text(b"Low")), Ok(Level::Low));
        assert_eq!(
            Level::column_result(ValueRef::Text(b"Medium")),
            Ok(Level::Medium)
        );
        assert_eq!(
            Level::column_result(ValueRef::Text(b"High")),
            Ok(Level::High)
        );
    }

    #[test]
    fn test_to_sql() {
        use rusqlite::types::{ToSql, ToSqlOutput, ValueRef};

        assert_eq!(
            Level::Low.to_sql(),
            Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Low"))),
        );

        assert_eq!(
            Level::Medium.to_sql(),
            Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"Medium"))),
        );

        assert_eq!(
            Level::High.to_sql(),
            Ok(ToSqlOutput::Borrowed(ValueRef::Text(b"High"))),
        );
    }
}
