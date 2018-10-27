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

#define MOCK 0

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 300;
vector<VI> trans(const vector<VI> &a) {
  vector<VI> ret(N, VI(N));
  REP(i, 0, N) REP(j, 0, N) ret[i][j] = a[j][i];
  return ret;
}

ll calc(const vector<VI> &sig) {
  ll tot = 0;
  vector<VI> acc(N, VI(N + 1, 0));
  REP(i, 0, N) {
    REP(j, 0, N) {
      acc[i][j + 1] = acc[i][j] + sig[i][j];
    }
  }
  REP(i, 0, N) {
    REP(j, 0, N) {
      if (sig[i][j] == 0) continue;
      REP(k, j + 1, N) {
        if (sig[i][k] == 0) continue;
        int dif = k - j;
        if (i - dif >= 0) {
          int idx = i - dif;
          tot += 2 * (acc[idx][k + 1] - acc[idx][j]);
          if (sig[i - dif][j] == 1) tot--;
          if (sig[i - dif][k] == 1) tot--;
        }
        if (i + dif < N) {
          int idx = i + dif;
          tot += 2 * (acc[idx][k + 1] - acc[idx][j]);
          if (sig[i + dif][j] == 1) tot--;
          if (sig[i + dif][k] == 1) tot--;
        }
      }
    }
  }
  return tot;
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<string> s(h);
#if MOCK
  REP(i, 0, h) s[i] = string(w, '#');
#else
  REP(i, 0, h) cin >> s[i];
#endif
  ll tot = 0;
  REP(par, 0, 2) {
    vector<VI> sig(N, VI(N, 0));
    REP(i, 0, h) {
      REP(j, 0, w) {
        if (s[i][j] == '.') continue;
        if ((i + j) % 2 == par) {
          int x = (i + j - par) / 2;
          int y = (i - j - par + w) / 2;
          sig[x][y] = 1;
        }
      }
    }
    tot += calc(sig);
    sig = trans(sig);
    tot += calc(sig);
  }
  cout << tot / 2 << endl;
}
