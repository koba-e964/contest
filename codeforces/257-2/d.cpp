#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>
#include <queue>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

#define DEBUG 0

using namespace std;
typedef long long int ll;
const ll INF = 1LL<<56;

const int N = 200000;
const int M = 300000;
const int K = 100000;
int n,m,k;
int u[M], v[M];
ll x[M];
int s[K];
ll y[K];

ll train[N];

ll d[N];
struct edge{int to; ll cost;};
typedef pair<ll,int> P;

vector<edge> g[N];

void dijk(int s){
	priority_queue<P, vector<P>, greater<P> > que;
	fill(d, d+n, INF);
	d[s] = 0;
	que.push(P(0,s));
	while(! que.empty()) {
		P p = que.top();
		que.pop();
		int v = p.second;
		if(d[v] < p.first) continue;
		REP(i, 0, g[v].size()) {
			edge e = g[v][i];
			if (d[e.to] > d[v] + e.cost) {
				d[e.to] = d[v] + e.cost;
				que.push(P(d[e.to], e.to));
			}
		}
	}
}

int main(void){
	int cnt=0;
	scanf("%d%d%d",&n,&m,&k);
	REP(i,0,m) {
		scanf("%d%d%d", u+i, v+i, x+i);
		u[i]--,v[i]--;
		g[u[i]].push_back((edge){v[i],x[i]});
		g[v[i]].push_back((edge){u[i],x[i]});
	}
	REP(i,0,n) {
		train[i] = INF;
	}
	REP(i,0,k) {
		scanf("%d%d", s+i, y+i);
		s[i]--;
		if( train[s[i]] == INF) {
			train[s[i]] = y[i];
		} else {
			train[s[i]] = min(train[s[i]], y[i]);
			cnt ++;
		}
	}
	dijk(0);
	if(DEBUG) {
		cout << "d:"<<endl;
		REP(i,0,n) {
			cout << d[i] << endl;
		}
		cout << "train:" << endl;
		REP(i,0,n) {
			cout << train[i] << endl;
		}
	}
	REP(i,0,n) {
		if(train[i] != INF && d[i] <= train[i]) {
			cnt++;
		}
	}
	cout << cnt << endl;
}
