from __future__ import annotations
from Vector import *
from dataclasses import dataclass
import numpy as np

@dataclass
class Hit:
    """
    tri: Triangle that was hit
    obj: Object that was hit
    u, v: uv coordinates (local triangle coordinates, 0-1)
    t: paramterized t at which hit occured
    origin: start of ray
    dir: direction vector of ray
    """
    tri: Triangle | None
    obj: TriObject | None
    u: float | None
    v: float | None
    t: float | None
    origin: Vector | None
    dir: Vector | None
    didHit: bool
    def __init__(self, didHit: bool, 
                 tri: Triangle | None  = None,
                 obj: TriObject | None = None,
                 u: float | None       = None,
                 v: float | None       = None,
                 t: float | None       = None,
                 origin: Vector| None  = None,
                 dir: Vector | None    = None
                 ):
        self.didHit = didHit
        self.tri = tri
        self.obj = obj
        self.u = u
        self.v = v
        self.t = t
        self.origin = origin
        self.dir = dir
    
    def getPixel(self)-> Pixel:
        """
        Get the pixel that the hit falls within.
        @return:
        """
        if(self.obj is None or self.u is None or self.tri is None or self.v is None):
            raise ValueError("Hit has no object")
        
        texture_coords = np.array([self.tri.at, self.tri.bt, self.tri.ct])
        hit_pt = np.array([1 - self.u - self.v, self.u, self.v])
        texture_location = np.dot(hit_pt, texture_coords )
        x = int(self.obj.resolution * texture_location[0])
        y = int(self.obj.resolution*(1 - texture_location[1]))
        return self.obj.texture[y][x]
    def cartesian(self):
        if(self.t is None):
            raise ValueError("Hit did not terminate")
        
        return self.dir.calcCoord(self.origin, self.t)



class Pixel:
    def __init__(self) -> None:
        self.status = 0  # 0 not hit, 1 in view of source, 2 in view of both
    def getColor(self):
        green = (0, 255, 0)
        red = (255, 0, 0)
        grey = (150, 150, 150)
        if(self.status == 0):
            return grey
        elif(self.status == 1):
            return green
        elif (self.status == 2):
            return red
        

class Triangle:
    def __init__(self, coords: list[Vector]):
        self.a: Vector = coords[0]
        self.b: Vector = coords[1]
        self.c: Vector = coords[2]
        self.at: Vector2 = None
        self.bt: Vector2 = None
        self.ct: Vector2 = None
        self.object = None
        self.normal = None
    
    def setTexture(self, at, bt, ct):
        self.at = at
        self.bt = bt
        self.ct = ct


    def getArea(self) -> float:
        return 1 / 2 * abs(((self.a - self.b).cross(self.a - self.c)))
    
    def intersect(self, ray_start, ray_vec) -> Hit:  # tuple[bool, Vector]:
        """
        Detect intersection between this triangle and ray_vec originating from ray_start

        :param ray_start: Vector indicating the origin of the ray
        :param ray_vec: Vector indicating direction of the ray
        :return: Hit object
        
        DEPRECATED:
        bool indicating if it was a hit, and vector indicating parameterized t, and local coordinates u, v
        """
        # define a null intersection
        # null_inter = [None, None, None]  # np.array([np.nan, np.nan, np.nan])
        # null_inter = Vector(None, None, None)
        # ray_start = np.asarray(ray_start)
        # ray_vec = np.asarray(ray_vec)
        null_hit = Hit(False, origin=ray_start, dir=ray_vec) #Hit(None, None, None, None, None, ray_start, ray_vec)
        # break down triangle into the individual points
        v1, v2, v3 = self.a, self.b, self.c
        eps = 0.000001
        
        # compute edges
        edge1 = v2 - v1
        edge2 = v3 - v1
        # pvec = np.cross(ray_vec, edge2)
        pvec = ray_vec.cross(edge2)
        det = edge1.dot(pvec)
        
        if abs(det) < eps:  # no intersection
            # print('fail1')
            return null_hit
        inv_det = 1.0 / det
        tvec = ray_start - v1
        u = tvec.dot(pvec) * inv_det
        # print(u)
        if u < 0.0 or u > 1.0:  # if not intersection
            # print('fail2')
            return null_hit
        
        qvec = tvec.cross(edge1)
        v = ray_vec.dot(qvec) * inv_det
        if v < 0.0 or u + v > 1.0:  # if not intersection
            #  print('fail3')
            return null_hit
        
        t = edge2.dot(qvec) * inv_det
        if t < eps:
            #   print('fail4')
            return null_hit
        return Hit(True, self, self.object, u, v, t, ray_start, ray_vec)
    def plot(self):
        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')
        
        # Extracting the x, y, and z coordinates of the triangle vertices
        x = [self.a.x, self.b.x, self.c.x]
        y = [self.a.y, self.b.y, self.c.y]
        z = [self.a.z, self.b.z, self.c.z]
        
        # Plotting the triangle
        verts = [list(zip(x, y, z))]
        ax.add_collection3d(Poly3DCollection(verts, facecolors='cyan', linewidths=1, edgecolors='r', alpha=.25))
        
        # Setting the limits for better visualization
        ax.set_xlim(min(x) - 1, max(x) + 1)
        ax.set_ylim(min(y) - 1, max(y) + 1)
        ax.set_zlim(min(z) - 1, max(z) + 1)
        
        plt.show()
        # return True, Vector(t, u, v)


class TriObject:
    bounding_box: list[Vector]
    
    def __init__(
            self, name: str, triangles: list[Triangle], points: list[Vector], resolution: int = 100, skip = False
    ) -> None:
        self.resolution = resolution
        self.name = name
        self.triangles = triangles
        self.points = points
        self.texture: list[list[Pixel]] = []
        self.initTexture()
        self.boundingBox: list[Vector] = []
        self.bounding_box = self.boundingBox
        self.skip = skip
    
    def initTexture(self):
        self.texture = [[Pixel() for _ in range(self.resolution)] for _ in range(self.resolution)]
    
    def surfaceArea(self) -> float:
        SA = 0
        for triangle in self.triangles:
            SA += triangle.getArea()
        return SA

    
    def calcBoundingBox(self):
        minX = min([vec.x for vec in self.points])
        minY = min([vec.y for vec in self.points])
        minZ = min([vec.z for vec in self.points])
        maxX = max([vec.x for vec in self.points])
        maxY = max([vec.y for vec in self.points])
        maxZ = max([vec.z for vec in self.points])
        minPt = Vector(minX, minY, minZ)
        maxPt = Vector(maxX, maxY, maxZ)
        self.boundingBox = [minPt, maxPt]
