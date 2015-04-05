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
const ll mod = 1e9 + 7;

const int DEBUG = 1;

const int N = 100010;
int a[N];

ll modpow(ll x, int n) {
	ll cur = x % mod;
	ll sum = 1;
	while (n > 0) {
		if (n % 2 != 0) {
			sum *= cur;
			sum %= mod;
		}
		cur *= cur;
		cur %= mod;
		n /= 2;
	}
	return sum;
}

ll invmod(int x) {
	int n = (int)mod - 2;
	return modpow(x,n);
}

ll coef[8000];

ll fact[N];

ll mem[8000];

ll invs[N];

ll right_t[N];

int main(void){
	int n;
	ll t;
	cin >> n;
	ll cur = 1;
	REP(i, 0, N) {
		fact[i] = cur;
		cur *= i + 1;
		cur %= mod;
		if (i >= 1) {
			invs[i] = invmod(i);
		}
	}
	if (n >= 5000) {
		exit(1);
	}
	REP(i, 0, n + 1) {
		cin >> a[i];
	}
	cin >> t;
	REP(i,0,n + 1) {
		mem[i] = a[i];
	}
	cur = 1;
	right_t[0] = 1;
	REP(i, 1, n + 1) {
		cur *= (mod - t);
		cur %= mod;
		cur *= invmod(i);
		cur %= mod;
		right_t[i] = (right_t[i - 1] + cur) % mod;
		if(DEBUG) {
			cout << "right_t[" << i << "] = " << right_t[i] << endl;
		}
	}
	ll sum = a[0] * right_t[n] % mod;
	cur = 1;
	REP(i, 1, n + 1) {
		cur *= t;
		cur %= mod;
		cur *= invmod(i);
		cur %= mod;
		sum += (cur * a[i] % mod) * right_t[n - i];
		sum %= mod;
		if(DEBUG) cout << "cur:" << i << " => " << cur << endl;
	}
	cout << sum << endl;
#if 0
	// old
	for (int i = n; i >= 0; --i) {
		ll sum = 0;
		ll bicoef = 1;
		for (int j = 0; j <= i; j++) {
			sum += bicoef * mem[j];
			bicoef *= i - j;
			bicoef %= mod;
			bicoef *= invs[j + 1];
			bicoef %= mod;
			bicoef = (mod - bicoef) % mod;
			sum %= mod;
		}
		ll co = sum * invmod(fact[i]) % mod;
		co *= i % 2 == 0 ? 1 : mod - 1;
		co %= mod;
		REP(j,0,n) {
			mem[j] -= co * modpow(j, i) % mod;
			mem[j] += mod;
			mem[j] %= mod;
		}
		coef[i] = co;
		if(DEBUG) {
			cout << "coef[" << i << "] = " << co << endl;
		}
	}
	ll sum = 0;
	for (int i = n; i >= 0; --i) {
		sum += coef[i];
		if (i > 0) {
			sum *= t;
		}
		sum %= mod;
	}
	cout << sum << endl;
#endif
}
