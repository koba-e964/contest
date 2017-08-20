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
  int n;
  cin >> n;
  VL a(n);
  map<ll, int> freq;
  REP(i, 0, n) {
    cin >> a[i];
    freq[a[i]] += 1;
  }
  VL pool;
  for (auto t: freq) {
    REP(i, 0, t.second / 2) {
      pool.push_back(t.first);
    }
  }
  sort(pool.rbegin(), pool.rend());
  if (pool.size() < 2) {
    cout << 0 << endl;
  } else {
    cout << pool[0] * pool[1] << endl;
  }
}
