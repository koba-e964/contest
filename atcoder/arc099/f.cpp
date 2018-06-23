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

const int DEBUG = 1;

#define MOCK 1

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<ll, ll> PL;


ll powmod(ll a, ll e, ll mod) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}

const int A = 20;

int main(void) {
  const ll mod = 1e9 + 7;
  const ll mod2 = 1e9 + 9;
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  string s;
  cin >> n >> s;
  int p = 0;
  VI pos(n + 1);
  mt19937 mt;
  VL b(A);
  ll mods[A] = {};
  REP(i, 0, A) mods[i] = 1e9 + 7;
  
  REP(c, 0, A) {
    do {
      b[c] = mt() % mods[c];
    } while (b[c] == 0);
  }
  vector<VL> hsh(A, VL(n + 1));
  ll inv[A] = { 0 };
  REP(c, 0, A) inv[c] = powmod(b[c], mods[c] - 2, mods[c]);
  REP(c, 0, A) {
    p = 0;
    ll cur = 1;
    REP(i, 0, n) {
      ll tmp = hsh[c][i];
      if (s[i] == '>') {
        p++;
        cur = cur * b[c] % mods[c];
      }
      if (s[i] == '<') {
        p--;
        cur = cur * inv[c] % mods[c];
      }
      if (s[i] == '+') {
        tmp = (tmp + cur) % mods[c];
      }
      if (s[i] == '-') {
        tmp = (tmp + mods[c] - cur) % mods[c];
      }
      hsh[c][i + 1] = tmp;
      pos[i + 1] = p;
    }
  }
  if (DEBUG) {
    REP(c, 0, A) {
      REP(i, 0, n + 1) {
        cerr << i << " " << hsh[c][i] << endl;
      }
      cerr << endl;
    }
  }
  VL meguru(A);
  REP(c, 0, A) meguru[c] = hsh[c][n];
  map<VL, int> kirika;
  kirika[meguru] = 1;
  ll tot = 0;
  VL cur(A);
  REP(c, 0, A) cur[c] = powmod(b[c], pos[n], mods[c]);
  for (int i = n - 1; i >= 0; --i) {
    if (DEBUG) {
      DEBUGP(i);
      DEBUGP(pos[i]);
    }
    REP(c, 0, A) cur[c] = powmod(b[c], pos[i], mods[c]);
    VL ken(A);
    REP(c, 0, A) ken[c] = cur[c] * meguru[c] + hsh[c][i] % mods[c];
    tot += kirika[ken];
    VL tt(A);
    REP(c, 0, A) tt[c] = hsh[c][i];
    kirika[tt] += 1;
  }
  cout << tot << endl;
}
