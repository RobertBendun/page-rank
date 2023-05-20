import json
import networkx as nx
import numpy as np
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

pageTitles = dict()
pagerankValues = dict()

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
    global pagerankValues
    if len(G) == 0:
        return {}
    M = googleMatrix(G, alpha)
    
    eigenvalues, eigenvectors = np.linalg.eig(M.T)
    ind = np.argmax(eigenvalues)

    largest = np.array(eigenvectors[:, ind]).flatten().real
    norm = float(largest.sum())
    pagerankValues = dict(zip(G, map(float, largest / norm)))

def createGraphFromDict(pages):
    graph = nx.Graph()
    for page, pageProps in pages.items():
        graph.add_node(page)
        for neighbor in pageProps['references']:
            graph.add_edge(page, neighbor)
    return graph

def createTitleDict(pages):
    global pageTitles
    for page, pageProps in pages.items():
        pageTitles[page] = pageProps['title']

def drawAGraph(G):
    nx.draw_circular(G, node_size=400, with_labels=True)

def generateData(computePageRank = True):
    global pagerankValues
    with open('cppreference.json') as file:
        pages = json.load(file)['pages']
    file.close()

    createTitleDict(pages)
    if computePageRank: 
        graph = createGraphFromDict(pages)
        # to verify if our implementation is working properly
        #  pagerankValues = nx.pagerank(graph,0.85)
        pagerank(graph)

def getMatchingPages(query):
    #return false here if query not defined
    global pageTitles
    query=query.lower()
    matchingPages = list()
    for url in pageTitles.keys():
        if(query in (pageTitles[url].lower())):
            matchingPages.append(url)
    return matchingPages

def createResponse(pages):
    global pagerankValues
    response = dict()
    for url in pages:
        response[url] = pagerankValues[url]
    return json.dumps(response)

class PageRankHTTPRequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
            parsedUrl = urlparse(self.path)
            queryParams = parse_qs(parsedUrl.query)
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()

            searchVal = queryParams.get('q', [''])[0]

            matchingPages = getMatchingPages(searchVal)
            response = createResponse(matchingPages)
            response = response.encode('utf-8')

            self.wfile.write(response)

def createServer():
    server_address = ('localhost', 8000)
    httpd = HTTPServer(server_address, PageRankHTTPRequestHandler)
    print('Serwer HTTP działa na porcie 8000...')
    httpd.serve_forever()


generateData()
createServer()
print(pagerankValues)