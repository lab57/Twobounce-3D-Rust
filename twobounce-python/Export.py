from collections import defaultdict
import os
from PIL import Image
import numpy as np
from Triangle import *




def stripMaterialInformation(lines):  # strip any preexisting material information
    result = []
    for line in lines:
        if len(line) >= 6 and line[0:6] in ["mtllib", "usemtl"]:
            pass
        else:
            result.append(line)
    return result


def getMatLines(mtl_name, location):
    return [
        f"\nnewmtl mat_{mtl_name}",
        "Ns 250.000000",
        "Ka 1.000000 1.000000 1.000000",
        "Kd 0.097925 0.065680 0.800000",
        "Ks 0.500000 0.500000 0.500000",
        "Ke 0.000000 0.000000 0.000000",
        "Ni 1.450000",
        "d 1.000000",
        "illum 2",
        f"map_Kd ./images/{location}.png",
    ]


def writeNewMtl(OUT_LOCATION, MTL_FILE_NAME, objects):
    with open(OUT_LOCATION + MTL_FILE_NAME + ".mtl", "w+") as f:
        for object in objects:
            f.writelines(line + "\n" for line in getMatLines(object, f"mat_{object}"))


def writeNewObj(OUT_LOCATION, FILENAME, newFile):
    with open(OUT_LOCATION + FILENAME + "_textured.obj", "w+") as f:
        f.write(newFile)


class hit:
    def __init__(self, name, hit_num, coords) -> None:
        self.name = name
        self.hit_num = hit_num
        self.x, self.y = coords

    def __repr__(self) -> str:
        return f"{self.name} - {self.hit_num}: <{self.x}, {self.y}>"





def writeImages(objects: list[TriObject]):
    images = {}
    print("Writing images")
    for i, obj in enumerate(objects):
        print(f"Writing image for {obj.name} ({i+1}/{len(objects)})")
        
        img = np.zeros((obj.resolution, obj.resolution, 3), dtype=np.uint8)
        for i in range(obj.resolution):
            for j in range(obj.resolution):
                img[i][j] = obj.texture[i][j].getColor()
                #print(img[i][j])
        #print(img)
        image = Image.fromarray(img)  
        image.save(f"./Textured/images/mat_{obj.name}.png")

def main(FILENAME, objs):
    LOCATION = "./"
    OUT_LOCATION = "./Textured/"
    # FILENAME = "ReflectionTestWIthCube"
    MTL_FILE_NAME = None or FILENAME + "_MTL"
    newFile = ""  # string with content of modified file
    lines = None
    with open(LOCATION + FILENAME + ".obj", "r") as f:
        lines = f.readlines()

    lines = stripMaterialInformation(lines)

    lines.insert(0, f"mtllib {MTL_FILE_NAME}.mtl\n")

    objects = []

    curObject = None
    doJoin = True
    for line in lines:
        if line[0] == "o":
            print("object")
            curObject = line.split(" ")[1]
            objects.append(curObject.strip())
            doJoin = True
        elif line[0] == "s":
            newFile += line
            newFile += f"usemtl mat_{curObject}\n"
            doJoin = False
        elif len(line) >= 6 and line[0:6] == "usemtl":
            doJoin = False
        if doJoin:
            newFile += line
        else:
            doJoin = True
    print("Objs:", objects)
    writeNewMtl(OUT_LOCATION, MTL_FILE_NAME, objects)
    writeNewObj(OUT_LOCATION, FILENAME, newFile)
    writeImages(objs)


import sys

