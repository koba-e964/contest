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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;


double solve(int n, int m, const vector<double> &len, int i) {
  double tot = 0;
  REP(j, i, i + m) {
    double l1 = j >= n ? 0 : len[j];
    double l2 = j + m >= n ? 0 : len[j + m];
    tot += max(l1, l2);
  }
  vector<double> former(len.begin(), len.begin() + i);
  vector<double> latter(len.begin() + min(i + 2 * m, n), len.end());
  sort(former.rbegin(), former.rend());
  sort(latter.rbegin(), latter.rend());
  REP(j, 0, max(former.size(), latter.size())) {
    tot += max(j >= former.size() ? 0 : former[j], j >= latter.size() ? 0 : latter[j]);
  }
  return tot;
}

int main(void){
  int r, n, m;
  cin >> r >> n >> m;
  vector<double> len;
  REP(i, 0, n) {
    len.push_back(2 * r * sqrt(1 - pow(1 - 2.0 * i / n, 2)));
  }
  double mi = 1.0 / 0;
  REP(i, 0, n) {
    mi = min(mi, solve(n, m, len, i));
  }
  printf("%.10f\n", mi);
}
