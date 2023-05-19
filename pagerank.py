import json
import networkx as nx
import numpy as np
import time

def googleMatrix(G, alpha=0.85):
    matrix = np.asmatrix(nx.to_numpy_array(G))
    print(len(G))
    nodeCount = len(G)
    if nodeCount == 0:
        return matrix

    personalizationVector = np.repeat(1.0 / nodeCount, nodeCount)
    danglingWeights = personalizationVector
    danglingNodes = np.where(matrix.sum(axis=1) == 0)[0]


    for node in danglingNodes:
        matrix[node] = danglingWeights

    matrix /= matrix.sum(axis=1)

    return alpha * matrix + (1 - alpha) * personalizationVector

def pagerank(G, alpha=0.85):
    if len(G) == 0:
        return {}
    M = googleMatrix(G, alpha)
    
    eigenvalues, eigenvectors = np.linalg.eig(M.T)
    ind = np.argmax(eigenvalues)

    largest = np.array(eigenvectors[:, ind]).flatten().real
    norm = float(largest.sum())
    return dict(zip(G, map(float, largest / norm)))

def generateGraphFromDict(graph, data):
    for node, neighbors in data.items():
        graph.add_node(node)
        for neighbor in neighbors:
            graph.add_edge(node, neighbor)

def drafAGraph(G):
    # G = nx.random_k_out_graph(n=8, k=2, alpha=0.75)
    def draw_graph(G):
        nx.draw_circular(G, node_size=400, with_labels=True)
    draw_graph(G)
    # time.sleep(1000000)

with open('cppreference.json') as file:
    pages = json.load(file)['pages']
file.close()

graph = nx.Graph()
generateGraphFromDict(graph, pages)
print(pagerank(graph))

# to verify if our implementation is working properly
# print(nx.pagerank(graph,0.85))
