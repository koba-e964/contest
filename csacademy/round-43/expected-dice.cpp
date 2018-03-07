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
    VI a(6), b(6);
    REP(i, 0, 6) cin >> a[i];
    REP(i, 0, 6) cin >> b[i];
    VI freq(1000);
    REP(i, 0, 6) {
        REP(j, 0, 6) {
            freq[a[i] + b[j]] += 1;
        }
    }
    int ma = 0;
    REP(i, 0, 1000) {
        ma = max(ma, freq[i]);
    }
    REP(i, 0, 1000) {
        if (ma == freq[i]) {
            cout << i << endl;
            return 0;
        }
    }
}
