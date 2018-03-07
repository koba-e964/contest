#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
    ll d, l;
    int n;
    cin >> d >> l >> n;
    VL g(n + 2);
    REP(i, 0, n) {
        cin >> g[i + 1];
    }
    g[0] = 0;
    g[n + 1] = d;
    sort(g.begin(), g.end());
    ll tot = 0;
    REP(i, 0, n + 1) {
        ll diff = max(0LL, g[i + 1] - g[i] - l);
        tot += diff * diff;
    }
    printf("%.15f\n", (double) tot / (double) d / 4);
}
