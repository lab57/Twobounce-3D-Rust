
from GeometricObjects import *

half_size = 1  # Since the box is 2x2x2, half its size is 1
vertices = [
    Vector(-half_size, -half_size, -half_size),
    Vector(half_size, -half_size, -half_size),
    Vector(half_size, half_size, -half_size),
    Vector(-half_size, half_size, -half_size),
    Vector(-half_size, -half_size, half_size),
    Vector(half_size, -half_size, half_size),
    Vector(half_size, half_size, half_size),
    Vector(-half_size, half_size, half_size)
]

# Define the 12 triangles using the vertices
# The order is important to ensure the triangles are defined with the correct orientation
triangles = [
    Triangle([vertices[0], vertices[1], vertices[2]]),  # Front face
    Triangle([vertices[0], vertices[2], vertices[3]]),
    Triangle([vertices[4], vertices[5], vertices[6]]),  # Back face
    Triangle([vertices[4], vertices[6], vertices[7]]),
    Triangle([vertices[0], vertices[1], vertices[5]]),  # Bottom face
    Triangle([vertices[0], vertices[5], vertices[4]]),
    Triangle([vertices[2], vertices[3], vertices[7]]),  # Top face
    Triangle([vertices[2], vertices[7], vertices[6]]),
    Triangle([vertices[0], vertices[3], vertices[7]]),  # Left face
    Triangle([vertices[0], vertices[7], vertices[4]]),
    Triangle([vertices[1], vertices[2], vertices[6]]),  # Right face
    Triangle([vertices[1], vertices[6], vertices[5]])
]

# Create the TriObject instance
box = TriObject("CenteredBox", triangles, vertices)


def test_surface_area():
    assert box.surfaceArea() == 24


def test_calc_bounding_box():
    box.calcBoundingBox()
    bb = box.boundingBox
    dist = bb[0] - bb[1]
    assert abs(dist) - math.sqrt(12) <= 1e-7
    
    
def test_getPixel():
    src = Vector(3, 0, 0)
    dir = Vector(-1, 0, 0)
    for tri in box.triangles:
        hit = tri.intersect(src, dir)
        if(hit.u):
            print("Hit!")
            print(hit.cartesian())

def test_inter():
    src = Vector(3, 0, 0)
    dir = Vector(-1, 0, 0)
