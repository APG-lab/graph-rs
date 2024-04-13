
use crate::error;
use crate::graph;
use std::collections;

pub fn labels_and_attrs_eq (a: &graph::LabelledGraph, b: &graph::LabelledGraph)
    -> Result<bool, error::GraphError>
{
    if a.graph ().vertices ().len () == b.graph ().vertices ().len () && a.graph ().edges ().len () == b.graph ().edges ().len ()
    {
        if a.vertex_labels () == b.vertex_labels ()
        {
            let a_edges = a.edges ().iter ().map (|(e,w)| { Ok ( ( a.vertex_label (&e.0)?, a.vertex_label (&e.1)?, w ) ) }).collect::<Result<collections::HashSet<_>, error::GraphError>> ()?;
            let b_edges = b.edges ().iter ().map (|(e,w)| { Ok ( ( b.vertex_label (&e.0)?, b.vertex_label (&e.1)?, w ) ) }).collect::<Result<collections::HashSet<_>, error::GraphError>> ()?;

            if a_edges == b_edges
            {
                let vertex_attrs_eq = a.vertex_labels ().iter ().map (|vl| Ok (a.vertex_attrs (&vl)?.1 == b.vertex_attrs (&vl)?.1) ).collect::<Result<Vec<_>, error::GraphError>> ()?;
                if vertex_attrs_eq.iter ().all (|&x| x)
                {
                    let edge_attrs_eq = a.edge_labels ()?.iter ().map (|el| Ok (a.edge_attrs ( (&el.0, &el.1) )?.1 == b.edge_attrs ( (&el.0, &el.1) )?.1) ).collect::<Result<Vec<_>, error::GraphError>> ()?;
                    if edge_attrs_eq.iter ().all (|&x| x)
                    {
                        Ok (true)
                    }
                    else
                    {
                        Ok (false)
                    }
                }
                else
                {
                    Ok (false)
                }
            }
            else
            {
                Ok (false)
            }
        }
        else
        {
            Ok (false)
        }
    }
    else
    {
        Ok (false)
    }
}

pub fn labels_and_attrs_retain_eq (a: &graph::LabelledGraph, b: &graph::LabelledGraph, retain_vertex_attrs: Option<&collections::HashSet<String>>, retain_edge_attrs: Option<&collections::HashSet<String>>)
    -> Result<bool, error::GraphError>
{
    if a.graph ().vertices ().len () == b.graph ().vertices ().len () && a.graph ().edges ().len () == b.graph ().edges ().len ()
    {
        if a.vertex_labels () == b.vertex_labels ()
        {
            let a_edges = a.edges ().iter ().map (|(e,w)| { Ok ( ( a.vertex_label (&e.0)?, a.vertex_label (&e.1)?, w ) ) }).collect::<Result<collections::HashSet<_>, error::GraphError>> ()?;
            let b_edges = b.edges ().iter ().map (|(e,w)| { Ok ( ( b.vertex_label (&e.0)?, b.vertex_label (&e.1)?, w ) ) }).collect::<Result<collections::HashSet<_>, error::GraphError>> ()?;

            if a_edges == b_edges
            {
                let vertex_attrs_eq = a.vertex_labels ()
                    .iter ()
                    .map (|vl| {
                        if let Some (rv) = &retain_vertex_attrs
                        {
                            let mut vaa = a.vertex_attrs (&vl)?.1.clone ();
                            let mut vab = b.vertex_attrs (&vl)?.1.clone ();

                            vaa.retain (|k, _| rv.contains (k.as_str ()));
                            vab.retain (|k, _| rv.contains (k.as_str ()));
                            Ok (vaa == vab)
                        }
                        else
                        {
                            let vaa = a.vertex_attrs (&vl)?.1;
                            let vab = b.vertex_attrs (&vl)?.1;
                            Ok (vaa == vab)
                        }
                    }).collect::<Result<Vec<_>, error::GraphError>> ()?;
                if vertex_attrs_eq.iter ().all (|&x| x)
                {
                    let edge_attrs_eq = a.edge_labels ()?
                        .iter ()
                        .map (|el| {
                            if let Some (re) = &retain_edge_attrs
                            {
                                let mut eaa = a.edge_attrs ( (&el.0, &el.1) )?.1.clone ();
                                let mut eab = b.edge_attrs ( (&el.0, &el.1) )?.1.clone ();

                                eaa.retain (|k, _| re.contains (k.as_str ()));
                                eab.retain (|k, _| re.contains (k.as_str ()));

                                Ok ( eaa == eab )
                            }
                            else
                            {
                                let eaa = a.edge_attrs ( (&el.0, &el.1) )?.1;
                                let eab = b.edge_attrs ( (&el.0, &el.1) )?.1;

                                Ok ( eaa == eab )
                            }
                        }).collect::<Result<Vec<_>, error::GraphError>> ()?;
                    if edge_attrs_eq.iter ().all (|&x| x)
                    {
                        Ok (true)
                    }
                    else
                    {
                        Ok (false)
                    }
                }
                else
                {
                    Ok (false)
                }
            }
            else
            {
                Ok (false)
            }
        }
        else
        {
            Ok (false)
        }
    }
    else
    {
        Ok (false)
    }
}


