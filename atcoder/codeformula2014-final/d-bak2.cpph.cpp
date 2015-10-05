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

const int DEBUG = 0;

int n;
int h[N];
int m[N],s[N], e[N];

set<int> tm;
map<int,int> tmu;
int uniq = 0;

int dp[H][N];
int dpa[H];
int dpb[H];
int dpacc[H];


vector<int> pred[N];

void reg(int v) {
	if(tm.count(v)) {
		return;
	}
	tm.insert(v);
}


void rec(int x) {
	if(dpacc[x] >= 0) return;
		int mm = 0;
		REP(c,1,n+1) {
			int mmc = c == 1 ? h[0] : -1;
			REP(q,0,pred[x].size()) {
				int pr = pred[x][q];
				rec(pr);
				if(m[x] == m[pr] && c >= 2 && dp[pr][c-1] >= 0)
					mmc = max(mmc, dp[pr][c - 1] + h[c-1]);
				if( c == 1)
					mmc = max(mmc, dpacc[pr] + h[0]);
			}
			mm = max(mm,mmc);
			dp[x][c] = mmc;
			if(DEBUG) cout << "dp[" << x << "][" << c << "]=" << mmc << endl;
		}
		dpacc[x] = mm;
		if(DEBUG) cout << "dpacc[" << x << "]=" << mm << endl;

}

int main(void){
	cin >> n;
	REP(i,0,n) cin>>h[i];
	REP(i,0,n) {
		cin >> m[i] >> s[i] >> e[i];
	}
	REP(i,0,n) {
		REP(j,0,n) {
			if(e[j] <= s[i])
				pred[i].push_back(j);
		}
	}
	if(DEBUG) {
		REP(i,0,n) {
			cout << "pred[" << i << "]=";
			REP(j,0,pred[i].size()) {
				cout << pred[i][j] << " ";
			}
			cout << endl;
		}
	}
	REP(x,0,n) dpacc[x] = -1;
	REP(x,0,n) {
		rec(x);
	}
	int mm = 0;
	REP(i,0,n) {
		mm = max(mm, dpacc[i]);
	}
	cout << mm << endl;
}
