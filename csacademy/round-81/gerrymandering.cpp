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

const int N = 100100;
int val[N], ty[N];
int acc[N];

int main(void) {
    ios::sync_with_stdio(false);
    cin.tie(0);
    int n;
    cin >> n;
    REP(i, 0, n) {
        char a, b;
        cin >> a >> b;
        val[i] = a == 'A' ? 1 : -1;
        ty[i] = b == 'L' ? 1 : 0;
    }
    REP(i, 0, n) {
        acc[i + 1] = acc[i] + val[i];
    }
    VI large;
    REP(i, 0, n) {
        if (ty[i]) large.push_back(i);
    }
    int tot = 0;
    int cur = 0;
    REP(i, 0, large.size() - 1) {
        int st = large[i] + 1, en = large[i + 1];
        // look into [st, en]
        int mi = 1e8;
        int bestmi = 1e8;
        REP(i, st, en + 1) {
            if (acc[i] >= cur) bestmi = min(bestmi, acc[i]);
            mi = min(mi, acc[i]);
        }
        if (bestmi == 1e8) {
            cur = mi;
        } else {
            cur = bestmi;
            tot++;
        }
    }
    if (cur <= acc[n]) tot++;
    cout << tot << endl;
}
