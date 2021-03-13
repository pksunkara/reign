CREATE TABLE {{name}} (
  id SERIAL PRIMARY KEY{{#each fields}},
  {{name}} {{sql}}{{#if not_null}} NOT NULL{{/if}}{{/each}}
);
