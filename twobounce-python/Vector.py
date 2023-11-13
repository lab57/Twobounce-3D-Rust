from __future__ import annotations
import math
import copy
from math import sin, cos

class Vector2:
    """
    2D vector, used mainly for texture coordiantes (U V)
    """
    
    def __init__(self, u: float, v: float):
        self.u = u
        self.v = v
        self.w = 1 - self.u - self.v
    
    def norm(self) -> Vector2:
        """
        :return: Normalized copy of self
        """
        return self / abs(self)
    
    def dot(self, other: "Vector2") -> float:
        """
        :param other: Vector2
        :return: Dot product of self and other
        """
        return self.u * other.u + self.v * other.v
    
    def __repr__(self):
        return f"<{self.u : .1f}, {self.v : .1f}>"
    
    def __str__(self):
        return f"<{self.u : .1f}, {self.v : .1f}>"
    
    def __sub__(self, other: "Vector2"):
        return Vector2(self.u - other.u, self.v - other.v)
    
    def __add__(self, other: "Vector2"):
        return Vector2(self.u + other.u, self.v + other.v)
    
    def __mul__(self, other: float):
        new = copy.copy(self)
        new.u *= other
        new.v *= other
        return new
    
    def __truediv__(self, other: float):
        new = copy.copy(self)
        new.u /= other
        new.v /= other
        return new
    
    def __abs__(self):
        """
        :return: Norm of self
        """
        return math.sqrt(self.u ** 2 + self.v ** 2)


class Vector:
    def __init__(self, x: float, y: float, z: float):
        """
        Vector class representing a vector in 3-D space.
        @param x:
        @param y:
        @param z:
        """
        self.x: float = x
        self.y: float = y
        self.z: float = z
        self.t = self.x  # alias for other use cases
        self.u = self.y
        self.v = self.z
        self.arr: list[float] = [self.x, self.y, self.z] # array form
    
    def norm(self):
        """
        :return: Normalized copy of self
        """
        return self / abs(self)
    
    def dot(self, other: "Vector") -> float:
        """
        Return dot product of vector with other
        :param other: Vector
        :return: Dot product of self and other
        """
        return self.x * other.x + self.y * other.y + self.z * other.z
    
    def cross(self, other: "Vector") -> "Vector":
        """
        :param other: Vector
        :return: Cross product of self and other
        """
        x = self.y * other.z - self.z * other.y
        y = self.z * other.x - self.x * other.z
        z = self.x * other.y - self.y * other.x
        return Vector(x, y, z)
    
    def calcCoord(self, st: "Vector", t: float) -> Vector:
        """
        :param st: Starting point
        :param t: Parameterization
        :return: 3D coordinates corresponding to parameterization t if self starts at st
        """
        parallel = self - st
        coord = st + self * t
        return coord
    
    def find_t(self, st: Vector, endPt):
        """*Assuming parallel*

        Args:
            st (Vector): _description_
            endPt (_type_): _description_

        Returns:
            _type_: _description_
        """
        diff = endPt - st
        return diff.x/self.x
    
    def rotate(self, axis, angle):
        """Rotate the vector around the given axis by the given angle."""
        # Using Rodrigues' rotation formula
        term1 = self * cos(angle)
        term2 = axis.cross(self) * sin(angle)
        term3 = axis * (axis.dot(self) * (1 - cos(angle)))
        return term1 + term2 + term3
    
    def __repr__(self):
        return f"<{self.x : .1f}, {self.y : .1f}, {self.z : .1f}>"
    
    def __sub__(self, other):
        return Vector(self.x - other.x, self.y - other.y, self.z - other.z)
    
    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)
    
    def __mul__(self, other):
        new = copy.copy(self)
        new.x *= other
        new.y *= other
        new.z *= other
        return new
    
    __rmul__ = __mul__
    
    def __truediv__(self, other):
        new = copy.copy(self)
        new.x /= other
        new.y /= other
        new.z /= other
        return new
    
    def __abs__(self):
        """
        :return: Norm of self
        """
        return math.sqrt(self.x ** 2 + self.y ** 2 + self.z ** 2)
    
    def __getitem__(self, item):
        return self.arr[item]
