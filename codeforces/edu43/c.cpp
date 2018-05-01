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
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<PI, int> PPII;



int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  vector<PPII> pool;
  int n;
  cin >> n;
  REP(i, 0, n) {
    int l, r;
    cin >> l >> r;
    pool.push_back(PPII(PI(l, -r), i + 1));
  }
  sort(pool.begin(), pool.end());
  REP(i, 0, n - 1) {
    if (pool[i].first.second <= pool[i + 1].first.second) {
      // bingo!
      cout << pool[i + 1].second << " " << pool[i].second << endl;
      return 0;
    }
  }
  cout << "-1 -1\n";
}
