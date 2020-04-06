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


class VTrackerException(Exception):
    """Base exception for all VTracker exceptions thrown."""

    def __init__(self, message=''):
        Exception.__init__(self, message)


class MissingVersion(VTrackerException):
    """Thrown when a version is specified which doesn't exist."""

    def __init__(self, message=''):
        VTrackerException.__init__(self, message)


class DuplicateEntity(VTrackerException):
    """Thrown when a duplicate entity is added to the tracker.."""

    def __init__(self, message=''):
        VTrackerException.__init__(self, message)


class GraphException(VTrackerException):
    """Base exception for all VTracker graph exceptions thrown."""

    def __init__(self, message=''):
        VTrackerException.__init__(self, message)


class DuplicateNode(GraphException):
    """Thrown when a non-identical duplicate node is added to the graph."""

    def __init__(self, message=''):
        GraphException.__init__(self, message)


class DuplicateEdge(GraphException):
    """Thrown when a non-identical duplicate edge is added to the graph."""

    def __init__(self, message=''):
        GraphException.__init__(self, message)
