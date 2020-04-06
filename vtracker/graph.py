from typing import Optional, Dict, Tuple, Generator, Set, Hashable

from .exceptions import DuplicateNode, DuplicateEdge


class Graph(object):
    """A basic graph containing nodes and directed edges."""

    def __init__(self):
        """Instantiate a blank graph."""
        self._nodes = dict()  # type: Dict[str, Node]
        self._edges = dict()  # type: Dict[Tuple[str, str], Edge]
        self._node_id = 0  # type: int
        self._edge_id = 0  # type: int

    def add_node(self, key, attrs=None):
        # type: (Hashable, Optional[dict]) -> None
        """Add a node to the graph.

        Parameters
        ----------
        key: str
            The string representing this node.
        attrs: Optional[dict]
            Any attributes to store with this node.

        Raises
        ------
        DuplicateNode
            If a duplicate node is added with different attributes.
        """
        if key not in self._nodes:
            self._nodes[key] = Node(self._node_id, key, attrs)
            self._node_id += 1
        elif attrs != self._nodes[key].attrs:
            raise DuplicateNode('Duplicate node with inconsistent attributes.')

    def get_node(self, key):
        # type: (str) -> Optional[Node]
        """Retrieve a node from the graph.

        Parameters
        ----------
        key: str
            The string representing this node.

        Returns
        -------
        Optional[Node]
            Returns the node, or None if it doesn't exist.
        """
        return self._nodes.get(key)

    def add_edge(self, from_key, to_key, attrs=None):
        # type: (str, str, Optional[dict]) -> None
        """Create a directed edge between two nodes.

        Parameters
        ----------
        from_key: str
            The key of the source node.
        to_key : str
            The key of the destination node.
        attrs : Optional[dict]
            Any attributes to store with this node.

        Raises
        ------
        DuplicateEdge
            If a duplicate edge is added with different attributes.
        """

        if (from_key, to_key) not in self._edges:
            from_node = self.get_node(from_key)
            to_node = self.get_node(to_key)
            self._edges[(from_key, to_key)] = Edge(self._edge_id, from_node, to_node, attrs)
            from_node.add_edge_out(to_key)
            to_node.add_edge_in(from_key)
            self._edge_id += 1
        elif attrs != self._edges[(from_key, to_key)].attrs:
            raise DuplicateEdge('Duplicate edge with inconsistent attributes.')

    def get_edge(self, from_key, to_key):
        # type: (str, str) -> Optional[Edge]
        """Get an edge which connects two nodes.

        Parameters
        ----------
        from_key : str
            The key of the source node.
        to_key : str
            The key of the destination node.

        Returns
        -------
        Optional[Edge]
            Returns the edge, or None if it doesn't exist.
        """
        return self._edges.get((from_key, to_key))

    def iter_nodes(self):
        # type: () -> Generator[Node]
        """Iterate over all nodes in the graph.

        Returns
        -------
        Generator[Node]
            Yields all nodes in the graph."""
        for node in self._nodes.values():
            yield node

    def iter_edges(self):
        # type: () -> Generator[Edge]
        """Iterate over all edges in the graph.

        Returns
        -------
        Generator[Edge]
            Yields all edges in the graph."""
        for edge in self._edges.values():
            yield edge


class Node(object):
    """A basic node in a graph."""

    def __init__(self, node_id, key, attrs):
        # type: (int, str, Optional[dict]) -> None
        """Create a node.

        Parameters
        ----------
        node_id : int
            The unique ID of this node in the graph.
        key: str
            The key which represents this node.
        attrs : Optional[dict]
            Any attributes to store with this node.
        """
        self._node_id = node_id  # type: int
        self._key = key  # type: str
        self.attrs = attrs  # type: Optional[dict]
        self._edges_out = set()  # type: Set[Tuple[str, str]]
        self._edges_in = set()  # type: Set[Tuple[str, str]]

    def add_edge_out(self, to_key):
        # type: (str) -> None
        """Adds an edge from this node node to a destination node.

        Parameters
        ----------
        to_key: str
            The key of the destination node.
        """
        self._edges_out.add((self._key, to_key))

    def add_edge_in(self, from_key):
        # type: (str) -> None
        """Adds an edge from a source node to this node.

        Parameters
        ----------
        from_key: str
            The key of the source node.
        """
        self._edges_in.add((from_key, self._key))


class Edge(object):
    """A directed edge in a graph."""

    def __init__(self, edge_id, from_node, to_node, attrs):
        # type: (int, Node, Node, Optional[dict]) -> None
        """Instantiate the edge.

        Parameters
        ----------
        edge_id: int
            The unique ID of this edge in the graph.
        from_node: Node
            The source node of this edge.
        to_node: Node
            The destination node of this edge.
        attrs : Optional[dict]
            Any attributes to store with this node.
        """
        self._edge_id = edge_id  # type: int
        self._from_node = from_node  # type: Node
        self._to_node = to_node  # type: Node
        self.attrs = attrs  # type: Optional[dict]
