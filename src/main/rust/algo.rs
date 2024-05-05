
use crate::error;
use crate::graph;
use crate::prng;
use std::collections;
use std::iter;

pub fn bfs_edges<G: graph::GraphAny> (g: &G, source: usize)
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

pub fn connected_components<G: graph::GraphAny> (g: &G)
    -> Result<Vec<collections::HashSet<usize>>, error::GraphError>
{
    let mut r = Vec::<collections::HashSet<usize>>::new ();
    let mut seen = collections::HashSet::<usize>::new ();
    for v in g.vertices ()
    {
        if !seen.contains (v)
        {
            let component = bfs_edges (g, *v)?.iter ()
                .fold (collections::HashSet::<usize>::new (), |mut acc, item| {
                acc.insert (item.0);
                acc.insert (item.1);
                acc
            });
            seen.extend (component.clone ());
            r.push (component);
        }
    }
    Ok (r)
}

pub fn fast_label_propagation<G: graph::GraphAny> (g: &G, seed: &mut u64)
    -> Result<(collections::HashMap<usize, usize>, collections::HashMap<usize, collections::HashSet<usize>>), error::GraphError>
{
    let neighbours = g.vertices ().iter ().fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
        acc.insert (*item, g.neighbours (item).unwrap ());
        acc
        });

    let mut q = collections::VecDeque::<usize>::from (Vec::from_iter (g.vertices ().iter ().copied ()));
    prng::shuffle (q.make_contiguous (), seed)?;

    let mut labels = collections::HashMap::<usize, usize>::from_iter (iter::zip (q.as_slices ().0.iter ().copied (), 0..q.len ()));

    while let Some (v) = q.pop_front ()
    {
        let nlc = neighbours[&v].iter ()
            .fold (collections::HashMap::<usize,usize>::new (), |mut acc, item| {
            *acc.entry (labels[item]).or_insert (0) += 1;
            acc
        });
        let max_freq = *nlc.values ().max ().unwrap_or (&0);

        let most_popular_labels = nlc.iter ()
            .filter (|(_,&v)| v == max_freq)
            .map (|(k,_)| k)
            .collect::<Vec<_>> ();

        let mpli = prng::wyrng_range (0..u64::try_from (most_popular_labels.len ())?, seed);
        let mpl = *most_popular_labels[TryInto::<usize>::try_into (mpli)?];

        if labels[&v] != mpl
        {
            labels.insert (v, mpl);
            for vn in neighbours[&v].iter ()
            {
                if labels[vn] != mpl
                {
                    q.push_back (*vn);
                }
            }
        }
    }

    let mut r_node_to_label = collections::HashMap::<usize, usize>::new ();
    let mut r_label_to_node = collections::HashMap::<usize, collections::HashSet<usize>>::new ();

    for (k,v) in labels
    {
        r_node_to_label.insert (k, v);
        r_label_to_node.entry (v).or_insert (collections::HashSet::<usize>::new ()).insert (k);
    }
    Ok ( (r_node_to_label, r_label_to_node) )
}

pub fn single_shortest_path<G: graph::GraphAny> (g: &G, source: usize)
    -> Result<collections::HashMap<usize, Vec<usize>>, error::GraphError>
{
    let neighbours = g.vertices ().iter ().fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
        acc.insert (*item, g.neighbours (item).unwrap ());
        acc
        });
    let mut neighbours_iters = neighbours.iter ().fold (collections::HashMap::<usize, collections::hash_set::Iter<'_, usize>>::new (), |mut acc, (k, v)| {
        acc.insert (*k, v.iter () );
        acc
    });

    let mut r = collections::HashMap::<usize, Vec<usize>>::from ([ (source, vec![source]) ]);
    let mut queue = collections::VecDeque::<(usize, usize)>::from (vec![(source, 0)]);

    while let Some ( (parent, current_depth ) ) = queue.front ()
    {
        if let Some (child) = neighbours_iters.get_mut (parent).ok_or (error::GraphError::AlgorithmError (format! ("No neigbours iter for {}", parent)))?.next ()
        {
            if !r.contains_key (child)
            {
                //r.push ( (*parent, *child) );
                let mut path = Vec::with_capacity (r[parent].len () +1);
                path.extend (r[parent].as_slice ());
                path.push (*child);
                r.insert (*child, path);
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

    #[test]
    fn test_connected_components ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //   1       4
        //  / \     / \
        // 2   3   5   6
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");
        g.add_edge_raw (5,4,0).expect ("Failed to add edge 5 -> 4");
        g.add_edge_raw (6,4,0).expect ("Failed to add edge 6 -> 4");

        let solution = vec![
            collections::HashSet::from_iter (vec![1,2,3]),
            collections::HashSet::from_iter (vec![4,5,6])
        ];

        let mut r = connected_components (&g).unwrap ();
        r.sort_by_key (|k| *k.iter ().min ().expect ("Failed to find min of component"));
        assert_eq! (r,solution);
    }

    #[test]
    fn test_fast_label_propagation ()
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

        let solutions = Vec::from ([
            Vec::from ([ collections::HashSet::<usize>::from ([1,2,4,5]), collections::HashSet::<usize>::from ([3,6,7]) ]),
            Vec::from ([ collections::HashSet::<usize>::from ([1,3,6,7]), collections::HashSet::<usize>::from ([2,4,5]) ])
        ]);

        let (nl, ln) = fast_label_propagation (&g, &mut 42u64).expect ("Failed fast label propagation");

        let mut r = ln.values ().cloned ().collect::<Vec<_>> ();
        r.sort_by_key (|k| *k.iter ().min ().expect ("Failed to find min"));

        assert_eq! ((1..8).collect::<collections::HashSet<_>> (), nl.keys ().copied ().collect::<collections::HashSet<_>> ());
        assert! (nl.values ().all (|x| ln.contains_key (x)));
        assert! (nl.iter ().all (|(n,l)| ln[l].contains (n)));
        assert! (solutions.contains (&r));
    }

    #[test]
    fn test_single_shortest_path ()
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

        let solution = collections::HashMap::from ([
            (1, vec![1]),
            (2, vec![1,2]),
            (3, vec![1,3]),
            (4, vec![1,2,4]),
            (5, vec![1,2,5]),
            (6, vec![1,3,6]),
            (7, vec![1,3,7])
        ]);

        let r = single_shortest_path (&g, 1).unwrap ();
        assert_eq! (r,solution);
    }
}

