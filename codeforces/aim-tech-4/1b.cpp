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
typedef pair<ll, int> PLI;

#ifdef DEBUG
const int N = 51247;
ll ary[N];
int nxt[N];
#endif


int getcount = 0;
PLI get(int idx) {
  getcount += 1;
  if (getcount > 1999) {
    assert(0);
  }
  ll value;
  int next;
#ifdef DEBUG
  return PLI(ary[idx], nxt[idx]);
#else
  cout << "? " << idx + 1 << endl;
  cin >> value >> next;
  next--;
  return PLI(value, next);
#endif
}

int main(void) {
  int n, start;
  ll x;
  cin >> n >> start >> x;
  start--;
#ifdef DEBUG
  REP(i, 0, n) {
    cin >> ary[i];
  }
  REP(i, 0, n) {
    cin >> nxt[i];
  }
#endif
  mt19937 rand(0xe869120);
  const int FIRST = 500;
  vector<PLI> pool;
  pool.push_back(PLI(get(start).first, start));
  REP(etinterrapax, 0, FIRST) {
    int idx = rand() % n;
    PLI res = get(idx);
    pool.push_back(PLI(res.first, idx));
  }
  sort(pool.begin(), pool.end());
  int st = lower_bound(pool.begin(), pool.end(), PLI(x, -1e8)) - pool.begin() - 1;
  if (st == -1) {
    cout << "! " << pool[0].first << endl;
    return 0;
  }
  int idx = pool[st].second;
  while (true) {
    if (idx < 0) {
      break;
    }
    PLI res = get(idx);
    if (res.first >= x) {
      cout << "! " << res.first << endl;
      return 0;
    }
    idx = res.second;
  }
  cout << "! -1" << endl;
}
