from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import matplotlib.pyplot as plt
from Triangle import *
from RTree import *



def createAx():
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    return ax


def plotGeometry(ax, R: RTree):
    limits = (-10, 10)
    ax.set_xlim(limits[0], limits[1])
    ax.set_ylim(limits[0], limits[1])
    ax.set_zlim(limits[0], limits[1])
    for tri in R.triangles:
        x = [tri.a.x, tri.b.x, tri.c.x]
        y = [tri.a.y, tri.b.y, tri.c.y]
        z = [tri.a.z, tri.b.z, tri.c.z]
        
        # Plotting the triangle
        verts = [list(zip(x, y, z))]
        ax.add_collection3d(Poly3DCollection(verts, facecolors='cyan', linewidths=1, edgecolors='r', alpha=.25))
    return ax

def plotHit(ax, hit: Hit):
    ax.quiver(*hit.origin.arr, *(-1*hit.origin + hit.cartesian()).arr)


def plotDisk(ax, detDisk):
    for pt in detDisk.surfacePoints:
        ax.scatter(*pt.arr)

        
