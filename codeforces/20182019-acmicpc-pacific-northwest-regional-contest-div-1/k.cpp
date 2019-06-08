#include <iostream>
#include <string>
#include <iomanip>
#include <algorithm>
#include <utility>
#include <vector>
#include <cassert>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

const int N = 10;
double ma[1 << N], mi[1 << N];
double dist[50];

const double inf = 1e8;
double repr[1 << N];

int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  int given_bits;
  int dice;
  {
    string s;
    int a, b;
    cin >> s >> a >> b;
    dice = a + b;
    given_bits = 0;
    for (char c: s) {
      given_bits |= 1 << (c - '0');
    }
  }
  REP(i, 2, 13) {
    dist[i] = min(i - 1, 13 - i) / 36.0;
  }
  REP(bits, 0, 1 << N) {
    REP(i, 0, N) {
      if (bits & 1 << i) repr[bits] = 10 * repr[bits] + i;
    }
  }
  mi[0] = 0;
  REP(bits, 2, 1 << N) {
    if (bits & 1) continue;
    vector<double> cand_ma(13, -inf), cand_mi(13, inf);
    REP(sub, 1, bits + 1) {
      if ((sub & bits) != sub) continue;
      int sum = 0;
      REP(i, 0, N) if (sub & 1 << i) sum += i;
      if (sum >= 13) continue;
      cand_ma[sum] = max(cand_ma[sum], ma[bits - sub]);
      cand_mi[sum] = min(cand_mi[sum], mi[bits - sub]);
    }
    double rest = 1.0;
    double sum_ma = 0, sum_mi = 0;
    REP(i, 2, 13) {
      if (cand_ma[i] > -inf / 2) {
        rest -= dist[i];
        sum_ma += cand_ma[i] * dist[i];
        sum_mi += cand_mi[i] * dist[i];
      }
    }
    assert (rest > -1e-8);
    sum_ma += repr[bits] * rest;
    sum_mi += repr[bits] * rest;
    ma[bits] = sum_ma;
    mi[bits] = sum_mi;
  }
  pair<double, int> ans_ma(-inf, -1);
  pair<double, int> ans_mi(inf, -1);
  REP(sub, 0, given_bits + 1) {
    if ((sub & given_bits) != sub) continue;
    int sum = 0;
    REP(i, 0, N) if(sub & 1 << i) sum += i;
    if (sum == dice) {
      ans_ma = max(ans_ma, make_pair(ma[given_bits - sub], sub));
      ans_mi = min(ans_mi, make_pair(mi[given_bits - sub], sub));
    }
  }
  if (ans_ma.first < -inf / 2) {
    cout << "-1 " << fixed << setprecision(5) << repr[given_bits] << endl;
    cout << "-1 " << fixed << setprecision(5) << repr[given_bits] << endl;
  } else {
    cout << (int)repr[ans_mi.second] << fixed << setprecision(5) << " " << ans_mi.first << endl;
    (cout << (int)repr[ans_ma.second] << fixed << setprecision(5) << " ")
          << ans_ma.first << endl;
  }
}
