"""
Test all functions with all ./examples/lir/*.lir files
"""
import os
import json
import sys
import argparse

from rich.console import Console
console = Console()

parser = argparse.ArgumentParser(description='Benchmark constant analysis')
parser.add_argument('analyzer', type=str, help='The analyzer to use')
parser.add_argument('--json', action='store_true',
                    default=False, help='Use json files')
args = parser.parse_args()


with open('./examples/prog_funcs.json', 'r') as f:
    prog_funcs = json.load(f)


for prog, funcs in prog_funcs.items():
    console.rule(prog, style="bold red")
    for func in funcs:
        console.print(">>>>>> {}: {} <<<<<<".format(
            prog, func), style="bold green")
        print()
        if args.json:
            os.system(
                "./{} ./examples/json/{}.json {}".format(args.analyzer, prog, func))
        else:
            os.system(
                "./{} ./examples/lir/{}.lir {}".format(args.analyzer, prog, func))
