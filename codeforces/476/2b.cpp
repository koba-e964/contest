#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 110;
string s[N];

int tbl[N][N];

int main(void) {
  int n, k;
  cin >> n >> k;
  REP(i, 0, n) cin >> s[i];
  REP(i, 0, n) {
    REP(j, 0, n - k + 1) {
      bool ok = true;
      REP(l, 0, k) {
	ok &= s[i][j + l] == '.';
      }
      if (ok) {
	REP(l, 0, k) tbl[i][j + l]++;
      }
    }
  }
  REP(i, 0, n - k + 1) {
    REP(j, 0, n) {
      bool ok = true;
      REP(l, 0, k) {
	ok &= s[i + l][j] == '.';
      }
      if (ok) {
	REP(l, 0, k) tbl[i + l][j]++;
      }
    }
  }
  pair<int, PI> ma(-1, PI(-1, -1));
  REP(i, 0, n) {
    REP(j, 0, n) {
      ma = max(ma, make_pair(tbl[i][j], PI(i, j)));
    }
  }
  cout << ma.second.first + 1 << " " << ma.second.second + 1 << endl;
}
