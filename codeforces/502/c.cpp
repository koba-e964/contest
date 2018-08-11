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
typedef pair<int, PI> PIPI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  PIPI mi(1e8, PI(-1, -1));
  REP(a, 1, n + 1) {
    int b = (n + a - 1) / a;
    mi = min(mi, PIPI(a + b, PI(a, b)));
  }
  int a = mi.second.first;
  int b = mi.second.second;
  VI cluster(a, 1);
  int rem = n - a;
  REP(i, 0, a) {
    int c = min(rem, b - 1);
    rem -= c;
    cluster[i] += c;
  }
  VI ans(n);
  int ptr = 0;
  int bias = 0;
  REP(i, 0, a) {
    REP(j, 0, cluster[i]) {
      ans[ptr++] = cluster[i] - j + bias;
    }
    bias += cluster[i];
  }
  REP(i, 0, n) cout << ans[i] << (i == n - 1 ? "\n" : " ");
}
