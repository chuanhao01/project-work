// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use indradb::Vertex;
use serde_json::{json, Value};

fn main() -> indradb::Result<()> {
    // Create an in-memory datastore
    let db: indradb::Database<indradb::MemoryDatastore> = indradb::MemoryDatastore::new_db();

    // // Create a couple of vertices
    // let out_v = indradb::Vertex::new(indradb::Identifier::new("person")?);
    // let in_v = indradb::Vertex::new(indradb::Identifier::new("movie")?);
    // db.create_vertex(&out_v)?;
    // db.create_vertex(&in_v)?;

    // // Add an edge between the vertices
    // let edge = indradb::Edge::new(out_v.id, indradb::Identifier::new("likes")?, in_v.id);
    // db.create_edge(&edge)?;

    // // Query for the edge
    // let output: Vec<indradb::QueryOutputValue> =
    //     db.get(indradb::SpecificEdgeQuery::single(edge.clone()))?;
    // // Convenience function to extract out the edges from the query results
    // let e = indradb::util::extract_edges(output.clone()).unwrap();
    // println!("{:?}", output);
    // println!("{:?}", e);
    // assert_eq!(e.len(), 1);
    // assert_eq!(edge, e[0]);

    let node_idn = indradb::Identifier::new("node")?;
    let root = indradb::Vertex::new(node_idn);
    db.create_vertex(&root)?;
    db.bulk_insert(vec![indradb::BulkInsertItem::VertexProperty(
        root.id,
        indradb::Identifier::new("node_doc")?,
        indradb::Json::new(json!({"name": "root", "doc-id": Option::<String>::None})),
    )])?;

    // Loading in some testdata
    let node_struct: Value = json!({
        "school": {
            "nus": {
                "MA1521": Option::<String>::None,
                "CS1231S": Option::<String>::None,
            },
            "sp": Option::<String>::None,
        },
        "personal": {
            "project-work": Option::<String>::None,
        },
        "single": Option::<String>::None,
    });
    println!("{:?}", node_struct);
    fn make_nodes(node_struct: &Value) -> Vec<Vertex>{
        let node_idn = indradb::Identifier::new("node").unwrap();
        match node_struct {
            Value::Object(node_map) => {
                // check if last
                for (k, v) in node_map.iter() {
                    println!("{}, {:?}", k, v);
                    let ver = indradb::Vertex::new(node_idn);
                    match v{
                        Value::Null => {
                            // reached the end

                        }
                    }
                    return
                }
            }
            _ => {
                panic!("should not happen")
            }
        }
        // Should not hit this
        return Vec::new();
    }
    make_nodes(&node_struct);

    let o = db.get(indradb::AllVertexQuery)?;
    println!("{:?}", o);

    let o = db.get(indradb::PipePropertyQuery {
        inner: Box::new(indradb::Query::AllVertex),
        name: None,
    })?;
    println!("{:?}", o);

    Ok(())
}
