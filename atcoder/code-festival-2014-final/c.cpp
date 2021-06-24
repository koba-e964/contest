#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

ll f(ll n) {
	int d[5];
	ll c = n;
	REP(i, 0, 5) {
		d[i] = c % 10;
		c /= 10;
	}
	assert (c == 0);
	ll t = 0;
	c = 1;
	REP(i, 0, 5) {
		t += c * d[i];
		c *= n;
	}
	return t;
}

int main(void){
	ll n;
	cin >> n;
	for (ll i = 10; i <= 10000; ++i) {
		if (n == f(i)) {
			cout << i << endl;
			return 0;
		}
	}
	cout << -1 << endl;
}
