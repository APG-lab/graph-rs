
//use log::debug;
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

    pub fn set_bool_literal (&mut self, val: bool)
    {
        if let AttributeValue::BooleanLiteral (ref mut wrapped_val) = *self
        {
            *wrapped_val = val;
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

pub trait MaybeAttributeValue
{
    fn maybe_string_literal (&self) -> Result<Option<String>, crate::error::GraphError>;
}

impl MaybeAttributeValue for Option<AttributeValue>
{
    fn maybe_string_literal (&self) -> Result<Option<String>, crate::error::GraphError>
    {
        match self
        {
            None => Ok (None),
            Some (AttributeValue::StringLiteral (val)) => Ok (Some (val.to_string ())),
            Some (_) => Err (crate::error::GraphError::DataError (String::from ("Not a StringLiteral")))
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
    pub (crate) name: String,
    pub (crate) vertices: collections::HashSet <usize>,
    #[serde(with="crate::sd::key_pair_usize")]
    pub (crate) edges: collections::HashMap <(usize,usize), i64>,
    #[serde(with="crate::sd::key_usize")]
    pub (crate) connected: collections::HashMap <usize, collections::HashSet <usize>>,
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

    pub fn has_vertex_raw (&self, a: &usize)
        -> bool
    {
        self.vertices.contains (a)
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

    pub fn incident (&self, a: &usize)
        -> Result<collections::HashSet<(usize, usize)>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let mut r = collections::HashSet::<(usize,usize)>::new ();
            if let Some (children) = self.inbound.get (a)
            {
                r.extend (children.iter ().map (|&x| (x, *a)));
            }
            if let Some (parents) = self.outbound.get (a)
            {
                r.extend (parents.iter ().map (|&x| (*a, x)));
            }
            Ok (r)
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
    }

    pub fn vertex_identification (&mut self, m: &collections::BTreeSet<usize>)
        -> Result<(), crate::error::GraphError>
    {
        let mut it = m.iter ();
        if let Some (tv) = it.next ()
        {
            while let Some (mv) = it.next ()
            {
                for mvi in self.inbound (mv)?
                {
                    let me = (mvi, *mv);
                    let te = (mvi, *tv);
                    let w = self.edges.get (&me)
                        .copied ()
                        .ok_or (crate::error::GraphError::DataError (format! ("Failed to find inbound merging edge: {:?}", me)))?;
                    self.remove_edge_raw (&me.0, &me.1)?;
                    if !self.has_edge_raw (&te)
                    {
                        self.add_edge_raw (te.0, te.1, w)?;
                    }
                }

                for mvo in self.outbound (mv)?
                {
                    let me = (*mv, mvo);
                    let te = (*tv, mvo);
                    let w = self.edges.get (&me)
                        .copied ()
                        .ok_or (crate::error::GraphError::DataError (format! ("Failed to find outbound merging edge: {:?}", me)))?;
                    self.remove_edge_raw (&me.0, &me.1)?;
                    if !self.has_edge_raw (&te)
                    {
                        self.add_edge_raw (te.0, te.1, w)?;
                    }
                }

                self.remove_vertex_raw (mv)?;
            }
            Ok (())
        }
        else
        {
            Err (crate::error::GraphError::DataError (String::from ("No vertices to merge")))
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

    pub fn endpoints (&self)
        -> collections::HashSet<usize>
    {
        let mut r = collections::HashSet::<usize>::with_capacity (self.vertices.len ());
        r.extend (self.inbound.keys ().copied ());
        r.extend (self.outbound.keys ().copied ());
        r
    }

    pub fn edges (&self)
        -> &collections::HashMap <(usize,usize), i64>
    {
        &self.edges
    }

    pub fn is_source (&self, a: &usize)
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

    pub fn is_sink (&self, a: &usize)
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

    pub fn sources (&self)
        -> collections::HashSet::<usize>
    {
        self.vertices.iter ().filter (|x| !self.inbound.contains_key (x)).copied ().collect::<collections::HashSet::<usize>> ()
    }

    pub fn parent (&self, a: &usize)
        -> Result<Option<usize>, crate::error::GraphError>
    {
        if self.vertices.contains (a)
        {
            let outbound = self.outbound (&a)?;
            match outbound.len ()
            {
                0 | 1 => Ok (outbound.into_iter ().next ()),
                _ => Err (crate::error::GraphError::VertexError (format! ("Vertex: {} has more than one parent", a)))
            }
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Vertex: {} not found in graph", a)))
        }
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
                if inbound_b.is_empty ()
                {
                    self.inbound.remove (b);
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
                if outbound_a.is_empty ()
                {
                    self.outbound.remove (a);
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
        self.inbound.remove (a);
        self.outbound.remove (a);
        Ok (())
    }

    pub fn rename (&mut self, name: String)
    {
        self.name = name;
    }

    pub fn sinks (&self)
        -> collections::HashSet <usize>
    {
        self.vertices.iter ().filter (|x| !self.outbound.contains_key (x)).copied ().collect::<collections::HashSet::<usize>> ()
    }

    pub fn remap (&mut self, map: &collections::HashMap<usize, usize>)
        -> Result<(), crate::error::GraphError>
    {
        let ks = map.keys ().copied ().collect::<collections::HashSet<_>> ();
        if ks.is_subset (&self.vertices)
        {
            let vs = map.values ().copied ().collect::<collections::HashSet<_>> ();
            if ks.len () == vs.len ()
            {
                let rvs = vs.intersection (&self.vertices).copied ().collect::<collections::HashSet<_>> ();
                if rvs.is_subset (&ks)
                {
                    let edges_new = self.edges.drain ()
                        .map (|(k,v)| {
                            (
                                (
                                    map.get (&k.0).copied ().unwrap_or (k.0),
                                    map.get (&k.1).copied ().unwrap_or (k.1),
                                )
                                , v
                            )
                        }).collect::<Vec<_>> ();
                    self.edges.extend (edges_new);
                    let inbound_new = self.inbound.drain ()
                        .map (|(k,v)| {
                            (
                                map.get (&k).copied ().unwrap_or (k),
                                v.iter ().map (|x| map.get (x).copied ().unwrap_or (*x) ).collect::<collections::HashSet::<_>> ()
                            )
                        })
                        .collect::<Vec<_>> ();
                    self.inbound.extend (inbound_new);
                    let outbound_new = self.outbound.drain ()
                        .map (|(k,v)| {
                            (
                                map.get (&k).copied ().unwrap_or (k),
                                v.iter ().map (|x| map.get (x).copied ().unwrap_or (*x) ).collect::<collections::HashSet::<_>> ()
                            )
                        })
                        .collect::<Vec<_>> ();
                    self.outbound.extend (outbound_new);
                    let vertices_new = self.vertices.drain ()
                        .map (|x| map.get (&x).copied ().unwrap_or (x))
                        .collect::<Vec<_>> ();
                    self.vertices.extend (vertices_new);
                    Ok (())
                }
                else
                {
                    Err (crate::error::GraphError::DataError (String::from ("Both directions are required when swapping existing vertices")))
                }
            }
            else
            {
                Err (crate::error::GraphError::DataError (String::from ("Every key must have a unique value")))
            }
        }
        else
        {
            Err (crate::error::GraphError::DataError (String::from ("Remap keys must be a subset of graph vertices")))
        }
    }

    pub fn retain (&mut self, vertices_retain: &collections::HashSet<usize>)
        -> Result<(), crate::error::GraphError>
    {
        for vd in &self.vertices - vertices_retain
        {
            self.remove_vertex_raw (&vd)?;
        }
        Ok (())
    }

    pub fn retain_edges (&mut self, edges_retain: &collections::HashSet<(usize,usize)>)
        -> Result<(), crate::error::GraphError>
    {
        for e in &self.edges.keys ().cloned ().collect::<collections::HashSet<_>> () - edges_retain
        {
            self.remove_edge_raw (&e.0, &e.1)?;
        }
        Ok (())
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
        self.connected.remove (a);
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
        self.add_edge_weighted (a, b, attrs, 0)
    }

    pub fn add_edge_weighted (&mut self, a: String, b: String, attrs: Option<collections::HashMap::<String, AttributeValue>>, weight: i64)
        -> Result<(usize, usize), crate::error::GraphError>
    {
        //debug! ("add {} to {}", a , b);
        if a == b
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertices must be distinct")))
        }
        else
        {
            // Check that we didn't already add the vertices
            // create b before a for nice vertex ids
            let b_id = if self.vertex_lookup.contains_key (&b) { self.vertex_lookup[&b] } else { self.add_vertex (b, None)? };
            let a_id = if self.vertex_lookup.contains_key (&a) { self.vertex_lookup[&a] } else { self.add_vertex (a, None)? };
            self.graph.add_edge_raw (a_id, b_id, weight)?;
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

    pub fn add_edge_raw (&mut self, a_id: usize, a: String, b_id: usize, b: String, attrs: Option<collections::HashMap::<String, AttributeValue>>, weight: i64)
        -> Result<(usize, usize), crate::error::GraphError>
    {
        if a == b
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertices must be distinct")))
        }
        else if a_id == b_id
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertex ids must be distinct")))
        }
        else
        {
            self.add_vertex_raw (a_id, a, None)?;
            self.add_vertex_raw (b_id, b, None)?;
            self.graph.add_edge_raw (a_id, b_id, weight)?;
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

    pub fn add_vertex_raw (&mut self, a_id: usize, a: String, attrs: Option<collections::HashMap::<String, AttributeValue>>)
        -> Result<usize, crate::error::GraphError>
    {
        // If the vertices already exist, check the the ids are consistent
        if let Some (a_id_e) = self.vertex_lookup.get (&a)
        {
            if *a_id_e == a_id
            {
                if let Some (vertex_attrs) = attrs
                {
                    self.vertex_attrs.insert (a_id, vertex_attrs);
                }
                else
                {
                    self.vertex_attrs.insert (a_id, collections::HashMap::<String, AttributeValue>::new ());
                }

                Ok (*a_id_e)
            }
            else
            {
                Err (crate::error::GraphError::VertexError (String::from ("vertex already exists but has a different id")))
            }
        }
        else
        {
                if let Some (a_e) = self.vertex_label.get (&a_id)
                {
                    if *a_e == a
                    {
                        unreachable! ("Recovered vertex label for source id but it should have been found in the vertex lookup");
                    }
                    else
                    {
                        Err (crate::error::GraphError::VertexError (String::from ("vertex id already exists but has a different label")))
                    }
                }
                else
                {
                    self.vertex_id.fetch_max (a_id + 1, sync::atomic::Ordering::Relaxed);
                    self.vertex_lookup.insert (a.clone (), a_id);
                    self.graph.add_vertex_raw (a_id)?;
                    self.vertex_label.insert (a_id, a.clone ());
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
        }
    }

    pub fn edge (&self, (a, b): &(String, String))
        -> Result<(usize, usize), crate::error::GraphError>
    {
        match ( self.vertex_lookup.get (a), self.vertex_lookup.get (b) )
        {
            (Some (a_id), Some (b_id)) => {
                if self.edge_attrs.contains_key ( &(*a_id, *b_id) )
                {
                    Ok ( (*a_id, *b_id) )
                }
                else
                {
                    Err (crate::error::GraphError::EdgeError (format! ("Could not find edge attributes for ({}:{},{}:{})", *a_id, a, *b_id, b)))
                }
            },
            (Some (a_id), None) => Err (crate::error::GraphError::EdgeError (format! ("Found vertex a: {}:{}. Failed to find vertex b: {}", a_id, a, b))),
            (None, Some (b_id)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found vertex b: {}:{}", a, b_id, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
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

    pub fn vertex_attrs (&self, a: &str)
        -> Result<(usize, &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", a_id, a)))
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
            match self.vertex_attrs.get_mut (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", a_id, a)))
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

    pub fn vertex_attrs_raw (&self, a_id: &usize)
        -> Result<&collections::HashMap::<String, AttributeValue>, crate::error::GraphError>
    {
        self.vertex_attrs.get (a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}", a_id)))
    }

    pub fn vertex_attrs_raw_mut (&mut self, a_id: &usize)
        -> Result<&mut collections::HashMap::<String, AttributeValue>, crate::error::GraphError>
    {
        self.vertex_attrs.get_mut (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}", a_id)))
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

    pub fn edge_attrs_raw (&self, ev @ (a_id, b_id): &(usize, usize))
        -> Result<&collections::HashMap::<String, AttributeValue>, crate::error::GraphError>
    {
        if let Some (ea) = self.edge_attrs.get (ev)
        {
            Ok (ea)
        }
        else
        {
            match ( self.vertex_label.get (a_id), self.vertex_label.get (b_id) )
            {
                (Some (a), Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("Found both vertices {}:{} {}:{} but no edge", a_id, a, b_id, b))),
                (Some (a), None) => Err (crate::error::GraphError::EdgeError (format! ("Found vertex a: {}:{}. Failed to find vertex b: {}", a_id, a, b_id))),
                (None, Some (b)) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found vertex b: {}:{}", a_id, b_id, b))),
                _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a_id, b_id)))
            }
        }
    }

    pub fn edge_attrs_mut (&mut self, (a, b): &(String, String))
        -> Result<( (usize, usize), &mut collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        match ( self.vertex_lookup.get (a), self.vertex_lookup.get (b) )
        {
            (Some (a_id), Some (b_id)) => {
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

    pub fn remap_raw (&mut self, map: &collections::HashMap<usize, usize>)
        -> Result<(), crate::error::GraphError>
    {
        self.graph.remap (map)?;
        let edge_attrs_new = self.edge_attrs.drain ()
            .map (|(k,v)| {
                (
                    (
                        map.get (&k.0).copied ().unwrap_or (k.0),
                        map.get (&k.1).copied ().unwrap_or (k.1),
                    ),
                    v
                )
            })
            .collect::<Vec<_>> ();
        self.edge_attrs.extend (edge_attrs_new);
        let vertex_attrs_new = self.vertex_attrs.drain ()
            .map (|(k,v)| {
                (
                    map.get (&k).copied ().unwrap_or (k),
                    v
                )
            })
            .collect::<Vec<_>> ();
        self.vertex_attrs.extend (vertex_attrs_new);
        let vertex_label_new = self.vertex_label.drain ()
            .map (|(k,v)| {
                (
                    map.get (&k).copied ().unwrap_or (k),
                    v
                )
            })
            .collect::<Vec<_>> ();
        self.vertex_label.extend (vertex_label_new);
        for val in self.vertex_lookup.values_mut ()
        {
            *val = map.get (val).copied ().unwrap_or (*val);
        }
        Ok (())
    }

    pub fn remove_edge (&mut self, (a, b): &(String, String))
        -> Result<(), crate::error::GraphError>
    {
        match ( self.has_vertex (&a), self.has_vertex (&b) )
        {
            (true, true) => {
                let er = ( self.vertex (&a)?, self.vertex (&b)? );
                self.edge_attrs.remove (&er);
                self.graph.remove_edge_raw (&er.0, &er.1)?;
                Ok (())
            },
            (true, false) => Err (crate::error::GraphError::EdgeError (format! ("Found {}. Failed to find vertex b: {}", a, b))),
            (false, true) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {}", a, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
    }

    pub fn remove_vertex (&mut self, a: &str)
        -> Result<(), crate::error::GraphError>
    {
        let a_id = self.vertex (a)?;
        if self.graph.neighbours (&a_id)?.is_empty ()
        {
            self.vertex_lookup.remove (a);
            self.vertex_label.remove (&a_id);
            self.vertex_attrs.remove (&a_id);
            self.graph.remove_vertex_raw (&a_id)?;
            Ok (())
        }
        else
        {
            Err (crate::error::GraphError::VertexError (format! ("Cannot delete vertex {} with edges", a)))
        }
    }

    pub fn rename (&mut self, name: String)
    {
        self.graph.rename (name);
    }

    pub fn retain (&mut self, vertices_retain: &collections::HashSet<usize>)
        -> Result<(), crate::error::GraphError>
    {
        for vd in self.graph.vertices () - vertices_retain
        {
            let vdl = self.vertex_label.get (&vd).cloned ().ok_or (crate::error::GraphError::VertexError (format! ("vertex {} not found in graph", vd)))?;
            self.vertex_lookup.remove (&vdl);
            self.vertex_label.remove (&vd);
            self.vertex_attrs.remove (&vd);

            for vdi in self.graph.inbound (&vd)?
            {
                self.edge_attrs.remove ( &(vdi, vd) );
            }
            for vdo in self.graph.outbound (&vd)?
            {
                self.edge_attrs.remove ( &(vd, vdo) );
            }
        }
        self.graph.retain (vertices_retain)
    }

    pub fn retain_edges (&mut self, edges_retain: &collections::HashSet<(usize,usize)>)
        -> Result<(), crate::error::GraphError>
    {
        for e in &self.graph.edges.keys ().cloned ().collect::<collections::HashSet<_>> () - &edges_retain
        {
            self.edge_attrs.remove (&e);
        }

        self.graph.retain_edges (edges_retain)
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
        self.add_edge_weighted (a, b, attrs, 0)
    }

    pub fn add_edge_weighted (&mut self, a: String, b: String, attrs: Option<collections::HashMap::<String, AttributeValue>>, weight: i64)
        -> Result<(usize, usize), crate::error::GraphError>
    {
        //debug! ("add {} to {}", a , b);
        if a == b
        {
            Err (crate::error::GraphError::EdgeError (String::from ("edge vertices must be distinct")))
        }
        else
        {
            // Check that we didn't already add the vertices
            let a_id = if self.vertex_lookup.contains_key (&a) { self.vertex_lookup[&a] } else { self.add_vertex (a, None)? };
            let b_id = if self.vertex_lookup.contains_key (&b) { self.vertex_lookup[&b] } else { self.add_vertex (b, None)? };
            self.graph.add_edge_raw (a_id, b_id, weight)?;
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

    pub fn vertex_attrs (&self, a: &str)
        -> Result<(usize, &collections::HashMap::<String, AttributeValue>), crate::error::GraphError>
    {
        if let Some (a_id) = self.vertex_lookup.get (a)
        {
            match self.vertex_attrs.get (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", a_id, a)))
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
            match self.vertex_attrs.get_mut (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}:{}", a_id, a)))
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

    pub fn vertex_attrs_raw (&self, a_id: &usize)
        -> Result<&collections::HashMap::<String, AttributeValue>, crate::error::GraphError>
    {
        self.vertex_attrs.get (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}", a_id)))
    }

    pub fn vertex_attrs_raw_mut (&mut self, a_id: &usize)
        -> Result<&mut collections::HashMap::<String, AttributeValue>, crate::error::GraphError>
    {
        self.vertex_attrs.get_mut (&a_id).ok_or (crate::error::GraphError::VertexError (format! ("Could not find vertex attributes for {}", a_id)))
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

    pub fn remove_edge (&mut self, (a, b): &(String, String))
        -> Result<(), crate::error::GraphError>
    {
        match ( self.has_vertex (&a), self.has_vertex (&b) )
        {
            (true, true) => {
                let er = ( self.vertex (&a)?, self.vertex (&b)? );
                self.edge_attrs.remove (&er);
                self.graph.remove_edge_raw (&er.0, &er.1)?;
                Ok (())
            },
            (true, false) => Err (crate::error::GraphError::EdgeError (format! ("Found {}. Failed to find vertex b: {}", a, b))),
            (false, true) => Err (crate::error::GraphError::EdgeError (format! ("failed to find vertex a: {}. Found {}", a, b))),
            _ => Err (crate::error::GraphError::EdgeError (format! ("failed to find both vertices: {} {}", a, b)))
        }
    }

    pub fn remove_vertex (&mut self, a: &str)
        -> Result<(), crate::error::GraphError>
    {
        let a_id = self.vertex (a)?;

        if self.graph.neighbours (&a_id)?.is_empty ()
        {
            self.vertex_lookup.remove (a);
            self.vertex_label.remove (&a_id);
            self.vertex_attrs.remove (&a_id);
            self.graph.remove_vertex_raw (&a_id)?;
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
    fn adjacent (&self, a: &usize) -> Result<collections::HashSet<usize>, crate::error::GraphError>;
    fn has_edge_raw (&self, ev: &(usize, usize)) -> bool;
    fn neighbours (&self, a: &usize) -> Result<collections::HashSet<usize>, crate::error::GraphError>;
    fn vertices (&self) -> &collections::HashSet <usize>;
    fn weight (&self, ev: &(usize, usize)) -> Result<i64, crate::error::GraphError>;
}

impl GraphAny for Graph
{
    fn adjacent (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        self.outbound (a)
    }

    fn has_edge_raw (&self, ev: &(usize, usize))
        -> bool
    {
        self.has_edge_raw (ev)
    }

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

    fn weight (&self, ev: &(usize, usize))
        -> Result<i64, crate::error::GraphError>
    {
        self.edges ().get (ev).copied ().ok_or (crate::error::GraphError::EdgeError (format! ("No edge found for {:?}", ev)))
    }
}

impl GraphAny for UGraph
{
    fn adjacent (&self, a: &usize)
        -> Result<collections::HashSet<usize>, crate::error::GraphError>
    {
        self.neighbours (a)
    }

    fn has_edge_raw (&self, ev: &(usize, usize))
        -> bool
    {
        self.has_edge_raw (ev)
    }

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

    fn weight (&self, ev: &(usize, usize))
        -> Result<i64, crate::error::GraphError>
    {
        let t = if ev.0 < ev.1 { (ev.0, ev.1) } else { (ev.1, ev.0) };
        self.edges ().get (&t).copied ().ok_or (crate::error::GraphError::EdgeError (format! ("No edge found for {:?}", ev)))
    }
}

#[cfg(test)]
mod tests
{
    //use log::debug;
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
    fn test_add_vertex_raw_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        let a_id = g.add_vertex (String::from ("a"), None).expect ("Failed to add vertex 'a'");

        assert_eq! (g.add_vertex_raw (a_id, String::from ("b"), None).unwrap_err ().to_string (), "Vertex error: vertex id already exists but has a different label");
        assert_eq! (g.add_vertex_raw (a_id + 10, String::from ("a"), None).unwrap_err ().to_string (), "Vertex error: vertex already exists but has a different id");

        let b_id = g.add_vertex_raw (a_id + 10, String::from ("b"), None).expect ("Failed to add_vertex_raw");

        assert_eq! (b_id, a_id + 10);
        assert! (g.has_vertex ("b"));
        assert_eq! (g.vertex_label (&b_id).expect ("Failed to get vertex label"), String::from ("b"));

        let c_id = g.add_vertex (String::from ("c"), None).expect ("Failed to add vertex 'c'");
        assert_eq! (c_id, b_id + 1);
    }

    #[test]
    fn test_vertex_identification ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");


        g.vertex_identification (&collections::BTreeSet::from ([2,3])).expect ("Merge failed");
        let expected_edges = collections::HashMap::<(usize, usize), i64>::from ([
            ( (2,1), 0 )
        ]);

        assert_eq! (g.vertices (), &(1..3).collect::<collections::HashSet<usize>> (), "Failed to obtain correct vertex ids");
        assert_eq! (g.edges (), &expected_edges);
    }

    #[test]
    fn test_vertex_identification_multi ()
    {
        init ();
        let mut ga = Graph::new ();
        ga.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        ga.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        ga.add_edge_raw (4,1,0).expect ("Failed to add edge 4 -> 1");

        let mut gb = ga.clone ();
        ga.vertex_identification (&collections::BTreeSet::from ([2,3,4])).expect ("Merge a failed");
        gb.vertex_identification (&collections::BTreeSet::from ([2,4,3])).expect ("Merge b failed");
        let expected_edges = collections::HashMap::<(usize, usize), i64>::from ([
            ( (2,1), 0 )
        ]);

        assert_eq! (ga.vertices (), &(1..3).collect::<collections::HashSet<usize>> (), "Failed to obtain correct vertex ids a");
        assert_eq! (ga.edges (), &expected_edges, "Unexpected edges b");
        assert_eq! (gb.vertices (), &(1..3).collect::<collections::HashSet<usize>> (), "Failed to obtain correct vertex ids a");
        assert_eq! (gb.edges (), &expected_edges, "Unexpected edges b");
    }

    #[test]
    fn test_vertex_identification_inbound ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (4,2,0).expect ("Failed to add edge 4 -> 2");
        g.add_edge_raw (4,3,0).expect ("Failed to add edge 4 -> 3");

        g.vertex_identification (&collections::BTreeSet::from ([2,3])).expect ("Merge failed");
        let expected_edges = collections::HashMap::<(usize, usize), i64>::from ([
            ( (2,1), 0 ),
            ( (4,2), 0 )
        ]);

        assert_eq! (g.vertices (), &collections::HashSet::<usize>::from ([1,2,4]), "Failed to obtain correct vertex ids");
        assert_eq! (g.edges (), &expected_edges);
    }


    #[test]
    fn test_sources_and_sinks ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (1,0,0).expect ("Failed to add edge 1 -> 0");
        g.add_edge_raw (6,5,0).expect ("Failed to add edge 6 -> 5");
        g.add_edge_raw (7,5,0).expect ("Failed to add edge 7 -> 5");
        g.add_edge_raw (5,4,0).expect ("Failed to add edge 5 -> 4");

        let expected_sources = collections::HashSet::<usize>::from ([3,2,7,6]);
        let expected_sinks = collections::HashSet::<usize>::from ([0,4]);

        assert_eq! (g.sources (), expected_sources, "Failed to obtain correct sources");
        assert_eq! (g.sinks (), expected_sinks, "Failed to obtain correct sinks");

        let expected_is_source = vec![false,false,true,true,false,false,true,true];
        let expected_is_sink = vec![true,false,false,false,true,false,false,false];

        assert_eq! (g.vertices (), &(0..8).collect::<collections::HashSet<usize>> (), "Failed to obtain correct vertex ids");
        assert_eq! ((0..8).map (|x| g.is_source (&x).expect ("Failed to call is_source")).collect::<Vec<_>> (), expected_is_source, "Failed is_source");
        assert_eq! ((0..8).map (|x| g.is_sink (&x).expect ("Failed to call is_sink")).collect::<Vec<_>> (), expected_is_sink, "Failed is_sink");
    }

    #[test]
    fn test_parent ()
    {
        init ();
        let mut g = Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (4,2,0).expect ("Failed to add edge 4 -> 2");
        g.add_edge_raw (4,3,0).expect ("Failed to add edge 4 -> 3");

        assert_eq! (g.parent (&0).unwrap_err ().to_string (), "Vertex error: Vertex: 0 not found in graph");

        assert_eq! (g.parent (&1).expect ("Failed to obtain maybe parent for vertex 1"), None, "root should not have a parent");
        assert_eq! (g.parent (&2).expect ("Failed to obtain maybe parent for vertex 2"), Some (1), "2 should have parent 1");
        assert_eq! (g.parent (&3).expect ("Failed to obtain maybe parent for vertex 3"), Some (1), "3 should have parent 1");

        assert_eq! (g.parent (&4).unwrap_err ().to_string (), "Vertex error: Vertex: 4 has more than one parent");
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
    fn test_has_vertex_raw ()
    {
        init ();
        let mut g = Graph::new ();

        let r_before = g.has_vertex_raw (&2);
        g.add_vertex_raw (2).expect ("Failed to add vertex");
        let r_after = g.has_vertex_raw (&2);

        assert_eq! (r_before, false, "Should not have vertex yet");
        assert_eq! (r_after, true, "Should have vertex");
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
        g.add_edge (String::from ("b"), String::from ("a"), None).expect ("Failed to add edge b -> a");

        let va = g.vertex ("a").expect ("Failed to get vertex id for a");
        let vb = g.vertex ("b").expect ("Failed to get vertex id for b");

        let el = g.edge_label ( &(vb, va) ).expect ("Failed to get edge_labels for b -> a");

        assert! (g.has_edge (&el));
        assert_eq! (el, (String::from ("b"), String::from ("a")));
        assert_eq! (g.edge_label ( &(va, vb) ).unwrap_err ().to_string (), "Edge error: No edge found between 1 and 2");
    }

    #[test]
    fn test_edge_labelled_raw ()
    {
        init ();
        let mut g = LabelledGraph::new ();

        assert_eq! (g.add_edge_raw (1, String::from ("a"), 0, String::from ("a"), None, 0).unwrap_err ().to_string (), "Edge error: edge vertices must be distinct");
        assert_eq! (g.add_edge_raw (0, String::from ("b"), 0, String::from ("a"), None, 0).unwrap_err ().to_string (), "Edge error: edge vertex ids must be distinct");

        g.add_edge_raw (1, String::from ("b"), 0, String::from ("a"), None, 0).expect ("Failed to add edge b -> a");

        let va = g.vertex ("a").expect ("Failed to get vertex id for a");
        let vb = g.vertex ("b").expect ("Failed to get vertex id for b");

        let el = g.edge_label ( &(vb, va) ).expect ("Failed to get edge_labels for b -> a");

        assert! (g.has_edge (&el));
        assert! (g.add_edge_raw (1, String::from ("b"), 0, String::from ("a"), None, 0).is_ok ());

        assert_eq! (g.add_edge_raw (2, String::from ("b"), 0, String::from ("a"), None, 0).unwrap_err ().to_string (), "Vertex error: vertex already exists but has a different id");
        assert_eq! (g.add_edge_raw (1, String::from ("c"), 0, String::from ("a"), None, 0).unwrap_err ().to_string (), "Vertex error: vertex id already exists but has a different label");
        assert_eq! (g.add_edge_raw (1, String::from ("b"), 2, String::from ("a"), None, 0).unwrap_err ().to_string (), "Vertex error: vertex already exists but has a different id");
        assert_eq! (g.add_edge_raw (1, String::from ("b"), 0, String::from ("c"), None, 0).unwrap_err ().to_string (), "Vertex error: vertex id already exists but has a different label");
    }

    #[test]
    fn test_edge_labelled_u ()
    {
        init ();
        let mut g = LabelledUGraph::new ();
        g.add_edge (String::from ("b"), String::from ("a"), None).expect ("Failed to add edge b -> a");

        let va = g.vertex ("a").expect ("Failed to get vertex id for a");
        let vb = g.vertex ("b").expect ("Failed to get vertex id for b");

        let elf = g.edge_label ( &(vb, va) ).expect ("Failed to get edge_labels for b -> a");
        let elr = g.edge_label ( &(va, vb) ).expect ("Failed to get edge_labels for a -> b");

        assert_eq! (elf, (String::from ("b"), String::from ("a")));
        assert_eq! (elr, (String::from ("a"), String::from ("b")));
    }

    #[test]
    fn test_remove_edge_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        g.add_edge (String::from ("a"), String::from ("b"), None).expect ("Failed to add edge a -> b");
        let e = ( String::from ("a"), String::from ("b") );
        g.remove_edge (&e).expect ("Failed to remove edge");
    }

    #[test]
    fn test_remove_edge_labelled_u ()
    {
        init ();
        let mut g = LabelledUGraph::new ();
        g.add_edge (String::from ("a"), String::from ("b"), None).expect ("Failed to add edge a -> b");
        let e = ( String::from ("a"), String::from ("b") );
        g.remove_edge (&e).expect ("Failed to remove edge");
    }

    #[test]
    fn test_endpoints ()
    {
        init ();
        let mut g = Graph::new ();
        //   1
        //   |
        //   2
        //  / \
        // 3   4
        g.add_edge_raw (2, 1, 0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3, 2, 0).expect ("Failed to add edge 3 -> 2");
        g.add_edge_raw (4, 2, 0).expect ("Failed to add edge 4 -> 2");

        assert_eq! (g.vertices (), &(1..5).collect::<collections::HashSet<_>> ());
        assert_eq! (g.endpoints (), collections::HashSet::<usize>::from ([1,2,3,4]), "endpoints not equal");

        let mut rg = g.clone ();
        rg.remove_vertex_raw (&2).expect ("Failed to remove vertex 2");

        assert! (rg.endpoints ().is_empty (), "endpoints should be empty");
    }

    #[test]
    fn test_remap ()
    {
        init ();
        let mut g = Graph::new ();
        // 1
        // |
        // *
        // 2
        g.add_edge_raw (1, 2, 0).expect ("Failed to add edge 1 -> 2");
        let m = collections::HashMap::<usize,usize>::from ([
            ( 1, 2 ),
            ( 2, 1 ),
        ]);
        g.remap (&m).expect ("Failed to remap");

        let expected_edges = collections::HashMap::<(usize, usize), i64>::from ([
            ( ( 2, 1 ), 0 )
        ]);
        assert_eq! (g.edges (), &expected_edges, "Edges should be remapped");
    }

    #[test]
    fn test_remap_partial ()
    {
        init ();
        let mut g = Graph::new ();
        //    1
        //   / \
        //  *   *
        // 2     3
        g.add_edge_raw (1, 2, 0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1, 3, 1).expect ("Failed to add edge 1 -> 3");
        let m = collections::HashMap::<usize,usize>::from ([
            ( 1, 2 ),
            ( 2, 1 ),
        ]);
        g.remap (&m).expect ("Failed to remap");

        let expected_edges = collections::HashMap::<(usize, usize), i64>::from ([
            ( ( 2, 1 ), 0 ),
            ( ( 2, 3 ), 1 ),
        ]);
        let expected_vertices = collections::HashSet::<usize>::from ([1,2,3]);
        assert_eq! (g.edges (), &expected_edges, "Edges should be remapped");
        assert_eq! (g.vertices (), &expected_vertices, "Vertices should be remapped");
    }

    #[test]
    fn test_remap_invalid ()
    {
        init ();
        let mut g = Graph::new ();
        //    1
        //   / \
        //  *   *
        // 2     3
        g.add_edge_raw (1, 2, 0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1, 3, 0).expect ("Failed to add edge 1 -> 3");

        let m_duplicate_value = collections::HashMap::<usize,usize>::from ([
            ( 1, 2 ),
            ( 3, 2 ),
        ]);
        let m_invalid_vertex = collections::HashMap::<usize,usize>::from ([
            ( 4, 1 ),
        ]);
        let m_reciprocal = collections::HashMap::<usize,usize>::from ([
            ( 1, 2 ),
        ]);
        assert_eq! (g.remap (&m_duplicate_value), Err (crate::error::GraphError::DataError (String::from ("Every key must have a unique value"))));
        assert_eq! (g.remap (&m_invalid_vertex), Err (crate::error::GraphError::DataError (String::from ("Remap keys must be a subset of graph vertices"))));
        assert_eq! (g.remap (&m_reciprocal), Err (crate::error::GraphError::DataError (String::from ("Both directions are required when swapping existing vertices"))));
    }

    #[test]
    fn test_remap_raw_labelled ()
    {
        init ();
        let mut lg = LabelledGraph::new ();
        //    a
        //    |
        //    *
        //    b
        //   / \
        //  *   *
        // c     d
        lg.add_edge_raw (1, String::from ("a"), 2, String::from ("b"), None, 0).expect ("Failed to add edge 1:a -> 2:b");
        lg.add_edge_raw (2, String::from ("b"), 3, String::from ("c"), None, 1).expect ("Failed to add edge 2:b -> 3:c");
        lg.add_edge_raw (2, String::from ("b"), 4, String::from ("d"), None, 2).expect ("Failed to add edge 2:b -> 4:d");

        let m = collections::HashMap::<usize,usize>::from ([
            ( 1, 4 ),
            ( 2, 3 ),
            ( 3, 2 ),
            ( 4, 1 ),
        ]);

        lg.remap_raw (&m).expect ("Failed to remap");

        let expected_edges = collections::HashMap::<(usize,usize), i64>::from ([
            ( (4,3), 0 ),
            ( (3,2), 1 ),
            ( (3,1), 2 ),
        ]);
        let expected_vertex_labels = vec![
            String::from ("a"),
            String::from ("b"),
            String::from ("c"),
            String::from ("d"),
        ];
        assert_eq! (lg.edges (), &expected_edges, "Edges should be remapped");
        assert_eq! ([4,3,2,1].iter ().map (|x| lg.vertex_label (x).expect ("Failed to find vertex_label")).collect::<Vec<_>> (), expected_vertex_labels, "Vertex labels not remaped");
    }

    #[test]
    fn test_retain ()
    {
        init ();
        let mut g = Graph::new ();
        //   1
        //   |
        //   2
        //  / \
        // 3   4
        g.add_edge_raw (2, 1, 0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3, 2, 0).expect ("Failed to add edge 3 -> 2");
        g.add_edge_raw (4, 2, 0).expect ("Failed to add edge 4 -> 2");

        assert_eq! (g.vertices (), &(1..5).collect::<collections::HashSet<_>> ());

        let retain = collections::HashSet::<usize>::from ([1,3,4]);
        let mut rg = g.clone ();
        rg.retain (&retain).expect ("Failed retain");

        assert_eq! (rg.sources (), collections::HashSet::<usize>::from ([1,3,4]));
        assert_eq! (rg.sinks (), collections::HashSet::<usize>::from ([1,3,4]));
        assert_eq! (rg.edges (), &collections::HashMap::<(usize,usize), i64>::new ());
    }

    #[test]
    fn test_retain_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        //   a
        //   |
        //   b
        //  / \
        // c   d
        g.add_edge (String::from ("b"), String::from ("a"), None).expect ("Failed to add edge b -> a");
        g.add_edge (String::from ("c"), String::from ("b"), None).expect ("Failed to add edge c -> b");
        g.add_edge (String::from ("d"), String::from ("b"), None).expect ("Failed to add edge d -> b");

        assert_eq! (g.graph ().vertices (), &(1..5).collect::<collections::HashSet<_>> ());

        let vertex_labels_expected = ["a","b","c","d"].into_iter ().map (String::from).collect::<Vec<_>> ();
        let vertex_labels = (1..5).map (|x| g.vertex_label (&x).expect (&format! ("Failed to find vertex '{}'", x))).collect::<Vec<_>> ();

        assert_eq! (vertex_labels, vertex_labels_expected);

        let retain = collections::HashSet::<usize>::from ([1,3,4]);
        let mut rg = g.clone ();
        rg.retain (&retain).expect ("Failed retain");

        let vertex_labels_retained_expected = ["a","c","d"].into_iter ().map (String::from).collect::<Vec<_>> ();
        let vertex_labels_retained = vec![1,3,4].iter ().map (|x| rg.vertex_label (&x).expect (&format! ("Failed to find vertex '{}'", x))).collect::<Vec<_>> ();

        assert_eq! (vertex_labels_retained, vertex_labels_retained_expected);

        assert_eq! (rg.graph ().sources (), collections::HashSet::<usize>::from ([1,3,4]));
        assert_eq! (rg.graph ().sinks (), collections::HashSet::<usize>::from ([1,3,4]));
        assert_eq! (rg.edges (), &collections::HashMap::<(usize,usize), i64>::new ());
    }

    #[test]
    fn test_retain_edges ()
    {
        init ();
        let mut g = Graph::new ();
        //    1
        //    *
        //    *
        //    2
        //   * *
        //  *   \
        // 3     4
        g.add_edge_raw (1, 2, 0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (2, 1, 0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (2, 3, 0).expect ("Failed to add edge 2 -> 3");
        g.add_edge_raw (3, 2, 0).expect ("Failed to add edge 3 -> 2");
        g.add_edge_raw (4, 2, 0).expect ("Failed to add edge 4 -> 2");

        assert_eq! (g.vertices (), &(1..5).collect::<collections::HashSet<_>> ());

        let retain = collections::HashSet::<(usize,usize)>::from ([ (3,2), (4,2) ]);
        let mut rg = g.clone ();
        rg.retain_edges (&retain).expect ("Failed retain_edges");
        let edges_retained_expected = retain.iter ().fold (collections::HashMap::<(usize,usize), i64>::new (), |mut acc, item| { acc.insert (*item, 0);acc });

        assert_eq! (rg.sources (), collections::HashSet::<usize>::from ([1,3,4]), "sources not equal");
        assert_eq! (rg.sinks (), collections::HashSet::<usize>::from ([1,2]), "sinks not equal");
        assert_eq! (rg.edges (), &edges_retained_expected, "edges not as expected");
    }

    #[test]
    fn test_retain_edges_labelled ()
    {
        init ();
        let mut g = LabelledGraph::new ();
        //    a
        //    *
        //    *
        //    b
        //   * *
        //  *   \
        // c     d
        g.add_edge (String::from ("b"), String::from ("a"), None).expect ("Failed to add edge b -> a");
        g.add_edge (String::from ("a"), String::from ("b"), None).expect ("Failed to add edge a -> b");
        g.add_edge (String::from ("b"), String::from ("c"), None).expect ("Failed to add edge b -> c");
        g.add_edge (String::from ("c"), String::from ("b"), None).expect ("Failed to add edge c -> b");
        g.add_edge (String::from ("d"), String::from ("b"), None).expect ("Failed to add edge d -> b");

        assert_eq! (g.graph ().vertices (), &(1..5).collect::<collections::HashSet<_>> ());

        let vertex_labels_expected = ["a","b","c","d"].into_iter ().map (String::from).collect::<Vec<_>> ();
        let vertex_labels = (1..5).map (|x| g.vertex_label (&x).expect (&format! ("Failed to find vertex '{}'", x))).collect::<Vec<_>> ();

        assert_eq! (vertex_labels, vertex_labels_expected);

        let retain = collections::HashSet::<(usize,usize)>::from ([ (3,2), (4,2) ]);
        let mut rg = g.clone ();
        rg.retain_edges (&retain).expect ("Failed retain_edges");

        let vertex_labels_retained_expected = ["b","c","d"].into_iter ().map (String::from).collect::<Vec<_>> ();
        let vertex_labels_retained = vec![2,3,4].iter ().map (|x| rg.vertex_label (&x).expect (&format! ("Failed to find vertex '{}'", x))).collect::<Vec<_>> ();

        assert_eq! (vertex_labels_retained, vertex_labels_retained_expected);
        // TODO: test more
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
