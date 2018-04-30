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
const ll mod = 1e9 + 7;

struct node {
  node *ch[2];
  int sz;
  node(): sz(0) {
    REP(i, 0, 2) ch[i] = 0;
  }
  void add(ll val, int pos, int cnt) {
    this->sz += cnt;
    if (pos < 0) return;
    int idx = (val >> pos) & 1;
    if (ch[idx] == 0) ch[idx] = new node();
    ch[idx]->add(val, pos - 1, cnt);
  }
  ll rec(ll bias, ll lim, int pos) {
    if (pos < 0) return 0;
    if (this->sz <= 0) return -1;
    int b = (bias >> pos) & 1;
    int l = (lim >> pos) & 1;
    if (ch[b] && ch[b]->sz > 0) {
      return ch[b]->rec(bias, lim, /* TODO */ pos - 1);
    }
    return ch[1 - b]->rec(bias, lim, pos - 1) ^ (1 << pos);
  }
} root;

void fail(void) {
  cout << "No\n";
  exit(0);
}

void output(const VL &ans) {
  cout << "Yes\n";
  REP(i, 0, ans.size()) cout << ans[i] << (i == (int) ans.size() - 1 ? "\n" : " ");
  exit(0);
}

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL b(n);
  REP(i, 0, n) cin >> b[i];
  multiset<ll> bank;
  REP(i, 0, n) bank.insert(b[i]);
  ll sum = 0;
  VL ans;
  REP(i, 0, n) {
    ll found = -1;
    REP(j, 0, 60) {
      if (sum & 1LL << j) continue;
      ll lo = 1LL << j;
      ll hi = 2 * lo;
      auto en = bank.upper_bound(hi);
      auto st = bank.lower_bound(lo);
      if(st==en) continue;
      found = *st;
      bank.erase(st);
      break;
    }
    if (found == -1) fail();
    ans.push_back(found);
    //cerr << i << " " << sum << endl;
    sum ^= found;
  }
  output(ans);
}
