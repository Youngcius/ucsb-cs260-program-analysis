"""
Convert all .cb files in ./cflat to .lir files in ./lir
"""

import os


CFLAT_DIR = "./cflat"
LIR_DIR = "./lir"

if not os.path.exists(LIR_DIR):
    os.makedirs(LIR_DIR)

cflat_files = os.listdir(CFLAT_DIR)

for cflat_file in cflat_files:
    lir_file = cflat_file.replace(".cb", ".lir")

    os.system("./cb2lir {} > {}".format(
        os.path.join(CFLAT_DIR, cflat_file),
        os.path.join(LIR_DIR, lir_file)
    ))
    print("Converted {} to {}".format(
        os.path.join(CFLAT_DIR, cflat_file),
        os.path.join(LIR_DIR, lir_file)
    ))
