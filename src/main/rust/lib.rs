
use std::collections;
use std::fs;
use std::io::Write;

pub mod algo;
pub mod eq;
pub mod error;
pub mod graph;
pub mod prng;
pub mod sd;

pub fn ugraph_from_graph (g: &graph::Graph)
    -> Result<graph::UGraph, error::GraphError>
{
    let edges = g.edges ()
        .iter ()
        .try_fold (collections::HashMap::<(usize,usize),i64>::new (), |mut acc, (e, w)| {
            let t = if e.0 < e.1 { ( e.0, e.1 ) } else { ( e.1, e.0 ) };
            if let Some (wo) = acc.get (&t)
            {
                if w == wo
                {
                    Ok (acc)
                }
                else
                {
                    Err (error::GraphError::ConversionError (format! ("Cannot merge bidirectional edge {:?} with different weights", t)))
                }
            }
            else
            {
                acc.insert (t,*w);
                Ok (acc)
            }
        })?;
    let connected = g.vertices ()
        .iter ()
        .try_fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
            acc.insert (*item, g.neighbours (item)?);
            Ok::<_, error::GraphError> (acc)
        })?;

    Ok (graph::UGraph {
        name: g.name (),
        vertices: g.vertices ().clone (),
        edges: edges,
        connected: connected
    })
}

pub fn base_graph (gv: &Vec<graph::LabelledGraph>)
    -> Result<&graph::LabelledGraph, error::GraphError>
{
    if let Some (maybe_base_graph) = gv.last ()
    {
        if maybe_base_graph.graph ().name () == ""
        {
            Ok (maybe_base_graph)
        }
        else
        {
            Err (error::GraphError::DataError (String::from ("The last graph in the graph vector must be unnamed to be the base graph.")))
        }
    }
    else
    {
        Err (error::GraphError::DataError (String::from ("No graphs found")))
    }
}

pub fn first_graph (gv: &Vec<graph::LabelledGraph>)
    -> Result<&graph::LabelledGraph, error::GraphError>
{
    if let Some (maybe_first_graph) = gv.first ()
    {
        Ok (maybe_first_graph)
    }
    else
    {
        Err (error::GraphError::DataError (String::from ("No graphs found")))
    }
}

pub fn last_graph (gv: &Vec<graph::LabelledGraph>)
    -> Result<&graph::LabelledGraph, error::GraphError>
{
    if let Some (maybe_base_graph) = gv.last ()
    {
        Ok (maybe_base_graph)
    }
    else
    {
        Err (error::GraphError::DataError (String::from ("No graphs found")))
    }
}

pub fn named_graph<'a> (gv: &'a Vec<graph::LabelledGraph>, graph_name: &str)
    -> Result<&'a graph::LabelledGraph, error::GraphError>
{
    if let Some (maybe_base_graph) = gv.iter ()
        .find (|x| x.graph ().name () == graph_name)
    {
        Ok (maybe_base_graph)
    }
    else
    {
        Err (error::GraphError::DataError (String::from ("No graphs found")))
    }
}

pub fn graph_to_dot (graph: graph::Graph, file_path: &str)
    -> Result<(), error::GraphError>
{
    let mut file = fs::File::create (file_path)?;

    write! (file, "digraph {} {{\n", graph.name ())?;
    for (edge, _weight) in graph.edges ()
    {
        write! (file, "\t{} -> {};\n", edge.0, edge.1)?;
    }
    write! (file, "}}\n")?;
    Ok (())
}


#[cfg(test)]
mod tests
{
    use crate::graph;

    #[test]
    fn simple_to_dot ()
    {
        let mut g = graph::Graph::new ();
        g.add_edge_raw (1, 2, 0).expect ("added edge 1,2");
        g.add_edge_raw (1, 3, 0).expect ("added edge 1,3");
        super::graph_to_dot (g, "simple.dot").expect ("Failed to write dot");
    }

    #[test]
    fn to_undirected ()
    {
        let mut g = graph::Graph::new_with_name ("g");
        g.add_edge_raw (1,2,1).expect ("added edge 1,2 1");

        let gu = super::ugraph_from_graph (&g).expect ("Failed to convert to undirected");

        assert_eq! (gu.name (), String::from ("g"));
        assert! (gu.has_edge_raw (&(1,2)));
        assert! (gu.has_edge_raw (&(2,1)));
    }
}

