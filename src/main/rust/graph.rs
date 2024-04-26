
use log::debug;
use serde::{Serialize,Deserialize};
use std::collections;
use std::fmt;
use std::sync;

#[derive(Clone,Debug,Deserialize,Serialize,PartialEq)]
pub enum AttributeValue
{
    BooleanLiteral (bool),
    BooleanMap (collections::HashMap<String,bool>),
    IntegerLiteral (i64),
    FloatLiteral (f64),
    StringLiteral (String),
    StringArray (Vec<String>),
    StringMap (collections::HashMap<String,String>),
    StringSet (collections::HashSet<String>)
}

impl AttributeValue
{
    pub fn get_bool_literal (&self)
    -> bool
    {
        if let AttributeValue::BooleanLiteral (val) = *self
        {
            val
        }
        else
        {
            unreachable! ("Must only be called with BooleanLiteral");
        }
    }

    pub fn get_integer_literal (&self)
    -> i64
    {
        if let AttributeValue::IntegerLiteral (val) = *self
        {
            val
        }
        else
        {
            unreachable! ("Must only be called with IntegerLiteral");
        }
    }

    pub fn get_string_literal (&self)
        -> &String
    {
        if let AttributeValue::StringLiteral (val) = self
        {
            val
        }
        else
        {
            unreachable! ("Must only be called with StringLiteral");
        }
    }

    pub fn get_string_map (&self, key: &str)
    -> Option<&String>
    {
        if let AttributeValue::StringMap (ref map) = *self
        {
            map.get (key)
        }
        else
        {
            unreachable! ("Must only be called with StringMap");
        }
    }

    pub fn get_boolean_map (&self, key: &str)
    -> Option<&bool>
    {
        if let AttributeValue::BooleanMap (ref map) = *self
        {
            map.get (key)
        }
        else
        {
            unreachable! ("Must only be called with BooleanMap");
        }
    }

    pub fn insert_string_map (&mut self, key: String, value: String)
    -> Option<String>
    {
        if let AttributeValue::StringMap (ref mut map) = *self
        {
            map.insert (key, value)
        }
        else
        {
            unreachable! ("Must only be called with StringMap");
        }
    }

    pub fn insert_boolean_map (&mut self, key: String, value: bool)
    -> Option<bool>
    {
        if let AttributeValue::BooleanMap (ref mut map) = *self
        {
            map.insert (key, value)
        }
        else
        {
            unreachable! ("Must only be called with BooleanMap");
        }
    }

    pub fn insert_string_set (&mut self, value: String)
    -> bool
    {
        if let AttributeValue::StringSet (ref mut hset) = *self
        {
            hset.insert (value)
        }
        else
        {
            unreachable! ("Must only be called with StringSet");
        }
    }
}

impl From<bool> for AttributeValue
{
    fn from (value: bool)
        -> Self
    {
        AttributeValue::BooleanLiteral (value)
    }
}

impl From<f64> for AttributeValue
{
    fn from (value: f64)
        -> Self
    {
        AttributeValue::FloatLiteral (value)
    }
}

impl From<i64> for AttributeValue
{
    fn from(value: i64)
        -> Self
    {
        AttributeValue::IntegerLiteral (value)
    }
}

impl From<String> for AttributeValue
{
    fn from (value: String)
        -> Self
    {
        AttributeValue::StringLiteral (value)
    }
}

impl From<&str> for AttributeValue
{
    fn from (value: &str)
        -> Self
    {
        AttributeValue::StringLiteral (value.into ())
    }
}

impl From<Vec<String>> for AttributeValue
{
    fn from (value: Vec<String>)
        -> Self
    {
        AttributeValue::StringArray (value)
    }
}

impl From<collections::HashMap<String,String>> for AttributeValue
{
    fn from (value: collections::HashMap<String,String>)
        -> Self
    {
        AttributeValue::StringMap (value)
    }
}

impl From<collections::HashSet<String>> for AttributeValue
{
    fn from (value: collections::HashSet<String>)
        -> Self
    {
        AttributeValue::StringSet (value)
    }
}

impl fmt::Display for AttributeValue
{
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>)
        -> fmt::Result
    {
        match self {
            AttributeValue::BooleanLiteral (boolean) => write!(f, "{}", boolean),
            AttributeValue::BooleanMap (boolean_map) => f.pad(&boolean_map.iter ().map (|(k,v)| format! ("{}:{}", k,v)).collect::<Vec<String>> ().join (",")),
            AttributeValue::FloatLiteral (float) => write!(f, "{}", float),
            AttributeValue::IntegerLiteral (integer) => write!(f, "{}", integer),
            AttributeValue::StringLiteral (string) => f.pad(string),
            AttributeValue::StringArray (string_vec) => f.pad(&Vec::from_iter (string_vec.iter ().cloned ()).join (",")),
            AttributeValue::StringMap (string_map) => f.pad(&string_map.iter ().map (|(k,v)| format! ("{}:{}", k,v)).collect::<Vec<String>> ().join (",")),
            AttributeValue::StringSet (string_set) => f.pad(&Vec::from_iter (string_set.iter ().cloned ()).join (","))
        }
    }
}

