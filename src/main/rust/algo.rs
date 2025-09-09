
//use log::debug;
use crate::error;
use crate::graph;
use crate::prng;
use std::cmp;
use std::collections;
use std::iter;

#[derive(Copy, Clone, Eq, PartialEq)]
struct MultiSourceState
{
    cost: i64,
    source: usize,
    v: usize
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for MultiSourceState {
    fn cmp(&self, other: &Self) -> cmp::Ordering
    {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp (&self.cost)
            .then_with (|| self.v.cmp(&other.v))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for MultiSourceState {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
    {
        Some(self.cmp(other))
    }
}

pub fn all_shortest_paths<G: graph::GraphAny> (g: &G, sources: &collections::HashSet<usize>)
    -> Result<collections::HashMap<usize, (collections::HashMap<usize, i64>, collections::HashMap<usize, collections::HashSet<Vec<usize>>>)>, error::GraphError>
{
    let mut r = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize, collections::HashSet<Vec<usize>>>)>::new ();
    let mut fringe = collections::BinaryHeap::<MultiSourceState>::new ();

    for source in sources
    {
        fringe.push (MultiSourceState { cost: 0, source: *source, v: *source });
        r.insert (*source, ( collections::HashMap::<usize,i64>::from ([ ( *source, 0 ) ]), collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([ ( *source, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([*source]) ]) ) ])) );
    }

    while let Some (MultiSourceState { cost, source, v }) = fringe.pop ()
    {
        if cost > *r.get (&source).unwrap ().0.get (&v).unwrap ()
        {
            continue;
        }
        else
        {
            for v_child in g.adjacent (&v)?
            {
                let e = (v, v_child);
                let ew = g.weight (&e)?;

                let mss_next = MultiSourceState { cost: cost + ew, source: source, v: v_child };

                let paths_nxt = r.get (&source).unwrap ().1.get (&v_child)
                    .cloned ()
                    .unwrap_or (collections::HashSet::<Vec<usize>>::new ())
                    .into_iter ()
                    .filter (|x| {
                            let mut xr = x.iter ().rev ();
                            let _ = xr.next ();
                            if let Some (sl) = xr.next () { sl != &v } else { false }
                        })
                    .collect::<collections::HashSet::<Vec<_>>> ();

                let maybe_len_shortest_path = paths_nxt.iter ()
                    .filter (|&x| x.last ().unwrap () == &mss_next.v)
                    .map (|x| x.len ())
                    .min ();

                let paths_old = r.get (&source).unwrap ().1.get (&v)
                    .ok_or (error::GraphError::AlgorithmError (format! ("Path for {} not found for source {}", v, source)))?;

                let paths_new = paths_old.iter ()
                    .filter (|&x| !x.contains (&mss_next.v))
                    .filter (|&x| { if let Some (lsp) = maybe_len_shortest_path { x.len () < lsp } else { true } })
                    .cloned ()
                    .collect::<collections::HashSet::<Vec<usize>>> ();

                let paths_combined = paths_new.iter ()
                    .fold (paths_nxt, |mut acc, item| {
                        let mut item_new = item.clone ();
                        match item.last ()
                        {
                            Some (v_last) if g.has_edge_raw ( &( *v_last, mss_next.v) ) => item_new.push (mss_next.v),
                            None => item_new.push (mss_next.v),
                            _ => {}
                        }
                        acc.insert (item_new);
                        acc
                    });

                if let Some (dist_next) = r.get (&source).unwrap ().0.get (&mss_next.v).copied ()
                {
                    if mss_next.cost <= dist_next && !paths_new.is_empty ()
                    {
                        r.get_mut (&source).unwrap ().0.insert (mss_next.v, mss_next.cost);
                        r.get_mut (&source).unwrap ().1.insert (mss_next.v, paths_combined);
                        fringe.push (mss_next);
                    }
                }
                else
                {
                    // dist_next is infinite
                    r.get_mut (&source).unwrap ().0.insert (mss_next.v, mss_next.cost);
                    r.get_mut (&source).unwrap ().1.insert (mss_next.v, paths_combined);
                    fringe.push (mss_next);
                }
            }
        }
    }

    Ok (r)
}

