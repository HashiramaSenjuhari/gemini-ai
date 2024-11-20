pub fn schema_store(schema: &[String]) -> String {
    let mut schemaa = String::new();
    for schema in schema {
        let scheaaa = format!("{},\r\n", schema);
        schemaa.push_str(&scheaaa);
    }
    let schema = schemaa.trim_end_matches(",\r\n");
    let schema = format!("{{{}}}", schema);
    schema.to_string()
    // let response = format!("{{{},\r\n{},\r\n{}}}", value, property, propertyy);
}
