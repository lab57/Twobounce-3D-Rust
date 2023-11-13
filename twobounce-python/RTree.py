from __future__ import annotations
import math
from Vector import *
from Triangle import *

class BoundingBox:
    def __init__(self, min_point: list[float] | Vector, max_point: list[float] | Vector):
        """Bounding box used for R-tree construction and searching.

        Args:
            min_point (list[float] | Vector): _description_
            max_point (list[float] | Vector): _description_
        """
        self.min_point = min_point
        self.max_point = max_point

    def intersects(self, origin: Vector, direction: Vector)-> bool:
        def compute_t(plane_coord, origin_coord, direction_coord)->float:
                if direction_coord == 0:
                    if origin_coord < plane_coord:
                        return float('inf')
                    else:
                        return float('-inf')
                return (plane_coord - origin_coord) / direction_coord

        t1 = compute_t(self.min_point[0], origin[0], direction[0])
        t2 = compute_t(self.max_point[0], origin[0], direction[0])
        t3 = compute_t(self.min_point[1], origin[1], direction[1])
        t4 = compute_t(self.max_point[1], origin[1], direction[1])
        t5 = compute_t(self.min_point[2], origin[2], direction[2])
        t6 = compute_t(self.max_point[2], origin[2], direction[2])

        tmin: float = max(min(t1, t2), min(t3, t4), min(t5, t6))
        tmax: float = min(max(t1, t2), max(t3, t4), max(t5, t6))
        return tmax > max(tmin, 0)


class RTreeNode:
    def __init__(self, bounding_box: BoundingBox, triangles: list[Triangle] | None =None, children: list[RTreeNode] | None =None):
        """R-Tree node.

        Args:
            bounding_box (BoundingBox): _description_
            triangles (_type_, optional): _description_. Defaults to None.
            children (_type_, optional): _description_. Defaults to None.
        """
        self.bounding_box = bounding_box
        self.triangles = triangles if triangles is not None else []
        self.children = children if children is not None else []


class RTree:
    def __init__(self, triangles: list[Triangle], max_triangles_per_leaf: int =8):
        self.nnodes = 0
        self.root: RTreeNode | None = self.build_tree(triangles, max_triangles_per_leaf)
        self.triangles = triangles

    def build_tree(self, triangles: list[Triangle], max_triangles_per_leaf):
        if len(triangles) == 0:
            return
        #self.nnodes += 1
        #print(len(triangles), max_triangles_per_leaf)
        if len(triangles) <= max_triangles_per_leaf:
            min_point, max_point = self.compute_bounds(triangles)
            bounding_box = BoundingBox(min_point, max_point)
            #print("new node!")
            self.nnodes += 1
            return RTreeNode(bounding_box, triangles=triangles)

        # Choose an axis to split along
        axis = self.choose_split_axis(triangles)

        # Sort triangles along the chosen axis
        triangles.sort(key=lambda t: t.a.arr[axis])

        # Split triangles into two halves
        mid = len(triangles) // 2
        left_triangles = triangles[:mid]
        right_triangles = triangles[mid:]

        left_child = self.build_tree(left_triangles, max_triangles_per_leaf)
        right_child = self.build_tree(right_triangles, max_triangles_per_leaf)

        min_point = [
            min(
                left_child.bounding_box.min_point[i],
                right_child.bounding_box.min_point[i],
            )
            for i in range(3)
        ]
        max_point = [
            max(
                left_child.bounding_box.max_point[i],
                right_child.bounding_box.max_point[i],
            )
            for i in range(3)
        ]
        bounding_box = BoundingBox(min_point, max_point)

        return RTreeNode(bounding_box, children=[left_child, right_child])

    def compute_bounds(self, triangles: list[Triangle]):
        min_point = [math.inf] * 3
        max_point = [-math.inf] * 3

        for triangle in triangles:
            for vertex in [triangle.a, triangle.b, triangle.c]:
                for i in range(3):
                    min_point[i] = min(min_point[i], vertex.arr[i])
                    max_point[i] = max(max_point[i], vertex.arr[i])

        return min_point, max_point

    def choose_split_axis(self, triangles: list[Triangle]):
        extents = []
        for axis in range(3):
            min_coord, max_coord = zip(
                *[(t.a.arr[axis], t.c.arr[axis]) for t in triangles]
            )
            extents.append(max(max_coord) - min(min_coord))
        return extents.index(max(extents))

    def query_ray(self, origin: Vector, direction: Vector):
        return self._query_ray_recursive(self.root, origin, direction)

    def _query_ray_recursive(self, node: RTreeNode, origin: Vector, direction: Vector):
        if not node.bounding_box.intersects(origin, direction):
            return []

        if node.children:
            result = []
            for child in node.children:
                result.extend(self._query_ray_recursive(child, origin, direction))
            return result
        else:
            return node.triangles
        
    def checkIntersections(self, st: Vector, dir: Vector)-> Hit:
        """Checks intersections and returns the Hit object that has
        the minimum t value (first object of intersection)

        Args:
            st (Vector): Starting point of vec
            dir (Vector): Direction vector

        Returns:
            Hit : Return hit that has nones for most values, only start and dir 
            are populated
        """
        min_t = math.inf
        best_hit = Hit(False, origin=st, dir=dir) #Hit(None, None, None, None, None, st, dir)
        triangles = self.query_ray(st, dir)
        for tri in triangles:
            newHit: Hit = tri.intersect(st, dir)
            if(newHit.t and newHit.t < min_t):
                min_t = newHit.t
                best_hit = newHit
        return best_hit




if __name__ == "__main__":
    triangles = []  # List of Triangle objects
    rtree = RTree(triangles)
    origin = [0, 0, 0]
    direction = [1, 1, 1]
    intersected_triangles = rtree.query_ray(origin, direction)