pub fn bfs_edges<G: graph::GraphAny> (g: &G, source: usize)
    -> Result<Vec<(usize, usize)>, error::GraphError>
{
    let neighbours = g.vertices ().iter ().fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
        acc.insert (*item, g.adjacent (item).unwrap ());
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
        if let Some (child) = neighbours_iters.get_mut (parent).ok_or (error::GraphError::AlgorithmError (format! ("No neighbours iter for {}", parent)))?.next ()
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

pub fn edge_bfs<G: graph::GraphAny> (g: &G, source: usize)
    -> Result<Vec<(usize, usize)>, error::GraphError>
{
    let neighbours = g.vertices ().iter ().fold (collections::HashMap::<usize, collections::HashSet<usize>>::new (), |mut acc, item| {
        acc.insert (*item, g.adjacent (item).unwrap ());
        acc
        });
    let mut neighbours_iters = neighbours.iter ().fold (collections::HashMap::<usize, collections::hash_set::Iter<'_, usize>>::new (), |mut acc, (k, v)| {
        acc.insert (*k, v.iter () );
        acc
    });

    let mut visited_vertices = collections::HashSet::<usize>::from ([source]);
    let mut visited_edges = collections::HashSet::<(usize, usize)>::new ();
    let mut r = Vec::<(usize, usize)>::new ();

    let source_edges = neighbours_iters.get_mut (&source)
        .ok_or (error::GraphError::AlgorithmError (format! ("No neighbours iter for {}", source)))?
        .map (|x| {
            let ex = (source, *x);
            if g.has_edge_raw (&ex) { ex } else { (ex.1, ex.0) }
        })
        .collect::<collections::HashSet<(usize,usize)>> ();
    let mut queue = collections::VecDeque::<(usize, collections::HashSet<(usize, usize)>)>::from (vec![(source, source_edges)]);

    while let Some ( (parent, edges) ) = queue.pop_front ()
    {
        //debug! ("parent: {} edges: {:?}", parent, edges);
        for ev in edges
        {
            if parent == ev.0
            {
                if !visited_vertices.contains (&ev.1)
                {
                    visited_vertices.insert (ev.1);
                    let child_edges = neighbours_iters.get_mut (&ev.1)
                        .ok_or (error::GraphError::AlgorithmError (format! ("No neighbours iter for {}", ev.1)))?
                        .map (|x| {
                            let ex = (ev.1, *x);
                            if g.has_edge_raw (&ex) { ex } else { (ex.1, ex.0) }
                        })
                        .collect::<collections::HashSet<(usize,usize)>> ();
                    queue.push_back ( (ev.1, child_edges) );
                }
            }
            else
            {
                if !visited_vertices.contains (&ev.0)
                {
                    visited_vertices.insert (ev.0);
                    let child_edges = neighbours_iters.get_mut (&ev.0)
                        .ok_or (error::GraphError::AlgorithmError (format! ("No neighbours iter for {}", ev.0)))?
                        .map (|x| {
                            let ex = (ev.0, *x);
                            if g.has_edge_raw (&ex) { ex } else { (ex.1, ex.0) }
                        })
                        .collect::<collections::HashSet<(usize,usize)>> ();
                    queue.push_back ( (ev.0, child_edges) );
                }
            }

            if !visited_edges.contains (&ev)
            {
                visited_edges.insert (ev);
                r.push (ev);
            }
        }
    }
    Ok (r)
}

pub fn connected_components (g: &graph::UGraph)
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

