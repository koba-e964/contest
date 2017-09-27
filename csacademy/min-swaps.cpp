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

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

// Not proven thoroughly
int calc_sub(VI p) {
    int n = p.size();
    // p[0] <- n / 2, p[n - 1] <- n / 2 + 1, p[0],p[2],...: [1, n/2]
    int cnt = 0;
    if (p[0] != n / 2) {
        int idx = find(p.begin(), p.end(), n / 2) - p.begin();
        swap(p[idx], p[0]);
        cnt += 1;
    }
    if (p[n - 1] != n / 2 + 1) {
        int idx = find(p.begin(), p.end(), n / 2 + 1) - p.begin();
        swap(p[idx], p[n - 1]);
        cnt += 1;
    }
    REP(i, 0, n / 2) {
        if (p[2 * i] > n / 2) {
            cnt += 1;
        }
    }
    return cnt;
}

int calc(VI p) {
    int mi = calc_sub(p);
    reverse(p.begin(), p.end());
    mi = min(mi, calc_sub(p));
    return mi;
}

int main(void) {
    int t;
    cin >> t;
    while (t--) {
        int n;
        cin >> n;
        VI p(n);
        REP(i, 0, n) {
            cin >> p[i];
        }
        cout << calc(p) << endl;
    }
}
