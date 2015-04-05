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

struct edge {
	int u,v,w;
	edge(int u,int v,int w): u(u),v(v),w(w) {}
	bool operator<(const edge & o) const{
		return w < o.w;
	}
};

const int N=301000;
vector<edge> e;
int n,m;
int dp[N] = {0};
int dp2[N] = {0};

vector<int> queue;

void save(void) {
	REP(i,0,queue.size()) {
		int p = queue[i];
		dp[p] = max(dp[p], dp2[p]);
	}
	queue.clear();
}
int main(void){
	scanf("%d%d",&n,&m);
	REP(i,0,m) {
		int u,v,w;
		scanf("%d%d%d", &u,&v,&w);
		u--,v--;
		e.push_back(edge(u,v,w));
	}
	sort(e.begin(), e.end());
	int cur = -1;
	REP(i,0,m) {
		if ( cur != e[i].w) {
			save();
		}
		cur = e[i].w;
		int u=e[i].u;
		int v=e[i].v;
		dp2[v] = max(dp2[v], dp[u]+1);
		queue.push_back(v);
	}
	save();
	int ma=0;
	REP(i,0,n) {
		ma = max(ma, dp[i]);
	}
	cout << ma << endl;
}