pub fn dfs_edges (g: &graph::Graph, source: usize)
    -> Result<Vec<((usize, usize), usize)>, error::GraphError>
{
    let children = g.vertices ().iter ().fold (collections::HashMap::<usize, Vec<usize>>::new (), |mut acc, item| {
        let mut outbound_sorted = g.outbound (item).unwrap ().into_iter ().collect::<Vec<_>> ();
        outbound_sorted.sort ();
        acc.insert (*item, outbound_sorted);
        acc
        });
    let mut children_iters = children.iter ().fold (collections::HashMap::<usize, std::slice::Iter<'_, usize>>::new (), |mut acc, (k, v)| {
        acc.insert (*k, v.iter () );
        acc
    });

    let mut visited = collections::HashSet::<usize>::new ();
    let mut queue = collections::VecDeque::<(usize, usize, usize)>::from (vec![(0, source, source)]);
    let mut r = Vec::<((usize, usize), usize)>::new ();

    while let Some ( (current_depth , parent, _) ) = queue.back ().copied ()
    {
        if let Some (child) = children_iters.get_mut (&parent).ok_or (error::GraphError::AlgorithmError (format! ("No outbound iter for {}", parent)))?.next ()
        {
            if !visited.contains (child)
            {
                visited.insert (*child);
                queue.push_back ( ( current_depth + 1, *child, parent) );
                r.push ( ( (parent, *child), current_depth ) );
            }
        }
        else
        {
            queue.pop_back ();
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
        acc.insert (*item, g.adjacent (item).unwrap ());
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

// I want to reuse as much as possible between paths
pub fn multi_source_dijkstra<G: graph::GraphAny> (g: &G, sources: &collections::HashSet<usize>)
    -> Result<collections::HashMap<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize, Vec<usize>>)>, error::GraphError>
{
    let mut r = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize, Vec<usize>>)>::new ();
    let mut fringe = collections::BinaryHeap::<MultiSourceState>::new ();

    for source in sources
    {
        fringe.push (MultiSourceState { cost: 0, source: *source, v: *source });
        r.insert (*source, ( collections::HashMap::<usize,i64>::from ([ ( *source, 0 ) ]), collections::HashMap::<usize, Vec<usize>>::from ([ ( *source, Vec::from ([*source]) ) ])) );
    }

    while let Some (MultiSourceState { cost, source, v }) = fringe.pop ()
    {
        //debug! ("current mss ({}, {}, {})", cost, source, v);

        if cost > *r.get (&source).unwrap ().0.get (&v).unwrap ()
        {
            continue;
        }
        else
        {
            for v_child in g.adjacent (&v)?
            {
                let e = (v, v_child);
                let ew = g.weight (&e)?;

                let mss_next = MultiSourceState { cost: cost + ew, source: source, v: v_child };

                let mut path_new = r.get (&source).unwrap ().1.get (&v)
                    .ok_or (error::GraphError::AlgorithmError (format! ("Path for {} not found for source {}", v, source)))?
                    .clone ();
                path_new.push (mss_next.v);
                if let Some (dist_next) = r.get (&source).unwrap ().0.get (&mss_next.v).copied ()
                {
                    if mss_next.cost < dist_next
                    {
                        r.get_mut (&source).unwrap ().0.insert (mss_next.v, mss_next.cost);
                        fringe.push (mss_next);
                    }
                }
                else
                {
                    // dist_next is infinite
                    r.get_mut (&source).unwrap ().0.insert (mss_next.v, mss_next.cost);
                    r.get_mut (&source).unwrap ().1.insert (mss_next.v, path_new);
                    fringe.push (mss_next);
                }
            }
        }
    }
    Ok (r)
}

