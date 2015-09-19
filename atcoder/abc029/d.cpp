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

ll onec(ll n) {
	ll c = 0;
	while (n > 0) {
		if (n % 10 == 1) {
			c++;
		}
		n /= 10;
	}
	return c;
}

ll onec2(ll n) {
	if (n < 0) return 0;
	ll cnt = 0;
	ll r = n % 10;
	if (r >= 1) { cnt++; }
	cnt += onec(n / 10) * (r + 1);
	cnt += onec2(n / 10 - 1) * 10;
	return cnt + n / 10;
}

int main(void){
	ll n;
	cin >> n;
	cout << onec2(n) << endl;
}
