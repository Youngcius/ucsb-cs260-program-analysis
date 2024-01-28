"""
Convert all .lir files in ./lir to .json files in ./json
"""

import os


LIR_DIR = "./lir"
JSON_DIR = "./json"

if not os.path.exists(JSON_DIR):
    os.makedirs(JSON_DIR)

lir_files = os.listdir(LIR_DIR)

for lir_file in lir_files:
    json_file = lir_file.replace(".lir", ".json")

    os.system("./lir2json {} > {}".format(
        os.path.join(LIR_DIR, lir_file),
        os.path.join(JSON_DIR, json_file)
    ))
    print("Converted {} to {}".format(
        os.path.join(LIR_DIR, lir_file),
        os.path.join(JSON_DIR, json_file)
    ))
