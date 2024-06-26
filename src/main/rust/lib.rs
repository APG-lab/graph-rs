
use std::fs;
use std::io::Write;

pub mod algo;
pub mod eq;
pub mod error;
pub mod graph;
pub mod prng;
pub mod sd;

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
        let mut graph = graph::Graph::new ();
        graph.add_edge_raw (1, 2, 0).expect ("added edge 1,2");
        graph.add_edge_raw (1, 3, 0).expect ("added edge 1,3");
        super::graph_to_dot (graph, "simple.dot").expect ("Failed to write dot");
    }

}

