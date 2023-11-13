from RTree import *
from Vector import *
import numpy as np

import math




class DiskDetector:
    def __init__(self, R: float, center: Vector, normal: Vector, n=500) -> None:
        self.R = R
        self.center = center
        self.normal = normal
        self.surfacePoints: list[Vector] = self.getSurfacePoints(n)
        print(f"Detector points {self.surfacePoints}")

    def getRotationAxisAndAngle(self, initial_normal: Vector, target_normal:Vector):
        """Compute the rotation axis and angle to align initial_normal with target_normal."""
        # Normalize the vectors
        initial_normal = initial_normal.norm()
        target_normal = target_normal.norm()
        #print(initial_normal.dot(target_normal) - 1)
        if (initial_normal.dot(target_normal) - 1)**2 < 1e-8:
            dot_product = initial_normal.dot(target_normal)
            if dot_product > 0:
                # No rotation needed
                return Vector(0, 0, 0), 0
            else:
                # 180-degree rotation around any axis perpendicular to the normals
                return Vector(1, 0, 0), np.pi

        # Compute rotation axis (it's perpendicular to the plane formed by initial_normal and target_normal)
        rotation_axis = initial_normal.cross(target_normal).norm()


        # If the rotation axis has a magnitude of 0, it means the initial and target normals are the same or opposite.

        # Compute rotation angle using dot product
        cos_angle = initial_normal.dot(target_normal)
        rotation_angle = np.arccos(cos_angle)

        return rotation_axis, rotation_angle

    def getSurfacePoints(self, n)-> list[Vector]:
        points = []

        # Calculate the side length of the squares in the grid
        side_length = 2 * self.R / np.sqrt(n)

        # Number of grid points along one dimension
        num_points = int(2 * self.R / side_length)
        opoints = []
        for i in range(num_points):
            for j in range(num_points):
                x = -self.R + i * side_length
                y = -self.R + j * side_length

                # Check if the point is inside the disk
                if x**2 + y**2 <= self.R**2:
                    point = Vector(x, y, 0)  # This is in the disk's local coordinates
                    #print("pt", point)
                    opoints.append(point)
                    # Rotate point based on disk's normal
                    rotation_axis, rotation_angle = self.getRotationAxisAndAngle(
                        Vector(0, 0, 1), self.normal
                    )
                    #print("angle, axis", rotation_angle, rotation_axis)
                    #print("1", point)
                    point = point.rotate(rotation_axis, rotation_angle)
                    #print("2", point)
                    # Translate to disk's position
                    point = point + self.center

                    points.append(point)

        return points
    
    def isVisible(self, rtree: RTree, source: Vector):
        for point in self.surfacePoints:
            dir = point - source
            hit = rtree.checkIntersections(source, dir)
            #print(hit)
            if(hit.tri is None or (hit.didHit and dir.find_t(source, point) < hit.t)):
                return True
        return False
    

    










