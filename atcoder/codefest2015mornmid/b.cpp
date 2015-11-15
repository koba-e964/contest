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

const int N = 110;
int dp[N][N];

int lcs(const string &a, const string  &b) {
    int n = a.length();
    int m = b.length();
    REP(i, 0, n + 1) {
        REP(j, 0, m + 1) {
            dp[i][j] = 0;
        }
    }
    REP(i, 0, n) {
        REP(j, 0, m) {
            int match = a[i] == b[j] ? 1 : 0;
            int ret = max(dp[i][j] + match, dp[i][j + 1]);
            ret = max(ret, dp[i + 1][j]);
            dp[i + 1][j + 1] = ret;
        }
    }
    return dp[n][m];
}


int main(void){
    int  n;
    string s;
    cin >> n >> s;
    int ma = 0;
    REP(i, 1, n) {
        ma = max(ma, lcs(s.substr(0, i), s.substr(i)));
    }
    cout << n - 2 * ma << endl;
}
