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
const int DEBUG = 1;

const int N= 100010;
int n;

struct edge {
	int to;
	ll cost, mdist;
};

vector<edge> adj[N];
map<int, int> inv[N];

ll calc(int v, int en) {
	if (adj[v][en].mdist >= 0) {
		return adj[v][en].mdist;
	}
	int to = adj[v][en].to;
	ll m = 0;
	REP(i, 0, adj[to].size()) {
		if (adj[to][i].to == v) {
			continue;
		}
		m = max(m, calc(to, i));
	}
	return adj[v][en].mdist = adj[v][en].cost + m;
}

ll far[N] = {0};

int main(void){
	cin >> n;
	REP(i, 0, n - 1) {
		int a, b, c;
		cin >> a >> b >> c;
		a--,b--;
		inv[a].insert(pair<int, int> (b, adj[a].size()));
		adj[a].push_back((edge) {b, c, -1});
		inv[b].insert(pair<int, int> (a, adj[b].size()));
		adj[b].push_back((edge) {a, c, -1});
	}
	REP(i, 0, n) {
		REP(j, 0, adj[i].size()) {
			far[i] = max(far[i], calc(i, j));
		}
	}
	if (DEBUG && n <= 10) {
		REP(i, 0, n) {
			cout << "far[" << i << "]=" << far[i] << endl;
		}
	}
	int q;
	cin >> q;
	multiset<ll> dat;
	REP(i, 0, n) {
		dat.insert(far[i]);
	}
	REP(i, 0, q) {
		int m = 0;
		multiset<ll> cp(dat);
		ll l;
		cin >> l;
		REP(j ,0, n) {
			ll low = far[j];
			ll upp = low + l;
			multiset<ll>::iterator itlow = cp.lower_bound(low);
			multiset<ll>::iterator itupp = cp.upper_bound(upp);
			if (cp.begin() != itlow) {
				itlow--;
				cp.erase(cp.begin(), itlow);
			}
			cp.erase(itupp, cp.end());
			m = max(m, (int)cp.size());
			if(DEBUG) {
				cout << "l=" << l << " low = " << low << " upp= " << upp << " cp:";
				  for (std::multiset<ll>::iterator it=cp.begin(); it!=cp.end(); ++it)
				    std::cout << ' ' << *it;
				  std::cout << '\n';
			}
		}
		cout << m << endl;
	}
}
