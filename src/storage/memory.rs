use crate::{KvError, Kvpair, Storage, Value};
use dashmap::{mapref::one::Ref, DashMap};

#[derive(Debug, Clone, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    // 创建默认的MemTable
    pub fn new() -> Self {
        Self::default()
    }
    // 当table存在则返回table，反之创建新的table
    pub fn get_or_create_table(&self, table: &str) -> Ref<String, DashMap<String, Value>> {
        let res = self.tables.get(table);
        match res {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(table.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let dashmap: Ref<'_, String, DashMap<String, Value>> = self.get_or_create_table(table);
        Ok(dashmap.get(key).map(|v| v.value().clone()))
    }

    fn set(&self, table: &str, key: &str, value: Value) -> Result<Option<Value>, KvError> {
        let dashmap: Ref<'_, String, DashMap<String, Value>> = self.get_or_create_table(table);
        Ok(dashmap.insert(key.into(), value))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let dashmap: Ref<'_, String, DashMap<String, Value>> = self.get_or_create_table(table);
        Ok(dashmap.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let dashmap: Ref<'_, String, DashMap<String, Value>> = self.get_or_create_table(table);
        Ok(dashmap.remove(key).map(|(k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let dashmap: Ref<'_, String, DashMap<String, Value>> = self.get_or_create_table(table);
        Ok(dashmap
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::KvError> {
        todo!()
    }
}
