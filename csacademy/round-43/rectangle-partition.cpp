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

const int N = 200000;

int main(void) {
    int h, w, n, m;
    cin >> h >> w >> n >> m;
    VI y(n + 2), x(m + 2);
    REP(i, 0, n) cin >> y[i + 1];
    REP(i, 0, m) cin >> x[i + 1];
    y[0] = 0, y[n + 1] = h;
    x[0] = 0, x[m + 1] = w;
    sort(y.begin(), y.end());
    sort(x.begin(), x.end());
    VI freq_x(N), freq_y(N);
    REP(i, 0, n + 1) {
        freq_y[y[i + 1] - y[i]] += 1;
    }
    REP(i, 0, m + 1) {
        freq_x[x[i + 1] - x[i]] += 1;
    }
    ll tot = 0;
    REP(i, 0, N) {
        tot += (ll)freq_y[i] * (ll)freq_x[i];
    }
    cout << tot << endl;
}
