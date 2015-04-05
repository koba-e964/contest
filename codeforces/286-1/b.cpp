#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>
#include <list>
#include <stack>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

const int N = 300010;

int disj[N];

int ww[N], bb[N];

int root(int x) {
	if (x == disj[x]) return x;
	return disj[x] = root(disj[x]);
}
void unite(int x, int y) {
	disj[root(x)] = root(y);
}

class Graph
{
    int V;    // No. of vertices
    list<int> *adj;    // An array of adjacency lists

    // Fills Stack with vertices (in increasing order of finishing times)
    // The top element of stack has the maximum finishing time
    void fillOrder(int v, bool visited[], stack<int> &Stack);

    // A recursive function to print DFS starting from v
    void DFSUtil(int v, bool visited[]);
    int csum(int v, bool visited[]);
public:
    Graph(int V);
    void addEdge(int v, int w);

    // The main function that finds and prints strongly connected components
    void printSCCs();
    int countEdges();

    // Function that returns reverse (or transpose) of this graph
    Graph getTranspose();

    /* function that marks vertices reachable from v as visited and checks whether they contain circuit. */
    pair<int, bool> hasCircuit(int v, bool visited[], bool path[]);
};

Graph::Graph(int V)
{
    this->V = V;
    adj = new list<int>[V];
}

// A recursive function to print DFS starting from v
void Graph::DFSUtil(int v, bool visited[])
{
    // Mark the current node as visited and print it
    visited[v] = true;
    cout << v << " ";

    // Recur for all the vertices adjacent to this vertex
    list<int>::iterator i;
    for (i = adj[v].begin(); i != adj[v].end(); ++i)
        if (!visited[*i])
            DFSUtil(*i, visited);
}

int Graph::csum(int v, bool visited[])
{
    // Mark the current node as visited and print it
    visited[v] = true;
    int s = 1;
    // Recur for all the vertices adjacent to this vertex
    list<int>::iterator i;
    for (i = adj[v].begin(); i != adj[v].end(); ++i)
        if (!visited[*i])
            s += csum(*i, visited);
    return s;
}

Graph Graph::getTranspose()
{
    Graph g(V);
    for (int v = 0; v < V; v++)
    {
        // Recur for all the vertices adjacent to this vertex
        list<int>::iterator i;
        for(i = adj[v].begin(); i != adj[v].end(); ++i)
        {
            g.adj[*i].push_back(v);
        }
    }
    return g;
}

void Graph::addEdge(int v, int w)
{
    adj[v].push_back(w); // Add w to vÅfs list.
}

void Graph::fillOrder(int v, bool visited[], stack<int> &Stack)
{
    // Mark the current node as visited and print it
    visited[v] = true;

    // Recur for all the vertices adjacent to this vertex
    list<int>::iterator i;
    for(i = adj[v].begin(); i != adj[v].end(); ++i)
        if(!visited[*i])
            fillOrder(*i, visited, Stack);

    // All vertices reachable from v are processed by now, push v to Stack
    Stack.push(v);
}

// The main function that finds and prints all strongly connected components
void Graph::printSCCs()
{
    stack<int> Stack;

    // Mark all the vertices as not visited (For first DFS)
    bool *visited = new bool[V];
    for(int i = 0; i < V; i++)
        visited[i] = false;

    // Fill vertices in stack according to their finishing times
    for(int i = 0; i < V; i++)
        if(visited[i] == false)
            fillOrder(i, visited, Stack);

    // Create a reversed graph
    Graph gr = getTranspose();

    // Mark all the vertices as not visited (For second DFS)
    for(int i = 0; i < V; i++)
        visited[i] = false;
    // Now process all vertices in order defined by Stack
    while (Stack.empty() == false)
    {
        // Pop a vertex from stack
        int v = Stack.top();
        Stack.pop();

        // Print Strongly connected component of the popped vertex
        if (visited[v] == false)
        {
            gr.DFSUtil(v, visited);
            cout << endl;
        }
    }
}

int Graph::countEdges()
{
    stack<int> Stack;
    int sum = 0;
    // Mark all the vertices as not visited (For first DFS)
    bool *visited = new bool[V];
    for(int i = 0; i < V; i++)
        visited[i] = false;

    // Fill vertices in stack according to their finishing times
    for(int i = 0; i < V; i++)
        if(visited[i] == false) {
            fillOrder(i, visited, Stack);
        }

    // Create a reversed graph
    Graph gr = getTranspose();

    // Mark all the vertices as not visited (For second DFS)
    for(int i = 0; i < V; i++)
        visited[i] = false;

    // Now process all vertices in order defined by Stack
    while (Stack.empty() == false)
    {
        // Pop a vertex from stack
        int v = Stack.top();
        Stack.pop();
        // Print Strongly connected component of the popped vertex
        if (visited[v] == false)
        {
            int res2 = gr.csum(v, visited);
            ww[root(v)] += res2;
            bb[root(v)] |= res2 >= 2;
           //cout << "v=" << v << ", " << res.first << " " << res.second << endl;
        }
    }
    REP(v, 0, N) {
    	sum += bb[v] ? ww[v] : max(0, ww[v]-1);
    }
    return sum;
}

pair<int, bool> Graph::hasCircuit(int v, bool visited[], bool path[]) {
	if (visited[v]) {
		return pair<int, bool>(0,false);
	}
	bool res = false;
	visited[v] = true;
	path[v] = true;
	int sum = 1;
    for (list<int>::iterator i = adj[v].begin(); i != adj[v].end(); ++i) {
    	//cout << "(" << v << ", " << *i << ")" << endl;
        if(!visited[*i]) {
        	pair<int, bool> sub = hasCircuit(*i, visited, path);
        	res |= sub.second;
        	sum += sub.first;
        }
        if (path[*i]) {
        	res = true;
        }
    }
    path[v] = false;
    return pair<int, bool>(sum, res);
}

// Driver program to test above functions
int main()
{
	int n, m;
	cin >> n >> m;
    // Create a graph given in the above diagram
    Graph g(n);
    REP(i,0,n) {
    	disj[i] = i;
    }
    REP(i, 0, m) {
    	int v,w;
    	cin >> v >> w;
    	v--, w--;
    	g.addEdge(v, w);
    	unite(v, w);
    }

    cout << g.countEdges() << endl;

    return 0;
}
