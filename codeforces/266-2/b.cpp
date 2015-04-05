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

ll n;

const ll INF = 0xdeadbeef0000LL;

vector<ll> solve(ll a,ll b) { // a<b
	ll min = INF;
	ll mini = -1;
	for(ll i=a;i*i <= n;i++) {
		ll q = max((n + i - 1) / i,b);
		if(min > i * q) {
			min = i * q;
			mini = i;
		}
	}
	if(min == INF) {
		ll ary[3] = {a*b, a, b};
		return vector<ll>(ary, ary+3);		
	}
	int ary[3] = {min, mini, min/mini};
	return vector<ll>(ary, ary+3);
}

int main(void){
	ll a,b;
	cin>>n>>a>>b;
	n *= 6;
	bool sw = a>b;
	if(sw) swap(a,b);
	vector<ll> res = solve(a,b);
	if(sw) swap(res[1], res[2]);
	cout << res[0] << endl << res[1] << " " << res[2] << endl;
}
