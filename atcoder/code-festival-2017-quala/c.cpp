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

void fail(void) {
  cout << "No\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int h, w;
  cin >> h >> w;
  vector<string> a(h);
  int freq[26] = {};
  REP(i, 0, h) {
    cin >> a[i];
    REP(j, 0, w) {
      freq[a[i][j] - 'a'] += 1;
    }
  }
  int f4 = (h / 2) * (w / 2);
  int f2 = (h / 2) * (w % 2) + (h % 2) * (w / 2);
  int f1 = (h % 2) * (w % 2);
  assert (4 * f4 + 2 * f2 + f1 == h * w);
  REP(i, 0, 26) {
    f4 -= freq[i] / 4;
    freq[i] %= 4;
    f2 -= freq[i] / 2;
    freq[i] %= 2;
    f1 -= freq[i];
  }
  // consistent?
  if (f4 > 0) {
    fail();
  }
  f2 += f4 * 2;
  if (f2 > 0) {
    fail();
  }
  cout << "Yes\n";
}
