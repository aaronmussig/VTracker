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

from vtracker import VTracker
from vtracker.exceptions import MissingVersion, DuplicateEntity


class TestVTracker(unittest.TestCase):

    def test___init__(self):
        vt = VTracker(('a', 'b', 'c'))
        self.assertDictEqual({'a': 0, 'b': 1, 'c': 2}, vt._ver_to_idx)
        self.assertTupleEqual(('a', 'b', 'c'), vt._idx_to_ver)
        self.assertEqual(0, len(vt._uid_to_node))
        self.assertEqual(0, len(vt._uid_to_edge))

    def test_add(self):
        """
        +--------------+--------+--------------+
        |     1        |   2    |      3       |
        +--------------+--------+--------------+
        | a: x         | a: x   | a: y         |
        |              | b: y,z |              |
        | Missing: y,z |        | Missing: x,z |
        +--------------+--------+--------------+
        """
        vt = VTracker(('1', '2', '3'))
        vt.add('x', {'1': 'a', '2': 'a'})
        vt.add('y', {'2': 'b', '3': 'a'})
        vt.add('z', {'2': 'b'})
        mis = vt.str_na

        # Check the nodes
        self.assertEqual(len(vt._graph._nodes), 6)
        n_1_x = vt._graph.get_node(('1', 'a'))
        self.assertSetEqual(n_1_x._edges_in, set())
        self.assertSetEqual(n_1_x._edges_out, {(('1', 'a'), ('2', 'a'))})
        self.assertSetEqual(n_1_x.attrs['uid'], {'x'})

        n_1_yz = vt._graph.get_node(('1', mis))
        self.assertSetEqual(n_1_yz._edges_in, set())
        self.assertSetEqual(n_1_yz._edges_out, {(('1', mis), ('2', 'b'))})
        self.assertSetEqual(n_1_yz.attrs['uid'], {'y', 'z'})

        n_2_x = vt._graph.get_node(('2', 'a'))
        self.assertSetEqual(n_2_x._edges_in, {(('1', 'a'), ('2', 'a'))})
        self.assertSetEqual(n_2_x._edges_out, {(('2', 'a'), ('3', mis))})
        self.assertSetEqual(n_2_x.attrs['uid'], {'x'})

        n_2_yz = vt._graph.get_node(('2', 'b'))
        self.assertSetEqual(n_2_yz._edges_in, {(('1', mis), ('2', 'b'))})
        self.assertSetEqual(n_2_yz._edges_out, {(('2', 'b'), ('3', 'a')),
                                                (('2', 'b'), ('3', mis))})
        self.assertSetEqual(n_2_yz.attrs['uid'], {'y', 'z'})

        n_3_y = vt._graph.get_node(('3', 'a'))
        self.assertSetEqual(n_3_y._edges_in, {(('2', 'b'), ('3', 'a'))})
        self.assertSetEqual(n_3_y._edges_out, set())
        self.assertSetEqual(n_3_y.attrs['uid'], {'y'})

        n_3_xz = vt._graph.get_node(('3', mis))
        self.assertSetEqual(n_3_xz._edges_in, {(('2', 'a'), ('3', mis)),
                                               (('2', 'b'), ('3', mis))})
        self.assertSetEqual(n_3_xz._edges_out, set())
        self.assertSetEqual(n_3_xz.attrs['uid'], {'x', 'z'})

        # Check the edges
        self.assertEqual(len(vt._graph._edges), 5)
        e_1a_2a = vt._graph.get_edge(('1', 'a'), ('2', 'a'))
        self.assertSetEqual(e_1a_2a.attrs['uid'], {'x'})

        e_mis1_2b = vt._graph.get_edge(('1', mis), ('2', 'b'))
        self.assertSetEqual(e_mis1_2b.attrs['uid'], {'y', 'z'})

        e_2a_3mis = vt._graph.get_edge(('2', 'a'), ('3', mis))
        self.assertSetEqual(e_2a_3mis.attrs['uid'], {'x'})

        e_2b_3mis = vt._graph.get_edge(('2', 'b'), ('3', mis))
        self.assertSetEqual(e_2b_3mis.attrs['uid'], {'z'})

        e_2b_3a = vt._graph.get_edge(('2', 'b'), ('3', 'a'))
        self.assertSetEqual(e_2b_3a.attrs['uid'], {'y'})

        # Check the node indices
        self.assertSetEqual(vt._uid_to_node['x'], {('1', 'a'), ('2', 'a'), ('3', mis)})
        self.assertSetEqual(vt._uid_to_node['y'], {('1', mis), ('2', 'b'), ('3', 'a')})
        self.assertSetEqual(vt._uid_to_node['z'], {('1', mis), ('2', 'b'), ('3', mis)})

        # Check the edge indices
        self.assertSetEqual(vt._uid_to_edge['x'], {(('1', 'a'), ('2', 'a')),
                                                   (('2', 'a'), ('3', mis))})
        self.assertSetEqual(vt._uid_to_edge['y'], {(('1', mis), ('2', 'b')),
                                                   (('2', 'b'), ('3', 'a'))})
        self.assertSetEqual(vt._uid_to_edge['z'], {(('1', mis), ('2', 'b')),
                                                   (('2', 'b'), ('3', mis))})

    def test_add_raises_MissingVersion(self):
        vt = VTracker(('1', '2', '3'))
        self.assertRaises(MissingVersion, vt.add, 'x', {'1': 'a', '9': 'a'})

    def test_add_raises_DuplicateEntity(self):
        vt = VTracker(('1', '2', '3'))
        vt.add('x', {'1': 'a', '2': 'a', '3': 'a'})
        self.assertRaises(DuplicateEntity, vt.add, 'x', {'1': 'a'})

    def test__build_uid_paths(self):
        """
        +--------------+--------+--------------+
        |     1        |   2    |      3       |
        +--------------+--------+--------------+
        | a: x         | a: x   | a: y         |
        |              | b: y,z |              |
        | Missing: y,z |        | Missing: x,z |
        +--------------+--------+--------------+
        """
        vt = VTracker(('1', '2', '3'))
        vt.add('x', {'1': 'a', '2': 'a'})
        vt.add('y', {'2': 'b', '3': 'a'})
        vt.add('z', {'2': 'b'})
        mis = vt.str_na
        nodes, edges = vt._build_uid_paths()

        # Check node highlighting
        node_x = set()
        node_x.add(vt._graph.get_node(('1', 'a'))._node_id)
        node_x.add(vt._graph.get_node(('2', 'a'))._node_id)
        node_x.add(vt._graph.get_node(('3', mis))._node_id)
        self.assertSetEqual(node_x, nodes['x'])

        node_y = set()
        node_y.add(vt._graph.get_node(('1', mis))._node_id)
        node_y.add(vt._graph.get_node(('2', 'b'))._node_id)
        node_y.add(vt._graph.get_node(('3', 'a'))._node_id)
        self.assertSetEqual(node_y, nodes['y'])

        node_z = set()
        node_z.add(vt._graph.get_node(('1', mis))._node_id)
        node_z.add(vt._graph.get_node(('2', 'b'))._node_id)
        node_z.add(vt._graph.get_node(('3', mis))._node_id)
        self.assertSetEqual(node_z, nodes['z'])

        # Check edge highlighting
        edge_x = set()
        edge_x.add(vt._graph.get_edge(('1', 'a'), ('2', 'a'))._edge_id)
        edge_x.add(vt._graph.get_edge(('2', 'a'), ('3', mis))._edge_id)
        self.assertSetEqual(edge_x, edges['x'])

        edge_y = set()
        edge_y.add(vt._graph.get_edge(('1', mis), ('2', 'b'))._edge_id)
        edge_y.add(vt._graph.get_edge(('2', 'b'), ('3', 'a'))._edge_id)
        self.assertSetEqual(edge_y, edges['y'])

        edge_z = set()
        edge_z.add(vt._graph.get_edge(('1', mis), ('2', 'b'))._edge_id)
        edge_z.add(vt._graph.get_edge(('2', 'b'), ('3', mis))._edge_id)
        self.assertSetEqual(edge_z, edges['z'])

    def test_as_sankey_json(self):
        """
        +--------------+--------+--------------+
        |     1        |   2    |      3       |
        +--------------+--------+--------------+
        | a: x         | a: x   | a: y         |
        |              | b: y,z |              |
        | Missing: y,z |        | Missing: x,z |
        +--------------+--------+--------------+
        """
        vt = VTracker(('1', '2', '3'))
        vt.add('x', {'1': 'a', '2': 'a'})
        vt.add('y', {'2': 'b', '3': 'a'})
        vt.add('z', {'2': 'b'})
        mis = vt.str_na
        nodes, edges = vt._build_uid_paths()

        # Convert the sankey results to a dict for easy checking
        sankey_json = vt.as_sankey_json()
        test_nodes = {x['id']: x for x in sankey_json['nodes']}
        test_edges = {x['id']: x for x in sankey_json['links']}

        nodes_exp = dict()
        id_a1 = vt._graph.get_node(('1', 'a'))._node_id
        nodes_exp[id_a1] = {'id': id_a1,
                            'name': 'a',
                            'col': '1',
                            'total': 1,
                            'linkHighlightId': list(edges['x']),
                            'nodeHighlightId': list(nodes['x'])}

        id_mis_1 = vt._graph.get_node(('1', mis))._node_id
        nodes_exp[id_mis_1] = {'id': id_mis_1,
                               'name': mis,
                               'col': '1',
                               'total': 2,
                               'linkHighlightId': list(edges['y'].union(edges['z'])),
                               'nodeHighlightId': list(nodes['y'].union(nodes['z']))}

        id_a2 = vt._graph.get_node(('2', 'a'))._node_id
        nodes_exp[id_a2] = {'id': id_a2,
                            'name': 'a',
                            'col': '2',
                            'total': 1,
                            'linkHighlightId': list(edges['x']),
                            'nodeHighlightId': list(nodes['x'])}

        id_b2 = vt._graph.get_node(('2', 'b'))._node_id
        nodes_exp[id_b2] = {'id': id_b2,
                            'name': 'b',
                            'col': '2',
                            'total': 2,
                            'linkHighlightId': list(edges['y'].union(edges['z'])),
                            'nodeHighlightId': list(nodes['y'].union(nodes['z']))}

        id_a3 = vt._graph.get_node(('3', 'a'))._node_id
        nodes_exp[id_a3] = {'id': id_a3,
                            'name': 'a',
                            'col': '3',
                            'total': 1,
                            'linkHighlightId': list(edges['y']),
                            'nodeHighlightId': list(nodes['y'])}

        id_mis_3 = vt._graph.get_node(('3', mis))._node_id
        nodes_exp[id_mis_3] = {'id': id_mis_3,
                               'name': mis,
                               'col': '3',
                               'total': 2,
                               'linkHighlightId': list(edges['x'].union(edges['z'])),
                               'nodeHighlightId': list(nodes['x'].union(nodes['z']))}

        for exp_node_id, exp_node in nodes_exp.items():
            test_node = test_nodes[exp_node_id]
            self.assertDictEqual(exp_node, test_node)

        edges_exp = dict()
        id_1 = vt._graph.get_edge(('1', 'a'), ('2', 'a'))._edge_id
        edges_exp[id_1] = {'id': id_1,
                           'source': vt._graph.get_node(('1', 'a'))._node_id,
                           'target': vt._graph.get_node(('2', 'a'))._node_id,
                           'value': 1,
                           'linkHighlightId': list(edges['x']),
                           'nodeHighlightId': list(nodes['x'])}

        id_2 = vt._graph.get_edge(('1', mis), ('2', 'b'))._edge_id
        edges_exp[id_2] = {'id': id_2,
                           'source': vt._graph.get_node(('1', mis))._node_id,
                           'target': vt._graph.get_node(('2', 'b'))._node_id,
                           'value': 2,
                           'linkHighlightId': list(edges['y'].union(edges['z'])),
                           'nodeHighlightId': list(nodes['y'].union(nodes['z']))}

        id_3 = vt._graph.get_edge(('2', 'a'), ('3', mis))._edge_id
        edges_exp[id_3] = {'id': id_3,
                           'source': vt._graph.get_node(('2', 'a'))._node_id,
                           'target': vt._graph.get_node(('3', mis))._node_id,
                           'value': 1,
                           'linkHighlightId': list(edges['x']),
                           'nodeHighlightId': list(nodes['x'])}

        id_4 = vt._graph.get_edge(('2', 'b'), ('3', 'a'))._edge_id
        edges_exp[id_4] = {'id': id_4,
                           'source': vt._graph.get_node(('2', 'b'))._node_id,
                           'target': vt._graph.get_node(('3', 'a'))._node_id,
                           'value': 1,
                           'linkHighlightId': list(edges['y']),
                           'nodeHighlightId': list(nodes['y'])}

        id_5 = vt._graph.get_edge(('2', 'b'), ('3', mis))._edge_id
        edges_exp[id_5] = {'id': id_5,
                           'source': vt._graph.get_node(('2', 'b'))._node_id,
                           'target': vt._graph.get_node(('3', mis))._node_id,
                           'value': 1,
                           'linkHighlightId': list(edges['z']),
                           'nodeHighlightId': list(nodes['z'])}

        for exp_edge_id, exp_edge in edges_exp.items():
            test_edge = test_edges[exp_edge_id]
            self.assertDictEqual(exp_edge, test_edge)
