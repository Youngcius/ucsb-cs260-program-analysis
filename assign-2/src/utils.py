import networkx as nx


def gene_dominators(cfg: nx.DiGraph, entry_node: str = 'entry', strict: bool = False):
    """
    Generate the dominators of the control flow graph
    :param cfg: the control flow graph
    :param entry_node: the entry node of the control flow graph
    :param strict: whether to generate the strict dominators
    :return: the dominators of the control flow graph
    """
    dominators = {node: set() for node in cfg.nodes}
    for node in cfg.nodes:
        if node == entry_node:
            continue
        through_nodes = []
        for path in nx.all_simple_paths(cfg, entry_node, node):
            through_nodes.append(set(path))

        dominators[node] = set.intersection(*through_nodes)
    if strict:
        for node in cfg.nodes:
            dominators[node] = dominators[node] - {node}
    else:
        for node in cfg.nodes:
            dominators[node] = dominators[node].union({node})
    return dominators


def gene_dom_rela(cfg: nx.DiGraph, entry_node: str = 'entry', strict: bool = False):
    """
    Generate the dominator relationship of the control flow graph, i.e., the reversed result of gene_dominators()
    :param cfg: the control flow graph
    :param entry_node: the entry node of the control flow graph
    :param strict: whether to generate the strict dominator relationship
    :return: the dominator relationship of the control flow graph
    """
    dominators = gene_dominators(cfg, entry_node, strict)
    dom_rela = {node: set() for node in cfg.nodes}
    for node in cfg.nodes:
        for dom_node in dominators[node]:
            dom_rela[dom_node].add(node)
    return dom_rela


def gene_imm_dominators(cfg: nx.DiGraph, entry_node: str = 'entry'):
    """
    Generate the immediate dominators of the control flow graph
    :param cfg: the control flow graph
    :param entry_node: the entry node of the control flow graph
    :return: the immediate dominators of the control flow graph
    """
    dominators = gene_dominators(cfg, entry_node, True)
    imm_dominators = {node: set() for node in cfg.nodes}
    for node in cfg.nodes:
        for dom_node in dominators[node]:
            if (dominators[node] - {dom_node}).issubset(dominators[dom_node]):
                imm_dominators[node].add(dom_node)
    return imm_dominators


def gene_imm_dom_rela(cfg, entry_node: str = 'entry'):
    """
    Generate the immediate dominator relationship of the control flow graph, i.e., the reversed result of gene_imm_dominators()
    :param cfg: the control flow graph
    :param entry_node: the entry node of the control flow graph
    :return: the immediate dominator relationship of the control flow graph
    """
    imm_dominators = gene_imm_dominators(cfg, entry_node)
    imm_dom_rela = {node: set() for node in cfg.nodes}
    for node in cfg.nodes:
        for imm_dom_node in imm_dominators[node]:
            imm_dom_rela[imm_dom_node].add(node)
    return imm_dom_rela


def gene_frontiers(cfg: nx.DiGraph):
    frontiers = {node: set() for node in cfg.nodes}
    dominators = gene_dominators(cfg)
    for node, dom_nodes in dominators.items():
        strict_dom_nodes = dom_nodes - {node}
        for pred in cfg.predecessors(node):
            for dom_pred in dominators[pred] - strict_dom_nodes:
                frontiers[dom_pred] = frontiers[dom_pred].union({node})
    return frontiers
