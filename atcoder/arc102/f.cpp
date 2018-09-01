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

const int DEBUG = 0;

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VI p(n);
  REP(i, 0, n) cin >> p[i];
  set<PI> even, odd;
  map<int, int> inv;
  REP(i, 0, n) {
    if (i % 2 == 0) even.insert(PI(i, p[i]));
    else odd.insert(PI(i, p[i]));
    inv[p[i]] = i;
  }
  int bias = 1;
  int iter = 0;
  while (even.size() > 0) {
    if (DEBUG) {
      cerr << "[" << iter << "] ";
      cerr << "even:";
      for (auto k: even) cerr << " " << k.first << "," << k.second;
      cerr << endl;
      cerr << "odd:";
      for (auto k: odd) cerr << " " << k.first << "," << k.second;
      cerr << endl;
    }
    if (even.begin()->second == bias) {
      even.erase(even.begin());
      swap(even, odd);
      bias++;
      continue;
    }
    assert (odd.size() > 0);
    if (odd.begin()->second != bias + 1) {
      cout << "No\n";
      return 0;
    }
    int zeroidx = inv[bias];
    even.erase(PI(zeroidx, bias));
    odd.erase(odd.begin());
    bias += 2;
  }
  cout << "Yes\n";
}
