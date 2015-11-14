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



int main(void){
    int n;
    cin >> n;
    VI s(n), t(n);
    REP(i, 0, n) {
        cin >> s[i] >> t[i];
    }
    vector<PI> tos(2*n);
    REP(i, 0, n) {
        tos[2 * i] = PI(s[i], 2);
        tos[2 * i + 1] = PI(t[i], 1);
    }
    sort(tos.begin(), tos.end());
    int ma= 0, cnt = 0;
    int ma_st = 0, ma_end = 0;
    REP(i, 0, 2 * n) {
        if (tos[i].second == 2) {
            cnt++;
            if (cnt > ma) {
                ma = cnt;
                ma_st = ma_end = tos[i].first;
            }
            if (cnt == ma) {
                ma_end = tos[i].first;
            }
        } else {
            if (cnt == ma) {
                ma_end = tos[i].first;
            }
            cnt --;
        }
    }
    int ok = 0;
    REP(i, 0, n) {
        if (ma_st >= s[i] && t[i] >= ma_end) {
            ok = 1;
        }
    }
    cout << (ma - ok) << endl;
}
