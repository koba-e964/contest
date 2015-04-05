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

ll f(ll val) {
	ll s = 0;
	while(val > 0) {
		s += val % 10;
		val /= 10;
	}
	return s;
}

ll sum(ll val) {
	if(val <= 0) {
		return 0;
	}
	ll sub = sum(val / 10 - 1) * 10 + val / 10 * 45;
	sub += f(val/10) * (val % 10 + 1);
	REP(i,0, val % 10 + 1) {
		sub += i;
	}
	return sub;
}

const int SIZE = 100000;

ll cache[SIZE];

ll bin_search(ll val) {
	ll l = 1;
	ll r = val + 1;
	while(r - l >= 2) {
		ll m = (l + r) / 2;
		if (sum(m) < val) {
			l = m;
		} else
			r = m;
	}
	return l;
}

int main(void){
	cin>>n;
	ll x = 1;
	REP(i,0,SIZE) {
		cache[i] = -1;
	}
	cache[0] = 0;
	while(1) {
		ll val = sum(x);
		if( val % n >= SIZE) {
			ll q = val / n * n + n;
			x = bin_search(q) + 1;
			continue;
		}
		val %= n;
		if(cache[val] < 0) {
			cache[val] = x;
		} else {
			cout << (cache[val] + 1) << " " << x << endl;
			return 0;
		}
		++x;
	}
}
