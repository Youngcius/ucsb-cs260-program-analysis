import os
import json

from rich.console import Console
console = Console()

with open('./examples/prog_funcs.json', 'r') as f:
    prog_funcs = json.load(f)


i = 0
for prog, funcs in prog_funcs.items():
    i += 1
    if i > 1:
        break
    console.rule(prog, style="bold red")
    for func in funcs:
        console.print(func, style="bold green")
        print()
        os.system("./constants_analysis ./examples/json/{}.json {}".format(prog, func))
