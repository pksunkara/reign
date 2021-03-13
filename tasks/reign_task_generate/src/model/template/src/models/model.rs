use reign::prelude::*;

#[derive(Debug, Model)]
pub struct {{name}} {
    #[model(no_write)]
    pub id: i32,{{#each fields}}
    pub {{name}}: {{{rust}}},{{/each}}
}
