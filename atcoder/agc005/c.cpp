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

void fail(void) {
  cout << "Impossible\n";
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI a(n);
  int d = 0; // diameter
  REP(i, 0, n) {
    cin >> a[i];
    d = max(d, a[i]);
  }
  VI freq(d + 1, 0);
  REP(i, 0, n) {
    freq[a[i]] += 1;
  }
  if (d % 2 == 0) {
    int k = d / 2;
    REP(i, k + 1, d + 1) {
      if (freq[i] < 2) { fail(); }
    }
    if (freq[k] != 1) { fail(); }
    cout << "Possible\n";
    return 0;
  }
  int k = d / 2;
  REP(i, k + 2, d + 1) {
    if (freq[i] < 2) { fail(); }
  }
  if (freq[k + 1] != 2) { fail(); }
  cout << "Possible\n";
}
