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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

class UnionFind {
private:
	vector<int> disj;
	vector<int> rank;
	/**
	 * Initializes the union-find tree. At first, there are no nodes in it.
	 * @param n
	 */
public:
	UnionFind(int n) : disj(n, -1), rank(n, 0) {}
	UnionFind(const UnionFind & other) : disj(other.disj), rank(other.rank) {}
	void add(int x) {
		disj[x] = x;
	}
	void unite(int x,int y) {
		if (disj[x] == -1) {
			fprintf(stderr, "node %d is not in this tree", x);
			assert(0);
		}
		if (disj[y] == -1) {
			fprintf(stderr, "node %d is not in this tree", y);
			assert(0);
		}
		x = root(x);
		y = root(y);
		if (x == y) {
			return;
		}
		if (rank[x] < rank[y]) {
			disj[x] = y;
		} else {
			disj[y] = x;
			if (rank[x] == rank[y]) {
				++rank[x];
			}
		}
	}
	int root(int x) {
		if (disj[x] == -1) {
			fprintf(stderr, "node %d is not in this tree", x);
			assert(0);
		}
		if(disj[x] == x) {
			return x;
		}
		return disj[x] = root(disj[x]);
	}
	/**
	 * Returns a subset that consists of nodes whose root is r.
	 * @param r
	 * @return A boolean array b s.t. b[i] <==> root(i) == r
	 */
	vector<bool> subset(int r){
		int n= disj.size();
		vector<bool> ret(n, false);
		for (int i = 0; i < n; ++i) {
			ret[i] = disj[i] >= 0 && root(i) == r;
		}
		return ret;
	}
	vector<bool> nodes() {
		int n= disj.size();
		vector<bool> ret(n, false);
		for (int i = 0; i < n; ++i) {
			ret[i] = disj[i] >= 0;
		}
		return ret;
	}
	bool isConnected() {
		int n= disj.size();
		int cur = -1;
		for (int i = 0; i < n; ++i) {
			if(disj[i] >= 0) {
				int r = root(i);
				if (cur == -1) {
					cur = r;
				} else {
					if(cur != r) {
						return false;
					}
				}
			}
		}
		return true;
	}
};

const int H = 10;
int island[H][H];

bool check() {
	UnionFind uf(100);
	REP(i,0,H) REP(j,0,H) {
		if(island[i][j])
			uf.add(i * H + j);
	}
	REP(i, 0, H - 1) REP(j, 0, H) {
		if (island[i][j] && island[i+1][j])
			uf.unite(i * H + j, i * H + j + H);
	}
	REP(i, 0, H) REP(j, 0, H - 1) {
		if (island[i][j] && island[i][j+1])
			uf.unite(i * H + j, i * H + j + 1);
	}
	return uf.isConnected();
}


int main(void){
	string dat[H];
	REP(i, 0, H) {
		cin >> dat[i];
		REP(j, 0, H) {
			island[i][j] = dat[i][j] == 'o' ? 1: 0;
		}
	}
	REP(i, 0, H) {
		REP(j, 0, H) {
			if(island[i][j] == 0) {
				island[i][j] = 1;
				if (check()) {
					cout << "YES" << endl;
					return 0;
				}
				island[i][j] = 0;
			}
		}
	}
	cout << "NO" << endl;
}