#[derive(Clone,Debug,Deserialize,Serialize,PartialEq)]
pub struct Graph
{
    name: String,
    vertices: collections::HashSet <usize>,
    #[serde(with="crate::sd::key_pair_usize")]
    edges: collections::HashMap <(usize,usize), i64>,
    #[serde(with="crate::sd::key_usize")]
    inbound: collections::HashMap <usize, collections::HashSet <usize>>,
    #[serde(with="crate::sd::key_usize")]
    outbound: collections::HashMap <usize, collections::HashSet <usize>>
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LabelledGraph
{
    vertex_id: sync::Arc::<sync::atomic::AtomicUsize>,
    #[serde(with="crate::sd::key_pair_usize")]
    edge_attrs: collections::HashMap<(usize,usize), collections::HashMap<String, AttributeValue>>,
    graph: Graph,
    #[serde(with="crate::sd::key_usize")]
    vertex_attrs: collections::HashMap<usize, collections::HashMap<String, AttributeValue>>,
    #[serde(with="crate::sd::key_usize")]
    vertex_label: collections::HashMap<usize, String>,
    vertex_lookup: collections::HashMap<String, usize>
}

#[derive(Clone,Debug,Deserialize,Serialize,PartialEq)]
pub struct UGraph
{
    name: String,
    vertices: collections::HashSet <usize>,
    #[serde(with="crate::sd::key_pair_usize")]
    edges: collections::HashMap <(usize,usize), i64>,
    #[serde(with="crate::sd::key_usize")]
    connected: collections::HashMap <usize, collections::HashSet <usize>>,
}

#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct LabelledUGraph
{
    vertex_id: sync::Arc::<sync::atomic::AtomicUsize>,
    #[serde(with="crate::sd::key_pair_usize")]
    edge_attrs: collections::HashMap<(usize,usize), collections::HashMap<String, AttributeValue>>,
    graph: UGraph,
    #[serde(with="crate::sd::key_usize")]
    vertex_attrs: collections::HashMap<usize, collections::HashMap<String, AttributeValue>>,
    #[serde(with="crate::sd::key_usize")]
    vertex_label: collections::HashMap<usize, String>,
    vertex_lookup: collections::HashMap<String, usize>
}

impl Graph
{
    pub fn new () -> Self
    {
        Self { name: String::from (""), vertices: collections::HashSet::<usize>::new (), edges: collections::HashMap::<(usize,usize), i64>::new (), inbound: collections::HashMap::<usize, collections::HashSet<usize>>::new (), outbound: collections::HashMap::<usize, collections::HashSet<usize>>::new () }
    }

    pub fn new_with_name (name: &str) -> Self
    {
        Self { name: name.to_string (), vertices: collections::HashSet::<usize>::new (), edges: collections::HashMap::<(usize,usize), i64>::new (), inbound: collections::HashMap::<usize, collections::HashSet<usize>>::new (), outbound: collections::HashMap::<usize, collections::HashSet<usize>>::new () }
    }

    pub fn add_edge_raw (&mut self, a: usize, b: usize, weight: i64)
        -> Result<(), crate::error::GraphError>
    {
        self.vertices.insert (a);
        self.vertices.insert (b);
        self.edges.insert ( (a, b), weight);
        self.inbound.entry ( b ).or_insert (collections::HashSet::<usize>::new ()).insert (a);
        self.outbound.entry ( a ).or_insert (collections::HashSet::<usize>::new ()).insert (b);
        Ok (())
    }

    pub fn add_vertex_raw (&mut self, a: usize)
        -> Result<(), crate::error::GraphError>
    {
        self.vertices.insert (a);
        Ok (())
    }

    pub fn has_edge_raw (&self, t: &(usize, usize))
        -> bool
    {
        self.edges.contains_key ( t )
    }

