###############################################################################
#                                                                             #
#    This program is free software: you can redistribute it and/or modify     #
#    it under the terms of the GNU General Public License as published by     #
#    the Free Software Foundation, either version 3 of the License, or        #
#    (at your option) any later version.                                      #
#                                                                             #
#    This program is distributed in the hope that it will be useful,          #
#    but WITHOUT ANY WARRANTY; without even the implied warranty of           #
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the            #
#    GNU General Public License for more details.                             #
#                                                                             #
#    You should have received a copy of the GNU General Public License        #
#    along with this program. If not, see <http://www.gnu.org/licenses/>.     #
#                                                                             #
###############################################################################

import unittest

from vtracker.exceptions import DuplicateNode, DuplicateEdge
from vtracker.graph import Graph, Node, Edge


class TestGraph(unittest.TestCase):

    def test_add_node(self):
        g = Graph()
        g.add_node('a', attrs={'foo': True})
        g.add_node('b', attrs={'bar': True})

        # Test adding the same node with same/different attributes
        g.add_node('b', attrs={'bar': True})
        self.assertRaises(DuplicateNode, g.add_node, 'a')

        node_a = g.get_node('a')
        node_b = g.get_node('b')

        self.assertSetEqual(node_a._edges_in, set())
        self.assertSetEqual(node_b._edges_in, set())
        self.assertEqual(node_a._key, 'a')
        self.assertEqual(node_b._key, 'b')
        self.assertEqual(node_a._node_id, 0)
        self.assertEqual(node_b._node_id, 1)
        self.assertDictEqual(node_a.attrs, {'foo': True})
        self.assertDictEqual(node_b.attrs, {'bar': True})

        self.assertEqual(g._edge_id, 0)
        self.assertDictEqual(g._edges, {})
        self.assertEqual(g._node_id, 2)
        self.assertDictEqual(g._nodes, {'a': g.get_node('a'), 'b': g.get_node('b')})

    def test_get_node(self):
        g = Graph()
        g.add_node('a', attrs={'foo': True})

        node_a = g.get_node('a')
        self.assertSetEqual(node_a._edges_in, set())
        self.assertEqual(node_a._key, 'a')
        self.assertEqual(node_a._node_id, 0)
        self.assertDictEqual(node_a.attrs, {'foo': True})

        self.assertIsNone(g.get_node('x'))

    def test_add_edge(self):
        g = Graph()
        g.add_node('a', attrs={'x': True})
        g.add_node('b', attrs={'y': True})
        g.add_node('c', attrs={'z': False})
        g.add_edge('a', 'b', attrs={'baz': 0})
        g.add_edge('b', 'c', attrs={'baz': 1})

        # Test adding the same edge with same/different attributes
        g.add_edge('a', 'b', attrs={'baz': 0})
        self.assertRaises(DuplicateEdge, g.add_edge, 'a', 'b', attrs={'foo': 1})

        edge_a = g.get_edge('a', 'b')
        edge_b = g.get_edge('b', 'c')

        self.assertEqual(edge_a._edge_id, 0)
        self.assertEqual(edge_b._edge_id, 1)
        self.assertEqual(edge_a._from_node, g.get_node('a'))
        self.assertEqual(edge_b._from_node, g.get_node('b'))
        self.assertEqual(edge_a._to_node, g.get_node('b'))
        self.assertEqual(edge_b._to_node, g.get_node('c'))
        self.assertDictEqual(edge_a.attrs, {'baz': 0})
        self.assertDictEqual(edge_b.attrs, {'baz': 1})

        self.assertEqual(g._edge_id, 2)
        self.assertDictEqual(g._edges, {('a', 'b'): g.get_edge('a', 'b'),
                                        ('b', 'c'): g.get_edge('b', 'c')})
        self.assertEqual(g._node_id, 3)
        self.assertDictEqual(g._nodes, {'a': g.get_node('a'), 'b': g.get_node('b'),
                                        'c': g.get_node('c')})

    def test_get_edge(self):
        g = Graph()
        g.add_node('a', attrs={'x': True})
        g.add_node('b', attrs={'y': True})
        g.add_node('c', attrs={'z': False})
        g.add_edge('a', 'b', attrs={'baz': 0})
        g.add_edge('b', 'c', attrs={'baz': 1})

        edge_b = g.get_edge('b', 'c')
        self.assertEqual(edge_b._edge_id, 1)
        self.assertEqual(edge_b._from_node, g.get_node('b'))
        self.assertEqual(edge_b._to_node, g.get_node('c'))
        self.assertDictEqual(edge_b.attrs, {'baz': 1})

        self.assertIsNone(g.get_edge('a', 'x'))

    def test_iter_nodes(self):
        g = Graph()
        g.add_node('a')
        g.add_node('a')
        g.add_node('b')
        g.add_node('c')
        self.assertEqual(len(list(g.iter_nodes())), 3)

    def test_iter_edges(self):
        g = Graph()
        g.add_node('a', attrs={'x': True})
        g.add_node('b', attrs={'y': True})
        g.add_node('c', attrs={'z': False})
        g.add_edge('a', 'b', attrs={'baz': 0})
        g.add_edge('a', 'b', attrs={'baz': 0})
        g.add_edge('b', 'c', attrs={'baz': 1})
        self.assertEqual(len(list(g.iter_edges())), 2)


class TestNode(unittest.TestCase):

    def test___init__(self):
        n = Node(0, 'a', {'foo': True})
        self.assertEqual(n._node_id, 0)
        self.assertEqual(n._key, 'a')
        self.assertDictEqual(n.attrs, {'foo': True})
        self.assertSetEqual(n._edges_out, set())
        self.assertSetEqual(n._edges_in, set())

    def test_add_edge_out_in(self):
        n_a = Node(0, 'a', None)
        n_b = Node(1, 'b', None)
        n_a.add_edge_out('b')
        n_b.add_edge_in('a')
        self.assertSetEqual(n_a._edges_out, {('a', 'b')})
        self.assertSetEqual(n_b._edges_in, {('a', 'b')})


class TestEdge(unittest.TestCase):

    def test___init__(self):
        n_a = Node(0, 'a', None)
        n_b = Node(1, 'b', None)
        edge = Edge(0, n_a, n_b, {'foo': True})
        self.assertEqual(edge._edge_id, 0)
        self.assertEqual(edge._from_node, n_a)
        self.assertEqual(edge._to_node, n_b)
        self.assertDictEqual(edge.attrs, {'foo': True})
