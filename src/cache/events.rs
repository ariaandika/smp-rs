use rusqlite::Connection;

use crate::app::events::Events;

use super::Cache;

#[derive(Debug, Clone)]
pub struct EventCache(Cache<Vec<Events>>);

impl std::ops::Deref for EventCache {
    type Target = Cache<Vec<Events>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl EventCache {
    pub fn setup(conn: &mut Connection) -> rusqlite::Result<Self> {
        let cache = Self(Cache::new(vec![]));
        cache.invalidate(conn)?;
        Ok(cache)
    }

    pub fn invalidate(&self, conn: &Connection) -> rusqlite::Result<()> {
        let mut stmt = conn.prepare_cached("select rowid,* from events")?;
        let mut rows = stmt.query([])?;
        let mut data = vec![];

        while let Some(row) = rows.next()? {
            data.push(Events {
                rowid: row.get("rowid")?,
                title: row.get("title")?,
                body: row.get("body")?,
                image: row.get("image")?,
            });
        }

        {
            let mut write = self.0.write();
            *write = data;
        }

        Ok(())
    }
}


