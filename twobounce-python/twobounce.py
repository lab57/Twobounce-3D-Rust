
from ObjectLoader import ObjLoader
from Triangle import *
from Vector import *
from RTree import *
from PencilSource import *
from DiskDetector import *
from PencilSource import *
import multiprocessing as mp
from pprint import pprint
import random as r
from math import sin, cos
from Plotting import *
from Export import *
import time
#from numba import jit, njit

N = 100000

def twobounce():
    loader: ObjLoader = ObjLoader("./")
    objects, triangles = loader.load("mollerMother.obj")
    R = loader.buildTree(triangles)
    print(f"{R.nnodes} nodes" )
    print(f"{len(triangles)} triangles")


    vector_sets = PencilSource(Vector(5, 0, -1), Vector(5, 0, 1)).getEmissionRays(N)
    detector    = DiskDetector(1, Vector(-5, 0, 0), Vector(-1, 0, 0), n=20)
    print("RAHH", len(detector.surfacePoints))

    print("Performing one bounce")
    t0 = time.perf_counter()
    vis_to_source: list[Hit] = [] #Onebounce hits from source
    for core in vector_sets:
        for vectors in core:
            res:Hit = R.checkIntersections(*vectors)
            if(res.didHit   ):
                vis_to_source.append(res)
                res.getPixel().status = 1
    print(f"Done, {time.perf_counter()-t0 }s elapsed")
    print("Performing twobounce")

    t1 = time.perf_counter()
    vis_to_detector = []
    for i,hit in enumerate(vis_to_source):
        if(detector.isVisible(R, hit.cartesian())):
            hit.getPixel().status = 2
            vis_to_detector.append(hit)
    print(f"Done, {time.perf_counter()-t1}s elapsed")
    print("v2s:", len(vis_to_source))
    print("v2d:", len(vis_to_detector))
    #for obj in objects:
       # print("aa", obj.name, obj.skip)

    # ax = createAx()
    # plotGeometry(ax, R)

    # for h in vis_to_source:
    #     plotHit(ax, h)

    # plotDisk(ax, detector)
    # h1 = vis_to_source[0]
    # print(h1.getPixel().status)


    #main("2bounce_test_geo2", objects)

    





twobounce()










