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

struct node {
  int sz;
  node *ch[2];
  node() : sz(0) {
    ch[0] = nullptr;
    ch[1] = nullptr;
  }
};

const int B = 30;
node root;

void add(node *v, ll x, int pos) {
  if (pos >= 0) {
    int idx = (x >> pos) & 1;
    if (v->ch[idx] == nullptr) {
      v->ch[idx] = new node;
    }
    add(v->ch[idx], x, pos - 1);
  }
  v->sz += 1;
}

void add(ll x) {
  add(&root, x, B - 1);
}

ll query(const node *v, ll s, ll k, int pos) {
  ll sum = 0;
  if (v == nullptr) {
    return 0;
  }
  if (pos >= 0) {
    int b = (k >> pos) & 1;
    int idx = (s >> pos) & 1;
    if (b == 1) {
      sum += query(v->ch[1 - idx], s, k, pos - 1);
    } else {
      sum += query(v->ch[idx], s, k, pos - 1);
      sum += v->ch[1 - idx] ? v->ch[1 - idx]->sz : 0;
    }
    return sum;
  }
  return v->sz;
}

// count #{x | (x ^ s) >= k }
ll query(ll s, ll k) {
  return query(&root, s, k, B - 1);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  ll k;
  cin >> n >> k;
  VL a(n);
  VL c(n + 1, 0);
  REP(i, 0, n) {
    cin >> a[i];
    c[i + 1] = c[i] ^ a[i];
  }

  ll tot = 0;
  REP(i, 0, n + 1) {
    tot += query(c[i], k);
    add(c[i]);
  }
  cout << tot << "\n";
}
