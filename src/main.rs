extern crate fdlayout;

use fdlayout::graphml::GraphMLParser;
use fdlayout::layout::layout;

fn main() {
    let mut parser = GraphMLParser::from_filename("graph.xml");
    let graph = parser.parse();
    layout(graph);
}
