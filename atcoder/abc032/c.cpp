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
    ll k;
    cin >> n >> k;
    vector<ll> s(n);
    REP(i, 0, n) {
        cin >> s[i];
    }
    int has0 = 0;
    REP(i, 0, n) {
        has0 |= s[i] == 0;
    }
    if(has0) {
        cout << n << endl;
        return 0;
    }
    if (k == 0) {
        cout << 0 << endl;
        return 0;
    }
    int ma = 0;
    int p = 0;
    ll cur = 1;
    REP(i, 0, n) {
        cur *= s[i];
        while (cur > k) {
            cur /= s[p];
            p++;
            assert (p <= i + 1);
        }
        ma = max(ma ,i - p + 1);
    }
    cout << ma << endl;
}
