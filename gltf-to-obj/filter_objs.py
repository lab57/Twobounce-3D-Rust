import argparse

"""
Filter a MOLLER obj file to remove undesired objects from the scene.



"""


def filter_obj_file(input_file, output_file, to_remove):
    with open(input_file, "r") as infile, open(output_file, "w") as outfile:
        write_line = True
        for line in infile:
            if line.startswith("o ") or line.startswith("g "):
                object_name = line.split()[1].split(".")[0]
                write_line = object_name not in to_remove
                print(object_name)
                if object_name in to_remove:
                    print(f"filtering out {object_name}")
            if write_line:
                outfile.write(line)


to_remove = [
    "logicMother_0",
    "logicHall_0",
    "sbsbunker_0",
    "targetRegion_0",
    "upstream_0",
    "psbunker_0",
    "logicDownstream_0",
    "coll1SideShield_logic_1",
    "upstreamTorusRegion_0",
    "DetectorArray_volume_0",
    "showerMaxMother_0",
    "donutSystem_logical_0",
    "target_chamber_vol3_0",
    "target_chamber_vol2_1",
    "TargetLadder_logical_0",
    "TargetLH2_logical_0",
    "targetLH2_LH2Volume_logical_0",
    "targetLH2_AlWindowDS_logical_0",
    "targetLH2_AlWindowUS_logical_0" "TargetPositionAlDummyHoleUS_logical_0",
    "TargetPositionAlDummyHoleDS_logical_0",
    "TargetPositionAlDummy1US_logical_0",
    "TargetPositionAlDummy1DS_logical_0",
    "TargetPositionAlDummy2US_logical_0",
    "TargetPositionAlDummy2DS_logical_0",
    "TargetPositionOptics1_logical_0",
    "TargetPositionOptics2_logical_0",
    "TargetAlDummyHoleUS_logical_0",
    "TargetAlDummy4pctUS_logical_0",
    "TargetAlDummy2pctUS_logical_0",
    "TargetCFoilUS_logical_1",
    "TargetCFoilUS_logical_0",
    "TargetCFoilDS_logical_1",
    "TargetCFoilDS_logical_0",
    "TargetAlDummy4pctDS_logical_0",
    "TargetAlDummy2pctDS_logical_0",
    "TargetAlDummyHoleDS_logical_0",
    # blocking the pipe
    "TubeVol1_0",
    "US_toroidMother_0",
    "logic_DSpipe_vacuumtube_0",
    "DSbeampipeMother_0",
    "diffuserRegion_0",
    "diffuserPlate_logic_0",
    "diffuserPlateBackDet_logic_0",
    # unsure if should be removed
    "PhotonBlocker_log_0",
]


def main():
    # Set up argument parser
    parser = argparse.ArgumentParser(description="Filter objects from an OBJ file.")
    parser.add_argument("input_file", help="Path to the input OBJ file")
    parser.add_argument("output_file", help="Path to the output OBJ file")

    # Parse arguments
    args = parser.parse_args()

    # Using the function with command line arguments
    filter_obj_file(args.input_file, args.output_file, to_remove)


if __name__ == "__main__":
    main()
