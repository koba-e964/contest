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
const ll mod = 1e9 + 7;

const int N = 243700;
const int BIAS = N / 2;
vector<PI> level[N];
vector<PI> dest[N];

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n, w, h;
  cin >> n >> w >> h;
  REP(i, 0, n) {
    int g, p, t;
    cin >> g >> p >> t;
    int idx = p - t + BIAS;
    level[idx].push_back(PI(g == 1 ? p : -p, i));
    dest[idx].push_back(g == 1 ? PI(p, -h) : PI(w, -p)); // > on 2nd elem
  }
  REP(i, 0, N) {
    sort(level[i].begin(), level[i].end());
    sort(dest[i].begin(), dest[i].end());
  }
  vector<PI> res(n);
  REP(i, 0, N) {
    REP(j, 0, dest[i].size()) {
      int idx = level[i][j].second;
      PI d = dest[i][j];
      res[idx] = d;
    }
  }
  REP(i, 0, n) {
    cout << res[i].first << " " << -res[i].second << "\n";
  }
}
