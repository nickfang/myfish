# myfish
My Financial Sheets App


## Diesel
Update src/schema.rs
`diesel migration generate name --diff-schema`
`diesel migration run`
if something needs to be changed.
`diesel migration generate name --diff-schema`
`diesel migration redo`