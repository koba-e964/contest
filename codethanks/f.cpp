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

const int N = 100;


int n,m;
int a[N], b[N];

vector<int> target[N];
int qq[N];

void rec(int v) {
	if (qq[v]) return;
	REP(i,0, target[v].size()) {
		int t = target[v][i];
		rec(t);
	}
	qq[v] = 1;
}


int main(void){
	cin >> n >> m;
	REP(i,0,N) {
		qq[i] = 0;
	}
	REP(i,0,m) {
		cin >> a[i] >> b[i];
		target[b[i]].push_back(a[i]);
	}
	rec(1);
	int cnt = 0;
	REP(i,1,n+1) {
		cnt += qq[i];
	}
	cout << cnt << endl;
}
