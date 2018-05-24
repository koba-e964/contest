#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <set>
#include <random>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;

#define MOCK 0

const int N = 200100;
ll hsh[2][26][N];

const ll mods[2] = {1000000007, 1000000009};

ll red(ll x, int kind) {
  return kind == 1 ? x % mods[1] : x % mods[0];
}

ll base;
ll pw[2][N];
void init(void) {
  mt19937 mt;
  base = mt() & 0x7ffffff;
  REP(j, 0, 2) {
    pw[j][0] = 1;
    REP(i, 1, N) {
      pw[j][i] = red(pw[j][i - 1] * base, j);
    }
  }
}

typedef VL hash_type;

hash_type get(int x, int len) {
  hash_type ret(26);
  REP(c, 0, 26) {
    ll res[2] = {0, 0};
    REP(i, 0, 1) {
      ll *row = hsh[i][c];
      res[i] = row[x + len] + mods[i] - red(pw[i][len] * row[x], i);
      if (res[i] >= mods[i]) res[i] -= mods[i];
      ret[c] = res[i];
    }
  }
  sort(ret.begin(), ret.end());
  return ret;
}

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  init();
  int n, m;
  cin >> n >> m;
  string s;
#if !MOCK
  cin >> s;
#else
  s = string(n, 'a');
#endif
  REP(j, 0, 2) {
    REP(c, 0, 26) {
      ll* row = hsh[j][c];
      row[0] = 0;
      REP(i, 0, n) {
	ll let = 'a' + c == s[i] ? 1 : 0;
	row[i + 1] = red(row[i] * base + let, j);
      }
    }
  }
  REP(_, 0, m) {
    int x, y, len;
#if MOCK
    x = 0, y = 1, len = n - 3;
#else
    cin >> x >> y >> len;
    x--, y--;
#endif
    hash_type xhash = get(x, len), yhash = get(y, len);
    cout << (xhash == yhash ? "YES" : "NO") << endl;
  }
}
