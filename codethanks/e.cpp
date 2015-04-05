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
const int N=10000;
const int W = 53;

int r,c,m,n;

int ra[N], rb[N], ca[N], cb[N];

int ss[W][W];

void update(int x, int y, int z , int w, int d){ 
	REP(i,x,y+1) {
		REP(j,z,w+1) {
			ss[i][j] = (ss[i][j] + d + 4) % 4;
		}
	}
}

int cc() {
	int cnt = 0;
	REP(i,1,r+1) {
		REP(j,1,c + 1) {
			cnt += ss[i][j] == 0 ? 1: 0;
		}
	}
	return cnt;
}

int main(void){
	cin >> r >> c >> m>>n;
	REP(i,0,W) {
		REP(j,0,W) {
			ss[i][j] = 0;
		}
	}
	REP(i,0,n) {
		cin >> ra[i] >> rb[i] >> ca[i] >> cb[i];
		update(ra[i], rb[i], ca[i], cb[i], 1);
	}
	REP(i,0,n) {
		update(ra[i], rb[i], ca[i], cb[i], -1);
		if (cc() == m) {
			cout << i + 1 << endl;
		}
		update(ra[i], rb[i], ca[i], cb[i], 1);
	}
	
}
