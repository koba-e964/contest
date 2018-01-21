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
ll powmod(ll a, ll e) {
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


const int M = 200000;
ll fact[M], invfact[M];
void init(void) {
  fact[0] = 1;
  REP(i, 1, M) {
    fact[i] = fact[i - 1] * i % mod;
  }
  invfact[M - 1] = powmod(fact[M - 1], mod - 2);
  for (int i = M - 2; i >= 0; --i) {
    invfact[i] = invfact[i + 1] * (i + 1) % mod;
  }
}

ll comb(int x, int y) {
  if (x < 0) {
    return y == 0 || y == x ? 1 : 1;
  }
  if (y < 0 || y > x) {
    return 0;
  }
  long r = fact[x] * (invfact[x - y] * invfact[y] % mod) % mod;
  return r;
}

void add(ll &x, ll y) {
  x = (x + y) % mod;
}


int n, k;
string s;

// This function depends on the editorial
ll match(const VI &v) {
  int m = v.size();
  VI r(m, -1);
  VI b(m, -1);
  VI used(k, 0);
  int pos = 0;
  REP(i, 0, m) {
    while (pos < k && s[pos] != 'r') {
      pos++;
    }
    if (pos == k) return 0;
    r[i] = pos;
    used[pos] = 1;
    pos++;
  }
  pos = 0;
  REP(i, 0, m) {
    if (v[i] <= 1) break;
    pos = max(pos, r[i] + 1);
    while (pos < k && s[pos] != 'b') { pos++; }
    if (pos == k) return 0;
    b[i] = pos;
    used[pos] = 1;
    pos++;
  }
  pos = 0;
  REP(i, 0, m) {
    if (v[i] <= 2) break;
    int res = v[i] - 2;
    pos = max(pos, b[i] + 1);
    REP(j, 0, res) {
      while (pos < k && used[pos] == 1) { pos++; }
      if (pos == k) return 0;
      used[pos] = 1;
      pos++;
    }
  }
  ll tmp = fact[v.size()];
  map<int, int> freq;
  REP(i, 0, v.size()) {
    freq[v[i]] += 1;
  }
  for (auto p: freq) {
    tmp = tmp * invfact[p.second] % mod;
  }
  int x = n + 1;
  int y = 0;
  REP(i, 0, v.size()) {
    if (v[i] >= 2) {
      x += 2;
    }
    y += 2 * v[i];
  }
  if(DEBUG){
    DEBUGP(x);
    DEBUGP(y);
  }
  tmp = tmp * comb(x, y) % mod;
  return tmp;
}

int cnt = 0;
ll tot = 0;
void output(const VI &vi) {
  cnt++;
  if (DEBUG && n <= 10) {
    cerr << "out:";
    for (auto c: vi) cerr << " " << c;
    cerr << endl;
  }
  ll res = match(vi);
  add(tot, res);
  if (DEBUG) {
    DEBUGP(res);
    cerr<<endl;
  }
}


void calc(int rem, int ma, int maxlen, VI &v) {
  if (rem < -1) return;
  output(v);
  if ((int) v.size() >= maxlen) return;
  for (int i = ma; i >= 1; --i) {
    v.push_back(i);
    int len = i <= 1 ? 1 : 2 * i - 3;
    calc(rem - len - 1, i, maxlen, v);
    v.pop_back();
  }
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  init();
  cin >> n >> k;
  cin >> s;
  VI ret;
  int rc = 0;
  REP(i, 0, k) {
    if (s[i] == 'r') rc++;
  }
  calc(n, k, rc, ret);
  DEBUGP(cnt);
  cout << tot << endl;
}
