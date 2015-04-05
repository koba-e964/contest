#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const ll MOD = 1e9+9;

struct BIT {
	vector<ll> ary;
	BIT(int n) {
		while( n & (n-1) ) {
			n += n & (-n);
		}
		ary.resize(n, 0);
	}
	ll sum(int x) { // [1..x]
		ll s = 0;
		while(x > 0) {
			s += ary[x];
			s %= MOD;
			x &= x-1;
		}
		return s;
	}
	void add(int x, ll val) {
		while(x <= ary.size()-1) {
			ary[x] += val;
			ary[x] %= MOD;
			x += x & (-x);
		}
	}
	ll sum(int x, int y) { // [x..y]
		if(x <= 0) {
			return sum(y);
		}
		ll s = sum(y) - sum(x) + MOD;
		return s % MOD;
	}
};

const int N = 1<<19;

BIT p(N);
BIT q(N);

int n,m;

int main(void){
	scanf("%d%d", &n, &m);
	REP(i,0,n) {
		int v;
		scanf("%d", &v);
		p.add(i+1,v);
	}
	REP(i,0,m) {
		
	}
}
