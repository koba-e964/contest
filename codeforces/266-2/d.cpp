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

const int N = 2014;
const ll mod = 1000000007;
int n,h;
int a[N];

int rem[N];

ll dp[N][N]={{0}};

ll rec(int v) {
	int cur = v == 0 ? 0 : rem[v-1];
	if( v >= n) {
		return cur >= 2 ? 0 : 1;
	}
	if(abs(rem[v] - cur) >= 2) {
		return 0;
	}
	ll fact = 0;
	switch(rem[v]-cur) {
	case 1:
	case -1:
		fact = 1;
		break;
	case 0:
		fact = 2;
	}
	return (fact * rec(v+1)) % mod;
}

int main(void){
	cin>>n>>h;
	REP(i,0,n) cin>>a[i];
	REP(i,0,n) rem[i] = h - a[i];
	cout << rec(0) << endl;
}
