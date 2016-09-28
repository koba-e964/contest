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
const int DEBUG = 0;

std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    ans[p[i]] = i;
  }
  return ans;
}

const int N = 100010;
bool done[N];

bool match(int n, int m, const VI &target, const VI &op, int tol) {
  assert (0 <= tol && tol < m);
  assert (target.size() == m);
  VI inv_t = perm_inv(target);
  VI prog(m + 1);
  VI pass(m + 1);
  prog[0] = n;
  REP(i, 0, op.size()) {
    int idx = inv_t[op[i]];
    if (prog[idx] == 0 && !pass[idx]) {
      return false;
    }
    if (prog[idx] == 0) {
      continue;
    }
    prog[idx]--;
    prog[idx + 1]++;
    pass[idx] = 1;
  }
  if (DEBUG) {
    REP(i, 0, m + 1) {
      cout << "prog[" << i << "]=" << prog[i] << endl;
    }
  }
  REP(i, 0, tol) {
    if (prog[i] > 0) {
      return 0;
    }
  }
  return 1;
}

int main(void){
  int n, m, q;
  cin >> n >> m >> q;
  VI a(q);
  REP(i, 0, q) {
    cin >> a[i];
    a[i]--;
  }
  reverse(a.begin(), a.end());
  vector<bool> used(m);
  int cur = 0;
  VI target(m);
  REP(i, 0, q) {
    if (used[a[i]]) {
      continue;
    }
    used[a[i]] = 1;
    target[cur] = a[i];
    cur++;
  }
  REP(i, 0, m) {
    if (!used[i]) {
      target[cur++] = i;
    }
  }
  assert (cur == m);
  int oldcur = m - 1;
  while (oldcur > 0) {
    if (target[oldcur - 1] < target[oldcur]) {
      oldcur--;
    } else {
      break;
    }
  }
  if (DEBUG) {
    cerr << "target:";
    REP(i, 0, m) {
      cerr << " " << target[i];
    }
    cerr << endl << "tol = " << oldcur << endl;
  }
  cout << (match(n, m, target, a, oldcur) ? "Yes" : "No") << endl;
}
