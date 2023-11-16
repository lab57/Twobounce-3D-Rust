import pyg4ometry as pyg4
import os

os.chdir("./remoll/geometry")
print("pyg4ometry imported")
r = pyg4.gdml.Reader("./mollerMother.gdml", skipMaterials=True)
print("mollerMother.gdml loaded")
l = r.getRegistry().getWorldVolume()
v = pyg4.visualisation.VtkViewerNew()
v.addLogicalVolume(l)
v.buildPipelinesAppend()
v.exportGLTFScene("../../mollerMother.gltf")
