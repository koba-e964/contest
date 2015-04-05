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

struct UF {
	vector<int> disj;
	UF(int n) {
		disj.reserve(n);
		REP(i, 0, n) {
			disj.push_back(i);
		}
	}
	int root(int x) {
		if(x == disj[x]) {
			return x;
		}
		return disj[x]=root(disj[x]);
	}
	int unify(int x, int y) {
		disj[root(x)] = root(y);
	}
	bool same_set(int x, int y) {
		return root(x) == root(y);
	}
};

const int N=51;
int n,m;
int e[N][N];

int main(void){
	cin >> n >> m;
	UF uf(n);
	REP(i, 0, m) {
		int x,y;
		cin >> x >> y;
		x--,y--;
		if(! uf.same_set(x, y)) {
			e[x][y] = e[y][x] = 1;
			uf.unify(x, y);
		}
	}
	set<int> s;
	REP(i, 0, n) {
		s.insert(uf.root(i));
	}
	int res = n - s.size();
	cout << (1LL << res) << endl;
}
