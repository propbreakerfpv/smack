
/// rules: Vec<Rule>
#[derive(Debug)]
pub struct StyleSheet {
    pub rules: Vec<Rule>,
}

impl StyleSheet {
    /// # TODO: impl clone so we dont need &mut self
    pub fn display(&mut self) {
        for i in &self.rules {
            i._display();
        }
    }
}

fn is_some<T>(v: &Option<T>) -> bool {
    match v {
        Some(_) => true,
        None => false,
    }
}

/// selectors: Vec<Selector>,
/// declarations: Vec<Declaration>
#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

impl Rule {
    fn _display(&self) {
        for i in &self.selectors {
            match i {
                Selector::Simple(s) => {
                    if is_some(&s.tag_name) {
                        print!("{}", s.tag_name.as_ref().unwrap());
                    }
                    if is_some(&s.id) {
                        print!("{}", s.id.as_ref().unwrap());
                    }
                    if s.class.len() > 0 {
                        s.class.iter().for_each(|e|print!("{}", e));
                    }

                }
            }
        }
        println!(" {{");
        for i in &self.declarations {
            println!("  {:?}: {:?};", i.name, i.value);
        }
        println!("}}");
        // match &self. {
        //     NodeType::Text(t) => {
        //         println!("{}{:?}", indent, t);
        //     },
        //     NodeType::Element(e) => {
        //         println!("{}{:?} - {:?}", indent, e.tag_name, e.attributes.attrs);
        //     },
        //     NodeType::Comment(_) => (),
        // }
        // // println!("{}{:?}", indent, self.node_type);
        // loop {
        //     let mut node = match self.nodes.pop() {
        //         Some(v) => v,
        //         None => return,
        //     };
        //     node._display(indent.clone() + &String::from("  "));
        // }
    }
}


/// Simple(SimpleSelector),
#[derive(Debug)]
pub enum Selector {
    Simple(SimpleSelector),
}


/// pub tag_name: Option<String>,
/// pub id: Option<String>,
/// pub class: Vec<String>,
#[derive(Debug)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>,
}
impl SimpleSelector {
    pub fn new() -> SimpleSelector {
        SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(), 
        }
    }
    /// # get the specificity value of the selector.
    /// # note 
    /// this may not be within spec because the specificity value is stored in a usize. in the spec
    /// it may be u64 or smth. probubly not a isue but its a thing to consider.
    /// the only reson it could not be in spec is cuz idk where to find this info right now
    pub fn calc_specificity(&self) -> usize {
        let mut specificity: usize = 0;
        if is_some(&self.tag_name) {
            specificity += 1;
        }
        specificity += self.class.len() * 10;

        if is_some(&self.id) {
            specificity += 100;
        }
        specificity
    }
}



/// pub name: String,
/// pub value: Value,
#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
}


/// Keyward(String),
/// Length(f32, Unit),
/// ColorValue(Color),
#[derive(Debug)]
pub enum Value {
    Keyward(String),
    Length(f32, Unit),
    ColorValue(Color),
}

#[derive(Debug)]
pub enum Unit {
    Px,
}

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
