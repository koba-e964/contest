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

const int DEBUG = 0;

const int N = 10010;
int n;
struct op {
	int u,v;
	op(int u,int v):u(u),v(v){}
};
vector<op> ops;
vector<op> ttops;
int x[N],y[N];
int xx[N],yy[N];

void go(vector<op> &ops, int u, int v) {
	ops.push_back(op(u,v));
}

void out(void) {
	cout << ops.size()  + ttops.size() << endl;
	REP(i,0,ops.size()) {
		cout << ops[i].u+1 << " "  << ops[i].v+1 << endl;
		xx[ops[i].u] ^= xx[ops[i].v];
	}
	if(DEBUG) cout << "=======" << endl;
	for(int i=ttops.size()-1;i>=0;i--) {
		cout << ttops[i].u+1 << " "  << ttops[i].v+1 << endl;
		xx[ttops[i].u] ^= xx[ttops[i].v];
	}
	if(DEBUG) {
		REP(i,0,n) {
			printf("modified x[%d] = %d, y[%d] = %d\n", i,xx[i],i,yy[i]);
		}
	}
}


int elim(int x[N],vector<op> &ops) {
	int r=0;
	REP(c,0,32) {
		REP(i,r,n) {
			if(x[i] & (1<<c)) {
				swap(x[r],x[i]);
				if(r != i) {
					go(ops,i,r); go(ops,r,i); go(ops,i,r);
				}
				REP(j,0,n) {
					if(r==j) continue;
					if(x[j] & (1 << c)) {
						x[j] ^= x[r];
						go(ops,j,r);
					}
				}
				if(DEBUG) {
					printf("row %d:%08x\n", r, x[r]);
				}
				r++;
				break;
			}
		}
	}
	if(DEBUG) {
		REP(i,0,r) {
			printf("x[%d] = %08x\n", i, x[i]);
		}
	}
	return r;
}


int main(void){
	cin >> n;
	REP(i,0,n) {cin >> x[i]; xx[i] = x[i];}
	REP(i,0,n) {cin >> y[i]; yy[i] = y[i];}
	int rx = elim(x,ops);
	int ry = elim(y,ttops);
	int u=0,v=0;
	while(v<ry) {
		int succ = 0;
		REP(u,0,rx) {
			int diff = y[v] ^ x[v];
			int lby = diff & -diff;
			int lbx = x[u] & -x[u];
			if(DEBUG) {
				cout << u << ": diff= " << diff << ", x[u]= " << x[u] <<endl;
			}
			if(diff ==0 ) { v++; succ = 1; break;}
			if(lbx > lby) {
				cout << -1 << endl;
				return 0;
			}
			if(lbx == lby) {
				x[v] ^= x[u];
				go(ops,v,u);
			}
		}
		if(succ == false)
			succ |= x[v] == y[v];
		if(!succ) {
			cout << -1 << endl;
			return 0;
		}
	}
	REP(v,ry,n) go(ops,v,v);
	out();
}