pub fn topological_sort (g: &graph::Graph)
    -> Result<Vec<usize>, error::GraphError>
{
    let mut r = Vec::<usize>::new ();
    let mut gc = g.clone ();
    let mut sources = gc.sources ();

    while !sources.is_empty ()
    {
        let n = *sources.iter ().next ().unwrap ();
        sources.remove (&n);
        r.push (n);
        //debug! ("leaves: {:?} n: {}", leaves, n);
        for m in gc.outbound (&n)?
        {
            gc.remove_edge_raw (&n, &m)?;
            //debug! ("are {} is_leaf? {}", m, gc.is_leaf (&m)?);
            if gc.is_source (&m)?
            {
                sources.insert  (m);
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
    //use log::debug;
    use crate::graph;
    use std::collections;
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
        //               1
        //              * *
        //             /   \
        //            2     3
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,1,0).expect ("Failed to add edge 3 -> 1");

        let solutions = collections::HashSet::from ([vec![2,3,1], vec![3,2,1]]);
        let r = super::topological_sort (&g).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_topo_sort_linear ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //               1
        //              * \
        //             /   *
        //            2     3

        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");

        let solutions = collections::HashSet::from ([vec![2,1,3]]);
        let r = super::topological_sort (&g).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_all_shortest_paths ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //    1
        //   /  \
        //  *    *
        // 2     3
        // |     |
        // *     *
        // 4     5
        //  \   /
        //   * *
        //    6
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (3,5,0).expect ("Failed to add edge 3 -> 5");
        g.add_edge_raw (4,6,0).expect ("Failed to add edge 4 -> 6");
        g.add_edge_raw (5,6,0).expect ("Failed to add edge 5 -> 6");

        let solution = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize,collections::HashSet<Vec<usize>>>)>::from ([
            ( 1, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 ),
                        ( 4, 0 ),
                        ( 5, 0 ),
                        ( 6, 0 ),
                    ]),
                    collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([
                        ( 1, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1]) ]) ),
                        ( 2, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2]) ]) ),
                        ( 3, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 3]) ]) ),
                        ( 4, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4]) ]) ),
                        ( 5, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 3, 5]) ]) ),
                        ( 6, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 6]), Vec::from ([1, 3, 5, 6]) ]) ),
                    ])
                )
            ),
        ]);

        let sources = collections::HashSet::from ([1]);
        let r = super::all_shortest_paths (&g, &sources).unwrap ();

        assert_eq! (r, solution);
    }

    #[test]
    fn test_all_shortest_paths_u ()
    {
        init ();
        let mut g = graph::UGraph::new ();
        //   1
        //  /
        // 2   3
        // |   |
        // 4   5
        //  \ /
        //   6
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (3,5,0).expect ("Failed to add edge 3 -> 5");
        g.add_edge_raw (4,6,0).expect ("Failed to add edge 4 -> 6");
        g.add_edge_raw (5,6,0).expect ("Failed to add edge 5 -> 6");

        let solution = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize,collections::HashSet<Vec<usize>>>)>::from ([
            ( 1, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 ),
                        ( 4, 0 ),
                        ( 5, 0 ),
                        ( 6, 0 ),
                    ]),
                    collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([
                        ( 1, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1]) ]) ),
                        ( 2, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2]) ]) ),
                        ( 4, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4]) ]) ),
                        ( 6, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 6]) ]) ),
                        ( 5, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 6, 5]) ]) ),
                        ( 3, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 6, 5, 3]) ]) ),
                    ])
                )
            ),
            ( 3, (
                collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 ),
                        ( 4, 0 ),
                        ( 5, 0 ),
                        ( 6, 0 ),
                    ]),
                    collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([
                        ( 3, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3]) ]) ),
                        ( 5, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5]) ]) ),
                        ( 6, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 6]) ]) ),
                        ( 4, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 6, 4]) ]) ),
                        ( 2, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 6, 4, 2]) ]) ),
                        ( 1, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 6, 4, 2, 1]) ]) ),
                    ])
                )
            )
        ]);

        let sources = collections::HashSet::from ([1,3]);
        let r = super::all_shortest_paths (&g, &sources).unwrap ();

        assert_eq! (r, solution);
    }

    #[test]
    fn test_all_shortest_paths_cycle_u ()
    {
        init ();
        let mut g = graph::UGraph::new ();
        //   1
        //  /
        // 2   3
        // |   |
        // 4 - 5
        //  \ /
        //   6
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (3,5,0).expect ("Failed to add edge 3 -> 5");
        g.add_edge_raw (4,5,0).expect ("Failed to add edge 4 -> 5");
        g.add_edge_raw (4,6,0).expect ("Failed to add edge 4 -> 6");
        g.add_edge_raw (5,6,0).expect ("Failed to add edge 5 -> 6");

        let solution = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize,collections::HashSet<Vec<usize>>>)>::from ([
            ( 1, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 ),
                        ( 4, 0 ),
                        ( 5, 0 ),
                        ( 6, 0 ),
                    ]),
                    collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([
                        ( 1, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1]) ]) ),
                        ( 2, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2]) ]) ),
                        ( 4, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4]) ]) ),
                        ( 5, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 5]) ]) ),
                        ( 6, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 6]) ]) ),
                        ( 3, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([1, 2, 4, 5, 3]) ]) ),
                    ])
                )
            ),
            ( 3, (
                collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 ),
                        ( 4, 0 ),
                        ( 5, 0 ),
                        ( 6, 0 ),
                    ]),
                    collections::HashMap::<usize, collections::HashSet<Vec<usize>>>::from ([
                        ( 3, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3]) ]) ),
                        ( 5, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5]) ]) ),
                        ( 4, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 4]) ]) ),
                        ( 6, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 6]) ]) ),
                        ( 2, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 4, 2]) ]) ),
                        ( 1, collections::HashSet::<Vec<usize>>::from ([ Vec::from ([3, 5, 4, 2, 1]) ]) ),
                    ])
                )
            )
        ]);

        let sources = collections::HashSet::from ([1,3]);
        let r = super::all_shortest_paths (&g, &sources).unwrap ();

        assert_eq! (r, solution);
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
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (2,5,0).expect ("Failed to add edge 2 -> 5");
        g.add_edge_raw (3,6,0).expect ("Failed to add edge 3 -> 6");
        g.add_edge_raw (3,7,0).expect ("Failed to add edge 3 -> 7");

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
        let r = super::bfs_edges (&g, 1).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_edge_bfs ()
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
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (2,5,0).expect ("Failed to add edge 2 -> 5");
        g.add_edge_raw (3,6,0).expect ("Failed to add edge 3 -> 6");
        g.add_edge_raw (3,7,0).expect ("Failed to add edge 3 -> 7");

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

        let r = super::edge_bfs (&g, 1).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_edge_bfs_all_edges ()
    {
        init ();
        let mut g = graph::Graph::new ();
        // 1
        // |
        // *
        // 2
        // *
        // *
        // 3
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (2,3,0).expect ("Failed to add edge 2 -> 3");
        g.add_edge_raw (3,2,0).expect ("Failed to add edge 3 -> 2");

        let solutions = collections::HashSet::from ([
                                                    vec![(1,2), (2,3), (3,2)]
            ]);

        let r = super::edge_bfs (&g, 1).unwrap ();
        assert! (solutions.contains (&r), "{:?} not found in {:?}", r, solutions);
    }

    #[test]
    fn test_connected_components ()
    {
        init ();
        let mut g = graph::UGraph::new ();
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

        let mut r = super::connected_components (&g).unwrap ();
        r.sort_by_key (|k| *k.iter ().min ().expect ("Failed to find min of component"));
        assert_eq! (r,solution);
    }

    #[test]
    fn test_dfs ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //       1
        //      / \
        //     *   *
        //    2     3
        //   / \
        //  *   *
        // 4     5
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (2,5,0).expect ("Failed to add edge 2 -> 5");

        let dfs_edges_root = vec![
            ( ( 1, 2 ), 0 ),
            ( ( 2, 4 ), 1 ),
            ( ( 2, 5 ), 1 ),
            ( ( 1, 3 ), 0 ),
        ];
        let dfs_edges_mid = vec![
            ( ( 2,4 ), 0 ),
            ( ( 2,5 ), 0 ),
        ];
        let dfs_edges_leaf = Vec::<((usize,usize), usize)>::new ();

        assert_eq! (super::dfs_edges (&g, 1).unwrap (), dfs_edges_root, "Failed dfs root");
        assert_eq! (super::dfs_edges (&g, 2).unwrap (), dfs_edges_mid, "Failed dfs mid");
        assert_eq! (super::dfs_edges (&g, 4).unwrap (), dfs_edges_leaf, "Failed dfs leaf");
    }

    #[test]
    fn test_dfs_more ()
    {
        init ();
        let mut g = graph::Graph::new ();
        //           1
        //         / | \
        //        *  *  *
        //       2   5   9
        //      /   / \   \
        //     *   *   *   *
        //    3    6    8  10
        //   /     |
        //  *      *
        // 4       7
        g.add_edge_raw (1,2 ,0).expect ("Failed to add edge 1 -> 2 ");
        g.add_edge_raw (1,5 ,0).expect ("Failed to add edge 1 -> 5 ");
        g.add_edge_raw (1,9 ,0).expect ("Failed to add edge 1 -> 9 ");
        g.add_edge_raw (2,3 ,0).expect ("Failed to add edge 2 -> 3 ");
        g.add_edge_raw (3,4 ,0).expect ("Failed to add edge 3 -> 4 ");
        g.add_edge_raw (5,6 ,0).expect ("Failed to add edge 5 -> 6 ");
        g.add_edge_raw (5,8 ,0).expect ("Failed to add edge 5 -> 8 ");
        g.add_edge_raw (6,7 ,0).expect ("Failed to add edge 6 -> 7 ");
        g.add_edge_raw (9,10,0).expect ("Failed to add edge 9 -> 10");
        let dfs_edges = vec![
            ( ( 1,  2 ), 0 ),
            ( ( 2,  3 ), 1 ),
            ( ( 3,  4 ), 2 ),
            ( ( 1,  5 ), 0 ),
            ( ( 5,  6 ), 1 ),
            ( ( 6,  7 ), 2 ),
            ( ( 5,  8 ), 1 ),
            ( ( 1,  9 ), 0 ),
            ( ( 9, 10 ), 1 ),
        ];

        assert_eq! (super::dfs_edges (&g, 1).unwrap (), dfs_edges, "Failed dfs");
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
            Vec::from ([ collections::HashSet::<usize>::from ([1,2,3,4,5,6,7]) ]),
            Vec::from ([ collections::HashSet::<usize>::from ([1,2,4,5]), collections::HashSet::<usize>::from ([3,6,7]) ]),
            Vec::from ([ collections::HashSet::<usize>::from ([1,3,6,7]), collections::HashSet::<usize>::from ([2,4,5]) ])
        ]);

        let (nl, ln) = super::fast_label_propagation (&g, &mut 42u64).expect ("Failed fast label propagation");

        let mut r = ln.values ().cloned ().collect::<Vec<_>> ();
        r.sort_by_key (|k| *k.iter ().min ().expect ("Failed to find min"));

        assert_eq! ((1..8).collect::<collections::HashSet<_>> (), nl.keys ().copied ().collect::<collections::HashSet<_>> ());
        assert! (nl.values ().all (|x| ln.contains_key (x)));
        assert! (nl.iter ().all (|(n,l)| ln[l].contains (n)));
        assert! (solutions.contains (&r), "{:?} not found in solutions", r);
    }

    #[test]
    fn test_multi_dijkstra ()
    {
        init ();
        let mut g = graph::Graph::new ();
        // 3     4
        //  \   /
        //   * *
        //    2
        //    |
        //    *
        //    1
        g.add_edge_raw (2,1,0).expect ("Failed to add edge 2 -> 1");
        g.add_edge_raw (3,2,0).expect ("Failed to add edge 3 -> 2");
        g.add_edge_raw (4,2,0).expect ("Failed to add edge 4 -> 2");

        let solution = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize,Vec<usize>>)>::from ([
            ( 3, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 3, 0 )
                    ]),
                    collections::HashMap::<usize, Vec<usize>>::from ([
                        ( 1, Vec::from ([3, 2, 1]) ),
                        ( 2, Vec::from ([3, 2]) ),
                        ( 3, Vec::from ([3]) )
                    ])
                )
            ),
            ( 4, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 0 ),
                        ( 2, 0 ),
                        ( 4, 0 )
                    ]),
                    collections::HashMap::<usize, Vec<usize>>::from ([
                        ( 1, Vec::from ([4, 2, 1]) ),
                        ( 2, Vec::from ([4, 2]) ),
                        ( 4, Vec::from ([4]) )
                    ])
                )
            )
        ]);

        let sources = collections::HashSet::<usize>::from ([3,4]);
        let r = super::multi_source_dijkstra (&g, &sources).unwrap ();

        assert_eq! (r, solution);
    }

    #[test]
    fn test_multi_dijkstra_weighted_u ()
    {
        init ();
        let mut g = graph::LabelledUGraph::new ();
        // Graph copied from https://raw.githubusercontent.com/networkx/networkx/main/examples/algorithms/plot_shortest_path.py

        g.add_edge_weighted (String::from ("A"), String::from ("B"), None,  4).expect ("Failed to add edge A -> B  4");//G.add_edge("A", "B", weight=4)
        g.add_edge_weighted (String::from ("A"), String::from ("H"), None,  8).expect ("Failed to add edge A -> H  8");//G.add_edge("A", "H", weight=8)
        g.add_edge_weighted (String::from ("B"), String::from ("C"), None,  8).expect ("Failed to add edge B -> C  8");//G.add_edge("B", "C", weight=8)
        g.add_edge_weighted (String::from ("B"), String::from ("H"), None, 11).expect ("Failed to add edge B -> H 11");//G.add_edge("B", "H", weight=11)
        g.add_edge_weighted (String::from ("C"), String::from ("D"), None,  7).expect ("Failed to add edge C -> D  7");//G.add_edge("C", "D", weight=7)
        g.add_edge_weighted (String::from ("C"), String::from ("F"), None,  4).expect ("Failed to add edge C -> F  4");//G.add_edge("C", "F", weight=4)
        g.add_edge_weighted (String::from ("C"), String::from ("I"), None,  2).expect ("Failed to add edge C -> I  2");//G.add_edge("C", "I", weight=2)
        g.add_edge_weighted (String::from ("D"), String::from ("E"), None,  9).expect ("Failed to add edge D -> E  9");//G.add_edge("D", "E", weight=9)
        g.add_edge_weighted (String::from ("D"), String::from ("F"), None, 14).expect ("Failed to add edge D -> F 14");//G.add_edge("D", "F", weight=14)
        g.add_edge_weighted (String::from ("E"), String::from ("F"), None, 10).expect ("Failed to add edge E -> F 10");//G.add_edge("E", "F", weight=10)
        g.add_edge_weighted (String::from ("F"), String::from ("G"), None,  2).expect ("Failed to add edge F -> G  2");//G.add_edge("F", "G", weight=2)
        g.add_edge_weighted (String::from ("G"), String::from ("H"), None,  1).expect ("Failed to add edge G -> H  1");//G.add_edge("G", "H", weight=1)
        g.add_edge_weighted (String::from ("G"), String::from ("I"), None,  6).expect ("Failed to add edge G -> I  6");//G.add_edge("G", "I", weight=6)
        g.add_edge_weighted (String::from ("H"), String::from ("I"), None,  7).expect ("Failed to add edge H -> I  7");//G.add_edge("H", "I", weight=7)

        let v_a = g.vertex ("A").expect ("Failed to find vertex for 'A'");
        let v_e = g.vertex ("E").expect ("Failed to find vertex for 'E'");

        let sources = collections::HashSet::<usize>::from ([v_a]);
        let r = super::multi_source_dijkstra (g.graph (), &sources).unwrap ();

        let path_ae = r[&v_a].1[&v_e].iter ().map (|x| g.vertex_label (x).expect (&format! ("Failed to find vertex label for '{}'", x))).collect::<Vec<_>> ();
        let weight_ae = r[&v_a].0[&v_e];

        assert_eq! (path_ae, "AHGFE".chars ().map(String::from).collect::<Vec<_>> ());
        assert_eq! (weight_ae, 21);
    }

    #[test]
    fn test_multi_dijkstra_weighted_multi_u ()
    {
        init ();
        let mut g = graph::UGraph::new ();
        // 3     4
        //  \   /
        //   \ /
        //    2
        //    |
        //    |
        //    1
        g.add_edge_raw (2,1,1).expect ("Failed to add edge 2 -> 1 1");
        g.add_edge_raw (3,2,1).expect ("Failed to add edge 3 -> 2 1");
        g.add_edge_raw (4,2,2).expect ("Failed to add edge 4 -> 2 2");

        let solution = collections::HashMap::<usize, (collections::HashMap<usize,i64>, collections::HashMap<usize,Vec<usize>>)>::from ([
            ( 3, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 2 ),
                        ( 2, 1 ),
                        ( 3, 0 ),
                        ( 4, 3 )
                    ]),
                    collections::HashMap::<usize, Vec<usize>>::from ([
                        ( 1, Vec::from ([3, 2, 1]) ),
                        ( 2, Vec::from ([3, 2]) ),
                        ( 3, Vec::from ([3]) ),
                        ( 4, Vec::from ([3, 2, 4]) )
                    ])
                )
            ),
            ( 4, (
                    collections::HashMap::<usize, i64>::from ([
                        ( 1, 3 ),
                        ( 2, 2 ),
                        ( 3, 3 ),
                        ( 4, 0 )
                    ]),
                    collections::HashMap::<usize, Vec<usize>>::from ([
                        ( 1, Vec::from ([4, 2, 1]) ),
                        ( 2, Vec::from ([4, 2]) ),
                        ( 3, Vec::from ([4,2,3,]) ),
                        ( 4, Vec::from ([4]) )
                    ])
                )
            )
        ]);

        let sources = collections::HashSet::<usize>::from ([3,4]);
        let r = super::multi_source_dijkstra (&g, &sources).unwrap ();

        assert_eq! (r, solution);
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
        g.add_edge_raw (1,2,0).expect ("Failed to add edge 1 -> 2");
        g.add_edge_raw (1,3,0).expect ("Failed to add edge 1 -> 3");
        g.add_edge_raw (2,4,0).expect ("Failed to add edge 2 -> 4");
        g.add_edge_raw (2,5,0).expect ("Failed to add edge 2 -> 5");
        g.add_edge_raw (3,6,0).expect ("Failed to add edge 3 -> 6");
        g.add_edge_raw (3,7,0).expect ("Failed to add edge 3 -> 7");

        let solution = collections::HashMap::from ([
            (1, vec![1]),
            (2, vec![1,2]),
            (3, vec![1,3]),
            (4, vec![1,2,4]),
            (5, vec![1,2,5]),
            (6, vec![1,3,6]),
            (7, vec![1,3,7])
        ]);

        let r = super::single_shortest_path (&g, 1).unwrap ();
        assert_eq! (r,solution);
    }
}

