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

#define ASK 1

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;

int n;

#if ASK
void init(int) {}
int ask(const set<int> &a){
  assert ((int) a.size() == n);
  cout << "?";
  for (int b: a) {
    cout << " " << b + 1;
  }
  cout << endl;
  string t;
  cin >> t;
  if (t == "Red") return 1;
  if (t == "Blue") return -1;
  exit(1);
}
#else
int tbl[200];
void init(int n){
  string s;
  cin >> s;
  REP(i, 0, 2 * n) {
    tbl[i] = s[i] == 'R' ? 1 : -1;
  }
}
int ask(const set<int> &a){
  assert ((int) a.size() == n);
  int s = 0;
  for (int b: a) {
    s += tbl[b];
  }
  if (s > 0) return 1;
  if (s < 0) return -1;
  assert (0);
}
#endif

int ask_range(int a) {
  set<int> q;
  REP(i, 0, n) q.insert((a + i) % (2 * n));
  return ask(q);
}

int main(void) {
  cin >> n;
  init(n);
  int f0 = ask_range(0);
  int lo = 0;
  int hi = n;
  while (hi - lo > 1) {
    int mid = (hi + lo) / 2;
    if (ask_range(mid) == f0) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  // lo is f0, lo + n is -f0.
  VI ans(2 * n);
  ans[lo] = f0;
  ans[lo + n] = -f0;
  REP(i, 0, n - 1) {
    int x = (lo + n + 1 + i) % (2 * n);
    set<int> a;
    // [lo + 1, lo + n - 1]
    REP(j, 0, n - 1) a.insert((lo + 1 + j) % (2 * n));
    a.insert(x);
    ans[x] = ask(a);
  }
  REP(i, 0, n - 1) {
    set<int> a;
    int x = (lo + 1 + i) % (2 * n);
    // [lo + n + 1, lo + 2 * n - 1]
    REP(j, 0, n - 1) a.insert((lo + n + 1 + j) % (2 * n));
    a.insert(x);
    ans[x] = ask(a);
  }
  cout << "! ";
  REP(i, 0, 2 * n) {
    cout << (ans[i] == 1 ? 'R' : 'B');
  }
  cout << endl;
}
