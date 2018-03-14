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


struct Trie {
  Trie() {
    ch[0] = NULL;
    ch[1] = NULL;
    sz = 0;
  }
  void add(int key, int pos = 29) {
    sz += 1;
    if (pos == -1) {
      return;
    }
    int b = (key >> pos) & 1;
    if (ch[b] == NULL) {
      ch[b] = new Trie();
    }
    ch[b]->add(key, pos - 1);
  }
  int get_min(int bias, int pos = 29) {
    assert (sz >= 1);
    sz--;
    if (pos == -1) {
      return 0;
    }
    int b = (bias >> pos) & 1;
    if (ch[b] != NULL && ch[b]->sz >= 1) {
      return ch[b]->get_min(bias, pos - 1) + (b << pos);
    }
    assert (ch[1 - b] != NULL);
    assert (ch[1 - b]->sz >= 1);
    return ch[1 - b]->get_min(bias, pos - 1) + ((1 - b) << pos) ;
  }
  Trie *ch[2];
  int sz;
} root;


int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  int n;
  cin >> n;
  VL a(n), p(n);
  REP(i, 0, n) cin >> a[i];
  REP(i, 0, n) cin >> p[i];
  REP(i, 0, n) {
    root.add(p[i]);
  }
  VL ans(n);
  REP(i, 0, n) {
    int k = root.get_min(a[i]);
    ans[i] = k ^ a[i];
  }
  REP(i, 0, n) {
    cout << ans[i] << (i == n - 1 ? "\n" : " ");
  }
}
