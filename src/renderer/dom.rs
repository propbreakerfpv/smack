use std::collections::{HashMap, HashSet};


/// nodes: Nodes,
/// node_type: NodeType,
#[derive(Debug)]
pub struct Node {
    nodes: Nodes,
    node_type: NodeType,
}

impl Node {
    pub fn from_text(text: String) -> Node {
        Node {
            nodes: Nodes::new(),
            node_type: NodeType::Text(text),
        }
    }
    pub fn from_elem(name: String, attrs: AttrMap, nodes: Nodes) -> Node {
        Node {
            nodes,
            node_type: NodeType::Element(ElementData {
                tag_name: (name),
                attributes: (attrs),
            }),
        }
    }

    pub fn from_comment(text: String) -> Node {
        Node {
            nodes: Nodes::new(),
            node_type: NodeType::Comment(text),
        }
    }

    pub fn display(&self) {
        let mut node = self.clone();
        node.nodes = node.nodes.rev();
        node._display(String::from(""));
    }
    fn _display(&mut self, indent: String) {
        // pub enum NodeType {
        //     Text(String),
        //     Element(ElementData),
        //     Comment(String),
        // }
        match &self.node_type {
            NodeType::Text(t) => {
                println!("{}{:?}", indent, t);
            },
            NodeType::Element(e) => {
                println!("{}{:?} - {:?}", indent, e.tag_name, e.attributes.attrs);
            },
            NodeType::Comment(_) => (),
        }
        // println!("{}{:?}", indent, self.node_type);
        loop {
            let mut node = match self.nodes.pop() {
                Some(v) => v,
                None => return,
            };
            node._display(indent.clone() + &String::from("  "));
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            nodes: self.nodes.clone(),
            node_type: self.node_type.clone()
        }
    }
}


#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.attrs.get("id")
    }
    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.attrs.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

impl Clone for ElementData {
    fn clone(&self) -> Self {
        ElementData {
            tag_name: self.tag_name.clone(),
            attributes: self.attributes.clone()
        }
    }
}

#[derive(Debug)]
pub struct AttrMap {
    pub attrs: HashMap<String, String>
}

impl Clone for AttrMap {
    fn clone(&self) -> Self {
        AttrMap { attrs: self.attrs.clone() }
    }
}

impl AttrMap {
    pub fn new() -> AttrMap {
        AttrMap { attrs: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct Nodes {
    pub nodes: Vec<Node>
}

impl Nodes {
    pub fn new() -> Nodes {
        Nodes{nodes: Vec::new()}
    }
    pub fn pop(&mut self) -> Option<Node> {
        self.nodes.pop()
    }
    pub fn push(&mut self, item: Node) {
        self.nodes.push(item);
    }

    pub fn rev(&mut self) -> Nodes {
        let mut new = Nodes::new();
        for i in self.nodes.iter().rev() {
            new.push(i.clone());
        }
        new
    }
}

impl Clone for Nodes {
    fn clone(&self) -> Self {
        Nodes {
            nodes: self.nodes.clone()
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

impl Clone for NodeType {
    fn clone(&self) -> Self {
        match self {
            NodeType::Text(a) => NodeType::Text(a.clone()),
            NodeType::Element(a) => NodeType::Element(a.clone()),
            NodeType::Comment(a) => NodeType::Comment(a.clone())
        }
    }
}