    pub fn inbound (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let mut r = collections::HashSet::<usize>::new ();
            if let Some (children) = self.inbound.get (a)
            {
                r.extend (children);
            }
            Ok (r)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn neighbours (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let mut r = collections::HashSet::<usize>::new ();
            if let Some (children) = self.inbound.get (a)
            {
                r.extend (children);
            }
            if let Some (parents) = self.outbound.get (a)
            {
                r.extend (parents);
            }
            Ok (r)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn outbound (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let mut r = collections::HashSet::<usize>::new ();
            if let Some (parents) = self.outbound.get (a)
            {
                r.extend (parents);
            }
            Ok (r)   
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn edges (&self)
        -> &collections::HashMap <(usize,usize), i64>
    {
        &self.edges
    }

    pub fn is_leaf (&self, a: &usize)
        -> Result<bool, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            if let Some (inbound_a) = self.inbound.get (a)
            {
                Ok (inbound_a.is_empty ())
            }
            else
            {
                Ok (true)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn is_root (&self, a: &usize)
        -> Result<bool, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            if let Some (outbound_a) = self.outbound.get (a)
            {
                Ok (outbound_a.is_empty ())
            }
            else
            {
                Ok (true)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn name (&self)
        -> String
    {
        self.name.clone ()
    }

    pub fn leaves (&self)
        -> collections::HashSet::<usize>
    {
        self.vertices.iter ().filter (|x| !self.inbound.contains_key (x)).copied ().collect::<collections::HashSet::<usize>> ()
    }

    pub fn remove_edge_raw (&mut self, a: &usize, b: &usize)
        -> Result<(), crate::error::GraphError>
    {
        //debug! ("remove_edge_raw: self: {:?}", self);
        //debug! ("remove_edge_raw: a: {} b: {}", a, b);
        if self.vertices.contains (a) && self.vertices.contains (b)
        {
            let mut errors = Vec::<String>::new ();
            if self.edges.contains_key ( &(*a ,*b) )
            {
                self.edges.remove ( &(*a ,*b) );
            }
            else
            {
                errors.push (format! ("remove_edge_raw: Could not find edge {} -> {}", a, b));
            }

            if let Some (inbound_b) = self.inbound.get_mut (b)
            {
                if inbound_b.contains (a)
                {
                    inbound_b.remove (a);
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: {} had no inbound edge from {}", b, a));
                }
            }
            else
            {
                errors.push (format! ("remove_edge_raw: {} has no inbound edges", b));
            }

            if let Some (outbound_a) = self.outbound.get_mut (a)
            {
                if outbound_a.contains (b)
                {
                    outbound_a.remove (b);
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: {} had no outbound edge to {}", a, b));
                }
            }
            else
            {
                errors.push (format! ("remove_edge_raw: {} has no outbound edges", a));
            }

            if errors.is_empty ()
            {
                Ok (())
            }
            else
            {
                Err (crate::error::GraphError::EdgeError (errors.join ("\n\t")))
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("remove_edge_raw: Failed to find all vertices {}:{} {}:{}", a, self.vertices.contains (a), b, self.vertices.contains (b))))
        }
    }

    pub fn remove_vertex_raw (&mut self, a: &usize)
        -> Result<(), crate::error::GraphError>
    {
        for v in self.outbound (a)?
        {
            //debug! ("rvr a -> v: {} -> {}", a, v);
            self.remove_edge_raw (a, &v)?;
        }
        for v in self.inbound (a)?
        {
            //debug! ("rvr v -> a: {} -> {}", v, a);
            self.remove_edge_raw (&v, a)?;
        }
        self.vertices.remove (a);
        Ok (())
    }

    pub fn roots (&self)
        -> collections::HashSet <usize>
    {
        self.vertices.iter ().filter (|x| !self.outbound.contains_key (x)).copied ().collect::<collections::HashSet::<usize>> ()
    }

    pub fn vertices (&self)
        -> &collections::HashSet <usize>
    {
        &self.vertices
    }
}

impl UGraph
{
    pub fn new () -> Self
    {
        Self { name: String::from (""), vertices: collections::HashSet::<usize>::new (), edges: collections::HashMap::<(usize,usize), i64>::new (), connected: collections::HashMap::<usize, collections::HashSet<usize>>::new () }
    }

    pub fn new_with_name (name: &str) -> Self
    {
        Self { name: name.to_string (), vertices: collections::HashSet::<usize>::new (), edges: collections::HashMap::<(usize,usize), i64>::new (), connected: collections::HashMap::<usize, collections::HashSet<usize>>::new () }
    }

    pub fn add_edge_raw (&mut self, a: usize, b: usize, weight: i64)
        -> Result<(), crate::error::GraphError>
    {
        self.vertices.insert (a);
        self.vertices.insert (b);
        if a < b
        {
            self.edges.insert ( (a, b), weight);
        }
        else
        {
            self.edges.insert ( (b, a), weight);
        }
        self.connected.entry ( b ).or_insert (collections::HashSet::<usize>::new ()).insert (a);
        self.connected.entry ( a ).or_insert (collections::HashSet::<usize>::new ()).insert (b);
        Ok (())
    }

    pub fn add_vertex_raw (&mut self, a: usize)
        -> Result<(), crate::error::GraphError>
    {
        self.vertices.insert (a);
        Ok (())
    }

    pub fn has_edge_raw (&self, t: &(usize, usize))
        -> bool
    {
        if t.0 < t.1
        {
            self.edges.contains_key ( t )
        }
        else
        {
            self.edges.contains_key ( &(t.1, t.0) )
        }
    }

    pub fn neighbours (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let mut r = collections::HashSet::<usize>::new ();
            if let Some (children) = self.connected.get (a)
            {
                r.extend (children);
            }
            Ok (r)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn edges (&self)
        -> &collections::HashMap <(usize,usize), i64>
    {
        &self.edges
    }

    pub fn name (&self)
        -> String
    {
        self.name.clone ()
    }

    pub fn remove_edge_raw (&mut self, a: &usize, b: &usize)
        -> Result<(), crate::error::GraphError>
    {
        //debug! ("remove_edge_raw: self: {:?}", self);
        //debug! ("remove_edge_raw: a: {} b: {}", a, b);
        if self.vertices.contains (a) && self.vertices.contains (b)
        {
            let mut errors = Vec::<String>::new ();
            if a < b
            {
                if self.edges.contains_key ( &(*a ,*b) )
                {
                    self.edges.remove ( &(*a ,*b) );
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: Could not find edge {} -> {}", a, b));
                }
            }
            else
            {
                if self.edges.contains_key ( &(*b ,*a) )
                {
                    self.edges.remove ( &(*b ,*a) );
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: Could not find edge {} -> {}", b, a));
                }
            }

            if let Some (connected_b) = self.connected.get_mut (b)
            {
                if connected_b.contains (a)
                {
                    connected_b.remove (a);
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: {} had no edge from {}", b, a));
                }
            }
            else
            {
                errors.push (format! ("remove_edge_raw: {} has no edges", b));
            }

            if let Some (connected_a) = self.connected.get_mut (a)
            {
                if connected_a.contains (b)
                {
                    connected_a.remove (b);
                }
                else
                {
                    errors.push (format! ("remove_edge_raw: {} had no edge to {}", a, b));
                }
            }
            else
            {
                errors.push (format! ("remove_edge_raw: {} has no edges", a));
            }

            if errors.is_empty ()
            {
                Ok (())
            }
            else
            {
                Err (crate::error::GraphError::EdgeError (errors.join ("\n\t")))
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("remove_edge_raw: Failed to find all vertices {}:{} {}:{}", a, self.vertices.contains (a), b, self.vertices.contains (b))))
        }
    }

    pub fn remove_vertex_raw (&mut self, a: &usize)
        -> Result<(), crate::error::GraphError>
    {
        let vertices = self.connected.get (a).ok_or (crate::error::GraphError::VertexError (format! ("Failed to find vertex {}", a)))?.clone ();
        for v in vertices
        {
            //debug! ("rvr a -> v: {} -> {}", a, v);
            self.remove_edge_raw (a, &v)?;
        }
        self.vertices.remove (a);
        Ok (())
    }

    pub fn vertices (&self)
        -> &collections::HashSet <usize>
    {
        &self.vertices
    }
}

impl LabelledGraph
{
    pub fn new () -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (1)),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: Graph::new (),
            vertex_attrs: collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (),
            vertex_label: collections::HashMap::<usize, String>::new (),
            vertex_lookup: collections::HashMap::<String, usize>::new ()
        }
    }

    pub fn new_with_name (name: &str) -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (1)),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: Graph::new_with_name (name),
            vertex_attrs: collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (),
            vertex_label: collections::HashMap::<usize, String>::new (),
            vertex_lookup: collections::HashMap::<String, usize>::new ()
        }
    }

    pub fn new_without_attributes (g: &LabelledGraph)
        -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (g.vertices ().iter ().max ().cloned ().unwrap_or (1))),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: g.graph ().clone (),
            vertex_attrs: g.graph ().vertices ().iter ().fold (collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (), |mut acc, item| {
                acc.insert (*item, collections::HashMap::<String, AttributeValue>::new ());
                acc
            }),
            vertex_label: g.vertex_label.clone (),
            vertex_lookup: g.vertex_lookup.clone ()
        }
    }

    pub fn add_edge (&mut self, a: String, b: String, attrs: Option<collections::HashMap::<String, AttributeValue>>)
        -> Result<(usize, usize), crate::error::GraphError>
    {
        debug! ("add {} to {}", a , b);
        if a == b
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertices must be distinct")))
        }
        else
        {
            // Check that we didn't already add the vertices
            let a_id = if self.vertex_lookup.contains_key (&a) { self.vertex_lookup[&a] } else { self.add_vertex (a, None)? };
            let b_id = if self.vertex_lookup.contains_key (&b) { self.vertex_lookup[&b] } else { self.add_vertex (b, None)? };
            self.graph.add_edge_raw (a_id, b_id, 0)?;
            if let Some (edge_attrs) = attrs
            {
                self.edge_attrs.insert ((a_id, b_id), edge_attrs);
            }
            else
            {
                self.edge_attrs.insert ((a_id, b_id), collections::HashMap::<String, AttributeValue>::new ());
            }
            Ok ((a_id, b_id))
        }
    }

    pub fn add_vertex (&mut self, a: String, attrs: Option<collections::HashMap::<String, AttributeValue>>)
        -> Result<usize, crate::error::GraphError>
    {
        let vertex_exists = self.vertex_lookup.contains_key (&a);
        let a_id = *self.vertex_lookup.entry (a.clone ()).or_insert_with (|| self.vertex_id.fetch_add (1, sync::atomic::Ordering::Relaxed));

        if ! vertex_exists
        {
            self.graph.add_vertex_raw (a_id)?;
            self.vertex_label.insert (a_id, a.clone ());
        }
        if let Some (vertex_attrs) = attrs
        {
            self.vertex_attrs.insert (a_id, vertex_attrs);
        }
        else
        {
            self.vertex_attrs.insert (a_id, collections::HashMap::<String, AttributeValue>::new ());
        }
        Ok (a_id)
    }

    pub fn graph (&self)
        -> &Graph
    {
        &self.graph
    }

    pub fn vertices (&self)
        -> &collections::HashSet <usize>
    {
        self.graph.vertices ()
    }

    pub fn vertex_attrs (&self, a: &str)
        -> Result<(usize, &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get ( &a_id ).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", *a_id, a)))
            {
                Ok (vertex_attrs) => {
                    Ok ( (*a_id, vertex_attrs ) )
                },
                Err (e) => Err (e)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex_attrs_mut (&mut self, a: &str)
        -> Result<(usize, &mut collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get_mut ( &a_id ).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", *a_id, a)))
            {
                Ok (vertex_attrs) => {
                    Ok ( (*a_id, vertex_attrs) )
                },
                Err (e) => Err (e)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex (&self, a: &str)
        -> Result<usize, crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            Ok (*a_id)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex_label (&self, a_id: &usize)
        -> Result<String, crate::error::GraphError>
    {
        self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))
    }

    pub fn vertex_labels (&self)
        -> collections::HashSet<String>
    {
        self.vertex_lookup.keys ().cloned ().collect ()
    }

    pub fn edges (&self)
        -> &collections::HashMap <(usize,usize), i64>
    {
        self.graph.edges ()
    }

    pub fn edge_attrs (&self, (a, b): &(String, String))
        -> Result<( (usize, usize), &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        match ( self.vertex_lookup.get (a), self.vertex_lookup.get (b) )
        {
            (Some (a_id), Some (b_id)) => {
                match self.edge_attrs.get ( &(*a_id, *b_id) ).ok_or (crate::error::GraphError::EdgeError (format! ("Could not find edge attributes for ({}:{},{}:{})", *a_id, a, *b_id, b)))
                {
                    Ok (edge_attrs) => {
                        Ok ( ( (*a_id, *b_id), edge_attrs ) )
                    },
                    Err (e) => Err (e)
                }
            },
            (Some (a_id), None) => Err (crate::error::GraphError::EdgeError (format! ("Found vertex a: {}:{}. Failed to find vertex b: {}", a_id, a, b))),
            (None, Some (b_id)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found vertex b: {}:{}", a, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
    }

    pub fn edge_attrs_mut (&mut self, (a, b): &(String, String))
        -> Result<( (usize, usize), &mut collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        match ( self.vertex_lookup.get (a), self.vertex_lookup.get (b) )
        {
            (Some (a_id), Some (b_id)) => {
                debug! ("found both vertex ids");
                match self.edge_attrs.get_mut ( &(*a_id, *b_id) ).ok_or (crate::error::GraphError::EdgeError (format! ("Could not find edge attributes for ({}:{},{}:{})", *a_id, a, *b_id, b)))
                {
                    Ok (edge_attrs) => {
                        Ok ( ( (*a_id, *b_id), edge_attrs ) )
                    },
                    Err (e) => Err (e)
                }
            },
            (Some (a_id), None) => Err (crate::error::GraphError::EdgeError (format! ("Found vertex a: {}:{}. Failed to find vertex b: {}", a_id, a, b))),
            (None, Some (b_id)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found vertex b: {}:{}", a, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
    }

    pub fn edge_label (&self, (a_id, b_id): &(usize, usize))
        -> Result<(String, String), crate::error::GraphError>
    {
        match ( self.vertex_label.get (&a_id), self.vertex_label.get (&b_id) )
        {
            (Some (a), Some (b)) => {
                if self.graph.has_edge_raw ( &(*a_id, *b_id) )
                {
                    Ok ( (a.clone (), b.clone ()) )
                }
                else
                {
                    Err (crate::error::GraphError::EdgeError (format! ("No edge found between {} and {}", a_id, b_id)))
                }
            },
            (Some (a), None) => Err (crate::error::GraphError::EdgeError (format! ("Found {} {}. Failed to find vertex b: {}", a_id, a, b_id))),
            (None, Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {} {}", a_id, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a_id, b_id)))
        }
    }

    pub fn edge_labels (&self)
        -> Result<collections::HashSet<(String, String)>, crate::error::GraphError>
    {
        self.graph ().edges ().keys ().map (|x| {
            match ( self.vertex_label.get (&x.0), self.vertex_label.get (&x.1) )
            {
                (Some (a), Some (b)) => Ok ( (a.clone (), b.clone ()) ),
                (Some (a), None) => Err (crate::error::GraphError::EdgeError (format! ("Found {} {}. Failed to find vertex b: {}", x.0, a, x.1))),
                (None, Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {} {}", x.0, x.1, b))),
                _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", x.0, x.1)))
            }
        }).collect::<Result<collections::HashSet<_>, crate::error::GraphError>> ()
    }

    pub fn has_edge (&self, (a, b): &(String, String))
        -> bool
    {
        self.vertex_lookup.contains_key (a) && self.vertex_lookup.contains_key (b) && self.graph.edges.contains_key ( &(self.vertex_lookup[a], self.vertex_lookup[b]) )
    }

    pub fn has_vertex (&self, a: &str)
        -> bool
    {
        self.vertex_lookup.contains_key (a)
    }

    pub fn relabel_vertex (&mut self, a_id: &usize, vertex_label_next: String)
        -> Result<(), crate::error::GraphError>
    {
        let vertex_label_prev = self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))?;

        if let Some (_) = self.vertex_lookup.get (&vertex_label_next)
        {
            Err (crate::error::GraphError::VertexError (format! ("vertex {} already in graph, duplicate nodes are forbidden", vertex_label_next)))
        }
        else
        {
            self.vertex_lookup.remove (&vertex_label_prev);
            self.vertex_lookup.insert (vertex_label_next.clone (), *a_id);
            self.vertex_label.insert (*a_id, vertex_label_next);
            Ok (())
        }
    }

    pub fn remove_vertex (&mut self, a_id: &usize)
        -> Result<(), crate::error::GraphError>
    {
        let vertex_label = self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))?;

        if self.graph.neighbours (a_id)?.is_empty ()
        {
            self.vertex_lookup.remove (&vertex_label);
            self.vertex_label.remove (a_id);
            self.vertex_attrs.remove (a_id);
            self.graph.remove_vertex_raw (a_id)?;
            Ok (())
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Cannot delete vertex {} with edges", a_id)))
        }
    }
}

impl PartialEq for LabelledGraph
    {
    fn eq(&self, other: &Self) -> bool {
        // TODO compare vertex labels
        self.graph == other.graph
    }
}

impl LabelledUGraph
{
    pub fn new () -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (1)),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: UGraph::new (),
            vertex_attrs: collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (),
            vertex_label: collections::HashMap::<usize, String>::new (),
            vertex_lookup: collections::HashMap::<String, usize>::new ()
        }
    }

    pub fn new_with_name (name: &str) -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (1)),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: UGraph::new_with_name (name),
            vertex_attrs: collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (),
            vertex_label: collections::HashMap::<usize, String>::new (),
            vertex_lookup: collections::HashMap::<String, usize>::new ()
        }
    }

    pub fn new_without_attributes (g: &LabelledUGraph)
        -> Self
    {
        Self {
            vertex_id: sync::Arc::new (sync::atomic::AtomicUsize::new (g.vertices ().iter ().max ().cloned ().unwrap_or (1))),
            edge_attrs: collections::HashMap::<(usize,usize), collections::HashMap::<String, AttributeValue>>::new (),
            graph: g.graph ().clone (),
            vertex_attrs: g.graph ().vertices ().iter ().fold (collections::HashMap::<usize, collections::HashMap::<String, AttributeValue>>::new (), |mut acc, item| {
                acc.insert (*item, collections::HashMap::<String, AttributeValue>::new ());
                acc
            }),
            vertex_label: g.vertex_label.clone (),
            vertex_lookup: g.vertex_lookup.clone ()
        }
    }

    pub fn add_edge (&mut self, a: String, b: String, attrs: Option<collections::HashMap::<String, AttributeValue>>)
        -> Result<(usize, usize), crate::error::GraphError>
    {
        debug! ("add {} to {}", a , b);
        if a == b
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertices must be distinct")))
        }
        else
        {
            // Check that we didn't already add the vertices
            let a_id = if self.vertex_lookup.contains_key (&a) { self.vertex_lookup[&a] } else { self.add_vertex (a, None)? };
            let b_id = if self.vertex_lookup.contains_key (&b) { self.vertex_lookup[&b] } else { self.add_vertex (b, None)? };
            self.graph.add_edge_raw (a_id, b_id, 0)?;
            if let Some (edge_attrs) = attrs
            {
                self.edge_attrs.insert ((a_id, b_id), edge_attrs);
            }
            else
            {
                self.edge_attrs.insert ((a_id, b_id), collections::HashMap::<String, AttributeValue>::new ());
            }
            if a_id < b_id
            {
                Ok ((a_id, b_id))
            }
            else
            {
                Ok ((b_id, a_id))
            }
        }
    }

    pub fn add_vertex (&mut self, a: String, attrs: Option<collections::HashMap::<String, AttributeValue>>)
        -> Result<usize, crate::error::GraphError>
    {
        let vertex_exists = self.vertex_lookup.contains_key (&a);
        let a_id = *self.vertex_lookup.entry (a.clone ()).or_insert_with (|| self.vertex_id.fetch_add (1, sync::atomic::Ordering::Relaxed));

        if ! vertex_exists
        {
            self.graph.add_vertex_raw (a_id)?;
            self.vertex_label.insert (a_id, a.clone ());
        }
        if let Some (vertex_attrs) = attrs
        {
            self.vertex_attrs.insert (a_id, vertex_attrs);
        }
        else
        {
            self.vertex_attrs.insert (a_id, collections::HashMap::<String, AttributeValue>::new ());
        }
        Ok (a_id)
    }

    pub fn graph (&self)
        -> &UGraph
    {
        &self.graph
    }

    pub fn vertices (&self)
        -> &collections::HashSet <usize>
    {
        self.graph.vertices ()
    }

    pub fn vertex_attrs (&self, a: &str)
        -> Result<(usize, &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get ( &a_id ).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", *a_id, a)))
            {
                Ok (vertex_attrs) => {
                    Ok ( (*a_id, vertex_attrs ) )
                },
                Err (e) => Err (e)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex_attrs_mut (&mut self, a: &str)
        -> Result<(usize, &mut collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get_mut ( &a_id ).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", *a_id, a)))
            {
                Ok (vertex_attrs) => {
                    Ok ( (*a_id, vertex_attrs) )
                },
                Err (e) => Err (e)
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex (&self, a: &str)
        -> Result<usize, crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            Ok (*a_id)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Failed to find vertex: {}", a)))
        }
    }

    pub fn vertex_label (&self, a_id: &usize)
        -> Result<String, crate::error::GraphError>
    {
        self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))
    }

    pub fn vertex_labels (&self)
        -> collections::HashSet<String>
    {
        self.vertex_lookup.keys ().cloned ().collect ()
    }

    pub fn edges (&self)
        -> &collections::HashMap <(usize,usize), i64>
    {
        self.graph.edges ()
    }

    pub fn edge_attrs (&self, (a, b): &(String, String))
        -> Result<( (usize, usize), &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        match ( self.vertex_lookup.get (a), self.vertex_lookup.get (b) )
        {
            (Some (a_id), Some (b_id)) => {
                let t = if a_id < b_id { (*a_id, *b_id) } else { (*b_id, *a_id) };
                match self.edge_attrs.get ( &t ).ok_or (crate::error::GraphError::EdgeError (format! ("Could not find edge attributes for ({}:{},{}:{})", *a_id, a, *b_id, b)))
                {
                    Ok (edge_attrs) => {
                        Ok ( (t, edge_attrs) )
                    },
                    Err (e) => Err (e)
                }
            },
            (Some (a_id), None) => Err (crate::error::GraphError::EdgeError (format! ("Found vertex a: {}:{}. Failed to find vertex b: {}", a_id, a, b))),
            (None, Some (b_id)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found vertex b: {}:{}", a, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
    }

    pub fn edge_label (&self, (a_id, b_id): &(usize, usize))
        -> Result<(String, String), crate::error::GraphError>
    {
        match ( self.vertex_label.get (&a_id), self.vertex_label.get (&b_id) )
        {
            (Some (a), Some (b)) => Ok ( (a.clone (), b.clone ()) ),
            (Some (a), None) => Err (crate::error::GraphError::EdgeError (format! ("Found {} {}. Failed to find vertex b: {}", a_id, a, b_id))),
            (None, Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {} {}", a_id, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a_id, b_id)))
        }
    }

    pub fn edge_labels (&self)
        -> Result<collections::HashSet<(String, String)>, crate::error::GraphError>
    {
        self.graph ().edges ().keys ().map (|x| {
            match ( self.vertex_label.get (&x.0), self.vertex_label.get (&x.0) )
            {
                (Some (a), Some (b)) => Ok ( (a.clone (), b.clone ()) ),
                (Some (a), None) => Err (crate::error::GraphError::EdgeError (format! ("Found {} {}. Failed to find vertex b: {}", x.0, a, x.1))),
                (None, Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {} {}", x.0, x.1, b))),
                _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", x.0, x.1)))
            }
        }).collect::<Result<collections::HashSet<_>, crate::error::GraphError>> ()
    }

    pub fn has_edge (&self, (a, b): &(String, String))
        -> bool
    {
        if self.vertex_lookup.contains_key (a) && self.vertex_lookup.contains_key (b)
        {
            let t = if self.vertex_lookup[a] < self.vertex_lookup[b] { (self.vertex_lookup[a], self.vertex_lookup[b]) } else { (self.vertex_lookup[b], self.vertex_lookup[a]) };
            self.graph.edges.contains_key ( &t )
        }
        else
        {
            false
        }
    }

    pub fn relabel_vertex (&mut self, a_id: &usize, vertex_label_next: String)
        -> Result<(), crate::error::GraphError>
    {
        let vertex_label_prev = self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))?;

        if let Some (_) = self.vertex_lookup.get (&vertex_label_next)
        {
            Err (crate::error::GraphError::VertexError (format! ("vertex {} already in graph, duplicate nodes are forbidden", vertex_label_next)))
        }
        else
        {
            self.vertex_lookup.remove (&vertex_label_prev);
            self.vertex_lookup.insert (vertex_label_next.clone (), *a_id);
            self.vertex_label.insert (*a_id, vertex_label_next);
            Ok (())
        }
    }

    pub fn remove_vertex (&mut self, a_id: &usize)
        -> Result<(), crate::error::GraphError>
    {
        let vertex_label = self.vertex_label.get (a_id).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", a_id)))?;

        if self.graph.neighbours (a_id)?.is_empty ()
        {
            self.vertex_lookup.remove (&vertex_label);
            self.vertex_label.remove (a_id);
            self.vertex_attrs.remove (a_id);
            self.graph.remove_vertex_raw (a_id)?;
            Ok (())
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Cannot delete vertex {} with edges", a_id)))
        }
    }
}

impl PartialEq for LabelledUGraph
    {
    fn eq(&self, other: &Self) -> bool {
        // TODO compare vertex labels
        self.graph == other.graph
    }
}

pub trait GraphAny
{
    fn neighbours (&self, a: &usize) -> Result<collections::HashSet<usize>, crate::error::GraphError>;
    fn vertices (&self) -> &collections::HashSet <usize>;
}

impl GraphAny for Graph
{
    fn neighbours (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        self.neighbours (a)
    }

    fn vertices (&self)
        -> &collections::HashSet<usize>
    {
        self.vertices ()
    }
}

impl GraphAny for UGraph
{
    fn neighbours (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        self.neighbours (a)
    }

    fn vertices (&self)
        -> &collections::HashSet<usize>
    {
        self.vertices ()
    }
}

#[cfg(test)]
mod tests
{
    use serde_json;
    use std::collections;
    use super::*;

    use std::sync;

    static INIT: sync::Once = sync::Once::new ();

    fn init ()
    {
        INIT.call_once (env_logger::init);
    }

    #[test]
    fn test_leaves_and_roots ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (1,0,0).expect ("Failed to add edge 1 -> 0");
        g.add_edge_raw (6,5,0).expect ("Failed to add edge 6 -> 5");
        g.add_edge_raw (7,5,0).expect ("Failed to add edge 7 -> 5");
        g.add_edge_raw (5,4,0).expect ("Failed to add edge 5 -> 4");

        let expected_leaves = collections::HashSet::<usize>::from ([3,2,7,6]);
        let expected_roots = collections::HashSet::<usize>::from ([0,4]);

        assert_eq! (g.leaves (), expected_leaves, "Failed to obtain correct leaves");
        assert_eq! (g.roots (), expected_roots, "Failed to obtain correct roots");

        let expected_is_leaf = vec![false,false,true,true,false,false,true,true];
        let expected_is_root = vec![true,false,false,false,true,false,false,false];

        assert_eq! (g.vertices (), &(0..8).collect::<collections::HashSet<usize>> (), "Failed to obtain correct vertex ids");
        assert_eq! ((0..8).map (|x| g.is_leaf (&x).expect ("Failed to call is_leaf")).collect::<Vec<_>> (), expected_is_leaf, "Failed is_leaf");
        assert_eq! ((0..8).map (|x| g.is_root (&x).expect ("Failed to call is_root")).collect::<Vec<_>> (), expected_is_root, "Failed is_root");
    }

    #[test]
    fn test_remove ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (1,0,0).expect ("Failed to add edge 1 -> 0");

        let edges_before = collections::HashMap::<(usize, usize), i64>::from ([
            ( (2,1), 0 ),
            ( (3,1), 0 ),
            ( (1,0), 0 )
        ]);

        assert_eq! (g.edges (), &edges_before);

        let r = g.remove_vertex_raw (&1);

        assert! (r.is_ok (), "remove_vertex_raw failed: {:?}", r);
        assert_eq! (g.edges (), &collections::HashMap::<(usize, usize), i64>::new ());
        assert_eq! (g.vertices (), &collections::HashSet::<usize>::from ([0,3,2]));
        assert_eq! (g.inbound (&1).unwrap_err ().to_string (), "Vertex error: Vertex: 1 not found in graph");
        assert_eq! (g.outbound (&1).unwrap_err ().to_string (), "Vertex error: Vertex: 1 not found in graph");
    }

    #[test]
    fn test_remove_u ()
    {
        init ();
        let mut g = UGraph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -- 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -- 1");
        g.add_edge_raw (1,0,0).expect ("Failed to add edge 1 -- 0");

        let edges_before = collections::HashMap::<(usize, usize), i64>::from ([
            ( (1,2), 0 ),
            ( (1,3), 0 ),
            ( (0,1), 0 )
        ]);

        assert_eq! (g.edges (), &edges_before);

        let r = g.remove_vertex_raw (&1);

        assert! (r.is_ok (), "remove_vertex_raw failed: {:?}", r);
        assert_eq! (g.edges (), &collections::HashMap::<(usize, usize), i64>::new ());
        assert_eq! (g.vertices (), &collections::HashSet::<usize>::from ([0,3,2]));
        assert_eq! (g.neighbours (&1).unwrap_err ().to_string (), "Vertex error: Vertex: 1 not found in graph");

    }

    #[test]
    fn test_undirected_edges_u ()
    {
        init ();
        let mut g = UGraph::new ();

        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -- 2");
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -- 1");
        assert_eq! (g.edges (), &collections::HashMap::<(usize, usize), i64>::from ([ ( (1,2), 0 )]));
    }

    #[test]
    fn test_has_edge_raw ()
    {
        init ();
        let mut g = Graph::new ();

        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -- 2");

        assert! (g.has_edge_raw ( &(1,2) ), "has_edge_raw 1 -> 2 failed");
        assert! (!g.has_edge_raw ( &(2,1) ), "has_edge_raw 2 -> 1 succeeded");
    }

    #[test]
    fn test_has_edge_raw_u ()
    {
        init ();
        let mut g = UGraph::new ();

        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -- 2");

        assert! (g.has_edge_raw ( &(1,2) ), "has_edge_raw 1 -> 2 failed");
        assert! (g.has_edge_raw ( &(2,1) ), "has_edge_raw 2 -> 1 failed");
    }

    #[test]
    fn test_edge_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        g.add_edge (String::from ("a"), String::from ("b"), None).expect ("Failed to add edge a -> b");

        let va = g.vertex ("a").expect ("Failed to get vertex id for a");
        let vb = g.vertex ("b").expect ("Failed to get vertex id for b");

        let el = g.edge_label ( &(va, vb) ).expect ("Failed to get edge_labels for a -> b");

        assert! (g.has_edge (&el));
        assert_eq! (el, (String::from ("a"), String::from ("b")));
        assert_eq! (g.edge_label ( &(vb, va) ).unwrap_err ().to_string (), "Edge error: No edge found between 2 and 1");
    }

    #[test]
    fn test_edge_labelled_u ()
    {
        init ();
        let mut g = LabelledUGraph::new ();
        g.add_edge (String::from ("a"), String::from ("b"), None).expect ("Failed to add edge a -> b");

        let va = g.vertex ("a").expect ("Failed to get vertex id for a");
        let vb = g.vertex ("b").expect ("Failed to get vertex id for b");

        let elf = g.edge_label ( &(va, vb) ).expect ("Failed to get edge_labels for a -> b");
        let elr = g.edge_label ( &(vb, va) ).expect ("Failed to get edge_labels for b -> a");


        assert_eq! (elf, (String::from ("a"), String::from ("b")));
        assert_eq! (elr, (String::from ("b"), String::from ("a")));
    }

    #[test]
    fn test_serde_json ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (1,0,0).expect ("Failed to add edge 1 -> 0");

        let v = serde_json::to_value (g.clone ()).expect ("Failed to serialize Graph");
        let h = serde_json::from_value (v).expect ("Failed to deserialize Graph");

        assert_eq! (g,h);
    }

    #[test]
    fn test_serde_json_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        g.add_edge (String::from ("c"), String::from ("b"), None).expect ("Failed to add edge c -> b");
        g.add_edge (String::from ("d"), String::from ("b"), None).expect ("Failed to add edge d -> b");
        g.add_edge (String::from ("b"), String::from ("a"), None).expect ("Failed to add edge b -> a");

        let v = serde_json::to_value (g.clone ()).expect ("Failed to serialize LabelledGraph");
        let h = serde_json::from_value (v).expect ("Failed to deserialize LabelledGraph");

        assert_eq! (g,h);
    }

    #[test]
    fn test_serde_json_labelled_attributes ()
    {
        init ();

        let bool_map = collections::HashMap::<String, bool>::from ([ (String::from ("false"), false), (String::from ("true"), true) ]);
        let string_vec = vec![String::from ("one"), String::from ("two")];
        let string_map = collections::HashMap::<String, String>::from ([ (String::from ("foo"), String::from ("bar")) ]);
        let string_set = collections::HashSet::<String>::from ([String::from ("foo"), String::from ("bar") ]);
        let attrs = collections::HashMap::<String, AttributeValue>::from ([
            (String::from ("bool"), AttributeValue::BooleanLiteral (true)),
            (String::from ("bool_map"), AttributeValue::BooleanMap (bool_map)),
            (String::from ("integer"), AttributeValue::IntegerLiteral (0)),
            (String::from ("float"), AttributeValue::FloatLiteral (0.0)),
            (String::from ("string_array"), AttributeValue::StringArray (string_vec)),
            (String::from ("string"), AttributeValue::StringLiteral (String::from ("string"))),
            (String::from ("string_map"), AttributeValue::StringMap (string_map)),
            (String::from ("string_set"), AttributeValue::StringSet (string_set))
        ]);

        let mut g = LabelledGraph::new ();
        g.add_vertex (String::from ("a"), Some (attrs.clone ())).expect ("Failed to add vertex_attrs for a");
        g.add_edge (String::from ("b"), String::from ("a"), Some (attrs)).expect ("Failed to add edge b -> a");

        let v = serde_json::to_value (g.clone ()).expect ("Failed to serialize LabelledGraph");
        let h = serde_json::from_value (v).expect ("Failed to deserialize LabelledGraph");

        assert_eq! (g,h);
    }
}
