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
typedef pair<ll, ll> PL;

const ll B = 1e9;

const int LOCAL = 0;

ll cx, cy, cr;

// CENTER: 0, HIT: 1, MISS: 2
int ask(ll x, ll y) {
  if (LOCAL) {
    x -= cx;
    y -= cy;
    ll dist = x * x + y * y;
    if (dist == 0) {
      return 0;
    }
    if (dist <= cr * cr) {
      return 1;
    }
    return 2;
  }
  cout << x << " " << y << endl;
  string ans;
  cin >> ans;
  if (ans == "CENTER") {
    return 0;
  }
  if (ans == "HIT") {
    return 1;
  }
  assert (ans == "MISS");
  return 2;
}

void one_query(void) {
  vector<PL> cand;
  ll fx = 0, fy = 0;
  cand.push_back(PL(0, 0));
  cand.push_back(PL(B / 2, -B / 2));
  cand.push_back(PL(B / 2, B / 2));
  cand.push_back(PL(-B / 2, -B / 2));
  cand.push_back(PL(-B / 2, B / 2));
  for (auto p: cand) {
    int res = ask(p.first, p.second);
    if (res == 0) {
      return;
    }
    if (res == 1) {
      fx = p.first;
      fy = p.second;
      break;
    }
  }
  ll fail = B + 1;
  ll pass = fy;
  while (fail - pass > 1) {
    ll mid = pass + (fail - pass) / 2;
    int res = ask(fx, mid);
    if (res == 0) {
      return;
    }
    if (res == 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  ll uy = pass;
  fail = -B - 1;
  pass = fy;
  while (pass - fail > 1) {
    ll mid = fail + (pass - fail) / 2;
    int res = ask(fx, mid);
    if (res == 0) return;
    if (res == 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  ll ly = pass;
  ll my = (ly + uy) / 2;
  pass = fx;
  fail = -B - 1;
  while (pass - fail > 1) {
    ll mid = fail + (pass - fail) / 2;
    int res = ask(mid, my);
    if (res == 0) return;
    if (res == 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  ll lx = pass;
  pass = fx;
  fail = B + 1;
  while (fail - pass > 1) {
    ll mid = pass + (fail - pass) / 2;
    int res = ask(mid, my);
    if (res == 0) return;
    if (res == 1) {
      pass = mid;
    } else {
      fail = mid;
    }
  }
  ll ux = pass;
  DEBUGP(ux);
  DEBUGP(lx);
  int res = ask((lx + ux) / 2, my);
  assert (res == 0);
}

int main(void) {
  int t;
  ll a, b;
  cin >> t >> a >> b;
  while (t--) {
    one_query();
  }
}
