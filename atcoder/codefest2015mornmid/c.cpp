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

const int N = 1e5;
ll a[N];

int main(void){
    int n;
    cin >> n;
    REP(i, 0, n) {
        cin >> a[i];
    }
    ll tot = 0;
    int st = 0, en = n - 1;
    while (st < en) {
        assert ((en - st)% 2 == 0);
        ll cur = 0;
        if (a[st] < a[en]) {
            cur = a[st] * 2 + a[st + 1] + 1;
            a[st + 2] += a[st] + a[st + 1] + 2;
            st += 2;
        } else {
            cur = a[en] * 2 + a[en - 1] + 1;
            a[en - 2] += a[en] + a[en - 1] + 2;
            en -= 2;
        }
        tot += cur;
    }
    cout << tot << endl;
}
