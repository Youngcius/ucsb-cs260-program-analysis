import os

file1 = './demos/std_inter_out/test16.inter.out'
file2 = './demos/inter_out/test16.inter.out'


def parse_to_dict(file):
    content = {}
    key = None
    with open(file, 'r') as f1:
        # loop to read lines 
        for line in f1:
            # line.strip()
            # if line is in format of "bbxx:", set it as key
            item = line.strip()
            if item == '':
                continue
            elif (item.startswith('bb') or item.startswith('entry')) and item.endswith(':'):
                key = item.strip(':')
                content[key] = []
            else:
                content[key].append(item)
    return content

content1 = parse_to_dict(file1)
content2 = parse_to_dict(file2)


keys = list(content1.keys())

for k in keys:
    if content1[k] != content2[k]:
        print(k)
        # print(content1[k])
        # print(content2[k])
        # print('-----------------')
