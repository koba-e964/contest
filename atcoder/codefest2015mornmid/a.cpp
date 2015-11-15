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

ll s[101];
ll n,k,m,r;


bool ok(ll sc) {
    vector<ll> t(n);
    REP(i, 0, n - 1) t[i] = s[i];
    t[n - 1] = sc;
    sort(t.begin(), t.end());
    ll tot = 0;
    REP(i, 0, k) {
        tot += t[n - 1 - i];
    }
    return tot >= k * r;
}

int main(void){
    cin >> n >> k >> m >> r;
    REP(i, 0, n - 1) {
        cin >> s[i];
    }
    ll lo = -1;
    ll hi = m + 1;
    while (hi - lo >= 2) {
        ll mid = (hi + lo) / 2;
        if (ok(mid)) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    cout << (hi == m + 1 ? -1 : hi) << endl;
}
