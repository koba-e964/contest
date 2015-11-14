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

const int N = 300;

ll dp[N][N], mul[N][N];
const ll mod = 1e9 + 7;

ll rec(VI &c, int x, int y) {
    if (dp[x][y] >= 0) {
        return dp[x][y];
    }
    ll &ret = dp[x][y];
    ll tot = 0;
    if (x == y) {
        return ret = 1;
    }
    REP(i, x + 2, y + 1) {
        if (c[x + 1] < c[i]) {
            tot += rec(c, x + 1, i - 1) * rec(c, i - 1, y) % mod;
            tot %= mod;
        }
    }
    tot += rec(c, x + 1, y);
    tot %= mod;
    return ret = tot;
}

int main(void){
    int n;
    cin >> n;
    VI c(n);
    REP(i, 0, n) {
        cin >> c[i];
    }
    if (c[0] != 1) {
        cout << 0 << endl;
        return 0;
    }
    REP(i, 0, n + 1) {
        REP(j, 0, n + 1) {
            dp[i][j] = -1;
        }
    }
    cout << rec(c, 0, n - 1) << endl;
}
