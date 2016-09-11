use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader};
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};
use petgraph::graph::Graph;
use super::graph::{Node, Edge};

fn find_attribute(attributes: &Vec<OwnedAttribute>, name: &str) -> Option<String> {
    for attribute in attributes {
        if attribute.name.local_name == name {
            return Some(attribute.value.clone())
        }
    }
    None
}

pub struct GraphMLParser {
    parser: EventReader<BufReader<File>>,
}

impl GraphMLParser {
    pub fn from_filename(filename: &str) -> GraphMLParser {
        let file = File::open(filename).unwrap();
        let buffer = BufReader::new(file);
        GraphMLParser { parser: EventReader::new(buffer) }
    }

    pub fn parse(&mut self) -> Graph<Node, Edge> {
        let mut graph = Graph::<Node, Edge>::new();
        let mut node_ids = HashMap::new();
        while let Ok(e) = self.parser.next() {
            match e {
                XmlEvent::StartElement { name, attributes, .. } => {
                    match name.local_name.as_ref() {
                        "node" => {
                            let node = self.parse_node(&attributes);
                            let id = node.id.clone();
                            let idx = graph.add_node(node);
                            node_ids.insert(id, idx);
                        }
                        "edge" => {
                            let edge = self.parse_edge(&attributes);
                            let source_id = find_attribute(&attributes, "source").unwrap();
                            let target_id = find_attribute(&attributes, "target").unwrap();
                            let source_idx = node_ids.get(&source_id).unwrap();
                            let target_idx = node_ids.get(&target_id).unwrap();
                            graph.add_edge(source_idx.clone(), target_idx.clone(), edge);
                        }
                        _ => {}
                    }
                }
                XmlEvent::EndDocument => {
                    break;
                }
                _ => {}
            }
        }
        graph
    }

    fn parse_node(&mut self, attributes: &Vec<OwnedAttribute>) -> Node {
        let id = find_attribute(&attributes, "id").unwrap();
        let mut values = HashMap::new();
        while let Ok(e) = self.parser.next() {
            match e {
                XmlEvent::StartElement { name, attributes, .. } => {
                    if name.local_name == "data" {
                        let (key, value) = self.parse_data(&attributes);
                        values.insert(key, value);
                    }
                }
                XmlEvent::EndElement { name, .. } => {
                    if name.local_name == "node" {
                        break;
                    }
                }
                _ => {}
            }
        }
        Node {
            id: id,
            degree: 0,
        }
    }

    fn parse_edge(&mut self, attributes: &Vec<OwnedAttribute>) -> Edge {
        Edge {}
    }

    fn parse_data(&mut self, attributes: &Vec<OwnedAttribute>) -> (String, String) {
        let key = find_attribute(&attributes, "key").unwrap();
        let value = match self.parser.next().ok().unwrap() {
            XmlEvent::Characters(value) => {
                value
            }
            _ => {
                String::from("")
            }
        };
        (key, value)
    }
}
