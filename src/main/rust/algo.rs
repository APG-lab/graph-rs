
use crate::error;
use crate::graph;
//use log::debug;
use std::collections;

pub fn bfs_edges (g: &graph::Graph, source: usize)
    -> Result<Vec<(usize, usize)>, error::GraphError>
{
    let neighbours = g.vertices ().iter ().fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
        acc.insert (*item, g.neighbours (item).unwrap ());
        acc
        });
    let mut neighbours_iters = neighbours.iter ().fold (collections::HashMap::<usize, collections::hash_set::Iter<'_, usize>>::new (), |mut acc, (k, v)| {
        acc.insert (*k, v.iter () );
        acc
    });
    let mut visited = collections::HashSet::<usize>::from ([source]);
    let mut r = Vec::<(usize, usize)>::new ();
    let mut queue = collections::VecDeque::<(usize, usize)>::from (vec![(source, 0)]);

    while let Some ( (parent, current_depth ) ) = queue.front ()
    {
        if let Some (child) = neighbours_iters.get_mut (parent).ok_or (error::GraphError::AlgorithmError (format! ("No neigbours iter for {}", parent)))?.next ()
        {
            if !visited.contains (child)
            {
                r.push ( (*parent, *child) );
                visited.insert (*child);
                queue.push_back ( (*child, *current_depth + 1) );
            }
        }
        else
        {
                queue.pop_front ();
        }
    }

    Ok (r)
}

pub fn topological_sort (g: &graph::Graph)
    -> Result<Vec<usize>, error::GraphError>
{
    let mut r = Vec::<usize>::new ();
    let mut gc = g.clone ();
    let mut leaves = gc.leaves ();

    while !leaves.is_empty ()
    {
        let n = *leaves.iter ().next ().unwrap ();
        leaves.remove (&n);
        r.push (n);
        //debug! ("leaves: {:?} n: {}", leaves, n);
        for m in gc.outbound (&n)?
        {
            gc.remove_edge_raw (&n, &m)?;
            //debug! ("are {} is_leaf? {}", m, gc.is_leaf (&m)?);
            if gc.is_leaf (&m)?
            {
                leaves.insert  (m);
            }
        }
    }
    if gc.edges ().is_empty ()
    {
        Ok (r)
    }
    else
    {
        Err (crate::error::GraphError::EdgeError (String::from ("Graph contains at least one cycle")))
    }
}

#[cfg(test)]
mod tests
{
    use std::collections;
    use super::*;

    use std::sync;

    static INIT: sync::Once = sync::Once::new ();

    fn init ()
    {
        INIT.call_once (env_logger::init);
    }

    #[test]
    fn test_topo_sort ()
    {
        init ();
        let mut g = graph::Graph::new ();
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");

        let solutions = collections::HashSet::from ([vec![2,3,1], vec![3,2,1]]);
        let r = topological_sort (&g).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_bfs ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //       1
        //      / \
        //     /   \
        //    /     \
        //   2       3
        //  / \     / \
        // 4   5   6   7
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (4,2,0).expect ("Failed to add edge 4 -> 2");
        g.add_edge_raw (5,2,0).expect ("Failed to add edge 5 -> 2");
        g.add_edge_raw (6,3,0).expect ("Failed to add edge 6 -> 3");
        g.add_edge_raw (7,3,0).expect ("Failed to add edge 7 -> 3");

        let solutions = collections::HashSet::from ([
                                                    vec![(1,2), (1,3), (2,4), (2,5), (3,6), (3,7)],
                                                    vec![(1,2), (1,3), (2,4), (2,5), (3,7), (3,6)],
                                                    vec![(1,2), (1,3), (2,5), (2,4), (3,6), (3,7)],
                                                    vec![(1,2), (1,3), (2,5), (2,4), (3,7), (3,6)],
                                                    vec![(1,3), (1,2), (3,6), (3,7), (2,4), (2,5)],
                                                    vec![(1,3), (1,2), (3,6), (3,7), (2,5), (2,4)],
                                                    vec![(1,3), (1,2), (3,7), (3,6), (2,4), (2,5)],
                                                    vec![(1,3), (1,2), (3,7), (3,6), (2,5), (2,4)]
            ]);
        let r = bfs_edges (&g, 1).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }
}

