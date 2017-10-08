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



int main(void) {
  int n, m;
  cin >> n;
  VL d(n);
  map<ll, int> freq;
  REP(i, 0, n) {
    cin >> d[i];
    freq[d[i]] += 1;
  }
  cin >> m;
  VL t(m);
  REP(i, 0, m) {
    cin >> t[i];
    freq[t[i]] -= 1;
  }
  bool ok = true;
  for (auto v: freq) {
    if (v.second < 0) {
      ok = false;
      break;
    }
  }
  cout << (ok ? "YES" : "NO") << endl;
}
