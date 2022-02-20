use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionCheck {
    ok: bool,
}

impl ConnectionCheck {
    pub fn ok(&self) -> bool {
        use serde::Deserialize;
        self.ok
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    slug: String,
    name: String,
    id: i64,
}

impl Operation {
    pub fn slug(&self) -> &str {
        &self.slug
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

pub type Operations = Vec<Operation>;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    name: String,
    color_name: String,
}

impl Tag {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color_name(&self) -> &str {
        &self.color_name
    }
}

pub type Tags = Vec<Tag>;
