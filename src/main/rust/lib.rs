
use std::collections;
use std::fs;
use std::io::Write;
use std::iter;

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

pub fn cut_graph_tree (
    (g_tree, g_vec): &(graph::LabelledGraph, Vec<graph::LabelledGraph>),
    v: usize,
    )
    -> Result<(graph::LabelledGraph, Vec<graph::LabelledGraph>), error::GraphError>
{
    let p_vertices = algo::dfs_edges (g_tree.graph (), v)?
        .into_iter ()
        .flat_map (|x| [x.0.0, x.0.1])
        .chain (iter::once (v))
        .collect::<collections::HashSet<_>> ();

    let (p_vec, remap) = g_vec.iter ()
        .enumerate ()
        .filter (|&(i,_)| p_vertices.contains (&i))
        .fold ( ( Vec::new (), collections::HashMap::<usize, usize>::new () ), |mut acc , item| {
            acc.1.insert (item.0, acc.0.len ());
            acc.0.push (item.1.clone ());
            acc
        });

    let mut p_tree = g_tree.clone ();
    p_tree.retain (&p_vertices)?;
    p_tree.remap_raw (&remap)?;

    Ok ( ( p_tree, p_vec ) )
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
mod tests_lib
{
    use crate::graph;

    #[test]
    fn test_cut_graph_tree ()
    {
        let mut g_tree = graph::LabelledGraph::new ();
        g_tree.add_edge_raw (0, String::from ("zero"), 1, String::from ("one"), None, 0).expect ("added edge 0:zero -> 1:one");

        let g_zero = graph::LabelledGraph::new_with_name ("zero");
        let g_one = graph::LabelledGraph::new_with_name ("one");

        let g_vec = vec![g_zero.clone (), g_one.clone ()];

        let gt = (g_tree, g_vec);

        let gt_zero = super::cut_graph_tree (&gt, 0).expect ("Failed to cut graph tree at 0");
        let gt_one  = super::cut_graph_tree (&gt, 1).expect ("Failed to cut graph tree at 1");

        let mut expected_gt_zero_tree = graph::LabelledGraph::new ();
        expected_gt_zero_tree.add_edge_raw (0, String::from ("zero"), 1, String::from ("one"), None, 0).expect ("added edge 0:zero -> 1:one");
        let expected_gt_zero_vec = vec![g_zero.clone (), g_one.clone ()];

        let mut expected_gt_one_tree = graph::LabelledGraph:: new ();
        expected_gt_one_tree.add_vertex_raw (0, String::from ("one"), None).expect ("Failed to add vertex 0:'one'");
        let expected_gt_one_vec = vec![g_one];


        assert_eq! (gt_zero.0, expected_gt_zero_tree);
        assert_eq! (gt_zero.1, expected_gt_zero_vec);
        assert_eq! (gt_one.0, expected_gt_one_tree);
        assert_eq! (gt_one.1, expected_gt_one_vec);
    }

    #[test]
    fn test_simple_to_dot ()
    {
        let mut g = graph::Graph::new ();
        g.add_edge_raw (1, 2, 0).expect ("added edge 1,2");
        g.add_edge_raw (1, 3, 0).expect ("added edge 1,3");
        super::graph_to_dot (g, "simple.dot").expect ("Failed to write dot");
    }

    #[test]
    fn test_to_undirected ()
    {
        let mut g = graph::Graph::new_with_name ("g");
        g.add_edge_raw (1,2,1).expect ("added edge 1,2 1");

        let gu = super::ugraph_from_graph (&g).expect ("Failed to convert to undirected");

        assert_eq! (gu.name (), String::from ("g"));
        assert! (gu.has_edge_raw (&(1,2)));
        assert! (gu.has_edge_raw (&(2,1)));
    }
}

