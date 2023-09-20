use std::{collections::BTreeMap, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TypeModel(pub BTreeMap<String, NamedType>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NamedType {
    Struct(BTreeMap<String, AnonymousType>),
    Enum(BTreeMap<String, AnonymousType>),
    Alias(AnonymousType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnonymousType {
    Option(Rc<AnonymousType>),
    Vec(Rc<AnonymousType>),
    Map(Rc<(AnonymousType, AnonymousType)>),
    Set(Rc<AnonymousType>),
    Reference(String),
    External(String),
    Void,
    Bool,
    Int,
    String,
}

impl TypeModel {
    pub fn upsert_type(&mut self, type_name: String, new_type: NamedType) {
        self.0.insert(type_name, new_type);
    }

    pub fn remove_type(&mut self, type_name: String) -> Result<(), anyhow::Error> {
        self.0
            .remove(&type_name)
            .map(|_| ())
            .ok_or_else(|| anyhow::anyhow!("Type not found: {}", type_name))
    }

    pub fn upsert_field(
        &mut self,
        type_name: &str,
        field_name: &str,
        new_type: AnonymousType,
    ) -> Result<(), anyhow::Error> {
        self.0
            .get_mut(type_name)
            .ok_or_else(|| anyhow::anyhow!("Type not found: {}", type_name))?
            .insert_field(field_name, new_type)
    }

    pub fn remove_field(&mut self, type_name: &str, field_name: &str) -> Result<(), anyhow::Error> {
        self.0
            .get_mut(type_name)
            .ok_or_else(|| anyhow::anyhow!("Type not found: {}", type_name))?
            .remove_field(field_name)
    }
}

impl NamedType {
    fn insert_field(
        &mut self,
        field_name: &str,
        new_type: AnonymousType,
    ) -> Result<(), anyhow::Error> {
        match self {
            Self::Struct(fields) => {
                fields.insert(field_name.to_string(), new_type);
                Ok(())
            }
            Self::Enum(fields) => {
                fields.insert(field_name.to_string(), new_type);
                Ok(())
            }
            Self::Alias(_) => Err(anyhow::anyhow!("Cannot insert into an alias")),
        }
    }

    fn remove_field(&mut self, field_name: &str) -> Result<(), anyhow::Error> {
        match self {
            Self::Struct(fields) => fields
                .remove(field_name)
                .map(|_| ())
                .ok_or_else(|| anyhow::anyhow!("Field not found: {}", field_name)),
            Self::Enum(fields) => fields
                .remove(field_name)
                .map(|_| ())
                .ok_or_else(|| anyhow::anyhow!("Field not found: {}", field_name)),
            Self::Alias(_) => Err(anyhow::anyhow!("Cannot remove from an alias")),
        }
    }
}
