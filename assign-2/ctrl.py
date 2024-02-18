import sys
import networkx as nx

from src import utils

LEFT_PARENTHESES = '{'
RIGHT_PARENTHESES = '}'

if __name__ == '__main__':
    cfg = nx.read_graphml(sys.argv[1])
    frontiers = utils.gene_frontiers(cfg)
    nodes = list(cfg.nodes)
    nodes.sort()

    for node in nodes:
        # print(node, frontiers[node])
        frontier = list(frontiers[node])
        frontier.sort()

        print('{} -> {}{}{}'.format(node, LEFT_PARENTHESES,
              ', '.join(frontier), RIGHT_PARENTHESES))
    print()
