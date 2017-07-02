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
typedef pair<PI, int> PPII;

const int N = 243700;

const ll inf = 1e15;

int main(void){
  int n, x;
  cin >> n >> x;
  vector<PPII> pool;
  REP(i, 0, n) {
    int l, r;
    ll c;
    cin >> l >> r >> c;
    pool.push_back(PPII(PI(l, r), c));
  }
  sort(pool.begin(), pool.end());
  vector<VI> checkpoint(N);
  REP(i, 0, n) {
    checkpoint[pool[i].first.second + 1].push_back(i);
  }
  vector<PPII> pool2(pool);
  REP(i, 0, N) {
    pool2.push_back(PPII(PI(i, -1), -1)); // check!
  }
  sort(pool2.rbegin(), pool2.rend());
  ll mi = inf;
  VL tbl(N, inf);
  REP(i, 0, pool2.size()) {
    PPII p = pool2[i];
    if (p.second >= 0) {
      int dur = p.first.second - p.first.first + 1;
      ll c = p.second;
      tbl[dur] = min(tbl[dur], c);
    } else { // check!
      int l = p.first.first;
      REP(j, 0, checkpoint[l].size()) {
	int idx = checkpoint[l][j];
	PPII p = pool[idx];
	ll comp = x - (p.first.second - p.first.first + 1);
	if (comp >= 0) {
	  mi = min(mi, tbl[comp] + p.second);
	}
      }
    }
  }
  cout << (mi == inf ? -1 : mi) << endl;
}
