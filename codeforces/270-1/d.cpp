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

const int N = 2014;
int d[N][N];

void t(int v) {
	cout << (v ? "YES" : "NO") << endl;
	exit(0);
}

struct edge{
	int x,y,dd;
	edge(int x,int y, int dd): x(x),y(y),dd(dd){}
	bool operator<(const edge& o)const { return dd < o.dd;}
};

int n;
int u[N] = {0};
int rem;
vector<edge> egs;
int main(void){
	cin >> n;
	rem = n - 1;
	REP(i,0,n) {
		REP(j,0,n) {
			cin>>d[i][j];
		}
	}
	REP(i,0,n) {
		if(d[i][i]) t(0);
	}
	REP(i,0,n) REP(j,0,n) if(d[i][j] != d[j][i]) t(0);
	REP(i,0,n) REP(j,i+1,n) {
		egs.push_back(edge(i,j,d[i][j]));
	}
	sort(egs.begin(), egs.end());
	reverse(egs.begin(),egs.end());
	int pos = 0;
	while(rem > 0) { // T has >= 2 vertices
		int maxi = -1,maxj = -1,maxd=0;
		while(pos < egs.size()) {
			edge e=egs[pos];
			int i = e.x;
			int j = e.y;
			if(u[i] || u[j]) {
				pos ++;
				continue;
			}
			maxd = d[i][j];
			maxi = i,maxj=j;
			pos++;
			break;
		}
		if(maxd == 0) t(0);
		if(DEBUG) {
			cout << "max path: " << maxi << "~" << maxj << " " << maxd << endl;
		}
		int xc = -1, xd = 0x3fffffff;
		REP(i,0,n) {
			if(u[i] || maxi == i) continue;
			if(xd > d[maxi][i]) {
				xc = i;
				xd = d[maxi][i];
			}
		}
		if(xd == 0) t(0);
		if(DEBUG) {
			cout << "xc=" << xc << ", xd=" << xd << endl; 
		}
		REP(y,0,n) {
			if(u[y] || y == maxi) continue;
			if(d[maxi][y] != d[maxi][xc] + d[xc][y]) {
				t(0);
			}
		}
		u[maxi] = 1;
		rem --;
		if(DEBUG) {
			cout << "removed " << maxi << endl;
		}
	}
	t(1);
}
