use crate::model::response::Page;
use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sql_types::BigInt;
use diesel::MysqlConnection;

pub type PooledConn = PooledConnection<ConnectionManager<MysqlConnection>>;

pub trait Paginate: AsQuery + Sized {
    fn page(self, page: i64) -> Paginated<Self::Query>;
}

impl<T: AsQuery> Paginate for T {
    fn page(self, page: i64) -> Paginated<Self::Query> {
        Paginated {
            query: self.as_query(),
            limit: DEFAULT_LIMIT,
            page,
        }
    }
}

const DEFAULT_LIMIT: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    limit: i64,
    page: i64,
}

impl<T> Paginated<T> {
    pub fn limit(self, limit: i64) -> Self {
        Paginated { limit, ..self }
    }

    pub fn paginate<U>(self, conn: &PooledConn) -> QueryResult<Page<U>>
    where
        Self: LoadQuery<MysqlConnection, (U, i64)>,
    {
        let page = self.page;
        let limit = self.limit;
        let result = self.load::<(U, i64)>(conn)?;
        let total = result.get(0).map(|x| x.1).unwrap_or(0);
        let record = result.into_iter().map(|x| x.0).collect();
        let offset = limit * page;
        Ok(Page::build(record, limit, offset, total))
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<MysqlConnection> for Paginated<T> {}

impl<T> QueryFragment<Mysql> for Paginated<T>
where
    T: QueryFragment<Mysql>,
{
    fn walk_ast(&self, mut out: AstPass<Mysql>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.limit)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.limit;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}
