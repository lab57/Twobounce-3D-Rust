from Vector import *
import random as r

r.seed(1)
class PencilSource:
    def __init__(self, start: Vector, end: Vector):
        """Sweeps along the line connecting start and end, 
        producing vectors starting along the line with uniformly random directions.
        Call getEmissionRays(n, numcores=2) to get n vectors produced, organized in numcores lists.

        Args:
            start (Vector): Start point of pencil
            end (Vector): End point of pencil
        """
        self.start = start
        self.end = end

    def randomSphericalVector(self):
        theta = 2*math.pi*r.random()
        phi = math.pi*r.random()
        sinphi = math.sin(phi)
        return Vector(sinphi*cos(theta), sinphi*sin(theta), cos(phi))
    
    def getEmissionRays(self, n, numcores = 1)-> list[list[tuple[Vector, Vector]]]:
        """Get n emission rays across numcores cores

        Args:
            n (_type_): _description_
            numcores (int, optional): _description_. Defaults to 1.

        Returns:
            list[list[tuple[Vector, Vector]]]: List sets of vectors divided for cores. Each set is a list of pairs of vectors
        """
        add_vec = (self.end - self.start ) / n
        out = []
        #print(f"{add_vec=}")
        st_i = 0    
        end_i = n//numcores
        for core in range(numcores):
            #print(f"{core=}")
            core_vecs = []
           # print(f"{st_i=}, {end_i=}")
            for i in range(st_i, end_i):
                #print(f"{n=}")
                core_vecs.append((self.start + i*add_vec, self.randomSphericalVector() ))
            st_i = end_i
            end_i += n // numcores
            out.append(core_vecs)
        out[-1].append((self.end, self.randomSphericalVector()))
        return out
    
class PencilSourceDebug:
    def __init__(self, start: Vector, end: Vector):
        """Sweeps along the line connecting start and end, 
        producing vectors starting along the line with uniformly random directions.
        Call getEmissionRays(n, numcores=2) to get n vectors produced, organized in numcores lists.

        Args:
            start (Vector): Start point of pencil
            end (Vector): End point of pencil
        """
        self.start = start
        self.end = end

    def randomSphericalVector(self):
        return Vector(1, 0, 0)
    
    def getEmissionRays(self, n, numcores = 1)-> list[list[tuple[Vector, Vector]]]:
        """Get n emission rays across numcores cores

        Args:
            n (_type_): _description_
            numcores (int, optional): _description_. Defaults to 1.

        Returns:
            list[list[tuple[Vector, Vector]]]: List sets of vectors divided for cores. Each set is a list of pairs of vectors
        """
        add_vec = (self.end - self.start ) / n
        out = []
        #print(f"{add_vec=}")
        st_i = 0    
        end_i = n//numcores
        for core in range(numcores):
            #print(f"{core=}")
            core_vecs = []
           # print(f"{st_i=}, {end_i=}")
            for i in range(st_i, end_i):
                #print(f"{n=}")
                core_vecs.append((self.start + i*add_vec, self.randomSphericalVector() ))
            st_i = end_i
            end_i += n // numcores
            out.append(core_vecs)
        out[-1].append((self.end, self.randomSphericalVector()))
        return out