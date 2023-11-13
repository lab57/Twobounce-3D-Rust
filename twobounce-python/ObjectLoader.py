from Triangle import *
from Vector import *
from RTree import *

class ObjLoader:
    def __init__(self, path):
        """
        :param path: Path to folder containing object files. Ex: "./"
        """
        self.path = path

    def load(self, filename) -> tuple[list[TriObject], list[Triangle]]:
        """
        :param filename: Name of file in path, ex: test_file.obj
        :return: Tuple containing list of objects and full list of triangles
        """
        verticies = []
        objects = []
        triangles = []
        normals = []
        texture_verticies = []
        with open(self.path + filename, "r") as f:
            curObject = None
            for line in f.readlines():
                if line[0] in ["o", "g"]:
                    name = line.split(" ")[1].strip("\n")
                    args = name.split("-")

                    print(f"Loading {name}")
                    curObject = TriObject(name, [], []) #"crit" in args)

                    if("skip" in args):
                        curObject.skip = True
                    if(not curObject.skip):
                        objects.append(curObject)
                if line[0] == "#":
                    continue
                if line[0] == "v" and line[1] == " ":
                    split = line.split(" ")
                    verticies.append(
                        Vector(float(split[1]), float(split[2]), float(split[3]))
                    )
                    curObject.points.append(
                        Vector(float(split[1]), float(split[2]), float(split[3]))
                    )
                if line[0] == "v" and line[1] == "n":
                    split = line.split(" ")
                    normals.append(
                        Vector(float(split[1]), float(split[2]), float(split[3]))
                    )
                if line[0] == "v" and line[1] == "t":
                    split = line.split(" ")
                    texture_verticies.append([float(split[1]), float(split[2])])
                if line[0] == "f" and not curObject.skip:
                    split = line.split(" ")
                    v1 = split[1].split("/")
                    v2 = split[2].split("/")
                    v3 = split[3].split("/")
                    triangle = Triangle(
                        [
                            verticies[int(v1[0]) - 1],
                            verticies[int(v2[0]) - 1],
                            verticies[int(v3[0]) - 1],
                        ]
                    )
                    if v1[1] != "":
                        triangle.setTexture (
                            texture_verticies[int(v1[1]) - 1],
                            texture_verticies[int(v2[1]) - 1],
                            texture_verticies[int(v3[1]) - 1],
                        )
                    triangle.normal = normals[int(v1[2]) - 1]
                    curObject.triangles.append(triangle)
                    triangle.object = curObject
                    triangle
                    triangles.append(triangle)
        
        # print(f"{len(triangles)} polygons loaded")
        for o in objects:
            o.calcBoundingBox()
        return objects, triangles

    def buildTree(self, triangles):
        print("Building R-Tree")
        rtree = RTree(triangles, 8)
        print("Done")
        return rtree