#[cfg(test)]
mod tests
{
    use crate::graph;
    use std::collections;

    use std::sync;

    static INIT: sync::Once = sync::Once::new ();

    fn init ()
    {
        INIT.call_once (env_logger::init);
    }

    fn attrs_a ()
        -> collections::HashMap<String, graph::AttributeValue>
    {
        let bool_map = collections::HashMap::<String, bool>::from ([ (String::from ("v"), true) ]);
        let string_vec = vec![String::from ("a"), String::from ("b")];
        let string_map = collections::HashMap::<String, String>::from ([ (String::from ("v"), String::from ("a")) ]);
        let string_set = collections::HashSet::<String>::from ([String::from ("a")]);

        collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("boolean"), graph::AttributeValue::BooleanLiteral (true) ),
            ( String::from ("bool_map"), graph::AttributeValue::BooleanMap (bool_map) ),
            ( String::from ("integer"), graph::AttributeValue::IntegerLiteral (0)),
            ( String::from ("float"), graph::AttributeValue::FloatLiteral (0.0)),
            ( String::from ("string_array"), graph::AttributeValue::StringArray (string_vec)),
            ( String::from ("string"), graph::AttributeValue::StringLiteral (String::from ("a"))),
            ( String::from ("string_map"), graph::AttributeValue::StringMap (string_map)),
            ( String::from ("string_set"), graph::AttributeValue::StringSet (string_set))
        ])
    }

    fn attrs_b ()
        -> collections::HashMap<String, graph::AttributeValue>
    {
        let bool_map = collections::HashMap::<String, bool>::from ([ (String::from ("v"), false) ]);
        let string_vec = vec![String::from ("b"), String::from ("a")];
        let string_map = collections::HashMap::<String, String>::from ([ (String::from ("v"), String::from ("b")) ]);
        let string_set = collections::HashSet::<String>::from ([String::from ("b")]);

        collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("boolean"), graph::AttributeValue::BooleanLiteral (false) ),
            ( String::from ("bool_map"), graph::AttributeValue::BooleanMap (bool_map) ),
            ( String::from ("integer"), graph::AttributeValue::IntegerLiteral (1)),
            ( String::from ("float"), graph::AttributeValue::FloatLiteral (1.0)),
            ( String::from ("string_array"), graph::AttributeValue::StringArray (string_vec)),
            ( String::from ("string"), graph::AttributeValue::StringLiteral (String::from ("b"))),
            ( String::from ("string_map"), graph::AttributeValue::StringMap (string_map)),
            ( String::from ("string_set"), graph::AttributeValue::StringSet (string_set))
        ])
    }

    #[test]
    fn empty ()
    {
        init ();
        let a = graph::LabelledGraph::new ();
        let b = graph::LabelledGraph::new ();
        assert! (super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "Empty graphs not equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "Empty graphs not equal");
    }

    #[test]
    fn vertex_label ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        assert! (super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "With one vertex should be equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "With one vertex should be equal");
    }

    #[test]
    fn vertex_label_ne ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");

        assert! (!super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "With one vertex should not be equal");
        assert! (!super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "With one vertex should not be equal");
    }

    #[test]
    fn ignores_vertex_id ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        assert_eq! (a.vertex ("vertex_one").expect ("Failed to obtain vertex id"), b.vertex ("vertex_two").expect ("Failed to obtain vertex id"), "Vertex Id of first node not equal");
        assert_eq! (a.vertex ("vertex_two").expect ("Failed to obtain vertex id"), b.vertex ("vertex_one").expect ("Failed to obtain vertex id"), "Vertex Id of second node not equal");

        assert! (super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "Ignores vertex id should be equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "Ignores vertex id should be equal");
    }

    #[test]
    fn edge ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        a.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");
        b.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");

        assert! (super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "edge should be equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "edge should be equal");
    }

    #[test]
    fn edge_ne ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        a.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");
        b.add_edge (String::from ("vertex_two"), String::from ("vertex_one"), None).expect ("Failed to add edge");

        assert! (!super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "edge should not be equal");
        assert! (!super::labels_and_attrs_retain_eq (&a, &b, None, None).expect ("Failed eq check"), "edge should not be equal");
    }

    #[test]
    fn vertex_attrs ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        let attrs_a = attrs_a ();

        for attr_name in vec!["boolean", "bool_map", "integer", "float","string", "string_array", "string_map", "string_set"]
        {
            let mut a_copy = a.clone ();
            let mut b_copy = b.clone ();

            a_copy.vertex_attrs_mut ("vertex_one").expect ("Failed to get vertex_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());
            b_copy.vertex_attrs_mut ("vertex_one").expect ("Failed to get vertex_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());

            assert! (super::labels_and_attrs_eq (&a_copy, &b_copy).expect ("Failed eq check"), "Vertex attrs should be equal. Failed '{}'", attr_name);
            assert! (super::labels_and_attrs_retain_eq (&a_copy, &b_copy, None, None).expect ("Failed eq check"), "Vertex attrs should be equal. Failed '{}'", attr_name);
        }
    }

    #[test]
    fn vertex_attrs_ne ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        let attrs_a = attrs_a ();
        let attrs_b = attrs_b ();

        for attr_name in vec!["boolean", "bool_map", "integer", "float","string", "string_array", "string_map", "string_set"]
        {
            let mut a_copy = a.clone ();
            let mut b_copy = b.clone ();

            a_copy.vertex_attrs_mut ("vertex_one").expect ("Failed to get vertex_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());
            b_copy.vertex_attrs_mut ("vertex_one").expect ("Failed to get vertex_attrs_mut").1.insert (attr_name.to_string (), attrs_b[attr_name].clone ());

            assert! (!super::labels_and_attrs_eq (&a_copy, &b_copy).expect ("Failed eq check"), "Vertex attrs should not be equal. Failed '{}'", attr_name);
            assert! (!super::labels_and_attrs_retain_eq (&a_copy, &b_copy, None, None).expect ("Failed eq check"), "Vertex attrs should not be equal. Failed '{}'", attr_name);
        }
    }

    #[test]
    fn edge_attrs ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        a.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");
        b.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");

        let attrs_a = attrs_a ();

        for attr_name in vec!["boolean", "bool_map", "integer", "float","string", "string_array", "string_map", "string_set"]
        {
            let mut a_copy = a.clone ();
            let mut b_copy = b.clone ();

            a_copy.edge_attrs_mut ( ("vertex_one", "vertex_two") ).expect ("Failed to get edge_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());
            b_copy.edge_attrs_mut ( ("vertex_one", "vertex_two") ).expect ("Failed to get edge_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());

            assert! (super::labels_and_attrs_eq (&a_copy, &b_copy).expect ("Failed eq check"), "Edge attrs should be equal. Failed '{}'", attr_name);
            assert! (super::labels_and_attrs_retain_eq (&a_copy, &b_copy, None, None).expect ("Failed eq check"), "Edge attrs should be equal. Failed '{}'", attr_name);
        }
    }

    #[test]
    fn edge_attrs_ne ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");

        a.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");
        b.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), None).expect ("Failed to add edge");

        let attrs_a = attrs_a ();
        let attrs_b = attrs_b ();

        for attr_name in vec!["boolean", "bool_map", "integer", "float","string", "string_array", "string_map", "string_set"]
        {
            let mut a_copy = a.clone ();
            let mut b_copy = b.clone ();

            a_copy.edge_attrs_mut ( ("vertex_one", "vertex_two") ).expect ("Failed to get edge_attrs_mut").1.insert (attr_name.to_string (), attrs_a[attr_name].clone ());
            b_copy.edge_attrs_mut ( ("vertex_one", "vertex_two") ).expect ("Failed to get edge_attrs_mut").1.insert (attr_name.to_string (), attrs_b[attr_name].clone ());

            assert! (!super::labels_and_attrs_eq (&a_copy, &b_copy).expect ("Failed eq check"), "Edge attrs should not be equal. Failed '{}'", attr_name);
            assert! (!super::labels_and_attrs_retain_eq (&a_copy, &b_copy, None, None).expect ("Failed eq check"), "Edge attrs should not be equal. Failed '{}'", attr_name);
        }
    }

    #[test]
    fn vertex_attrs_retain ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        let attrs_a = collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("equal"), graph::AttributeValue::BooleanLiteral (true) ),
            ( String::from ("not_equal"), graph::AttributeValue::BooleanLiteral (true) )
        ]);

        let attrs_b = collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("equal"), graph::AttributeValue::BooleanLiteral (true) ),
            ( String::from ("not_equal"), graph::AttributeValue::BooleanLiteral (false) )
        ]);

        a.add_vertex (String::from ("vertex_one"), Some (attrs_a)).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), Some (attrs_b)).expect ("Failed to add vertex");

        let keep = collections::HashSet::<String>::from ([String::from ("equal")]);

        assert! (!super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "Should not be equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, Some (&keep), None).expect ("Failed eq check"), "Should be equal");
    }

    #[test]
    fn edge_attrs_retain ()
    {
        init ();
        let mut a = graph::LabelledGraph::new ();
        let mut b = graph::LabelledGraph::new ();

        let attrs_a = collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("equal"), graph::AttributeValue::BooleanLiteral (true) ),
            ( String::from ("not_equal"), graph::AttributeValue::BooleanLiteral (true) )
        ]);

        let attrs_b = collections::HashMap::<String, graph::AttributeValue>::from ([
            ( String::from ("equal"), graph::AttributeValue::BooleanLiteral (true) ),
            ( String::from ("not_equal"), graph::AttributeValue::BooleanLiteral (false) )
        ]);

        a.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        a.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_one"), None).expect ("Failed to add vertex");
        b.add_vertex (String::from ("vertex_two"), None).expect ("Failed to add vertex");

        a.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), Some (attrs_a)).expect ("Failed to add edge");
        b.add_edge (String::from ("vertex_one"), String::from ("vertex_two"), Some (attrs_b)).expect ("Failed to add edge");

        let keep = collections::HashSet::<String>::from ([String::from ("equal")]);

        assert! (!super::labels_and_attrs_eq (&a, &b).expect ("Failed eq check"), "Should not be equal");
        assert! (super::labels_and_attrs_retain_eq (&a, &b, None, Some (&keep)).expect ("Failed eq check"), "Should be equal");
    }
}

