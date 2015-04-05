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
const int N = 3010;
const int H = 6100;

const int DEBUG = 1;

int n;
int h[N];
int m[N],s[N], e[N];

set<int> tm;
map<int,int> tmu;
int uniq = 0;

int dp[H][N];
int dpacc[H];

vector<int> ep[H];

vector<int> pred[N];

void reg(int v) {
	if(tm.count(v)) {
		return;
	}
	tm.insert(v);
}

int main(void){
	cin >> n;
	REP(i,0,n) cin>>h[i];
	REP(i,0,n) {
		cin >> m[i] >> s[i] >> e[i];
		reg(s[i]);
		reg(e[i]);
	}
	int cc = 0;
	for(set<int>::iterator it = tm.begin(); it != tm.end(); it++) {
		tmu.insert(pair<int,int>(*it,cc));
		cc++;
	}
	REP(i,0,n) {
		s[i] = tmu[s[i]];
		e[i] = tmu[e[i]];
		ep[e[i]].push_back(i);
	}
	REP(i,0,n) {
		REP(j,0,n) {
			if(e[j] > s[i]) break;
			pred[i].push_back(j);
		}
	}
	REP(i,0,H) REP(j,0,n) { dp[i][j] = -1;}
	REP(x,0,H) {
		int mm = 0;
		REP(c,1,n+1) {
			int mmc = 0;
			REP(q,0,pred[x].size()) {
				int pr = pred[x][q];
				if(m[x] == m[pr] && c >= 2)
					mmc = max(mmc, dp[pr][c - 1] + h[c-1]);
				mmc = max(mmc, dpacc[pr] + h[0]);
			}
			mm = max(mm,mmc);
			dp[x][c] = mmc;
		}
		dpacc[x] = mm;
	}
	int mm = 0;
	REP(i,0,n) {
		mm = max(mm, dpacc[i]);
	}
	cout << mm << endl;
}
