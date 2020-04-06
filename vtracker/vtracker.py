from collections import defaultdict

from typing import Iterable, Dict, Tuple, Union, Set, List

from .exceptions import MissingVersion, DuplicateEntity
from .graph import Graph


class VTracker(object):
    str_na = 'Not Present'

    def __init__(self, versions):
        # type: (Iterable[str]) -> None
        """Instantiate the VTracker for the specified versions.

        Parameters
        ----------
        versions: Iterable[str]
            A collection of versions in order of oldest to newest.
        """
        self._ver_to_idx = {v: i for (i, v) in enumerate(versions)}  # type: Dict[str, int]
        self._idx_to_ver = tuple(versions)  # type: Tuple[str]
        self._graph = Graph()  # type: Graph

        # Track the nodes and edges each uid appears in.
        self._uid_to_node = defaultdict(set)
        self._uid_to_edge = defaultdict(set)

    def add(self, uid, ver_states):
        # type: (str, Dict[str, str]) -> None
        """For a uniquely identified entity, add the state at versions.

        Parameters
        ----------
        uid : str
            The unique identifier of this entity.
        ver_states: Dict[str, str]
            The Dict[version, state] of this entity at specified versions.

        Raises
        ------
        MissingVersion
            When a version in ver_states isn't in the tracker.
        DuplicateEntity
            When a duplicate uid is added to the tracker.
        """
        if len(set(ver_states).difference(set(self._ver_to_idx))) > 0:
            raise MissingVersion('Specified version which is not a part of this tracker.')
        if uid in self._uid_to_node:
            raise DuplicateEntity('The specified uid is already in the graph: %s' % uid)

        # Iterate over each expected version.
        for ver in self._idx_to_ver:

            # Check if this uid appears in this version.
            key = (ver, self.str_na) if ver not in ver_states else (ver, ver_states[ver])
            node = self._graph.get_node(key)

            # Create the node associated with this key.
            if node:
                node.attrs['uid'].add(uid)
            else:
                self._graph.add_node(key, attrs={'uid': {uid}})
            self._uid_to_node[uid].add(key)

        # Create each of the edges.
        for i in range(len(self._idx_to_ver) - 1):
            ver_from, ver_to = self._idx_to_ver[i], self._idx_to_ver[i + 1]

            # Check if this uid appears in this or the next version.
            if ver_from not in ver_states:
                key_from = (ver_from, self.str_na)
            else:
                key_from = (ver_from, ver_states[ver_from])
            if ver_to not in ver_states:
                key_to = (ver_to, self.str_na)
            else:
                key_to = (ver_to, ver_states[ver_to])

            # Create the edge associated with this key.
            edge = self._graph.get_edge(key_from, key_to)
            if edge:
                edge.attrs['uid'].add(uid)
            else:
                self._graph.add_edge(key_from, key_to, attrs={'uid': {uid}})
            self._uid_to_edge[uid].add((key_from, key_to))

    def _build_uid_paths(self):
        # type: () -> Tuple[Dict[str, Set[int]], Dict[str, Set[int]]]
        """Create a set of all nodes and links which each uid is a part of.

        Returns
        -------
        Tuple[Dict[str, Set[int]], Dict[str, Set[int]]]
            Returns Dict[uid, Set[node_ids]] for nodes, likewise for edges.
        """
        edges = defaultdict(set)
        nodes = defaultdict(set)
        for uid in self._uid_to_node.keys():
            for edge_from, edge_to in self._uid_to_edge[uid]:
                edge = self._graph.get_edge(edge_from, edge_to)
                edges[uid].add(edge._edge_id)
            for node_key in self._uid_to_node[uid]:
                cur_node = self._graph.get_node(node_key)
                nodes[uid].add(cur_node._node_id)
        return nodes, edges

    def as_sankey_json(self):
        # type: () -> Dict[str, List[dict]]
        """Generate the JSON used for creating a D3 Sankey diagram.

        Returns
        -------
        Dict[str, List[dict]]
            A dictionary formatted for D3.
        """

        # Step 1: Get the core layout and edge/node counts

        # Compute the uid paths
        uid_travel_node_id, uid_travel_edge_id = self._build_uid_paths()

        # Step 2: calculate link highlighting

        out = {'links': list(),
               'nodes': list()}

        # Create each of the nodes in the sankey.
        for node in self._graph.iter_nodes():

            ver, state = node._key
            link_highlight_id = set()
            node_highlight_id = set()

            # Iterate over each of the uids which this
            for uid in node.attrs['uid']:
                node_highlight_id = node_highlight_id.union(uid_travel_node_id[uid])
                link_highlight_id = link_highlight_id.union(uid_travel_edge_id[uid])

            out['nodes'].append({'col': ver,
                                 'id': node._node_id,
                                 'linkHighlightId': list(link_highlight_id),
                                 'name': state,
                                 'nodeHighlightId': list(node_highlight_id),
                                 'total': len(node.attrs['uid'])})

        # Create each of the edges in the sankey.
        for edge in self._graph.iter_edges():
            link_highlight_id = set()
            node_highlight_id = set()

            for uid in edge.attrs['uid']:
                node_highlight_id = node_highlight_id.union(uid_travel_node_id[uid])
                link_highlight_id = link_highlight_id.union(uid_travel_edge_id[uid])

            out['links'].append({'id': edge._edge_id,
                                 'linkHighlightId': list(link_highlight_id),
                                 'nodeHighlightId': list(node_highlight_id),
                                 'source': edge._from_node._node_id,
                                 'target': edge._to_node._node_id,
                                 'value': len(edge.attrs['uid'])})

        return out
